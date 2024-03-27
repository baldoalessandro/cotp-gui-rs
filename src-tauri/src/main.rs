// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;

use commands::{list_otps, unlock_db};

fn main() {
    let state = state::AppState {
        database: None.into(),
        key: None.into(),
        salt: None.into(),
    };

    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![unlock_db, list_otps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
