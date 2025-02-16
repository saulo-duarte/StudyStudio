use serde::{Serialize, Deserialize};
use crate::errors::TagError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
  pub id: Option<u32>,
  pub name: String,
  pub color: String,
}

impl Tag {
  pub fn new(
    name: String,
    color: String,
  ) -> Result<Self, TagError> {
    if name.trim().is_empty() {
      return Err(TagError::InvalidName("Tag name cannot be empty".into()))
    }
    
    Ok(Tag {
      id: None,
      name,
      color,
    })
  }
}
