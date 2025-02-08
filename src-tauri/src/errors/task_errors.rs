use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum TaskError {
    InvalidName(String),
    InvalidStatus(String),
    InvalidDate(String),
    DatabaseError(String),
}

impl std::error::Error for TaskError {}

impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskError::InvalidName(msg) => write!(f, "Invalid task name: {}", msg),
            TaskError::InvalidStatus(status) => write!(f, "Invalid task status: {}", status),
            TaskError::InvalidDate(date) => write!(f, "Invalid date: {}", date),
            TaskError::DatabaseError(err) => write!(f, "Database error: {}", err),
        }
    }
}

