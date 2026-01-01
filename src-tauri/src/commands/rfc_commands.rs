//! RFC Tauri Commands

use crate::api::groq::GroqClient;
use crate::api::rfc_editor::RfcEditorClient;
use crate::db::{self, models::{Rfc, RfcFilter, RfcListResponse, RfcBookmark, RfcHistory, RfcCategory, SummaryLevel}};
use crate::db::rfc_queries;
use crate::AppState;
use serde::Serialize;
use tauri::State;

#[derive(Debug, Serialize)]
pub struct RfcCommandError {
    pub message: String,
}

impl From<rusqlite::Error> for RfcCommandError {
    fn from(e: rusqlite::Error) -> Self {
        RfcCommandError { message: e.to_string() }
    }
}

impl From<crate::api::rfc_editor::RfcEditorError> for RfcCommandError {
    fn from(e: crate::api::rfc_editor::RfcEditorError) -> Self {
        RfcCommandError { message: e.to_string() }
    }
}

impl From<crate::api::groq::GroqError> for RfcCommandError {
    fn from(e: crate::api::groq::GroqError) -> Self {
        RfcCommandError { message: e.to_string() }
    }
}

// ============================================================================
// RFC List Commands
// ============================================================================

/// Get RFCs with filtering and pagination
#[tauri::command]
pub async fn get_rfcs(
    state: State<'_, AppState>,
    filter: Option<RfcFilter>,
    page: Option<i32>,
    limit: Option<i32>,
) -> Result<RfcListResponse, RfcCommandError> {
    let db_path = &state.db_path;
    let conn = db::get_connection(db_path)?;
    
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(20);
    
    let response = rfc_queries::get_rfcs(&conn, filter.as_ref(), page, limit)?;
    Ok(response)
}

/// Fetch RFCs from RFC Editor and save to database
#[tauri::command]
pub async fn fetch_rfcs(state: State<'_, AppState>) -> Result<i64, RfcCommandError> {
    let db_path = &state.db_path;
    let client = RfcEditorClient::new();
    
    println!("Fetching RFC index...");
    let entries = client.fetch_rfc_index().await?;
    
    let conn = db::get_connection(db_path)?;
    let mut count = 0;
    
    for entry in entries {
        let number = match entry.number() {
            Some(n) => n,
            None => continue,
        };
        
        let rfc = Rfc {
            id: entry.doc_id.clone(),
            number,
            title: entry.title.clone(),
            r#abstract: entry.r#abstract.clone(),
            status: entry.status.clone().unwrap_or_default(),
            published_date: entry.published_date(),
            authors: entry.authors.clone(),
            keywords: entry.keywords.clone(),
            categories: vec![],
            summary_easy: None,
            summary_normal: None,
            summary_technical: None,
            implementation_guide: None,
            title_ja: None,
            abstract_ja: None,
            is_bookmarked: false,
        };
        
        rfc_queries::upsert_rfc(&conn, &rfc)?;
        
        // Auto-categorize based on keywords
        let categories = RfcCategory::categorize(&entry.title, &entry.keywords);
        for cat in categories {
            rfc_queries::insert_rfc_category(&conn, &entry.doc_id, &cat)?;
        }
        
        count += 1;
    }
    
    println!("Saved {} RFCs to database", count);
    Ok(count)
}

// ============================================================================
// RFC Detail Commands
// ============================================================================

/// Get a single RFC by ID
#[tauri::command]
pub async fn get_rfc_by_id(
    state: State<'_, AppState>,
    rfc_id: String,
) -> Result<Option<Rfc>, RfcCommandError> {
    let db_path = &state.db_path;
    let conn = db::get_connection(db_path)?;
    
    let rfc = rfc_queries::get_rfc_by_id(&conn, &rfc_id)?;
    Ok(rfc)
}

/// Get RFC full text
#[tauri::command]
pub async fn get_rfc_content(rfc_number: i32) -> Result<String, RfcCommandError> {
    let client = RfcEditorClient::new();
    let content = client.fetch_rfc_text(rfc_number).await?;
    Ok(content)
}

// ============================================================================
// AI Summary Commands
// ============================================================================

