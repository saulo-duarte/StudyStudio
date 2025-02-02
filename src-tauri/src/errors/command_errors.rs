use crate::errors::UserError;

#[derive(Debug, serde::Serialize)]
pub enum CommandError {
    Validation(String),
    Database(String),
    LockFailed,
}

impl From<UserError> for CommandError {
    fn from(e: UserError) -> Self {
        match e {
            UserError::InvalidName(msg) => CommandError::Validation(msg), 
            _ => CommandError::Database(e.to_string()),
        }
    }
}