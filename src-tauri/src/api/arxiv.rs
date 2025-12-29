//! arXiv API Client
//! Fetches papers from arXiv.org covering all CS fields

use thiserror::Error;

const ARXIV_API_URL: &str = "http://export.arxiv.org/api/query";

#[derive(Error, Debug)]
pub enum ArxivError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("API error: {0}")]
    ApiError(String),
}

/// arXiv category mapping
#[derive(Debug, Clone)]
pub struct CategoryMapping {
    pub arxiv_codes: Vec<&'static str>,
    pub category_name: &'static str,
    pub task_slug: &'static str,
}

impl CategoryMapping {
    pub fn all() -> Vec<CategoryMapping> {
        vec![
            CategoryMapping {
                arxiv_codes: vec!["cs.AI", "cs.LG", "cs.NE"],
                category_name: "AI全般",
                task_slug: "machine-learning",
            },
            CategoryMapping {
                arxiv_codes: vec!["cs.CL"],
                category_name: "LLM",
                task_slug: "language-modelling",
            },
            CategoryMapping {
                arxiv_codes: vec!["cs.SE", "cs.PL"],
                category_name: "コード生成",
                task_slug: "code-generation",
            },
            CategoryMapping {
                arxiv_codes: vec!["cs.DS", "cs.CC", "cs.DM"],
                category_name: "アルゴリズム",
                task_slug: "optimization",
            },
            CategoryMapping {
                arxiv_codes: vec!["cs.AR", "cs.DC", "cs.OS"],
                category_name: "アーキテクチャ",
                task_slug: "architecture",
            },
        ]
    }
}

/// Parsed paper from arXiv
#[derive(Debug, Clone)]
pub struct ArxivPaper {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub published: String,
    pub pdf_url: String,
    pub abs_url: String,
}

pub struct ArxivClient {
    client: reqwest::Client,
}

impl ArxivClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("AI-Paper-News/0.1.0")
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }

    /// Fetch papers by arXiv category codes
    async fn fetch_papers_by_categories(
        &self,
        arxiv_codes: &[&str],
        max_results: i32,
    ) -> Result<Vec<ArxivPaper>, ArxivError> {
        // Build search query: cat:cs.AI OR cat:cs.LG OR ...
        let query = arxiv_codes
            .iter()
            .map(|c| format!("cat:{}", c))
            .collect::<Vec<_>>()
            .join("+OR+");
        
        let url = format!(
            "{}?search_query={}&start=0&max_results={}&sortBy=submittedDate&sortOrder=descending",
            ARXIV_API_URL, query, max_results
        );
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(ArxivError::ApiError(format!(
                "API returned status: {}",
                response.status()
            )));
        }
        
        let xml_text = response.text().await?;
        Ok(self.parse_atom_feed(&xml_text))
    }

    /// Fetch all papers for our categories
    pub async fn fetch_all_categories(&self, papers_per_category: i32) -> Result<Vec<(ArxivPaper, String, String)>, ArxivError> {
        let mut all_papers = Vec::new();
        
        for mapping in CategoryMapping::all() {
            match self.fetch_papers_by_categories(&mapping.arxiv_codes, papers_per_category).await {
                Ok(papers) => {
                    for paper in papers {
                        all_papers.push((
                            paper,
                            mapping.category_name.to_string(),
                            mapping.task_slug.to_string(),
                        ));
                    }
                }
                Err(e) => {
                    eprintln!("Error fetching {} papers: {}", mapping.category_name, e);
                }
            }
            
            // Rate limiting: wait 500ms between requests to be nice to arXiv
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
        
        Ok(all_papers)
    }

    /// Parse Atom XML feed from arXiv
    fn parse_atom_feed(&self, xml: &str) -> Vec<ArxivPaper> {
        let mut papers = Vec::new();
        
        // Simple XML parsing without external crate
        // arXiv returns Atom format
        let entries: Vec<&str> = xml.split("<entry>").skip(1).collect();
        
        for entry in entries {
            if let Some(paper) = self.parse_entry(entry) {
                papers.push(paper);
            }
        }
        
        papers
    }

    fn parse_entry(&self, entry: &str) -> Option<ArxivPaper> {
        let id = self.extract_tag(entry, "id")?;
        // Extract arXiv ID from URL like http://arxiv.org/abs/2312.12345v1
        let arxiv_id = id.split('/').last()?.split('v').next()?.to_string();
        
        let title = self.extract_tag(entry, "title")?
            .replace('\n', " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        
        let summary = self.extract_tag(entry, "summary")?
            .replace('\n', " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        
        let published = self.extract_tag(entry, "published")?;
        
        // Build URLs
        let pdf_url = format!("https://arxiv.org/pdf/{}.pdf", arxiv_id);
        let abs_url = format!("https://arxiv.org/abs/{}", arxiv_id);
        
        Some(ArxivPaper {
            id: arxiv_id,
            title,
            summary,
            published,
            pdf_url,
            abs_url,
        })
    }

    fn extract_tag(&self, text: &str, tag: &str) -> Option<String> {
        let open_tag = format!("<{}>", tag);
        let close_tag = format!("</{}>", tag);
        
        let start = text.find(&open_tag)? + open_tag.len();
        let end = text.find(&close_tag)?;
        
        if start < end {
            Some(text[start..end].trim().to_string())
        } else {
            None
        }
    }
}

impl Default for ArxivClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_all_categories() {
        let client = ArxivClient::new();
        let result = client.fetch_all_categories(2).await;
        
        match result {
            Ok(papers) => {
                println!("Fetched {} total papers", papers.len());
                for (paper, cat, _) in &papers {
                    println!("[{}] {}", cat, paper.title);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
