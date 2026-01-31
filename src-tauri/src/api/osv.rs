//! OSV (Open Source Vulnerabilities) API クライアント
//! https://osv.dev/docs/

use reqwest::Client;
use serde::{Deserialize, Serialize};

const OSV_API_BASE: &str = "https://api.osv.dev/v1";

/// OSV API クライアント
pub struct OsvClient {
    client: Client,
}

impl OsvClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// パッケージの脆弱性をクエリ
    pub async fn query_package(
        &self,
        ecosystem: &str,
        package_name: &str,
        version: Option<&str>,
    ) -> Result<OsvQueryResponse, OsvError> {
        let request = OsvQueryRequest {
            package: OsvPackage {
                name: package_name.to_string(),
                ecosystem: ecosystem.to_string(),
            },
            version: version.map(|v| v.to_string()),
        };

        let response = self
            .client
            .post(format!("{}/query", OSV_API_BASE))
            .json(&request)
            .send()
            .await
            .map_err(|e| OsvError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(OsvError::Api(format!(
                "OSV API returned status: {}",
                response.status()
            )));
        }

        response
            .json()
            .await
            .map_err(|e| OsvError::Parse(e.to_string()))
    }

    /// 複数パッケージの脆弱性を一括クエリ
    pub async fn query_batch(
        &self,
        queries: Vec<OsvQueryRequest>,
    ) -> Result<OsvBatchResponse, OsvError> {
        let request = OsvBatchRequest { queries };

        let response = self
            .client
            .post(format!("{}/querybatch", OSV_API_BASE))
            .json(&request)
            .send()
            .await
            .map_err(|e| OsvError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(OsvError::Api(format!(
                "OSV API returned status: {}",
                response.status()
            )));
        }

        response
            .json()
            .await
            .map_err(|e| OsvError::Parse(e.to_string()))
    }

    /// 脆弱性IDから詳細を取得
    pub async fn get_vulnerability(&self, vuln_id: &str) -> Result<OsvVulnerability, OsvError> {
        let response = self
            .client
            .get(format!("{}/vulns/{}", OSV_API_BASE, vuln_id))
            .send()
            .await
            .map_err(|e| OsvError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(OsvError::Api(format!(
                "OSV API returned status: {}",
                response.status()
            )));
        }

        response
            .json()
            .await
            .map_err(|e| OsvError::Parse(e.to_string()))
    }
}

impl Default for OsvClient {
    fn default() -> Self {
        Self::new()
    }
}

// --- Request Types ---

