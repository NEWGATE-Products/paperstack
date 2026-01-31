//! 脆弱性データベースクエリ

use super::models::{ScanHistory, Vulnerability, VulnFilter, VulnListResponse};
use rusqlite::{params, Connection, Result};

/// 脆弱性を保存（upsert）
pub fn upsert_vulnerability(conn: &Connection, vuln: &Vulnerability) -> Result<()> {
    let references_json = serde_json::to_string(&vuln.references).unwrap_or_else(|_| "[]".to_string());
    
    conn.execute(
        "INSERT OR REPLACE INTO vulnerabilities 
         (id, source, severity, cvss_score, title, description, affected_package, 
          affected_ecosystem, affected_versions, fixed_versions, published_at, reference_urls, fetched_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, datetime('now'))",
        params![
            vuln.id,
            vuln.source,
            vuln.severity,
            vuln.cvss_score,
            vuln.title,
            vuln.description,
            vuln.affected_package,
            vuln.affected_ecosystem,
            vuln.affected_versions,
            vuln.fixed_versions,
            vuln.published_at,
            references_json,
        ],
    )?;
    Ok(())
}

/// 複数の脆弱性を一括保存
#[allow(dead_code)]
pub fn upsert_vulnerabilities(conn: &Connection, vulns: &[Vulnerability]) -> Result<()> {
    for vuln in vulns {
        upsert_vulnerability(conn, vuln)?;
    }
    Ok(())
}

/// 脆弱性を取得（フィルター付き、ページネーション）
pub fn get_vulnerabilities(
    conn: &Connection,
    filter: &VulnFilter,
    page: i32,
    limit: i32,
) -> Result<VulnListResponse> {
    let offset = (page - 1) * limit;
    
    // Build WHERE clause
    let mut conditions = Vec::new();
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    
    if let Some(ref ecosystem) = filter.ecosystem {
        conditions.push("affected_ecosystem = ?");
        params_vec.push(Box::new(ecosystem.clone()));
    }
    
    if let Some(ref severity) = filter.severity {
        conditions.push("severity = ?");
        params_vec.push(Box::new(severity.clone()));
    }
    
    if let Some(ref search) = filter.search {
        conditions.push("(title LIKE ? OR affected_package LIKE ? OR id LIKE ?)");
        let search_pattern = format!("%{}%", search);
        params_vec.push(Box::new(search_pattern.clone()));
        params_vec.push(Box::new(search_pattern.clone()));
        params_vec.push(Box::new(search_pattern));
    }
    
    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };
    
    // Get total count
    let count_sql = format!("SELECT COUNT(*) FROM vulnerabilities {}", where_clause);
    let total: i64 = {
        let mut stmt = conn.prepare(&count_sql)?;
        let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
        stmt.query_row(params_refs.as_slice(), |row| row.get(0))?
    };
    
    // Get vulnerabilities
    let sql = format!(
        "SELECT id, source, severity, cvss_score, title, description, affected_package, 
                affected_ecosystem, affected_versions, fixed_versions, published_at, reference_urls, fetched_at
         FROM vulnerabilities {} 
         ORDER BY published_at DESC NULLS LAST
         LIMIT ? OFFSET ?",
        where_clause
    );
    
    let mut stmt = conn.prepare(&sql)?;
    
    // Add limit and offset to params
    params_vec.push(Box::new(limit));
    params_vec.push(Box::new(offset));
    
    let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
    
    let vulns = stmt
        .query_map(params_refs.as_slice(), |row| {
            let references_json: String = row.get(11)?;
            let references: Vec<String> = serde_json::from_str(&references_json).unwrap_or_default();
            
            Ok(Vulnerability {
                id: row.get(0)?,
                source: row.get(1)?,
                severity: row.get(2)?,
                cvss_score: row.get(3)?,
                title: row.get(4)?,
                description: row.get(5)?,
                affected_package: row.get(6)?,
                affected_ecosystem: row.get(7)?,
                affected_versions: row.get(8)?,
                fixed_versions: row.get(9)?,
                published_at: row.get(10)?,
                references,
                fetched_at: row.get(12)?,
            })
        })?
        .collect::<Result<Vec<_>>>()?;
    
    Ok(VulnListResponse {
        vulnerabilities: vulns,
        total,
        page,
        limit,
    })
}

