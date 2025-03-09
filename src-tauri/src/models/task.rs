use chrono::{NaiveDateTime, Utc};
use rusqlite::{Connection, Row, types::{FromSql, FromSqlResult, ValueRef, FromSqlError}};
use serde::{Serialize, Deserialize};
use std::fmt;
use std::str::FromStr;

use crate::{errors::task_errors::TaskError, utils::{truncate_to_minute, SqliteDateTime}};
use crate::models::tag::Tag;

pub mod task_status {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum TaskStatus {
        Todo,
        InProgress,
        Done,
        Backlog,
    }

    impl TaskStatus {
        pub fn as_str(&self) -> &'static str {
            match self {
                Self::Todo => "todo",
                Self::InProgress => "in_progress",
                Self::Done => "done",
                Self::Backlog => "backlog",
            }
        }
    }

    impl FromStr for TaskStatus {
        type Err = TaskError;
    
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().replace('_', "").as_str() {
                "todo" => Ok(Self::Todo),
                "inprogress" | "in progress" => Ok(Self::InProgress),
                "done" => Ok(Self::Done),
                "backlog" => Ok(Self::Backlog),
                _ => Err(TaskError::InvalidStatus(s.to_string())),
            }
        }
    }
    

    impl FromSql for TaskStatus {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            value
                .as_str()?
                .parse()
                .map_err(|e: TaskError| FromSqlError::Other(Box::new(e)))
        }
    }

    impl fmt::Display for TaskStatus {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.as_str())
        }
    }
}

pub mod task_priority {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum TaskPriority {
        Low,
        Medium,
        High,
    }

    impl TaskPriority {
        pub fn as_str(&self) -> &'static str {
            match self {
                Self::Low => "low",
                Self::Medium => "medium",
                Self::High => "high",
            }
        }
    }

    impl FromStr for TaskPriority {
        type Err = TaskError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "low" => Ok(Self::Low),
                "medium" => Ok(Self::Medium),
                "high" => Ok(Self::High),
                _ => Err(TaskError::InvalidPriority(s.to_string())),
            }
        }
    }

    impl FromSql for TaskPriority {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            value
                .as_str()?
                .parse()
                .map_err(|e: TaskError| FromSqlError::Other(Box::new(e)))
        }
    }

    impl fmt::Display for TaskPriority {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.as_str())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Option<u32>,
    pub user_id: u32,
    pub title: String,
    pub description: Option<String>,
    pub status: task_status::TaskStatus,
    pub priority: task_priority::TaskPriority,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub due_date: NaiveDateTime,
    pub tags: Vec<Tag>,
}

impl<'a> TryFrom<(&Connection, &Row<'a>)> for Task {
    type Error = rusqlite::Error;

    fn try_from((conn, row): (&Connection, &Row<'a>)) -> Result<Self, Self::Error> {
        let task_id: u32 = row.get("id")?;
        
        let tags = crate::repository::TagRepository::get_task_tags(conn, task_id)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?
            .unwrap_or_else(Vec::new);

        Ok(Self {
            id: Some(task_id),
            user_id: row.get("user_id")?,
            title: row.get("title")?,
            description: row.get("description")?,
            status: row.get("status")?,
            priority: row.get("priority")?,
            created_at: row.get::<_, SqliteDateTime>("created_at")?.into(),
            updated_at: row.get::<_, SqliteDateTime>("updated_at")?.into(),
            due_date: row.get::<_, SqliteDateTime>("due_date")?.into(),
            tags,
        })
    }
}

#[derive(Deserialize)]
pub struct FrontendTag {
    pub id: Option<u32>,
    pub name: String,
    pub color: String,
}

impl Task {
    pub fn new(
        title: String,
        user_id: u32,
        description: Option<String>,
        priority: Option<task_priority::TaskPriority>,
    ) -> Result<Self, TaskError> {
        if title.trim().is_empty() {
            return Err(TaskError::InvalidName("Task title cannot be empty".to_string()));
        }

        let now = truncate_to_minute(Utc::now().naive_utc());
        Ok(Task {
            id: None,
            user_id,
            title,
            description,
            status: task_status::TaskStatus::Todo,
            priority: priority.unwrap_or(task_priority::TaskPriority::Medium),
            created_at: now,
            updated_at: now,
            due_date: now,
            tags: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_new_valid() {
        let task = Task::new(
            "Test Task".to_string(),
            1,
            Some("Test description".to_string()),
            Some(task_priority::TaskPriority::High),
        );
        assert!(task.is_ok());

        let task = task.unwrap();
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.user_id, 1);
        assert_eq!(task.description, Some("Test description".to_string()));
        assert_eq!(task.status, task_status::TaskStatus::Todo);
        assert_eq!(task.priority, task_priority::TaskPriority::High);
        assert_eq!(task.tags.len(), 0);
        assert!(task.id.is_none());
    }

    #[test]
    fn test_task_new_default_priority() {
        let task = Task::new(
            "Test Task".to_string(),
            1,
            None,
            None, 
        );
        assert!(task.is_ok());

        let task = task.unwrap();
        assert_eq!(task.priority, task_priority::TaskPriority::Medium);
    }

    #[test]
    fn test_task_new_invalid_name() {
        let task = Task::new(
            "".to_string(),
            1,
            None,
            Some(task_priority::TaskPriority::Low),
        );
        assert!(task.is_err());

        if let Err(e) = task {
            assert_eq!(
                e.to_string(),
                "Invalid task name: Task title cannot be empty"
            );
            assert!(matches!(e, TaskError::InvalidName(_)));
        }
    }

    #[test]
    fn test_task_status_display() {
        assert_eq!(
            task_status::TaskStatus::Todo.to_string(),
            "todo"
        );
        assert_eq!(
            task_status::TaskStatus::InProgress.to_string(),
            "in_progress"
        );
        assert_eq!(
            task_status::TaskStatus::Done.to_string(),
            "done"
        );
        assert_eq!(
            task_status::TaskStatus::Backlog.to_string(),
            "backlog"
        );
    }

    #[test]
    fn test_task_priority_display() {
        assert_eq!(
            task_priority::TaskPriority::Low.to_string(),
            "low"
        );
        assert_eq!(
            task_priority::TaskPriority::Medium.to_string(),
            "medium"
        );
        assert_eq!(
            task_priority::TaskPriority::High.to_string(),
            "high"
        );
    }

    #[test]
    fn test_task_status_from_str() {
        assert_eq!(
            "todo".parse::<task_status::TaskStatus>().unwrap(),
            task_status::TaskStatus::Todo
        );
        assert_eq!(
            "In Progress".parse::<task_status::TaskStatus>().unwrap(),
            task_status::TaskStatus::InProgress
        );
        assert!(matches!(
            "invalid".parse::<task_status::TaskStatus>(),
            Err(TaskError::InvalidStatus(_))
        ));
    }

    #[test]
    fn test_task_priority_from_str() {
        assert_eq!(
            "low".parse::<task_priority::TaskPriority>().unwrap(),
            task_priority::TaskPriority::Low
        );
        assert_eq!(
            "HIGH".parse::<task_priority::TaskPriority>().unwrap(),
            task_priority::TaskPriority::High
        );
        assert!(matches!(
            "invalid".parse::<task_priority::TaskPriority>(),
            Err(TaskError::InvalidPriority(_))
        ));
    }
}