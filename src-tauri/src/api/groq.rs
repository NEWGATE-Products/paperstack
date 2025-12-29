//! Groq API Client for generating Japanese summaries

use serde::{Deserialize, Serialize};
use thiserror::Error;

const BASE_URL: &str = "https://api.groq.com/openai/v1";
const MODEL: &str = "llama-3.3-70b-versatile";

#[derive(Error, Debug)]
pub enum GroqError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("API error: {0}")]
    ApiError(String),
    #[error("Missing API key - set GROQ_API_KEY environment variable")]
    MissingApiKey,
    #[error("No content in response")]
    NoContent,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: i32,
    temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

pub struct GroqClient {
    client: reqwest::Client,
    api_key: Option<String>,
}

impl GroqClient {
    pub fn new() -> Self {
        let api_key = std::env::var("GROQ_API_KEY").ok();
        Self {
            client: reqwest::Client::new(),
            api_key,
        }
    }

    pub fn with_api_key(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: Some(api_key),
        }
    }

    /// Generate a Japanese summary for a paper
    pub async fn generate_summary(&self, title: &str, abstract_text: &str) -> Result<String, GroqError> {
        let api_key = self.api_key.as_ref().ok_or(GroqError::MissingApiKey)?;

        let system_prompt = "あなたは学術論文の要約専門家です。与えられた論文のタイトルとアブストラクトから、日本語で1〜2文の簡潔な要約を生成してください。専門用語は適切に日本語に訳すか、カタカナで表記してください。";
        
        let user_prompt = format!(
            "以下の論文を日本語で1〜2文で要約してください。\n\nタイトル: {}\n\nアブストラクト: {}",
            title, abstract_text
        );

        let request = ChatRequest {
            model: MODEL.to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: user_prompt,
                },
            ],
            max_tokens: 256,
            temperature: 0.3,
        };

        let response = self
            .client
            .post(format!("{}/chat/completions", BASE_URL))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(GroqError::ApiError(format!(
                "API returned status {}: {}",
                status, error_text
            )));
        }

        let chat_response: ChatResponse = response.json().await?;
        
        chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or(GroqError::NoContent)
    }
}

impl Default for GroqClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_summary_without_key() {
        let client = GroqClient::new();
        // If no API key is set, this should fail with MissingApiKey
        if client.api_key.is_none() {
            let result = client.generate_summary("Test", "Test abstract").await;
            assert!(matches!(result, Err(GroqError::MissingApiKey)));
        }
    }
}
