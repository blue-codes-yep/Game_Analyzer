// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod server;
mod write_config;
mod game_data;

use actix_web::rt::Runtime;
use serde_json::Value;
use server::SharedState;
use std::sync::Mutex;
use std::thread;
use tauri::command;

#[derive(serde::Deserialize)]
struct ConfigPath {
    path: String,
}

#[command]
fn create_config(args: Vec<String>) -> Result<(), String> {
    if let Some(arg) = args.get(0) {
        let config_path: ConfigPath =
            serde_json::from_str(arg).map_err(|_| "Failed to parse path".to_string())?;
        write_config::create_config_file(config_path.path)
    } else {
        Err("No path provided".to_string())
    }
}

#[command]
fn get_game_state(state: tauri::State<SharedState>) -> Option<Value> {
    let game_state = state.game_state.lock().unwrap();
    game_state.clone()
}

fn main() {
    env_logger::init();
    let database = db::Database::new().expect("Failed to create database");
    
    let state = SharedState {
        game_state: Mutex::new(None),
        db: database,
    };
    // Start the Actix Web server in a separate thread / Seperate thread dedicated to backend tasks
    thread::spawn(|| {
        let server = server::start_server();
        let runtime = Runtime::new().unwrap();
        runtime.block_on(server);
    });

    // Initialize the database and execute a query in a separate thread
    thread::spawn(|| {
        let runtime = Runtime::new().unwrap();
        runtime.block_on(async {
            let database = db::Database::new().expect("Failed to create database");
            database.execute_query("SELECT * FROM players").await.expect("Failed to execute query");            
        });
    });

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![create_config, get_game_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
