use serde::Serialize;
use crate::models::task::Task;
use crate::utils::format_date::format_datetime;

#[derive(Debug, Serialize)]
pub struct TaskView {
  pub id: Option<u32>,
  pub user_id: u32,
  pub name: String,
  pub description: Option<String>,
  pub status: String,
  pub created_at: String,
  pub update_at: String,
  pub due_date: String,
}

impl From<Task> for TaskView {
  fn from(task: Task) -> Self {
      TaskView {
        id: task.id,
        user_id: task.user_id,
        name: task.name,
        status: task.status.as_str().to_string(),
        description: task.description,
        created_at: format_datetime(task.created_at),
        update_at: format_datetime(task.updated_at),
        due_date: format_datetime(task.due_date),
      }
  }
}

