//! 脆弱性スキャナー Tauri Commands

use crate::api::osv::{OsvClient, OsvPackage, OsvQueryRequest, OsvVulnerability};
use crate::db::{self, models::{ScanHistory, ScanResult, VulnFilter, VulnListResponse, VulnMatch, Vulnerability}};
use crate::scanner::{self, Dependency};
use crate::AppState;
use super::CommandError;
use std::path::Path;
use tauri::State;

impl From<crate::api::osv::OsvError> for CommandError {
    fn from(e: crate::api::osv::OsvError) -> Self {
        CommandError {
            message: e.to_string(),
        }
    }
}

impl From<crate::scanner::ScanError> for CommandError {
    fn from(e: crate::scanner::ScanError) -> Self {
        CommandError {
            message: e.to_string(),
        }
    }
}

/// 脆弱性一覧を取得（キャッシュから）
#[tauri::command]
pub async fn get_vulnerabilities(
    state: State<'_, AppState>,
    filter: VulnFilter,
    page: Option<i32>,
    limit: Option<i32>,
) -> Result<VulnListResponse, CommandError> {
    let db_path = &state.db_path;
    let conn = db::get_connection(db_path)?;
    
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(20);
    
    let response = db::vuln_queries::get_vulnerabilities(&conn, &filter, page, limit)?;
    Ok(response)
}

