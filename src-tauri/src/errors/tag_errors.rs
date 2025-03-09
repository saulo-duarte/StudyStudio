use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum TagError {
    InvalidName(String),
    InvalidColor(String),
    DatabaseError(String),
}

impl std::error::Error for TagError {}

impl fmt::Display for TagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TagError::InvalidName(msg) => write!(f, "Invalid tag name: {}", msg),
            TagError::InvalidColor(msg) => write!(f, "Invalid tag color: {}", msg),
            TagError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_name_error() {
        let error = TagError::InvalidName("Tag name cannot be empty".to_string());
        assert_eq!(
            error.to_string(),
            "Invalid tag name: Tag name cannot be empty"
        );
        assert!(matches!(error, TagError::InvalidName(_)));
    }

    #[test]
    fn test_invalid_color_error() {
        let error = TagError::InvalidColor("Tag color cannot be empty".to_string());
        assert_eq!(
            error.to_string(),
            "Invalid tag color: Tag color cannot be empty"
        );
        assert!(matches!(error, TagError::InvalidColor(_)));
    }

    #[test]
    fn test_database_error() {
        let error = TagError::DatabaseError("Failed to insert tag".to_string());
        assert_eq!(
            error.to_string(),
            "Database error: Failed to insert tag"
        );
        assert!(matches!(error, TagError::DatabaseError(_)));
    }

    #[test]
    fn test_tag_error_display() {
        assert_eq!(
            TagError::InvalidName("Invalid name".into()).to_string(),
            "Invalid tag name: Invalid name"
        );
        assert_eq!(
            TagError::InvalidColor("Invalid color".into()).to_string(),
            "Invalid tag color: Invalid color"
        );
        assert_eq!(
            TagError::DatabaseError("Database error".into()).to_string(),
            "Database error: Database error"
        );
    }

    #[test]
    fn test_debug_output() {
        let error = TagError::InvalidName("Test error".to_string());
        let debug_output = format!("{:?}", error);
        assert_eq!(debug_output, "InvalidName(\"Test error\")");

        let error = TagError::InvalidColor("Color error".to_string());
        let debug_output = format!("{:?}", error);
        assert_eq!(debug_output, "InvalidColor(\"Color error\")");

        let error = TagError::DatabaseError("DB error".to_string());
        let debug_output = format!("{:?}", error);
        assert_eq!(debug_output, "DatabaseError(\"DB error\")");
    }
}