/// IDで脆弱性を取得
pub fn get_vulnerability_by_id(conn: &Connection, vuln_id: &str) -> Result<Option<Vulnerability>> {
    let mut stmt = conn.prepare(
        "SELECT id, source, severity, cvss_score, title, description, affected_package, 
                affected_ecosystem, affected_versions, fixed_versions, published_at, reference_urls, fetched_at
         FROM vulnerabilities WHERE id = ?1"
    )?;
    
    let mut vulns = stmt.query_map([vuln_id], |row| {
        let references_json: String = row.get(11)?;
        let references: Vec<String> = serde_json::from_str(&references_json).unwrap_or_default();
        
        Ok(Vulnerability {
            id: row.get(0)?,
            source: row.get(1)?,
            severity: row.get(2)?,
            cvss_score: row.get(3)?,
            title: row.get(4)?,
            description: row.get(5)?,
            affected_package: row.get(6)?,
            affected_ecosystem: row.get(7)?,
            affected_versions: row.get(8)?,
            fixed_versions: row.get(9)?,
            published_at: row.get(10)?,
            references,
            fetched_at: row.get(12)?,
        })
    })?;
    
    vulns.next().transpose()
}

/// パッケージ名とエコシステムで脆弱性を検索
#[allow(dead_code)]
pub fn find_vulnerabilities_for_package(
    conn: &Connection,
    ecosystem: &str,
    package_name: &str,
) -> Result<Vec<Vulnerability>> {
    let mut stmt = conn.prepare(
        "SELECT id, source, severity, cvss_score, title, description, affected_package, 
                affected_ecosystem, affected_versions, fixed_versions, published_at, reference_urls, fetched_at
         FROM vulnerabilities 
         WHERE affected_ecosystem = ?1 AND affected_package = ?2
         ORDER BY cvss_score DESC NULLS LAST"
    )?;
    
    let vulns = stmt
        .query_map([ecosystem, package_name], |row| {
            let references_json: String = row.get(11)?;
            let references: Vec<String> = serde_json::from_str(&references_json).unwrap_or_default();
            
            Ok(Vulnerability {
                id: row.get(0)?,
                source: row.get(1)?,
                severity: row.get(2)?,
                cvss_score: row.get(3)?,
                title: row.get(4)?,
                description: row.get(5)?,
                affected_package: row.get(6)?,
                affected_ecosystem: row.get(7)?,
                affected_versions: row.get(8)?,
                fixed_versions: row.get(9)?,
                published_at: row.get(10)?,
                references,
                fetched_at: row.get(12)?,
            })
        })?
        .collect::<Result<Vec<_>>>()?;
    
    Ok(vulns)
}

/// スキャン履歴を保存
pub fn add_scan_history(
    conn: &Connection,
    directory: &str,
    ecosystem: &str,
    vuln_count: i32,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO scan_history (directory, ecosystem, vuln_count) VALUES (?1, ?2, ?3)",
        params![directory, ecosystem, vuln_count],
    )?;
    Ok(conn.last_insert_rowid())
}

