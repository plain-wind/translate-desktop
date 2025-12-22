use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ShortcutConfig {
    pub key: Option<String>, // 比如 "F7"
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AppConfig {
    pub baidu: BaiduConfig,
    pub window: Option<WindowConfig>,
    pub shortcut: Option<ShortcutConfig>,
    pub is_topmost: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BaiduConfig {
    pub appid: String,
    pub secret: String,
}

fn config_path(app: &AppHandle) -> PathBuf {
    let dir = app.path().app_config_dir().expect("config dir");
    if !dir.exists() {
        fs::create_dir_all(&dir).unwrap();
    }
    dir.join("config.json")
}

#[tauri::command]
pub fn get_config(app: AppHandle) -> AppConfig {
    let path = config_path(&app);
    if !path.exists() {
        return AppConfig::default();
    }

    fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

#[tauri::command]
pub fn set_config(app: AppHandle, config: AppConfig) -> Result<(), String> {
    let path = config_path(&app);
    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn has_baidu_key(app: AppHandle) -> bool {
    let config = get_config(app);
    !config.baidu.appid.is_empty() && !config.baidu.secret.is_empty()
}

#[tauri::command]
pub fn get_window_size(app: AppHandle) -> Option<WindowConfig> {
    let config = get_config(app);
    config.window
}
