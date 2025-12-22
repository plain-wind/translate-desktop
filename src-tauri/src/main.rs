#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::{config::*, translate::*, window::*};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            window_minimize,
            window_toggle_maximize,
            window_close,
            window_start_drag,
            translate_text,
            get_config,
            set_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
