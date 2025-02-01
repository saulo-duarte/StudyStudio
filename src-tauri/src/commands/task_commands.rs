use tauri::{command, State};
use crate::models::{NewTask, Task};
use crate::AppState;
use crate::views::display_task;

#[command]
pub fn create_task(
    state: State<'_, AppState>,
    new_task: NewTask,
) -> Result<String, String> {
    let db_conn = state.db_conn();
    let mut conn = db_conn.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;

    let task = Task::create(&mut *conn, new_task)
        .map_err(|e| format!("Database error: {}", e))?;

    Ok(display_task(&task))
}