/// 最新の脆弱性をAPIから取得してキャッシュに保存
#[tauri::command]
pub async fn fetch_vulnerabilities(
    state: State<'_, AppState>,
    ecosystems: Vec<String>,
) -> Result<i32, CommandError> {
    let db_path = &state.db_path;
    let osv_client = OsvClient::new();
    
    let mut total_fetched = 0;
    
    // 各エコシステムの主要パッケージに対して脆弱性を取得
    // 実際の運用では、より多くのパッケージを対象にするか、
    // 別のエンドポイント（脆弱性一覧）を使用する
    
    let ecosystem_packages: Vec<(&str, Vec<&str>)> = vec![
        ("npm", vec!["lodash", "express", "axios", "react", "webpack", "typescript", "eslint", "jest", "next", "vue"]),
        ("crates.io", vec!["serde", "tokio", "reqwest", "hyper", "actix-web", "diesel", "clap", "rand", "chrono", "regex"]),
        ("PyPI", vec!["django", "flask", "requests", "numpy", "pandas", "tensorflow", "pytorch", "pillow", "scipy", "celery"]),
        ("Go", vec!["github.com/gin-gonic/gin", "github.com/gorilla/mux", "github.com/go-sql-driver/mysql", "github.com/lib/pq"]),
    ];
    
    let conn = db::get_connection(db_path)?;
    
    for (ecosystem, packages) in ecosystem_packages {
        // フィルターに含まれているエコシステムのみ処理
        if !ecosystems.is_empty() && !ecosystems.iter().any(|e| e == ecosystem) {
            continue;
        }
        
        for package in packages {
            match osv_client.query_package(ecosystem, package, None).await {
                Ok(response) => {
                    for osv_vuln in response.vulns {
                        let vuln = convert_osv_vulnerability(&osv_vuln, ecosystem, package);
                        if let Err(e) = db::vuln_queries::upsert_vulnerability(&conn, &vuln) {
                            eprintln!("Failed to save vulnerability {}: {}", vuln.id, e);
                        } else {
                            total_fetched += 1;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to query {} {}: {}", ecosystem, package, e);
                }
            }
        }
    }
    
    Ok(total_fetched)
}

/// ディレクトリをスキャンして脆弱性を検出
#[tauri::command]
pub async fn scan_directory(
    state: State<'_, AppState>,
    path: String,
) -> Result<ScanResult, CommandError> {
    let db_path = &state.db_path;
    let dir_path = Path::new(&path);
    
    if !dir_path.exists() {
        return Err(CommandError {
            message: format!("ディレクトリが存在しません: {}", path),
        });
    }
    
    if !dir_path.is_dir() {
        return Err(CommandError {
            message: format!("ディレクトリではありません: {}", path),
        });
    }
    
    // 依存関係をスキャン
    let scan_results = scanner::scan_directory(dir_path)?;
    
    let osv_client = OsvClient::new();
    let conn = db::get_connection(db_path)?;
    
    let mut all_vulnerabilities: Vec<VulnMatch> = Vec::new();
    let mut ecosystems_found: Vec<String> = Vec::new();
    let mut total_packages = 0;
    
    for scan in &scan_results {
        if !ecosystems_found.contains(&scan.ecosystem) {
            ecosystems_found.push(scan.ecosystem.clone());
        }
        
        total_packages += scan.dependencies.len() as i32;
        
        // バッチクエリを構築（最大1000件ずつ）
        let chunks: Vec<&[Dependency]> = scan.dependencies.chunks(100).collect();
        
        for chunk in chunks {
            let queries: Vec<OsvQueryRequest> = chunk
                .iter()
                .map(|dep| OsvQueryRequest {
                    package: OsvPackage {
                        name: dep.name.clone(),
                        ecosystem: dep.ecosystem.clone(),
                    },
                    version: Some(dep.version.clone()),
                })
                .collect();
            
            match osv_client.query_batch(queries).await {
                Ok(batch_response) => {
                    for (i, result) in batch_response.results.iter().enumerate() {
                        if let Some(dep) = chunk.get(i) {
                            for osv_vuln in &result.vulns {
                                let vuln = convert_osv_vulnerability(
                                    osv_vuln,
                                    &dep.ecosystem,
                                    &dep.name,
                                );
                                
                                // キャッシュに保存
                                let _ = db::vuln_queries::upsert_vulnerability(&conn, &vuln);
                                
                                all_vulnerabilities.push(VulnMatch {
                                    package_name: dep.name.clone(),
                                    installed_version: dep.version.clone(),
                                    vulnerability: vuln,
                                });
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Batch query error: {}", e);
                }
            }
        }
    }
    
    // 深刻度でソート（critical -> high -> medium -> low）
    all_vulnerabilities.sort_by(|a, b| {
        severity_order(&b.vulnerability.severity).cmp(&severity_order(&a.vulnerability.severity))
    });
    
    // スキャン履歴を保存
    for ecosystem in &ecosystems_found {
        let vuln_count = all_vulnerabilities
            .iter()
            .filter(|v| v.vulnerability.affected_ecosystem == *ecosystem)
            .count() as i32;
        let _ = db::vuln_queries::add_scan_history(&conn, &path, ecosystem, vuln_count);
    }
    
    // 現在時刻を取得
    let scanned_at = chrono_now();
    
    Ok(ScanResult {
        directory: path,
        ecosystems: ecosystems_found,
        vulnerabilities: all_vulnerabilities,
        scanned_at,
        total_packages,
    })
}

/// 脆弱性の詳細を取得
#[tauri::command]
pub async fn get_vulnerability_detail(
    state: State<'_, AppState>,
    vuln_id: String,
) -> Result<Option<Vulnerability>, CommandError> {
    let db_path = &state.db_path;
    let conn = db::get_connection(db_path)?;
    
    // まずキャッシュから取得を試みる
    if let Some(vuln) = db::vuln_queries::get_vulnerability_by_id(&conn, &vuln_id)? {
        return Ok(Some(vuln));
    }
    
    // キャッシュにない場合はAPIから取得
    let osv_client = OsvClient::new();
    match osv_client.get_vulnerability(&vuln_id).await {
        Ok(osv_vuln) => {
            // パッケージ情報を取得
            let (ecosystem, package) = if let Some(affected) = osv_vuln.affected.first() {
                if let Some(ref pkg) = affected.package {
                    (pkg.ecosystem.clone(), pkg.name.clone())
                } else {
                    ("unknown".to_string(), "unknown".to_string())
                }
            } else {
                ("unknown".to_string(), "unknown".to_string())
            };
            
            let vuln = convert_osv_vulnerability(&osv_vuln, &ecosystem, &package);
            
            // キャッシュに保存
            let _ = db::vuln_queries::upsert_vulnerability(&conn, &vuln);
            
            Ok(Some(vuln))
        }
        Err(_) => Ok(None),
    }
}

/// スキャン履歴を取得
#[tauri::command]
pub async fn get_scan_history(
    state: State<'_, AppState>,
    limit: Option<i32>,
) -> Result<Vec<ScanHistory>, CommandError> {
    let db_path = &state.db_path;
    let conn = db::get_connection(db_path)?;
    
    let limit = limit.unwrap_or(20);
    let history = db::vuln_queries::get_scan_history(&conn, limit)?;
    
    Ok(history)
}

/// 脆弱性の総数を取得
#[tauri::command]
pub async fn get_vulnerability_count(
    state: State<'_, AppState>,
    ecosystem: Option<String>,
) -> Result<i64, CommandError> {
    let db_path = &state.db_path;
    let conn = db::get_connection(db_path)?;
    
    let count = db::vuln_queries::get_vulnerability_count(&conn, ecosystem.as_deref())?;
    Ok(count)
}

// --- Helper Functions ---

/// OSV脆弱性をアプリ内モデルに変換
fn convert_osv_vulnerability(
    osv_vuln: &OsvVulnerability,
    ecosystem: &str,
    package_name: &str,
) -> Vulnerability {
    Vulnerability {
        id: osv_vuln.id.clone(),
        source: "osv".to_string(),
        severity: osv_vuln.severity_level(),
        cvss_score: osv_vuln.cvss_score(),
        title: osv_vuln.summary.clone().unwrap_or_else(|| osv_vuln.id.clone()),
        description: osv_vuln.details.clone(),
        affected_package: package_name.to_string(),
        affected_ecosystem: ecosystem.to_string(),
        affected_versions: Some(osv_vuln.affected_versions_string()),
        fixed_versions: {
            let fixed = osv_vuln.fixed_versions();
            if fixed.is_empty() {
                None
            } else {
                Some(fixed.join(", "))
            }
        },
        published_at: osv_vuln.published.clone(),
        references: osv_vuln.reference_urls(),
        fetched_at: None,
    }
}

/// 深刻度の順序（ソート用）
fn severity_order(severity: &str) -> i32 {
    match severity.to_lowercase().as_str() {
        "critical" => 4,
        "high" => 3,
        "medium" => 2,
        "low" => 1,
        _ => 0,
    }
}

/// 現在時刻をISO 8601形式で取得（簡易版）
fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // 簡易的なISO 8601変換
    let days = now / 86400;
    let secs_in_day = now % 86400;
    let hours = secs_in_day / 3600;
    let mins = (secs_in_day % 3600) / 60;
    let secs = secs_in_day % 60;
    
    // 日付計算（簡易版）
    let (year, month, day) = days_to_ymd(days);
    
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hours, mins, secs
    )
}

fn days_to_ymd(days: u64) -> (i32, u32, u32) {
    let mut remaining = days as i64;
    let mut year = 1970i32;
    
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        year += 1;
    }
    
    let leap = is_leap_year(year);
    let days_in_months: [i64; 12] = [
        31, if leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
    ];
    
    let mut month = 1u32;
    for &dim in &days_in_months {
        if remaining < dim {
            break;
        }
        remaining -= dim;
        month += 1;
    }
    
    (year, month, remaining as u32 + 1)
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
