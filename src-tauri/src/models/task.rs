use chrono::{NaiveDateTime, Utc};
use rusqlite::{Row, types::{FromSql, FromSqlResult, ValueRef, FromSqlError}};
use std::{convert::TryFrom, str::FromStr};
use std::fmt;
use serde::{Serialize, Deserialize};

use crate::{errors::task_errors::TaskError, utils::format_date::truncate_to_minute};
use crate::utils::sql_types::SqliteDateTime;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Paused,
    Done,
    Archived,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Todo => "todo",
            Self::InProgress => "in_progress",
            Self::Paused => "paused",
            Self::Done => "done",
            Self::Archived => "archived",
        }
    }
}

impl FromStr for TaskStatus {
    type Err = TaskError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "todo" => Ok(Self::Todo),
            "in_progress" | "in progress" => Ok(Self::InProgress),
            "paused" => Ok(Self::Paused),
            "done" => Ok(Self::Done),
            "archived" => Ok(Self::Archived),
            _ => Err(TaskError::InvalidStatus(s.to_string())),
        }
    }
}

impl FromSql for TaskStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_str()?
            .parse()
            .map_err(|e: TaskError| FromSqlError::Other(Box::new(e)))
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Option<u32>,
    pub user_id: u32,
    pub name: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub due_date: NaiveDateTime,
}

impl<'a> TryFrom<&Row<'a>> for Task {
    type Error = rusqlite::Error;

    fn try_from(row: &Row<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.get("id")?,
            user_id: row.get("user_id")?,
            name: row.get("name")?,
            description: row.get("description")?,
            status: row.get("status")?,
            created_at: row.get::<_, SqliteDateTime>("created_at")?.into(), 
            updated_at: row.get::<_, SqliteDateTime>("updated_at")?.into(), 
            due_date: row.get::<_, SqliteDateTime>("due_date")?.into(), 
        })
    }
}


impl Task {
    pub fn new(
        name: String,
        user_id: u32,
        description: Option<String>,
        due_date: NaiveDateTime,
    ) -> Result<Self, TaskError> {
        let now = truncate_to_minute(Utc::now().naive_utc());

        if name.trim().is_empty() {
            return Err(TaskError::InvalidName("Task name cannot be empty".into()));
        }

        if due_date < now {
            return Err(TaskError::InvalidDate("Due date cannot be in the past".into()));
        }

        Ok(Self {
            id: None,
            user_id,
            name,
            description,
            status: TaskStatus::Todo,
            created_at: now,
            updated_at: now,
            due_date,
        })
    }

    pub fn with_id(mut self, id: u32) -> Self {
        self.id = Some(id);
        self
    }
}

#[cfg(test)]mod tests {
    use super::*;
    use rusqlite::{Connection, params};

    fn test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE tasks (
                id INTEGER PRIMARY KEY,
                user_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                due_date TEXT NOT NULL
            )",
            [],
        ).unwrap();
        conn
    }

    #[test]
    fn test_invalid_status_conversion() {
        let conn = test_db();
        let now = Utc::now().naive_utc().to_string();
        
        conn.execute(
            "INSERT INTO tasks 
            (user_id, name, status, created_at, updated_at, due_date)
            VALUES (1, 'Bad Task', 'invalid', ?, ?, ?)",
            params![now, now, now],
        ).unwrap();

        let mut stmt = conn.prepare("SELECT * FROM tasks").unwrap();
        let result = stmt.query_row([], |row| Task::try_from(row));
        
        assert!(result.is_err());
    }
}