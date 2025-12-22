use std::str::FromStr;
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::commands::config::get_config;

/// 核心：应用当前配置里的快捷键（可反复调用）
fn apply_shortcut(app: &AppHandle) -> Result<(), String> {
    let manager = app.global_shortcut();

    // 🚿 1. 清空所有旧快捷键（包含旧监听）
    manager
        .unregister_all()
        .map_err(|e| format!("清除旧快捷键失败: {:?}", e))?;

    let config = get_config(app.clone());

    let Some(sc) = config.shortcut else {
        return Ok(());
    };

    if !sc.enabled {
        return Ok(());
    }

    let key = sc.key.as_deref().unwrap_or("").trim();
    if key.is_empty() {
        return Err("快捷键不能为空".into());
    }

    let shortcut = Shortcut::from_str(key).map_err(|_| format!("非法快捷键格式: {}", key))?;

    let app_handle = app.clone();

    // ⚠️ 2. 只使用 on_shortcut（不要 register）
    app.global_shortcut()
        .on_shortcut(shortcut.clone(), move |_app, _sc, event| {
            if event.state != ShortcutState::Pressed {
                return;
            }

            if let Some(window) = app_handle.get_webview_window("main") {
                let visible = window.is_visible().unwrap_or(false);

                if visible {
                    let _ = window.hide();
                } else {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .map_err(|e| format!("快捷键监听失败: {:?}", e))?;

    println!("[shortcut] registered: {}", key);
    Ok(())
}

/// 启动时调用
pub fn register_shortcut(app: &AppHandle) {
    if let Err(e) = apply_shortcut(app) {
        eprintln!("[shortcut] {}", e);
    }
}

/// 前端热更新调用
#[tauri::command]
pub fn reload_shortcut(app: AppHandle) -> Result<(), String> {
    apply_shortcut(&app)
}
