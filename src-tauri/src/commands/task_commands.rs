use chrono::NaiveDateTime;
use tauri::State;

use crate::{
    repository::{TaskRepository, TagRepository}, 
    errors::TaskError, 
    models::{Task, 
        task_status::TaskStatus, 
        task_priority::TaskPriority,
        Tag, FrontendTag}, 
    AppState,
};

#[tauri::command]
pub fn create_task(
    title: String,
    user_id: u32,
    description: Option<String>,
    due_date: String,
    priority: String,
    tags: Vec<FrontendTag>,
    state: State<AppState>,
) -> Result<Task, String> {
    let date_replaced = due_date.replace("Z", "");
    let trimmed_date = if date_replaced.len() > 16 {
        &date_replaced[..16]
    } else {
        &date_replaced
    };

    let _due_date = NaiveDateTime::parse_from_str(trimmed_date, "%Y-%m-%dT%H:%M")
        .map_err(|e| format!("Invalid date format: {}", e))?;

    let priority = Some(priority
        .parse::<TaskPriority>()
        .map_err(|_| format!("Invalid priority value: {}", priority))?);

    let mut task = Task::new(title, user_id, description, priority)
        .map_err(|err| err.to_string())?;

    let db_conn = state.db_conn();
    let conn = db_conn.lock().map_err(|_| "Database lock failed")?;

    TaskRepository::insert_task(&conn, &mut task)
        .map_err(|e| format!("Database error: {}", e))?;

    let tag_objects: Vec<Tag> = tags
        .iter()
        .map(|frontend_tag| Tag {
            id: None,
            name: frontend_tag.name.clone(),
            color: frontend_tag.color.clone(),
        })
        .collect();

    TagRepository::update_task_tags(&conn, task.id.unwrap(), &tag_objects)
        .map_err(|e| format!("Failed to associate tags: {}", e))?;

    Ok(task)
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
pub fn get_tasks_for_today(state: State<AppState>) -> Result<Vec<Task>, TaskError> {
    let db_conn = state.db_conn();
    let conn = db_conn.lock().map_err(|e| TaskError::DatabaseError(e.to_string()))?;
    
    TaskRepository::get_tasks_for_today(&conn)
}


#[tauri::command]
pub fn update_task(
    task_id: u32,
    title: Option<String>,
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
                id: frontend_tag.id,
                name: frontend_tag.name,
                color: frontend_tag.color,
            })
            .collect()
    });
    

    TaskRepository::update_task(
        &conn,
        task_id,
        title,
        description,
        status,
        None,
        due_date,
        tag_objs,
    )
    .map_err(|e| format!("Database error: {}", e))?;

    let updated_task = TaskRepository::get_task_by_id(&conn, task_id)
        .map_err(|e| format!("Error fetching updated task: {}", e))?;

    Ok(updated_task)
}
