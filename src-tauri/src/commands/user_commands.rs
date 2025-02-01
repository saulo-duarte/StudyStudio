use rusqlite::params;
use tauri::State;

use crate::errors::user_errors::UserError;
use crate::models::user::User;
use crate::views::user_view::UserView;
use crate::AppState;

#[tauri::command]
pub fn create_user(name: String, state: State<AppState>) -> Result<UserView, String> {
    
    let mut user =  User::new(name)
        .map_err(|err: UserError| err.to_string())?;

    let db_conn = state.db_conn();

    {
        let conn = db_conn
            .lock()
            .map_err(|_| "Failed to acquire DB connection lock".to_string())?;

            conn.execute(
                "INSERT INTO users (name, status) VALUES (?1, ?2)",
                params![user.name, user.status.as_str()],
            )
            .map_err(|e| e.to_string())?;

        user.id = Some(conn.last_insert_rowid() as u32);    
    }

    let user_view: UserView = user.into();

    Ok(user_view)
}


#[tauri::command]
pub fn get_activities_users_count_command(state: State<AppState>) -> Result<bool, String> {
    let db_conn = state.db_conn();

    let conn = db_conn
        .lock()
        .map_err(|_| "Failed to acquire DB connection lock".to_string())?;

    let count = User::get_activities_users_count(&conn)
        .map_err(|e| e.to_string())?;

    Ok(count > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_user_success() {
        let name = "Alice".to_string();
        let user = User::new(name.clone()).expect("User should be created");
        
        assert!(user.id.is_none());
        assert_eq!(user.name, name);
    }
    
    #[test]
    fn test_create_user_empty_name() {
        let name = "   ".to_string();
        let user = User::new(name);
        assert!(user.is_err());
    }
}
