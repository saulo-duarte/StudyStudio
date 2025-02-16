use database::initialize_database;
use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};
use std::fs;
use std::path::PathBuf;

pub mod database;
pub mod models;
pub mod views;
pub mod commands;
pub mod errors;
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

pub fn run() {
  let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
  db_path.push("study-studio");
  db_path.push("app.db");

  let app_state = AppState::new(db_path.to_str().unwrap())
      .expect("Failed to initialize the application state");

  tauri::Builder::default()
      .manage(app_state)
      .invoke_handler(tauri::generate_handler![
        commands::create_user,
        commands::get_active_users_count,
        commands::get_active_user_id,
        commands::create_task,
        commands::get_all_tasks,
        commands::maximize_window,
        commands::minimize_window,
        commands::close_window,
        commands::delete_task,
        commands::update_task,
        commands::create_tag,
        commands::get_all_tags,
      ])
      .setup(|app| {
          if cfg!(debug_assertions) {
              app.handle().plugin(
                  tauri_plugin_log::Builder::default()
                      .level(log::LevelFilter::Info)
                      .build(),
              )?;
          }
          Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}