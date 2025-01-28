use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result};

#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive (Debug, Clone)]
pub enum Status {
    Active,
    Inactive
}

#[derive(Debug, Clone)]
pub struct Login {
    pub id: u32,
    pub user_id: u32,
    pub timestamp: DateTime<Utc>
}

impl User {
    pub fn new(name: &str) -> Self {
        Self {
            id: 0,
            name: name.to_string(),
            status: Status::Active,
            created_at: Utc::now(),
            last_login: None,
        }
    }

    pub fn save(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO users (name, status, created_at, last_login)
                VALUES (?1, ?2, ?3, ?4)",
            params![
                self.name,
                match self.status {
                    Status::Active => "active",
                    Status::Inactive => "inactive",
                },
                self.created_at.to_rfc3339(),
                self.last_login.map(|dt| dt.to_rfc3339()),
            ],
        )?;
        Ok(())
    }

    pub fn count_activite_users(conn: &Connection) -> Result<i64> {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM users WHERE status = 'active'", 
            [], 
            |row| row.get(0)
        )?;
        Ok(count)
    }
}