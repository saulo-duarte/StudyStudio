use std::fmt;

#[derive(Debug)]
pub enum UserError {
  InvalidName(String),
  InvalidStatus(String),
  DatabaseError(String),
}

impl std::error::Error for UserError {}

impl fmt::Display for UserError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      UserError::InvalidName(msg) => write!(f, "Invalid user name: {}", msg),
      UserError::InvalidStatus(msg) => write!(f, "Invalid user status: {}", msg),
      UserError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
    }
  }
}

impl From<rusqlite::Error> for UserError {
  fn from(value: rusqlite::Error) -> Self {
    UserError::DatabaseError(value.to_string())
  }
}

#[cfg(test)]
use rusqlite;

#[test]
fn test_invalid_name_error_display() {
    let error = UserError::InvalidName("John123".to_string());
    assert_eq!(format!("{}", error), "Invalid user name: John123");
}

#[test]
fn test_invalid_status_error_display() {
    let error = UserError::InvalidStatus("Banned".to_string());
    assert_eq!(format!("{}", error), "Invalid user status: Banned");
}

#[test]
fn test_database_error_display() {
    let error = UserError::DatabaseError("Connection failed".to_string());
    assert_eq!(format!("{}", error), "Database error: Connection failed");
}

#[test]
fn test_from_rusqlite_error() {
    let sqlite_error = rusqlite::Error::InvalidQuery;
    let user_error: UserError = sqlite_error.into();
    assert!(matches!(user_error, UserError::DatabaseError(_)));
}
