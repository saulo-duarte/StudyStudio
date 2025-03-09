use tauri::State;
use crate::{
    models::user::User,
    repository::UserRepository,
    errors::command_errors::CommandError,
    AppState,
};

#[tauri::command]
pub fn create_user(name: String, state: State<AppState>) -> Result<String, CommandError> {
    let mut user = User::new(name)?;
    
    let db_conn = state.db_conn();
    let conn = db_conn.lock().map_err(|_| CommandError::LockFailed)?;

    UserRepository::create(&conn, &mut user)?;

    Ok(format!("Usuário {} criado com sucesso", user.name))
}

#[tauri::command]
pub fn get_active_users_count(state: State<AppState>) -> Result<u32, CommandError> { 
    let db_conn = state.db_conn();
    let conn = db_conn.lock().map_err(|_| CommandError::LockFailed)?;
    Ok(UserRepository::count_active(&conn)?)
}

#[tauri::command]
pub fn get_active_user_id(state: State<AppState>) -> Result<Option<u32>, CommandError> {
    let db_conn = state.db_conn();
    let conn = db_conn.lock().map_err(|_| CommandError::LockFailed)?;
    Ok(UserRepository::find_active_id(&conn)?)
}