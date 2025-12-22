use std::str::FromStr;
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::commands::config::get_config;

pub fn register_shortcut(app: &AppHandle) {
    let config = get_config(app.clone());

    let shortcut_cfg = match config.shortcut {
        Some(s) if s.enabled => s,
        _ => return,
    };

    let shortcut_str = match shortcut_cfg.key {
        Some(s) if !s.is_empty() => s,
        _ => return,
    };

    let shortcut = match Shortcut::from_str(&shortcut_str) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("快捷键格式错误: {:?}", e);
            return;
        }
    };

    let app_handle = app.clone();
    let manager = app.global_shortcut();

    println!("🚀 注册并监听快捷键: {}", shortcut_str);

    // ⚠️ 重点：不要再调用 register()
    let _ = manager.on_shortcut(shortcut, move |_app, _shortcut, event| {
        // 🔥 关键：只处理按下事件
        if event.state != ShortcutState::Pressed {
            return;
        }

        println!("🔥 快捷键 Pressed");

        if let Some(window) = app_handle.webview_windows().values().next() {
            let visible = window.is_visible().unwrap_or(false);

            if visible {
                let _ = window.hide();
            } else {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
    });
}