#[derive(Debug, Serialize)]
pub struct OsvQueryRequest {
    pub package: OsvPackage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OsvPackage {
    pub name: String,
    pub ecosystem: String,
}

#[derive(Debug, Serialize)]
struct OsvBatchRequest {
    queries: Vec<OsvQueryRequest>,
}

// --- Response Types ---

#[derive(Debug, Deserialize, Default)]
pub struct OsvQueryResponse {
    #[serde(default)]
    pub vulns: Vec<OsvVulnerability>,
}

#[derive(Debug, Deserialize)]
pub struct OsvBatchResponse {
    #[serde(default)]
    pub results: Vec<OsvQueryResponse>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OsvVulnerability {
    pub id: String,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub details: Option<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub modified: Option<String>,
    #[serde(default)]
    pub published: Option<String>,
    #[serde(default)]
    pub references: Vec<OsvReference>,
    #[serde(default)]
    pub affected: Vec<OsvAffected>,
    #[serde(default)]
    pub severity: Vec<OsvSeverity>,
    #[serde(default)]
    pub database_specific: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OsvReference {
    #[serde(rename = "type")]
    pub ref_type: Option<String>,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OsvAffected {
    pub package: Option<OsvAffectedPackage>,
    #[serde(default)]
    pub ranges: Vec<OsvRange>,
    #[serde(default)]
    pub versions: Vec<String>,
    #[serde(default)]
    pub ecosystem_specific: Option<serde_json::Value>,
    #[serde(default)]
    pub database_specific: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OsvAffectedPackage {
    pub name: String,
    pub ecosystem: String,
    #[serde(default)]
    pub purl: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OsvRange {
    #[serde(rename = "type")]
    pub range_type: String,
    #[serde(default)]
    pub events: Vec<OsvEvent>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OsvEvent {
    #[serde(default)]
    pub introduced: Option<String>,
    #[serde(default)]
    pub fixed: Option<String>,
    #[serde(default)]
    pub last_affected: Option<String>,
    #[serde(default)]
    pub limit: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OsvSeverity {
    #[serde(rename = "type")]
    pub severity_type: String,
    pub score: String,
}

// --- Error Type ---

#[derive(Debug, thiserror::Error)]
pub enum OsvError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("API error: {0}")]
    Api(String),
    #[error("Parse error: {0}")]
    Parse(String),
}

// --- Helper functions ---

impl OsvVulnerability {
    /// CVSSスコアを取得（存在する場合）
    pub fn cvss_score(&self) -> Option<f64> {
        for sev in &self.severity {
            if sev.severity_type == "CVSS_V3" || sev.severity_type == "CVSS_V2" {
                // CVSS vector stringからスコアを抽出
                // 形式: "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H" または単純なスコア
                if let Ok(score) = sev.score.parse::<f64>() {
                    return Some(score);
                }
                // ベクター文字列の場合はスコアを計算しない（複雑なため）
            }
        }
        None
    }

    /// 深刻度を文字列で取得
    pub fn severity_level(&self) -> String {
        if let Some(score) = self.cvss_score() {
            match score {
                s if s >= 9.0 => "critical".to_string(),
                s if s >= 7.0 => "high".to_string(),
                s if s >= 4.0 => "medium".to_string(),
                _ => "low".to_string(),
            }
        } else {
            // CVSSスコアがない場合はデフォルト
            "medium".to_string()
        }
    }

    /// 修正バージョンを取得
    pub fn fixed_versions(&self) -> Vec<String> {
        let mut fixed = Vec::new();
        for affected in &self.affected {
            for range in &affected.ranges {
                for event in &range.events {
                    if let Some(ref v) = event.fixed {
                        if !fixed.contains(v) {
                            fixed.push(v.clone());
                        }
                    }
                }
            }
        }
        fixed
    }

    /// 影響を受けるバージョン範囲を文字列で取得
    pub fn affected_versions_string(&self) -> String {
        let mut versions = Vec::new();
        for affected in &self.affected {
            // 明示的なバージョンリスト
            if !affected.versions.is_empty() {
                versions.extend(affected.versions.iter().cloned());
            }
            // 範囲指定
            for range in &affected.ranges {
                let mut introduced = None;
                let mut fixed = None;
                
                for event in &range.events {
                    if event.introduced.is_some() {
                        introduced = event.introduced.clone();
                    }
                    if event.fixed.is_some() {
                        fixed = event.fixed.clone();
                    }
                }
                
                if let Some(i) = introduced {
                    let range_str = if let Some(f) = fixed {
                        format!(">= {}, < {}", i, f)
                    } else {
                        format!(">= {}", i)
                    };
                    versions.push(range_str);
                }
            }
        }
        if versions.is_empty() {
            "不明".to_string()
        } else {
            versions.join("; ")
        }
    }

    /// 参照URLリストを取得
    pub fn reference_urls(&self) -> Vec<String> {
        self.references.iter().map(|r| r.url.clone()).collect()
    }

    /// CVE IDを取得（存在する場合）
    #[allow(dead_code)]
    pub fn cve_id(&self) -> Option<String> {
        // まずIDがCVE形式かチェック
        if self.id.starts_with("CVE-") {
            return Some(self.id.clone());
        }
        // aliasesからCVEを探す
        self.aliases.iter().find(|a| a.starts_with("CVE-")).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_package() {
        let client = OsvClient::new();
        // lodash has known vulnerabilities
        let result = client.query_package("npm", "lodash", Some("4.17.20")).await;
        assert!(result.is_ok());
    }
}
