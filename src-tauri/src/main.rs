// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn unlock_db(password: &str) -> bool {
    println!("You have unlock_db with {}", password);
    match password {
        "asdasd" => true,
        _ => false
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![unlock_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
