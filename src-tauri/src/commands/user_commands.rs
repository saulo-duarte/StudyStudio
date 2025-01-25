use tauri::{command, State};
use crate::controllers::user_controller::UserController;
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