/// Generate RFC summary at specified level
#[tauri::command]
pub async fn generate_rfc_summary(
    state: State<'_, AppState>,
    rfc_id: String,
    level: SummaryLevel,
) -> Result<String, RfcCommandError> {
    let db_path = &state.db_path;
    let conn = db::get_connection(db_path)?;
    
    // Get RFC from database
    let rfc = rfc_queries::get_rfc_by_id(&conn, &rfc_id)?
        .ok_or_else(|| RfcCommandError { message: format!("RFC not found: {}", rfc_id) })?;
    
    // Check if summary already exists
    let existing = match level {
        SummaryLevel::Easy => &rfc.summary_easy,
        SummaryLevel::Normal => &rfc.summary_normal,
        SummaryLevel::Technical => &rfc.summary_technical,
    };
    
    if let Some(summary) = existing {
        if !summary.is_empty() {
            return Ok(summary.clone());
        }
    }
    
    // Get abstract text
    let abstract_text = rfc.r#abstract.as_deref().unwrap_or("");
    if abstract_text.is_empty() {
        return Err(RfcCommandError { message: "RFC has no abstract to summarize".to_string() });
    }
    
    // Get API key
    let api_key = {
        let settings = state.settings.read().map_err(|e| RfcCommandError {
            message: format!("Failed to read settings: {}", e),
        })?;
        settings.get_groq_api_key()
    };
    
    let api_key = api_key.ok_or_else(|| RfcCommandError {
        message: "GROQ APIキーが設定されていません。設定画面からAPIキーを入力してください。".to_string(),
    })?;
    
    // Generate summary
    let groq_client = GroqClient::with_api_key(api_key);
    let summary = match level {
        SummaryLevel::Easy => groq_client.generate_rfc_summary_easy(rfc.number, &rfc.title, abstract_text).await?,
        SummaryLevel::Normal => groq_client.generate_rfc_summary_normal(rfc.number, &rfc.title, abstract_text).await?,
        SummaryLevel::Technical => groq_client.generate_rfc_summary_technical(rfc.number, &rfc.title, abstract_text).await?,
    };
    
    // Save to database
    let level_str = match level {
        SummaryLevel::Easy => "easy",
        SummaryLevel::Normal => "normal",
        SummaryLevel::Technical => "technical",
    };
    rfc_queries::update_rfc_summary(&conn, &rfc_id, level_str, &summary)?;
    
    Ok(summary)
}

/// Generate implementation guide
#[tauri::command]
pub async fn generate_rfc_implementation_guide(
    state: State<'_, AppState>,
    rfc_id: String,
) -> Result<String, RfcCommandError> {
    let db_path = &state.db_path;
    let conn = db::get_connection(db_path)?;
    
    // Get RFC from database
    let rfc = rfc_queries::get_rfc_by_id(&conn, &rfc_id)?
        .ok_or_else(|| RfcCommandError { message: format!("RFC not found: {}", rfc_id) })?;
    
    // Check if guide already exists
    if let Some(guide) = &rfc.implementation_guide {
        if !guide.is_empty() {
            return Ok(guide.clone());
        }
    }
    
    // Get abstract text
    let abstract_text = rfc.r#abstract.as_deref().unwrap_or("");
    if abstract_text.is_empty() {
        return Err(RfcCommandError { message: "RFC has no abstract".to_string() });
    }
    
    // Get API key
    let api_key = {
        let settings = state.settings.read().map_err(|e| RfcCommandError {
            message: format!("Failed to read settings: {}", e),
        })?;
        settings.get_groq_api_key()
    };
    
    let api_key = api_key.ok_or_else(|| RfcCommandError {
        message: "GROQ APIキーが設定されていません。".to_string(),
    })?;
    
    // Generate guide
    let groq_client = GroqClient::with_api_key(api_key);
    let guide = groq_client.generate_rfc_implementation_guide(rfc.number, &rfc.title, abstract_text).await?;
    
    // Save to database
    rfc_queries::update_rfc_implementation_guide(&conn, &rfc_id, &guide)?;
    
    Ok(guide)
}

// ============================================================================
// Translation Commands
// ============================================================================

/// Translate RFC section
#[tauri::command]
pub async fn translate_rfc_section(
    state: State<'_, AppState>,
    text: String,
) -> Result<String, RfcCommandError> {
    // Get API key
    let api_key = {
        let settings = state.settings.read().map_err(|e| RfcCommandError {
            message: format!("Failed to read settings: {}", e),
        })?;
        settings.get_groq_api_key()
    };
    
    let api_key = api_key.ok_or_else(|| RfcCommandError {
        message: "GROQ APIキーが設定されていません。".to_string(),
    })?;
    
    let groq_client = GroqClient::with_api_key(api_key);
    let translation = groq_client.translate_rfc_section(&text).await?;
    
    Ok(translation)
}

/// Translate RFC abstract and save to database
#[tauri::command]
pub async fn translate_rfc_abstract(
    state: State<'_, AppState>,
    rfc_id: String,
) -> Result<String, RfcCommandError> {
    let db_path = &state.db_path;
    let conn = db::get_connection(db_path)?;
    
    // Get RFC from database
    let rfc = rfc_queries::get_rfc_by_id(&conn, &rfc_id)?
        .ok_or_else(|| RfcCommandError { message: format!("RFC not found: {}", rfc_id) })?;
    
    // Check if translation already exists
    if let Some(ref abstract_ja) = rfc.abstract_ja {
        if !abstract_ja.is_empty() {
            return Ok(abstract_ja.clone());
        }
    }
    
    // Get abstract text
    let abstract_text = rfc.r#abstract.as_deref().ok_or_else(|| RfcCommandError {
        message: "RFC has no abstract to translate".to_string(),
    })?;
    
    if abstract_text.is_empty() {
        return Err(RfcCommandError { message: "RFC abstract is empty".to_string() });
    }
    
    // Get API key
    let api_key = {
        let settings = state.settings.read().map_err(|e| RfcCommandError {
            message: format!("Failed to read settings: {}", e),
        })?;
        settings.get_groq_api_key()
    };
    
    let api_key = api_key.ok_or_else(|| RfcCommandError {
        message: "GROQ APIキーが設定されていません。".to_string(),
    })?;
    
    // Translate
    let groq_client = GroqClient::with_api_key(api_key);
    let translation = groq_client.translate_rfc_section(abstract_text).await?;
    
    // Save to database
    rfc_queries::update_rfc_abstract_ja(&conn, &rfc_id, &translation)?;
    
    Ok(translation)
}

