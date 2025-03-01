use rusqlite::{Connection, params};
use crate::{models::User, errors::UserError};

pub struct UserRepository;

impl UserRepository {
    pub fn create(conn: &Connection, user: &mut User) -> Result<(), UserError> {
        conn.execute(
            "INSERT INTO users (name, status, created_at) VALUES (?1, ?2, ?3)",
            params![user.name, user.status.to_string(), user.created_at.format("%Y-%m-%d %H:%M:%S").to_string()],
        )
        .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        user.id = Some(conn.last_insert_rowid() as u32);
        Ok(())
    }

    pub fn count_active(conn: &Connection) -> Result<u32, UserError> {
        let mut stmt = conn
            .prepare("SELECT COUNT(*) FROM users WHERE status = 'active'")
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        let count: u32 = stmt
            .query_row([], |row| row.get(0))
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(count)
    }

    pub fn find_active_id(conn: &Connection) -> Result<Option<u32>, UserError> {
        let mut stmt = conn
            .prepare("SELECT id FROM users WHERE status = 'active' LIMIT 1")
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        let mut rows = stmt
            .query([])
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use chrono::{Utc, NaiveDateTime};
    use crate::models::UserStatus;
    use std::str::FromStr;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE users (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        ).unwrap();
        conn
    }

    #[test]
    fn test_create_user() {
        let conn = setup_db();
        let mut user = User::new("Alice".to_string()).unwrap();
        assert!(UserRepository::create(&conn, &mut user).is_ok());
        assert!(user.id.is_some());

        let saved_user: User = conn.query_row(
            "SELECT id, name, status, created_at FROM users WHERE id = ?",
            [user.id.unwrap()],
            |row| Ok(User {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                status: UserStatus::from_str(&row.get::<_, String>(2)?).unwrap(),
                created_at: NaiveDateTime::parse_from_str(&row.get::<_, String>(3)?, "%Y-%m-%d %H:%M:%S").unwrap(), // Coluna 3: created_at (TEXT)
            })
        ).unwrap();

        assert_eq!(saved_user.name, "Alice");
        assert_eq!(saved_user.status, UserStatus::Active);
    }

    #[test]
    fn test_count_active() {
        let conn = setup_db();
        let mut user1 = User::new("Alice".to_string()).unwrap();
        UserRepository::create(&conn, &mut user1).unwrap();
        
        let mut user2 = User { 
            id: None, 
            name: "Inactive".to_string(), 
            status: UserStatus::Inactive, 
            created_at: Utc::now().naive_utc() 
        };
        UserRepository::create(&conn, &mut user2).unwrap();

        assert_eq!(UserRepository::count_active(&conn).unwrap(), 1);
    }

    #[test]
    fn test_find_active_id() {
        let conn = setup_db();
        assert!(UserRepository::find_active_id(&conn).unwrap().is_none());

        let mut user = User::new("Alice".to_string()).unwrap();
        UserRepository::create(&conn, &mut user).unwrap();
        assert_eq!(UserRepository::find_active_id(&conn).unwrap(), Some(user.id.unwrap()));
    }
}