/// スキャン履歴を取得
pub fn get_scan_history(conn: &Connection, limit: i32) -> Result<Vec<ScanHistory>> {
    let mut stmt = conn.prepare(
        "SELECT id, directory, ecosystem, vuln_count, scanned_at
         FROM scan_history
         ORDER BY scanned_at DESC
         LIMIT ?1"
    )?;
    
    let history = stmt
        .query_map([limit], |row| {
            Ok(ScanHistory {
                id: row.get(0)?,
                directory: row.get(1)?,
                ecosystem: row.get(2)?,
                vuln_count: row.get(3)?,
                scanned_at: row.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>>>()?;
    
    Ok(history)
}

/// 脆弱性の総数を取得
pub fn get_vulnerability_count(conn: &Connection, ecosystem: Option<&str>) -> Result<i64> {
    if let Some(eco) = ecosystem {
        conn.query_row(
            "SELECT COUNT(*) FROM vulnerabilities WHERE affected_ecosystem = ?1",
            [eco],
            |row| row.get(0),
        )
    } else {
        conn.query_row("SELECT COUNT(*) FROM vulnerabilities", [], |row| row.get(0))
    }
}

/// 古い脆弱性を削除（キャッシュクリーンアップ用）
#[allow(dead_code)]
pub fn delete_old_vulnerabilities(conn: &Connection, days_old: i32) -> Result<usize> {
    let affected = conn.execute(
        "DELETE FROM vulnerabilities 
         WHERE fetched_at < datetime('now', ?1)",
        [format!("-{} days", days_old)],
    )?;
    Ok(affected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;
    use std::fs;
    
    fn create_test_db() -> (Connection, String) {
        let db_path = format!("/tmp/test_vuln_{}.db", std::process::id());
        let _ = fs::remove_file(&db_path);
        init_db(&db_path).unwrap();
        let conn = Connection::open(&db_path).unwrap();
        (conn, db_path)
    }
    
    fn cleanup_test_db(path: &str) {
        let _ = fs::remove_file(path);
    }
    
    #[test]
    fn test_upsert_vulnerability() {
        let (conn, path) = create_test_db();
        
        let vuln = Vulnerability {
            id: "CVE-2024-0001".to_string(),
            source: "osv".to_string(),
            severity: "high".to_string(),
            cvss_score: Some(7.5),
            title: "Test Vulnerability".to_string(),
            description: Some("A test vulnerability".to_string()),
            affected_package: "test-package".to_string(),
            affected_ecosystem: "npm".to_string(),
            affected_versions: Some(">= 1.0.0, < 2.0.0".to_string()),
            fixed_versions: Some("2.0.0".to_string()),
            published_at: Some("2024-01-01".to_string()),
            references: vec!["https://example.com".to_string()],
            fetched_at: None,
        };
        
        upsert_vulnerability(&conn, &vuln).unwrap();
        
        let retrieved = get_vulnerability_by_id(&conn, "CVE-2024-0001").unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.title, "Test Vulnerability");
        
        cleanup_test_db(&path);
    }
    
    #[test]
    fn test_get_vulnerabilities_with_filter() {
        let (conn, path) = create_test_db();
        
        // Insert test data
        let vulns = vec![
            Vulnerability {
                id: "CVE-2024-0001".to_string(),
                source: "osv".to_string(),
                severity: "high".to_string(),
                cvss_score: Some(7.5),
                title: "NPM Vuln".to_string(),
                description: None,
                affected_package: "lodash".to_string(),
                affected_ecosystem: "npm".to_string(),
                affected_versions: None,
                fixed_versions: None,
                published_at: Some("2024-01-01".to_string()),
                references: vec![],
                fetched_at: None,
            },
            Vulnerability {
                id: "CVE-2024-0002".to_string(),
                source: "osv".to_string(),
                severity: "medium".to_string(),
                cvss_score: Some(5.0),
                title: "Cargo Vuln".to_string(),
                description: None,
                affected_package: "serde".to_string(),
                affected_ecosystem: "crates.io".to_string(),
                affected_versions: None,
                fixed_versions: None,
                published_at: Some("2024-01-02".to_string()),
                references: vec![],
                fetched_at: None,
            },
        ];
        
        upsert_vulnerabilities(&conn, &vulns).unwrap();
        
        // Test filter by ecosystem
        let filter = VulnFilter {
            ecosystem: Some("npm".to_string()),
            ..Default::default()
        };
        let result = get_vulnerabilities(&conn, &filter, 1, 10).unwrap();
        assert_eq!(result.total, 1);
        assert_eq!(result.vulnerabilities[0].affected_ecosystem, "npm");
        
        cleanup_test_db(&path);
    }
}
