use tauri::State;

use crate::{
    models::tag::Tag,
    repository::tag_repository::TagRepository,
    errors::command_errors::CommandError,
    AppState,
};

#[tauri::command]
pub fn create_tag(name: String, color: String, state: State<AppState>) -> Result<String, CommandError> {
    let mut tag = Tag::new(name, color)?;
    
    let db_conn = state.db_conn();
    let conn = db_conn.lock().map_err(|_| CommandError::LockFailed)?;
    
    TagRepository::create(&conn, &mut tag)?;
    
    Ok(format!("Tag {} criada com sucesso", tag.name))
}


#[tauri::command]
pub fn get_tag_by_id(id: u32, state: State<AppState>) -> Result<Option<Tag>, CommandError> {
    let db_conn = state.db_conn();
    let conn = db_conn.lock().map_err(|_| CommandError::LockFailed)?;
    
    TagRepository::find_by_id(&conn, id)
        .map_err(Into::into)
}

#[tauri::command]
pub fn list_tags(state: State<AppState>) -> Result<Vec<Tag>, CommandError> {
    let db_conn = state.db_conn();
    let conn = db_conn.lock().map_err(|_| CommandError::LockFailed)?;
    
    TagRepository::list_all(&conn)
        .map_err(Into::into)
}

#[tauri::command]
pub fn update_tag(id: u32, tag_name: String, state: State<AppState>) -> Result<String, CommandError> {
    let db_conn = state.db_conn();
    let conn = db_conn.lock().map_err(|_| CommandError::LockFailed)?;
    
    TagRepository::update_tag(&conn, id, tag_name.clone())?;
    
    Ok(format!("Tag with id {} updated to {}", id, tag_name))
}

#[tauri::command]
pub fn delete_tag(id: u32, state: State<AppState>) -> Result<String, CommandError> {
    let db_conn = state.db_conn();
    let conn = db_conn.lock().map_err(|_| CommandError::LockFailed)?;

    TagRepository::delete_by_id(&conn, id)
        .map_err(|e| CommandError::Database(e.to_string()))?;

    Ok("Tag deleted successfully".to_string())
}