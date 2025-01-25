use crate::{models::{Status, User}, AppState};

pub struct UserController;

impl UserController {

    pub fn create_user(
        state: &AppState,
        name: &str
    ) -> Result<User, Box<dyn std::error::Error>> {
        let user = User::new(name);
        
        let db_conn = state.db_conn();
        let conn = db_conn.lock().unwrap();

        user.save(&conn)?;

        Ok(user)
    }

    pub fn update_user_status(user: &mut User, new_status: Status) {
        user.status = new_status
    }
}