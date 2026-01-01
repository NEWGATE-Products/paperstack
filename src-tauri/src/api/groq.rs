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

        self.chat_completion(api_key, system_prompt, &user_prompt, 256, 0.3).await
    }
    
    // ========================================================================
    // RFC Summary Methods
    // ========================================================================
    
    /// Generate RFC summary - Easy level (for elementary school students)
    pub async fn generate_rfc_summary_easy(
        &self,
        rfc_number: i32,
        title: &str,
        abstract_text: &str,
    ) -> Result<String, GroqError> {
        let api_key = self.api_key.as_ref().ok_or(GroqError::MissingApiKey)?;

        let system_prompt = r#"あなたは小学生にインターネットの仕組みを教える先生です。
以下のRFCの内容を、小学5年生でもわかるように1〜2文で説明してください。

ルール:
- 難しい言葉は使わない
- 身近な例えを使う
- 絵文字を1つ使ってもOK
- 「〜だよ」「〜なんだ」のような口調で"#;

        let user_prompt = format!(
            "RFC番号: {}\nタイトル: {}\n概要: {}",
            rfc_number, title, abstract_text
        );

        self.chat_completion(api_key, system_prompt, &user_prompt, 256, 0.5).await
    }
    
    /// Generate RFC summary - Normal level (for general audience)
    pub async fn generate_rfc_summary_normal(
        &self,
        rfc_number: i32,
        title: &str,
        abstract_text: &str,
    ) -> Result<String, GroqError> {
        let api_key = self.api_key.as_ref().ok_or(GroqError::MissingApiKey)?;

        let system_prompt = "以下のRFCの内容を、IT知識のない一般の方にもわかるように日本語で2〜3文で要約してください。";

        let user_prompt = format!(
            "RFC番号: {}\nタイトル: {}\n概要: {}",
            rfc_number, title, abstract_text
        );

        self.chat_completion(api_key, system_prompt, &user_prompt, 384, 0.3).await
    }
    
    /// Generate RFC summary - Technical level (for engineers)
    pub async fn generate_rfc_summary_technical(
        &self,
        rfc_number: i32,
        title: &str,
        abstract_text: &str,
    ) -> Result<String, GroqError> {
        let api_key = self.api_key.as_ref().ok_or(GroqError::MissingApiKey)?;

        let system_prompt = "以下のRFCの内容を、ソフトウェアエンジニア向けに技術的なポイントを含めて日本語で3〜4文で要約してください。";

        let user_prompt = format!(
            "RFC番号: {}\nタイトル: {}\n概要: {}",
            rfc_number, title, abstract_text
        );

        self.chat_completion(api_key, system_prompt, &user_prompt, 512, 0.3).await
    }
    
    /// Generate RFC implementation guide
    pub async fn generate_rfc_implementation_guide(
        &self,
        rfc_number: i32,
        title: &str,
        abstract_text: &str,
    ) -> Result<String, GroqError> {
        let api_key = self.api_key.as_ref().ok_or(GroqError::MissingApiKey)?;

        let system_prompt = r#"あなたはシニアソフトウェアエンジニアです。
以下のRFCが定義するプロトコル/仕様を実装する場合のガイドを日本語で作成してください。

## 出力フォーマット（Markdown）:

### 実装可否
- このRFCは実装可能か（プロトコル/アルゴリズム/データ形式など）
- 実装不可の場合（情報提供のみ、運用ガイドライン等）はその旨を記載

### 実装概要（実装可能な場合）
- 主要コンポーネント/モジュール構成
- データ構造の概要

### 推奨技術スタック
- 言語: (例: Rust, Go, Python)
- ライブラリ: (例: tokio, hyper)
- 既存実装の例: (例: curl, nginx)

### 実装時の注意点
- セキュリティ考慮事項
- パフォーマンス最適化ポイント
- エッジケース・例外処理

### 参考リソース
- 関連RFC
- 公式テストスイート（あれば）
- 参考となるOSS実装"#;

        let user_prompt = format!(
            "RFC番号: {}\nタイトル: {}\n概要: {}",
            rfc_number, title, abstract_text
        );

        self.chat_completion(api_key, system_prompt, &user_prompt, 1024, 0.3).await
    }
    
    /// Translate RFC section to Japanese
    pub async fn translate_rfc_section(&self, text: &str) -> Result<String, GroqError> {
        let api_key = self.api_key.as_ref().ok_or(GroqError::MissingApiKey)?;

        let system_prompt = "以下の英語のRFC文書を日本語に翻訳してください。技術用語は適切に訳すか、原語をカタカナで表記してください。";

        self.chat_completion(api_key, system_prompt, text, 2048, 0.2).await
    }
    
    /// Translate RFC title to Japanese
    pub async fn translate_rfc_title(&self, title: &str) -> Result<String, GroqError> {
        let api_key = self.api_key.as_ref().ok_or(GroqError::MissingApiKey)?;

        let system_prompt = "以下の英語のRFCタイトルを日本語に翻訳してください。
技術用語は適切に訳すか、原語をカタカナで表記してください。
翻訳結果のみを出力し、説明や補足は不要です。";

        self.chat_completion(api_key, system_prompt, title, 256, 0.2).await
    }
    
    // ========================================================================
    // Helper Methods
    // ========================================================================
    
    async fn chat_completion(
        &self,
        api_key: &str,
        system_prompt: &str,
        user_prompt: &str,
        max_tokens: i32,
        temperature: f32,
    ) -> Result<String, GroqError> {
        let request = ChatRequest {
            model: MODEL.to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
            max_tokens,
            temperature,
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
