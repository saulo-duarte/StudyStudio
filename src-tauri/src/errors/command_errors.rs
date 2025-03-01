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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::UserError;

    #[test]
    fn test_user_error_to_command_error_invalid_name() {
        let user_error = UserError::InvalidName("Nome inválido".to_string());

        let command_error: CommandError = user_error.into();

        if let CommandError::Validation(msg) = command_error {
            assert_eq!(msg, "Nome inválido");
        } else {
            panic!("Esperado CommandError::Validation");
        }
    }
    
}
