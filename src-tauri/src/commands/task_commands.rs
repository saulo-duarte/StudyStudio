use chrono::NaiveDateTime;
use tauri::State;

use crate::{
    database::repository::TaskRepository, 
    errors::TaskError, 
    models::Task, 
    views::TaskView, AppState,
};

#[tauri::command]
pub fn create_task(
    name: String,
    user_id: u32,
    description: Option<String>,
    due_date: String,
    state: State<AppState>
) -> Result<TaskView, String> {
    let date_replaced = due_date.replace("Z", "");

    let trimmed_date = if date_replaced.len() > 16 {
        &date_replaced[..16]
    } else {
        &date_replaced
    };

    let due_date = NaiveDateTime::parse_from_str(trimmed_date, "%Y-%m-%dT%H:%M")
        .map_err(|e| format!("Invalid date format: {}", e))?;
    
    let mut task = Task::new(name, user_id, description, due_date)
        .map_err(|err| err.to_string())?;

    let db_conn = state.db_conn();
    let conn = db_conn.lock()
        .map_err(|_| "Database lock failed")?;

    TaskRepository::insert_task(&conn, &mut task)
        .map_err(|e| format!("Database error: {}", e))?;

    Ok(task.into())
}

#[tauri::command]
pub fn get_all_tasks(state: State<AppState>) -> Result<Vec<Task>, TaskError> {
    let db_conn = state.db_conn();
    let conn = db_conn.lock().map_err(|e| TaskError::DatabaseError(e.to_string()))?;
    TaskRepository::get_all_tasks(&conn)
        .map_err(|e| TaskError::DatabaseError(e.to_string()))
}