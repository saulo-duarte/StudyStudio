use rusqlite::{Connection, params};

use crate::{errors::TaskError, models::Task};
use crate::utils::format_date::truncate_to_minute;

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
            .query_map([], |row| Task::try_from(row))
            .map_err(|e| TaskError::DatabaseError(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| TaskError::DatabaseError(e.to_string()))?;
    
        Ok(tasks)
    }
    
}