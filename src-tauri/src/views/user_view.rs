use chrono::NaiveDateTime;
use serde::Serialize;
use crate::models::user::User;

#[derive(Debug, Serialize)]
pub struct UserView {
    pub id: Option<u32>,
    pub name: String,
    pub status: String,
    pub created_at: String,
}

impl From<User> for UserView{
    fn from(user: User) -> Self {
        UserView {
            id: user.id,
            name: user.name,
            status: user.status.as_str().to_string(),
            created_at: format_datetime(user.created_at),        
        }
    }
}

fn format_datetime(dt: NaiveDateTime) -> String {
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}