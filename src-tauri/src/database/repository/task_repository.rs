use rusqlite::{params, Connection, ToSql};
use chrono::Utc;

use crate::models::{TaskStatus, Tag};
use crate::{errors::TaskError, models::Task};
use crate::utils::format_date::truncate_to_minute;
use crate::database::repository::TagRepository;

pub struct TaskRepository;

impl TaskRepository {
    pub fn insert_task(conn: &Connection, task: &mut Task) -> Result<(), rusqlite::Error> {

        let created_at = truncate_to_minute(task.created_at);
        let updated_at = truncate_to_minute(task.updated_at);
        let due_date = truncate_to_minute(task.due_date);

        conn.execute(
            "INSERT INTO tasks (name, user_id, description, status, created_at, updated_at, due_date)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                task.name,
                task.user_id,
                task.description,
                task.status.to_string(),
                created_at.format("%Y-%m-%dT%H:%M").to_string(),
                updated_at.format("%Y-%m-%dT%H:%M").to_string(),
                due_date.format("%Y-%m-%dT%H:%M").to_string(),
            ]
        )?;
        
        task.id = Some(conn.last_insert_rowid() as u32);
        Ok(())
    }

    pub fn get_task_by_id(conn: &Connection, task_id: u32) -> Result<Task, TaskError> {
        let mut stmt = conn.prepare(
            "SELECT 
                id, user_id, name, description, status, created_at, updated_at, due_date
             FROM tasks
             WHERE id = ?1",
        )
        .map_err(|e| TaskError::DatabaseError(e.to_string()))?;
    
        // Recupera a task usando a implementação de TryFrom que já recebe (conn, row)
        let mut task = stmt.query_row([task_id], |row| Task::try_from((conn, row)))
            .map_err(|e| TaskError::DatabaseError(e.to_string()))?;
    
        // Faz o JOIN para recuperar as tags associadas à task
        let mut tag_stmt = conn.prepare(
            "SELECT t.id, t.tag_name, t.tag_color
             FROM tags t
             JOIN task_tags tt ON t.id = tt.tag_id
             WHERE tt.task_id = ?1"
        ).map_err(|e| TaskError::DatabaseError(e.to_string()))?;
    
        let tags = tag_stmt
            .query_map([task_id], |row| {
                Ok(crate::models::tag::Tag {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    color: row.get(2)?,
                })
            })
            .map_err(|e| TaskError::DatabaseError(e.to_string()))?
            .collect::<Result<Vec<crate::models::tag::Tag>, _>>()
            .map_err(|e| TaskError::DatabaseError(e.to_string()))?;
    
        task.tags = tags;
    
        Ok(task)
    }
        
    pub fn get_all_tasks(conn: &Connection) -> Result<Vec<Task>, TaskError> {
        let mut stmt = conn
            .prepare(
                "SELECT 
                    id, 
                    user_id, 
                    name, 
                    description, 
                    status, 
                    created_at, 
                    updated_at, 
                    due_date
                FROM tasks",
            )
            .map_err(|e| TaskError::DatabaseError(e.to_string()))?;
    
        let tasks = stmt
            .query_map([], |row| Task::try_from((conn, row)))
            .map_err(|e| TaskError::DatabaseError(e.to_string()))?
            .collect::<Result<Vec<Task>, _>>()
            .map_err(|e| TaskError::DatabaseError(e.to_string()))?;
    
        let tasks_with_tags: Result<Vec<Task>, TaskError> = tasks
            .into_iter()
            .map(|mut task| {
                if let Some(task_id) = task.id {
                    let tags = TagRepository::get_task_tags(conn, task_id)
                        .map_err(|e| TaskError::DatabaseError(e.to_string()))?;
                    task.tags = tags.unwrap_or_default();
                }
                Ok(task)
            })
            .collect();
    
        tasks_with_tags
    }
    
    pub fn delete_task(conn: &Connection, task_id: u32) -> Result<(), TaskError> {
        conn.execute(
            "DELETE FROM tasks WHERE id = ?1",
            params![task_id],
        )
        .map_err(|e| TaskError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    pub fn update_task(
        conn: &Connection,
        task_id: u32,
        name: Option<String>,
        description: Option<String>,
        status: Option<TaskStatus>,
        due_date: Option<chrono::NaiveDateTime>,
        tags: Option<Vec<Tag>>,
    ) -> Result<(), TaskError> {
        let mut query = String::from("UPDATE tasks SET ");
        let mut updates = Vec::new();
        let mut params: Vec<Box<dyn ToSql>> = Vec::new();

        if let Some(name) = name {
            updates.push("name = ?");
            params.push(Box::new(name));
        }

        if let Some(description) = description {
            updates.push("description = ?");
            params.push(Box::new(description));
        }

        if let Some(status) = status {
            updates.push("status = ?");
            params.push(Box::new(status.to_string()));
        }

        if let Some(due_date) = due_date {
            let due_date = truncate_to_minute(due_date);
            updates.push("due_date = ?");
            params.push(Box::new(due_date.format("%Y-%m-%dT%H:%M").to_string()));
        }

        let updated_at = truncate_to_minute(Utc::now().naive_utc());    
        updates.push("updated_at = ?");
        params.push(Box::new(updated_at.format("%Y-%m-%dT%H:%M").to_string()));

        if updates.is_empty() {
            return Err(TaskError::InvalidStatus("Nenhum campo para atualizar".to_owned()));
        }

        query.push_str(&updates.join(", "));
        query.push_str(" WHERE id = ?");
        params.push(Box::new(task_id));

        let params_ref: Vec<&dyn ToSql> = params.iter().map(|p| &**p).collect();

        conn.execute(&query, params_ref.as_slice())
            .map_err(|e| TaskError::DatabaseError(e.to_string()))?;

        if let Some(tags) = tags {
            TagRepository::update_task_tags(conn, task_id, &tags)
                .map_err(|e| TaskError::DatabaseError(e.to_string()))?;
        }

        Ok(())
    }

    pub fn update_task_tags(conn: &Connection, task_id: u32, tags: &[String]) -> Result<(), TaskError> {
        conn.execute(
            "DELETE FROM task_tags WHERE task_id = ?1",
            params![task_id],
        ).map_err(|e| TaskError::DatabaseError(e.to_string()))?;

        for tag in tags {
            conn.execute(
                "INSERT INTO task_tags (task_id, tag_name) VALUES (?1, ?2)",
                params![task_id, tag],
            ).map_err(|e| TaskError::DatabaseError(e.to_string()))?;
        }

        Ok(())
    }
    
}
