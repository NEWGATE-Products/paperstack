//! Tauri IPC Commands

pub mod rfc_commands;

use crate::api::{arxiv::ArxivClient, groq::GroqClient, translate::TranslateClient};
use crate::db::{self, models::{Category, Paper}};
use crate::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize)]
pub struct CommandError {
    pub message: String,
}

impl From<rusqlite::Error> for CommandError {
    fn from(e: rusqlite::Error) -> Self {
        CommandError {
            message: e.to_string(),
        }
    }
}

impl From<crate::api::arxiv::ArxivError> for CommandError {
    fn from(e: crate::api::arxiv::ArxivError) -> Self {
        CommandError {
            message: e.to_string(),
        }
    }
}

impl From<crate::api::groq::GroqError> for CommandError {
    fn from(e: crate::api::groq::GroqError) -> Self {
        CommandError {
            message: e.to_string(),
        }
    }
}

impl From<crate::api::translate::TranslateError> for CommandError {
    fn from(e: crate::api::translate::TranslateError) -> Self {
        CommandError {
            message: e.to_string(),
        }
    }
}

/// Get papers from database, optionally filtered by category
#[tauri::command]
pub async fn get_papers(
    state: State<'_, AppState>,
    category: Option<String>,
    limit: Option<i64>,
) -> Result<Vec<Paper>, CommandError> {
    let db_path = &state.db_path;
    let papers = db::get_papers_from_db(db_path, category.as_deref(), limit)?;
    Ok(papers)
}

/// Fetch papers from arXiv API and save to database
/// Only fetches new papers that don't already exist in the database
#[tauri::command]
pub async fn fetch_papers(
    state: State<'_, AppState>,
    _tasks: Vec<String>,
) -> Result<Vec<Paper>, CommandError> {
    let db_path = &state.db_path;
    let arxiv_client = ArxivClient::new();
    let translate_client = TranslateClient::new();
    
    // Fetch papers from all categories (10 papers per category)
    let arxiv_papers = arxiv_client.fetch_all_categories(10).await?;
    
    // Save to database
    let conn = db::get_connection(db_path)?;
    
    // Collect NEW paper IDs that need translation (skip existing papers)
    let mut papers_to_translate: Vec<(String, String)> = Vec::new();
    let mut new_paper_count = 0;
    let mut skipped_count = 0;
    
    for (arxiv_paper, category_name, task_slug) in &arxiv_papers {
        // Check if paper already exists in database
        if db::paper_exists(&conn, &arxiv_paper.id)? {
            // Paper already exists - just add the task relationship if new
            db::insert_paper_task(&conn, &arxiv_paper.id, task_slug, category_name)?;
            skipped_count += 1;
            continue;
        }
        
        // New paper - save to database
        let paper = Paper {
            id: arxiv_paper.id.clone(),
            title: arxiv_paper.title.clone(),
            title_ja: None, // Will be translated below
            r#abstract: Some(arxiv_paper.summary.clone()),
            summary_ja: None,
            url_pdf: Some(arxiv_paper.pdf_url.clone()),
            url_paper: Some(arxiv_paper.abs_url.clone()),
            published: Some(arxiv_paper.published.clone()),
            fetched_at: None,
            tasks: vec![task_slug.clone()],
        };
        
        db::upsert_paper(&conn, &paper)?;
        db::insert_paper_task(&conn, &paper.id, task_slug, category_name)?;
        
        // Add to translation queue (only new papers)
        papers_to_translate.push((arxiv_paper.id.clone(), arxiv_paper.title.clone()));
        new_paper_count += 1;
    }
    
    println!("Fetch complete: {} new papers, {} skipped (already exist)", new_paper_count, skipped_count);
    
    // Translate titles for NEW papers only (with rate limiting built into the client)
    for (paper_id, title) in &papers_to_translate {
        match translate_client.translate_to_japanese(title).await {
            Ok(title_ja) => {
                let _ = db::update_paper_title_ja(&conn, paper_id, &title_ja);
            }
            Err(e) => {
                // Log error but continue - translation failure shouldn't stop the process
                eprintln!("Translation error for {}: {}", paper_id, e);
            }
        }
    }
    
    // Return updated papers from database
    let papers = db::get_papers_from_db(db_path, None, Some(100))?;
    Ok(papers)
}

