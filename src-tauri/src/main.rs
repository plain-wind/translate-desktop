// main.rs
mod commands;

use commands::{config, shortcut, translate, window};
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        // ✅ 正确注册 global-shortcut 插件（Tauri v2）
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let handle = app.handle().clone();

            // 获取主窗口
            let window = app
                .get_webview_window("main")
                .expect("failed to get main window");

            // 启动时恢复窗口大小
            if let Some(size) = config::get_window_size(handle.clone()) {
                let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize {
                    width: size.width as f64,
                    height: size.height as f64,
                }));
            }

            // 所有窗口状态恢复完，再显示窗口
            window.show().unwrap();
            window.set_focus().ok();

            // 🚀 启动时注册全局快捷键
            shortcut::register_shortcut(app.handle());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // config
            config::get_config,
            config::set_config,
            config::has_baidu_key,
            config::get_window_size,
            // window
            window::save_window_size,
            window::window_close,
            window::window_minimize,
            window::window_toggle_maximize,
            // translate
            translate::translate_text,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
