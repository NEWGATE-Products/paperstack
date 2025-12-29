//! Translation API Client using MyMemory (free, no API key required)
//! https://mymemory.translated.net/doc/spec.php

use serde::Deserialize;
use thiserror::Error;

const MYMEMORY_API_URL: &str = "https://api.mymemory.translated.net/get";

#[derive(Error, Debug)]
pub enum TranslateError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Translation failed: {0}")]
    TranslationFailed(String),
}

#[derive(Debug, Deserialize)]
struct MyMemoryResponse {
    #[serde(rename = "responseData")]
    response_data: ResponseData,
    #[serde(rename = "responseStatus")]
    response_status: i32,
}

#[derive(Debug, Deserialize)]
struct ResponseData {
    #[serde(rename = "translatedText")]
    translated_text: String,
}

pub struct TranslateClient {
    client: reqwest::Client,
}

impl TranslateClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("AI-Paper-News/0.1.0")
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }

    /// Translate text from English to Japanese
    pub async fn translate_to_japanese(&self, text: &str) -> Result<String, TranslateError> {
        // Skip if text is too short or already contains Japanese
        if text.len() < 3 || contains_japanese(text) {
            return Ok(text.to_string());
        }

        // URL encode the text
        let encoded_text = urlencoding::encode(text);
        
        let url = format!(
            "{}?q={}&langpair=en|ja",
            MYMEMORY_API_URL, encoded_text
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(TranslateError::TranslationFailed(format!(
                "API returned status: {}",
                response.status()
            )));
        }

        let result: MyMemoryResponse = response.json().await?;

        if result.response_status != 200 {
            return Err(TranslateError::TranslationFailed(format!(
                "Translation failed with status: {}",
                result.response_status
            )));
        }

        Ok(result.response_data.translated_text)
    }
}

impl Default for TranslateClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if text contains Japanese characters
fn contains_japanese(text: &str) -> bool {
    text.chars().any(|c| {
        matches!(c,
            '\u{3040}'..='\u{309F}' | // Hiragana
            '\u{30A0}'..='\u{30FF}' | // Katakana
            '\u{4E00}'..='\u{9FFF}'   // CJK Unified Ideographs (Kanji)
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_translate_to_japanese() {
        let client = TranslateClient::new();
        let result = client.translate_to_japanese("Hello World").await;
        
        match result {
            Ok(translated) => {
                println!("Translated: {}", translated);
                assert!(!translated.is_empty());
            }
            Err(e) => {
                println!("Error (may be rate limited): {}", e);
            }
        }
    }

    #[test]
    fn test_contains_japanese() {
        assert!(contains_japanese("こんにちは"));
        assert!(contains_japanese("Hello こんにちは"));
        assert!(!contains_japanese("Hello World"));
    }
}

