// window.rs
use crate::commands::config::{get_config, set_config, WindowConfig};
use tauri::Window;

#[tauri::command]
pub fn window_minimize(window: Window) {
    let _ = window.minimize();
}

#[tauri::command]
pub fn window_toggle_maximize(window: Window) {
    let _ = if window.is_maximized().unwrap_or(false) {
        window.unmaximize()
    } else {
        window.maximize()
    };
}

#[tauri::command]
pub fn window_close(window: Window) {
    let _ = window.close();
}

#[tauri::command]
pub fn save_window_size(app: tauri::AppHandle, window: Window) -> Result<(), String> {
    let physical = window.inner_size().map_err(|e| e.to_string())?;
    let scale_factor = window.scale_factor().map_err(|e| e.to_string())?;

    let logical = physical.to_logical::<f64>(scale_factor);

    let mut config = get_config(app.clone());
    config.window = Some(WindowConfig {
        width: logical.width as u32,
        height: logical.height as u32,
    });

    set_config(app, config)
}
