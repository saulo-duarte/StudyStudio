use crate::models::{User, Status};
use serde_json::json;

pub struct UserView;

impl UserView {

    pub fn render_user(user: &User) -> String {
        json!({
            "id": user.id,
            "name": user.name,
            "status": match user.status {
                Status::Active => "active",
                Status::Inactive => "inactive",
            },
            "created_at": user.created_at.to_rfc3339(),
            "last_login": user.last_login.map(|d| d.to_rfc3339())
        })
        .to_string() 
    }
}