/// Translate RFC title and save to database
#[tauri::command]
pub async fn translate_rfc_title(
    state: State<'_, AppState>,
    rfc_id: String,
) -> Result<String, RfcCommandError> {
    let db_path = &state.db_path;
    let conn = db::get_connection(db_path)?;
    
    // Get RFC from database
    let rfc = rfc_queries::get_rfc_by_id(&conn, &rfc_id)?
        .ok_or_else(|| RfcCommandError { message: format!("RFC not found: {}", rfc_id) })?;
    
    // Check if translation already exists
    if let Some(ref title_ja) = rfc.title_ja {
        if !title_ja.is_empty() {
            return Ok(title_ja.clone());
        }
    }
    
    // Get title text
    if rfc.title.is_empty() {
        return Err(RfcCommandError { message: "RFC has no title to translate".to_string() });
    }
    
    // Get API key
    let api_key = {
        let settings = state.settings.read().map_err(|e| RfcCommandError {
            message: format!("Failed to read settings: {}", e),
        })?;
        settings.get_groq_api_key()
    };
    
    let api_key = api_key.ok_or_else(|| RfcCommandError {
        message: "GROQ APIキーが設定されていません。".to_string(),
    })?;
    
    // Translate
    let groq_client = GroqClient::with_api_key(api_key);
    let translation = groq_client.translate_rfc_title(&rfc.title).await?;
    
    // Save to database
    rfc_queries::update_rfc_title_ja(&conn, &rfc_id, &translation)?;
    
    Ok(translation)
}

// ============================================================================
// Bookmark Commands
// ============================================================================

/// Add RFC bookmark
#[tauri::command]
pub async fn add_rfc_bookmark(
    state: State<'_, AppState>,
    rfc_id: String,
    memo: Option<String>,
) -> Result<(), RfcCommandError> {
    let db_path = &state.db_path;
    let conn = db::get_connection(db_path)?;
    
    rfc_queries::add_rfc_bookmark(&conn, &rfc_id, memo.as_deref())?;
    Ok(())
}

/// Remove RFC bookmark
#[tauri::command]
pub async fn remove_rfc_bookmark(
    state: State<'_, AppState>,
    rfc_id: String,
) -> Result<(), RfcCommandError> {
    let db_path = &state.db_path;
    let conn = db::get_connection(db_path)?;
    
    rfc_queries::remove_rfc_bookmark(&conn, &rfc_id)?;
    Ok(())
}

/// Get all RFC bookmarks
#[tauri::command]
pub async fn get_rfc_bookmarks(
    state: State<'_, AppState>,
) -> Result<Vec<RfcBookmark>, RfcCommandError> {
    let db_path = &state.db_path;
    let conn = db::get_connection(db_path)?;
    
    let bookmarks = rfc_queries::get_rfc_bookmarks(&conn)?;
    Ok(bookmarks)
}

// ============================================================================
// History Commands
// ============================================================================

/// Add RFC to history
#[tauri::command]
pub async fn add_rfc_history(
    state: State<'_, AppState>,
    rfc_id: String,
) -> Result<(), RfcCommandError> {
    let db_path = &state.db_path;
    let conn = db::get_connection(db_path)?;
    
    rfc_queries::add_rfc_history(&conn, &rfc_id)?;
    Ok(())
}

/// Get RFC history
#[tauri::command]
pub async fn get_rfc_history(
    state: State<'_, AppState>,
    limit: Option<i32>,
) -> Result<Vec<RfcHistory>, RfcCommandError> {
    let db_path = &state.db_path;
    let conn = db::get_connection(db_path)?;
    
    let history = rfc_queries::get_rfc_history(&conn, limit)?;
    Ok(history)
}

// ============================================================================
// Category Commands
// ============================================================================

/// Get all RFC categories
#[tauri::command]
pub async fn get_rfc_categories(
    state: State<'_, AppState>,
) -> Result<Vec<String>, RfcCommandError> {
    let db_path = &state.db_path;
    let conn = db::get_connection(db_path)?;
    
    let categories = rfc_queries::get_all_rfc_categories(&conn)?;
    Ok(categories)
}

/// Get RFC count
#[tauri::command]
pub async fn get_rfc_count(
    state: State<'_, AppState>,
) -> Result<i64, RfcCommandError> {
    let db_path = &state.db_path;
    let conn = db::get_connection(db_path)?;
    
    let count = rfc_queries::get_rfc_count(&conn)?;
    Ok(count)
}

