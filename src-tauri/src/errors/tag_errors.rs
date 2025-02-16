use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum TagError {
  InvalidName(String),
  DatabaseError(String),
}

impl std::error::Error for TagError {}

impl std::fmt::Display for TagError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
        TagError::InvalidName(msg) => write!(f, "Invalid task name: {}", msg),
        TagError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
    }
  }
}