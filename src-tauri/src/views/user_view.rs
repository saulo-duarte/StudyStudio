use serde::Serialize;

use crate::models::user::User;
use crate::utils::format_date::format_datetime;

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

#[cfg(test)]
mod user_view_tests {
    use super::*;
    use chrono::Utc;
    use crate::models::user::UserStatus;

    fn create_test_user(id: Option<u32>, name: &str, status: UserStatus) -> User {
        User {
            id,
            name: name.to_string(),
            status,
            created_at: Utc::now().naive_utc(),
        }
    }

    #[test]
    fn test_conversion_from_user() {
        let user = create_test_user(Some(1), "Alice", UserStatus::Active);
        let user_view = UserView::from(user.clone());

        assert_eq!(user_view.id, user.id);
        assert_eq!(user_view.name, user.name);
        assert_eq!(user_view.status, "active");
        assert_eq!(user_view.created_at, format_datetime(user.created_at));
    }

    #[test]
    fn test_inactive_user_status() {
        let user = create_test_user(None, "Bob", UserStatus::Inactive);
        let user_view = UserView::from(user);
        assert_eq!(user_view.status, "inactive");
    }

    #[test]
    fn tesast_serialization() {
        let user = create_test_user(Some(1), "Charlie", UserStatus::Active);
        let user_view = UserView::from(user);
        let json = serde_json::to_string(&user_view).unwrap();

        assert!(json.contains(r#""id":1"#));
        assert!(json.contains(r#""name":"Charlie""#));
        assert!(json.contains(r#""status":"active""#));
        assert!(json.contains(r#""created_at":"#));
    }
}