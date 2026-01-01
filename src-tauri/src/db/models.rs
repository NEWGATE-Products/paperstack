use serde::{Deserialize, Serialize};

// ============================================================================
// Paper Models (既存)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paper {
    pub id: String,
    pub title: String,
    #[serde(rename = "titleJa")]
    pub title_ja: Option<String>,
    #[serde(rename = "abstract")]
    pub r#abstract: Option<String>,
    #[serde(rename = "summaryJa")]
    pub summary_ja: Option<String>,
    #[serde(rename = "urlPdf")]
    pub url_pdf: Option<String>,
    #[serde(rename = "urlPaper")]
    pub url_paper: Option<String>,
    pub published: Option<String>,
    #[serde(rename = "fetchedAt")]
    pub fetched_at: Option<String>,
    pub tasks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub tasks: Vec<String>,
}

impl Category {
    pub fn all_categories() -> Vec<Category> {
        vec![
            Category {
                id: "all".to_string(),
                name: "すべて".to_string(),
                tasks: vec![],
            },
            Category {
                id: "ai".to_string(),
                name: "AI全般".to_string(),
                tasks: vec!["machine-learning".to_string()],
            },
            Category {
                id: "llm".to_string(),
                name: "LLM".to_string(),
                tasks: vec!["language-modelling".to_string()],
            },
            Category {
                id: "code".to_string(),
                name: "コード生成".to_string(),
                tasks: vec!["code-generation".to_string()],
            },
            Category {
                id: "algorithm".to_string(),
                name: "アルゴリズム".to_string(),
                tasks: vec!["optimization".to_string()],
            },
            Category {
                id: "architecture".to_string(),
                name: "アーキテクチャ".to_string(),
                tasks: vec!["architecture".to_string()],
            },
        ]
    }
}

// ============================================================================
// RFC Models (新規)
// ============================================================================

/// RFC基本情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rfc {
    pub id: String,                          // "RFC9114"
    pub number: i32,                         // 9114
    pub title: String,
    #[serde(rename = "abstract")]
    pub r#abstract: Option<String>,
    pub status: String,
    #[serde(rename = "publishedDate")]
    pub published_date: Option<String>,      // "2022-06"
    pub authors: Vec<String>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    
    // AI生成コンテンツ
    #[serde(rename = "summaryEasy")]
    pub summary_easy: Option<String>,
    #[serde(rename = "summaryNormal")]
    pub summary_normal: Option<String>,
    #[serde(rename = "summaryTechnical")]
    pub summary_technical: Option<String>,
    #[serde(rename = "implementationGuide")]
    pub implementation_guide: Option<String>,
    #[serde(rename = "titleJa")]
    pub title_ja: Option<String>,
    #[serde(rename = "abstractJa")]
    pub abstract_ja: Option<String>,
    
    // UI状態
    #[serde(rename = "isBookmarked")]
    pub is_bookmarked: bool,
}

/// RFCフィルター条件
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RfcFilter {
    pub search: Option<String>,
    #[serde(rename = "rfcNumber")]
    pub rfc_number: Option<i32>,
    pub status: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    #[serde(rename = "yearFrom")]
    pub year_from: Option<i32>,
    #[serde(rename = "yearTo")]
    pub year_to: Option<i32>,
}

/// RFC一覧レスポンス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RfcListResponse {
    pub rfcs: Vec<Rfc>,
    pub total: i64,
    pub page: i32,
    pub limit: i32,
}

/// RFCブックマーク
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RfcBookmark {
    #[serde(rename = "rfcId")]
    pub rfc_id: String,
    pub memo: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

/// RFC閲覧履歴
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RfcHistory {
    #[serde(rename = "rfcId")]
    pub rfc_id: String,
    #[serde(rename = "viewedAt")]
    pub viewed_at: String,
}

/// 要約レベル
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SummaryLevel {
    Easy,
    Normal,
    Technical,
}

/// RFCカテゴリ定義
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RfcCategory {
    pub id: String,
    pub name: String,
    pub keywords: Vec<String>,
}

impl RfcCategory {
    /// 定義済みRFCカテゴリ一覧
    pub fn all_categories() -> Vec<RfcCategory> {
        vec![
            RfcCategory {
                id: "http".to_string(),
                name: "HTTP".to_string(),
                keywords: vec!["http".to_string(), "web".to_string(), "uri".to_string(), "url".to_string(), "html".to_string()],
            },
            RfcCategory {
                id: "dns".to_string(),
                name: "DNS".to_string(),
                keywords: vec!["dns".to_string(), "domain".to_string(), "resolver".to_string()],
            },
            RfcCategory {
                id: "email".to_string(),
                name: "メール".to_string(),
                keywords: vec!["smtp".to_string(), "imap".to_string(), "pop".to_string(), "email".to_string(), "mail".to_string()],
            },
            RfcCategory {
                id: "security".to_string(),
                name: "セキュリティ".to_string(),
                keywords: vec!["tls".to_string(), "ssl".to_string(), "security".to_string(), "crypto".to_string(), "certificate".to_string()],
            },
            RfcCategory {
                id: "routing".to_string(),
                name: "ルーティング".to_string(),
                keywords: vec!["bgp".to_string(), "ospf".to_string(), "routing".to_string(), "router".to_string()],
            },
            RfcCategory {
                id: "ipv6".to_string(),
                name: "IPv6".to_string(),
                keywords: vec!["ipv6".to_string(), "icmpv6".to_string()],
            },
            RfcCategory {
                id: "transport".to_string(),
                name: "TCP/UDP".to_string(),
                keywords: vec!["tcp".to_string(), "udp".to_string(), "transport".to_string(), "quic".to_string()],
            },
            RfcCategory {
                id: "other".to_string(),
                name: "その他".to_string(),
                keywords: vec![],
            },
        ]
    }

    /// キーワードからカテゴリを判定
    pub fn categorize(title: &str, keywords: &[String]) -> Vec<String> {
        let categories = Self::all_categories();
        let mut matched: Vec<String> = Vec::new();
        
        let title_lower = title.to_lowercase();
        let keywords_lower: Vec<String> = keywords.iter().map(|k| k.to_lowercase()).collect();
        
        for cat in &categories {
            if cat.id == "other" {
                continue;
            }
            
            for kw in &cat.keywords {
                if title_lower.contains(kw) || keywords_lower.iter().any(|k| k.contains(kw)) {
                    if !matched.contains(&cat.id) {
                        matched.push(cat.id.clone());
                    }
                    break;
                }
            }
        }
        
        if matched.is_empty() {
            matched.push("other".to_string());
        }
        
        matched
    }
}