/// Generate Japanese summary for a paper using Groq API
#[tauri::command]
pub async fn generate_summary(
    state: State<'_, AppState>,
    paper_id: String,
) -> Result<String, CommandError> {
    let db_path = &state.db_path;
    
    // Get paper from database
    let paper = db::get_paper_by_id(db_path, &paper_id)?
        .ok_or_else(|| CommandError {
            message: format!("Paper not found: {}", paper_id),
        })?;
    
    // If summary already exists, return it
    if let Some(summary) = &paper.summary_ja {
        if !summary.is_empty() {
            return Ok(summary.clone());
        }
    }
    
    // Get abstract text
    let abstract_text = paper.r#abstract.as_deref().unwrap_or("");
    
    if abstract_text.is_empty() {
        return Err(CommandError {
            message: "Paper has no abstract to summarize".to_string(),
        });
    }
    
    // Get API key from settings
    let api_key = {
        let settings = state.settings.read().map_err(|e| CommandError {
            message: format!("Failed to read settings: {}", e),
        })?;
        settings.get_groq_api_key()
    };
    
    let api_key = api_key.ok_or_else(|| CommandError {
        message: "GROQ APIキーが設定されていません。設定画面からAPIキーを入力してください。".to_string(),
    })?;
    
    // Generate summary using Groq
    let groq_client = GroqClient::with_api_key(api_key);
    let summary = groq_client.generate_summary(&paper.title, abstract_text).await?;
    
    // Save summary to database
    db::update_paper_summary(db_path, &paper_id, &summary)?;
    
    Ok(summary)
}

/// Get all available categories
#[tauri::command]
pub fn get_categories() -> Vec<Category> {
    Category::all_categories()
}

/// Settings response for frontend
#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsResponse {
    pub groq_api_key: Option<String>,
    pub has_groq_api_key: bool,
}

/// Get current settings
#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> Result<SettingsResponse, CommandError> {
    let settings = state.settings.read().map_err(|e| CommandError {
        message: format!("Failed to read settings: {}", e),
    })?;
    
    // Don't expose the full API key, just indicate if it's set
    let has_key = settings.get_groq_api_key().is_some();
    
    Ok(SettingsResponse {
        groq_api_key: settings.groq_api_key.as_ref().map(|k| {
            // Mask the API key for display
            if k.len() > 8 {
                format!("{}...{}", &k[..4], &k[k.len()-4..])
            } else {
                "****".to_string()
            }
        }),
        has_groq_api_key: has_key,
    })
}

/// Settings input from frontend
#[derive(Debug, Deserialize)]
pub struct SettingsInput {
    pub groq_api_key: Option<String>,
}

/// Save settings
#[tauri::command]
pub fn save_settings(
    state: State<'_, AppState>,
    settings_input: SettingsInput,
) -> Result<SettingsResponse, CommandError> {
    let mut settings = state.settings.write().map_err(|e| CommandError {
        message: format!("Failed to write settings: {}", e),
    })?;
    
    // Update settings
    if let Some(key) = settings_input.groq_api_key {
        if key.is_empty() {
            settings.groq_api_key = None;
        } else {
            settings.groq_api_key = Some(key);
        }
    }
    
    // Save to file
    settings.save(&state.app_data_dir).map_err(|e| CommandError {
        message: format!("Failed to save settings: {}", e),
    })?;
    
    // Return updated settings
    let has_key = settings.get_groq_api_key().is_some();
    
    Ok(SettingsResponse {
        groq_api_key: settings.groq_api_key.as_ref().map(|k| {
            if k.len() > 8 {
                format!("{}...{}", &k[..4], &k[k.len()-4..])
            } else {
                "****".to_string()
            }
        }),
        has_groq_api_key: has_key,
    })
}
