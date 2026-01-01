mod api;
mod commands;
mod db;
mod settings;

use settings::AppSettings;
use std::path::PathBuf;
use std::sync::RwLock;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Get app data directory
            let app_handle = app.handle();
            let app_data_dir = app_handle.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;
            
            // Initialize database
            let db_path = app_data_dir.join("papers.db");
            let db_path_str = db_path.to_string_lossy().to_string();
            db::init_db(&db_path_str)?;
            
            // Load settings
            let settings = AppSettings::load(&app_data_dir);
            
            // Store state
            app.manage(AppState {
                db_path: db_path_str,
                app_data_dir,
                settings: RwLock::new(settings),
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Paper commands
            commands::get_papers,
            commands::fetch_papers,
            commands::generate_summary,
            commands::get_categories,
            commands::get_settings,
            commands::save_settings,
            // RFC commands
            commands::rfc_commands::get_rfcs,
            commands::rfc_commands::fetch_rfcs,
            commands::rfc_commands::get_rfc_by_id,
            commands::rfc_commands::get_rfc_content,
            commands::rfc_commands::generate_rfc_summary,
            commands::rfc_commands::generate_rfc_implementation_guide,
            commands::rfc_commands::translate_rfc_section,
            commands::rfc_commands::translate_rfc_abstract,
            commands::rfc_commands::translate_rfc_title,
            commands::rfc_commands::add_rfc_bookmark,
            commands::rfc_commands::remove_rfc_bookmark,
            commands::rfc_commands::get_rfc_bookmarks,
            commands::rfc_commands::add_rfc_history,
            commands::rfc_commands::get_rfc_history,
            commands::rfc_commands::get_rfc_categories,
            commands::rfc_commands::get_rfc_count,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub struct AppState {
    pub db_path: String,
    pub app_data_dir: PathBuf,
    pub settings: RwLock<AppSettings>,
}
