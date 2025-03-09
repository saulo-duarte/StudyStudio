use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum TaskError {
    InvalidName(String),
    InvalidStatus(String),
    InvalidPriority(String),
    InvalidDate(String),
    DatabaseError(String),
    InvalidTag(String),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::InvalidName(msg) => write!(f, "Invalid task name: {}", msg),
            TaskError::InvalidStatus(msg) => write!(f, "Invalid task status: {}", msg),
            TaskError::InvalidPriority(msg) => write!(f, "Invalid task priority: {}", msg),
            TaskError::InvalidDate(msg) => write!(f, "Invalid task date: {}", msg),
            TaskError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            TaskError::InvalidTag(msg) => write!(f, "Invalid tag: {}", msg),
        }
    }
}

impl std::error::Error for TaskError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_name_error() {
        let error = TaskError::InvalidName("Task name cannot be empty".to_string());
        assert_eq!(
            error.to_string(),
            "Invalid task name: Task name cannot be empty"
        );
        assert!(matches!(error, TaskError::InvalidName(_)));
    }

    #[test]
    fn test_invalid_status_error() {
        let error = TaskError::InvalidStatus("Status must be 'active' or 'inactive'".to_string());
        assert_eq!(
            error.to_string(),
            "Invalid task status: Status must be 'active' or 'inactive'"
        );
        assert!(matches!(error, TaskError::InvalidStatus(_)));
    }

    #[test]
    fn test_invalid_priority_error() {
        let error = TaskError::InvalidPriority("Priority must be between 1 and 5".to_string());
        assert_eq!(
            error.to_string(),
            "Invalid task priority: Priority must be between 1 and 5"
        );
        assert!(matches!(error, TaskError::InvalidPriority(_)));
    }

    #[test]
    fn test_invalid_date_error() {
        let error = TaskError::InvalidDate("Date format must be YYYY-MM-DD".to_string());
        assert_eq!(
            error.to_string(),
            "Invalid task date: Date format must be YYYY-MM-DD"
        );
        assert!(matches!(error, TaskError::InvalidDate(_)));
    }

    #[test]
    fn test_database_error() {
        let error = TaskError::DatabaseError("Failed to connect to database".to_string());
        assert_eq!(
            error.to_string(),
            "Database error: Failed to connect to database"
        );
        assert!(matches!(error, TaskError::DatabaseError(_)));
    }

    #[test]
    fn test_debug_output() {
        let error = TaskError::InvalidName("Test error".to_string());
        let debug_output = format!("{:?}", error);
        assert_eq!(debug_output, "InvalidName(\"Test error\")");
    }
}