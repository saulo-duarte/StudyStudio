use tauri::State;

use crate::{
  database::repository::TagRepository, errors::TagError, models::Tag, AppState
};


#[tauri::command]
pub fn create_tag(
  name: String,
  color: String,
  state: State<AppState>,
) -> Result<(), String> {
  
  let mut tag = Tag::new(name, color)
    .map_err(|err| err.to_string())?;

  let db_conn = state.db_conn();
  let conn = db_conn.lock()
    .map_err(|_| "Database lock failed")?;

  TagRepository::insert_tag(&conn, &mut tag)
    .map_err(|e| format!("Database error: {}", e))?;

  Ok(())
}

#[tauri::command]
pub fn get_all_tags(state: State<AppState>) -> Result<Vec<Tag>, TagError> {
    let db_conn = state.db_conn();
    let conn = db_conn
        .lock()
        .map_err(|e| TagError::DatabaseError(e.to_string()))?;
    
    TagRepository::get_all_tags(&conn)
        .map_err(|e| match e {
            TagError::DatabaseError(msg) => TagError::DatabaseError(msg),
            TagError::InvalidName(msg) => TagError::InvalidName(msg)
        })
}

#[tauri::command]
pub fn delete_tag(state: State<AppState>, tag_id: u32) -> Result<(), TagError> {
  let db_conn = state.db_conn();
  let conn = db_conn.lock().map_err(|e| TagError::DatabaseError(e.to_string()))?;
  TagRepository::delete_tag(&conn, tag_id)
    .map_err(|e| TagError::DatabaseError(e.to_string()))
}