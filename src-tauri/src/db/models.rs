use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paper {
    pub id: String,
    pub title: String,
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

