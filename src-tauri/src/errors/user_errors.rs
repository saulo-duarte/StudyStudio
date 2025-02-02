use std::fmt;

#[derive(Debug)]
pub enum UserError {
    InvalidName(String),
    InvalidStatus(String),
    DatabaseError(String),
}

impl std::error::Error for UserError {}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::InvalidName(msg) => write!(f, "Invalid user name: {}", msg),
            UserError::InvalidStatus(status) => write!(f, "Invalid user status: {}", status),
            UserError::DatabaseError(err) => write!(f, "Database error: {}", err),
        }
    }
}

impl From<rusqlite::Error> for UserError {
    fn from(e: rusqlite::Error) -> Self {
        UserError::DatabaseError(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_invalid_name_error_display() {
        let error = UserError::InvalidName("Empty".into());
        assert!(error.to_string().contains("Invalid user name"));
    }
    
    #[test]
    fn test_invalid_status_error_display() {
        let error = UserError::InvalidStatus("pending".into());
        assert!(error.to_string().contains("Invalid user status"));
    }
}
