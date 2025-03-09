use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};
use std::fs;
use std::path::PathBuf;

use crate::utils::initialize_database::initialize_database;

pub mod models;
pub mod errors;
pub mod repository;
pub mod commands;
pub mod utils;

pub struct AppState {
    pub db_conn: Arc<Mutex<Connection>>
}

impl AppState {
    pub fn new(db_path: &str) -> Result<Self, Box<dyn std::error::Error>>{
      let db_path = PathBuf::from(db_path);

      if let Some(parent) = db_path.parent(){
        fs::create_dir_all(parent).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
      }

      let conn = Connection::open(db_path)?;

      initialize_database(&conn)?;

      Ok(Self {
        db_conn: Arc::new(Mutex::new(conn)),
      })
    }

    pub fn db_conn(&self) -> Arc<Mutex<Connection>> {
      Arc::clone(&self.db_conn)
    }

}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("study-studio");
    db_path.push("app.db");

    let app_state = AppState::new(db_path.to_str().unwrap())
        .expect("Failed to initialize the application state");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state) 
        .invoke_handler(tauri::generate_handler![
          commands::create_user,
            commands::get_active_users_count,
            commands::get_active_user_id,
            commands::create_tag,
            commands::get_tag_by_id,
            commands::list_tags,
            commands::create_task,
            commands::update_task,
            commands::get_all_tasks,
            commands::delete_task,
            commands::update_tag,
            commands::delete_tag,
            commands::get_tasks_for_today,
          ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

