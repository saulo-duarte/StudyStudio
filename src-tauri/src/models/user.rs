use chrono::{NaiveDateTime, Utc};
use std::str::FromStr;
use crate::errors::user_errors::UserError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserStatus {
  Active,
  Inactive,
}

impl UserStatus {
  pub fn as_str(&self) -> &'static str {
    match self {
      UserStatus::Active => "active",
      UserStatus::Inactive => "inactive",
    }
  }
}

impl FromStr for UserStatus {
  type Err = UserError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s.to_lowercase().as_str() {
          "active" => Ok(UserStatus::Active),
          "inactive" => Ok(UserStatus::Inactive),
          _ => Err(UserError::InvalidStatus(s.to_string())),
      }
  }
}

impl std::fmt::Display for UserStatus {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self.as_str())
  }
}

#[derive(Debug, Clone)]
pub struct User {
  pub id: Option<u32>,
  pub name: String,
  pub status: UserStatus,
  pub created_at: NaiveDateTime,
}

impl User {
  pub fn new(name: String) -> Result<Self, UserError> {
    if name.trim().is_empty() {
      return Err(UserError::InvalidName("Name cannot be empty".to_string()));
    }
    Ok(User {
      id: None,
      name,
      status: UserStatus::Active,
      created_at: Utc::now().naive_utc(),
    })
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_status_as_str() {
        let active_status = UserStatus::Active;
        let inactive_status = UserStatus::Inactive;

        assert_eq!(active_status.as_str(), "active");
        assert_eq!(inactive_status.as_str(), "inactive");
    }

    #[test]
    fn test_user_status_from_str() {
        let active_status: Result<UserStatus, _> = "active".parse();
        assert_eq!(active_status.unwrap(), UserStatus::Active);

        let inactive_status: Result<UserStatus, _> = "inactive".parse();
        assert_eq!(inactive_status.unwrap(), UserStatus::Inactive);

        let invalid_status: Result<UserStatus, _> = "unknown".parse();
        assert!(invalid_status.is_err());
    }

    #[test]
    fn test_user_status_display() {
        let active_status = UserStatus::Active;
        let inactive_status = UserStatus::Inactive;

        assert_eq!(format!("{}", active_status), "active");
        assert_eq!(format!("{}", inactive_status), "inactive");
    }

    #[test]
    fn test_create_user_with_valid_name() {
        let user_name = "Alice".to_string();
        let user = User::new(user_name.clone());

        assert!(user.is_ok());
        let user = user.unwrap();
        assert_eq!(user.name, user_name);
        assert_eq!(user.status, UserStatus::Active);
    }

    #[test]
    fn test_create_user_with_empty_name() {
        let user_name = "".to_string();
        let user = User::new(user_name);

        assert!(user.is_err());
        if let Err(UserError::InvalidName(msg)) = user {
            assert_eq!(msg, "Name cannot be empty");
        } else {
            panic!("Expected InvalidName error");
        }
    }

    #[test]
    fn test_user_creation_has_valid_timestamp() {
        let user = User::new("Bob".to_string()).unwrap();
        let current_time = Utc::now().naive_utc();

        assert!(user.created_at <= current_time);
    }
}
