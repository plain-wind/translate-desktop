use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub source_lang: String,
    pub target_lang: String,
}

fn config_path(app: &AppHandle) -> PathBuf {
    app.path()
        .app_config_dir()
        .expect("failed to get app config dir")
        .join("config.json")
}

#[tauri::command]
pub fn get_config(app: AppHandle) -> AppConfig {
    let path = config_path(&app);

    if let Ok(data) = fs::read_to_string(path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        AppConfig::default()
    }
}

#[tauri::command]
pub fn set_config(app: AppHandle, config: AppConfig) -> Result<(), String> {
    let path = config_path(&app);

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    fs::write(path, serde_json::to_string_pretty(&config).unwrap()).map_err(|e| e.to_string())
}
