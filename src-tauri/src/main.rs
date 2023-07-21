// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod write_config;

use tauri::command;

#[derive(serde::Deserialize)]
struct ConfigPath {
    path: String,
}

#[command]
fn create_config(args: Vec<String>) -> Result<(), String> {
    if let Some(path) = args.get(0) {
        write_config::create_config_file(path.to_string())
    } else {
        Err("No path provided".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}