mod commands;

use commands::{config, translate, window};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            config::get_config,
            config::set_config,
            config::has_baidu_key,
            translate::translate_text,
            window::window_minimize,
            window::window_toggle_maximize,
            window::window_close
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
