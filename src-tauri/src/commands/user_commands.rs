use tauri::{command, State};
use crate::controllers::user_controller::UserController;
use crate::models::User;
use crate::views::user_view::UserView;
use crate::AppState;

#[command]
pub fn register_user(
    state: State<'_, AppState>,
    name: String
) -> Result<String, String> {
    match UserController::create_user(&state, &name){
        Ok(user) => {
            let user_json = UserView::render_user(&user);
            Ok(user_json)
        }
        Err(e) => Err(format!("Error creating user: {}", e)),
    }
}

#[command]
pub fn get_active_users_count(
    state: State<'_, AppState>
) -> Result<i64, String> {
    let db_conn = state.db_conn();
    let conn = db_conn.lock().map_err(|_| "Failed to acquire database lock")?;
    
    User::count_activite_users(&conn)
        .map_err(|e| format!("Error counting active users: {}", e))
}