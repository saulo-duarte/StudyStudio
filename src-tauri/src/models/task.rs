use chrono::{NaiveDateTime, Utc};
use rusqlite::{Connection, Row, types::{FromSql, FromSqlResult, ValueRef, FromSqlError}};
use std::{convert::TryFrom, str::FromStr};
use std::fmt;
use serde::{Serialize, Deserialize};

use crate::{errors::task_errors::TaskError, utils::format_date::truncate_to_minute};
use crate::utils::sql_types::SqliteDateTime;
use crate::models::tag::Tag;

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
    pub tags: Vec<Tag>,
}

#[derive(Deserialize)]
pub struct FrontendTag {
    pub name: String,
    pub color: String,
}

impl<'a> TryFrom<(&Connection, &Row<'a>)> for Task {
    type Error = rusqlite::Error;

    fn try_from((conn, row): (&Connection, &Row<'a>)) -> Result<Self, Self::Error> {
        let task_id: u32 = row.get("id")?;
        
        let tags = crate::database::repository::TagRepository::get_task_tags(conn, task_id)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?
            .unwrap_or_else(Vec::new);

        Ok(Self {
            id: Some(task_id),
            user_id: row.get("user_id")?,
            name: row.get("name")?,
            description: row.get("description")?,
            status: row.get("status")?,
            created_at: row.get::<_, SqliteDateTime>("created_at")?.into(),
            updated_at: row.get::<_, SqliteDateTime>("updated_at")?.into(),
            due_date: row.get::<_, SqliteDateTime>("due_date")?.into(),
            tags,
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
            tags: Vec::new(),
        })
    }
}