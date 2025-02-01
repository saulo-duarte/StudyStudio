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
            return Err(UserError::InvalidName("Name cannot be empty".into()));
        }
        Ok(User {
            id: None,
            name,
            status: UserStatus::Active,
            created_at: Utc::now().naive_utc(),
        })
    }
}