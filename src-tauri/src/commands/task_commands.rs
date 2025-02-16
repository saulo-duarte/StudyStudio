use chrono::NaiveDateTime;
use tauri::State;

use crate::{
    database::repository::{TaskRepository, TagRepository}, 
    errors::TaskError, 
    models::{Task, TaskStatus, Tag, FrontendTag}, 
    views::TaskView, AppState,
};

#[tauri::command]
pub fn create_task(
    name: String,
    user_id: u32,
    description: Option<String>,
    due_date: String,
    tags: Vec<FrontendTag>,
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
    let conn = db_conn.lock().map_err(|_| "Database lock failed")?;

    TaskRepository::insert_task(&conn, &mut task)
        .map_err(|e| format!("Database error: {}", e))?;

    let tag_objects: Vec<Tag> = tags.iter().map(|frontend_tag| Tag {
        id: None,
        name: frontend_tag.name.clone(),
        color: frontend_tag.color.clone(),
    }).collect();

    TagRepository::update_task_tags(&conn, task.id.unwrap(), &tag_objects)
        .map_err(|e| format!("Failed to associate tags: {}", e))?;

    Ok(task.into())
}

#[tauri::command]
pub fn get_all_tasks(state: State<AppState>) -> Result<Vec<Task>, TaskError> {
    let db_conn = state.db_conn();
    let conn = db_conn.lock().map_err(|e| TaskError::DatabaseError(e.to_string()))?;
    TaskRepository::get_all_tasks(&conn)
        .map_err(|e| TaskError::DatabaseError(e.to_string()))
}

#[tauri::command]
pub fn delete_task(state: State<AppState>, task_id: u32) -> Result<(), TaskError> {
    let db_conn = state.db_conn();
    let conn = db_conn.lock().map_err(|e| TaskError::DatabaseError(e.to_string()))?;
    TaskRepository::delete_task(&conn, task_id)
        .map_err(|e| TaskError::DatabaseError(e.to_string()))
}

#[tauri::command]
pub fn update_task(
    task_id: u32,
    name: Option<String>,
    description: Option<String>,
    status: Option<String>,
    due_date: Option<String>,
    tags: Option<Vec<FrontendTag>>,
    state: State<AppState>,
) -> Result<Task, String> {
    let status = match status {
        Some(s) => Some(s.parse::<TaskStatus>().map_err(|e| e.to_string())?),
        None => None,
    };

    let due_date = match due_date {
        Some(d) => {
            let date_replaced = d.replace("Z", "");
            let trimmed_date = if date_replaced.len() > 16 {
                &date_replaced[..16]
            } else {
                &date_replaced
            };
            Some(
                NaiveDateTime::parse_from_str(trimmed_date, "%Y-%m-%dT%H:%M")
                    .map_err(|e| format!("Invalid date format: {}", e))?,
            )
        }
        None => None,
    };

    let db_conn = state.db_conn();
    let conn = db_conn.lock().map_err(|_| "Database lock failed".to_string())?;

    let tag_objs: Option<Vec<crate::models::tag::Tag>> = tags.map(|frontend_tags| {
        frontend_tags
            .into_iter()
            .map(|frontend_tag| crate::models::tag::Tag {
                id: None,
                name: frontend_tag.name,
                color: frontend_tag.color,
            })
            .collect()
    });

    TaskRepository::update_task(
        &conn,
        task_id,
        name,
        description,
        status,
        due_date,
        tag_objs,
    )
    .map_err(|e| format!("Database error: {}", e))?;

    let updated_task = TaskRepository::get_task_by_id(&conn, task_id)
        .map_err(|e| format!("Error fetching updated task: {}", e))?;

    Ok(updated_task)
}
