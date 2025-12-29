//! Application settings management

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppSettings {
    #[serde(default)]
    pub groq_api_key: Option<String>,
}

impl AppSettings {
    /// Get the settings file path
    fn get_settings_path(app_data_dir: &PathBuf) -> PathBuf {
        app_data_dir.join("settings.json")
    }

    /// Load settings from file
    pub fn load(app_data_dir: &PathBuf) -> Self {
        let path = Self::get_settings_path(app_data_dir);
        
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    serde_json::from_str(&content).unwrap_or_default()
                }
                Err(_) => Self::default(),
            }
        } else {
            Self::default()
        }
    }

    /// Save settings to file
    pub fn save(&self, app_data_dir: &PathBuf) -> Result<(), String> {
        let path = Self::get_settings_path(app_data_dir);
        
        // Ensure directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| e.to_string())?;
        
        fs::write(&path, content).map_err(|e| e.to_string())?;
        
        Ok(())
    }

    /// Get the Groq API key (from settings or environment variable)
    pub fn get_groq_api_key(&self) -> Option<String> {
        // First check settings, then fall back to environment variable
        self.groq_api_key.clone()
            .or_else(|| std::env::var("GROQ_API_KEY").ok())
    }
}

