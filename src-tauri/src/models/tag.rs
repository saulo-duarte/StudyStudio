use serde::{Serialize, Deserialize};
use crate::errors::TagError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<u32>,
    pub name: String,
    pub color: String,
}

impl Tag {
    pub fn new(name: String, color: String) -> Result<Self, TagError> {
        if name.trim().is_empty() {
            return Err(TagError::InvalidName("Tag name cannot be empty".into()))
        }
        if color.trim().is_empty() {
            return Err(TagError::InvalidColor("Tag color cannot be empty".into()))
        }
        if !is_valid_color(&color) {
            return Err(TagError::InvalidColor(format!("Tag color '{}' is invalid", color)));
        }
        
        Ok(Tag {
            id: None,
            name,
            color,
        })
    }
}

fn is_valid_color(color: &str) -> bool {
    let trimmed = color.trim();
    
    if trimmed.starts_with('#') {
        let hex = &trimmed[1..];
        return (hex.len() == 3 || hex.len() == 6) && hex.chars().all(|c| c.is_digit(16));
    }
    
    let allowed_names = [
        "black", "silver", "gray", "white", "maroon", "red", "purple", "fuchsia",
        "green", "lime", "olive", "yellow", "navy", "blue", "teal", "aqua",
    ];
    let lower = trimmed.to_lowercase();
    allowed_names.contains(&lower.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_creation_valid_named_color() {
        let tag = Tag::new("Important".to_string(), "Red".to_string());
        assert!(tag.is_ok());
        let tag = tag.unwrap();
        assert_eq!(tag.name, "Important");
        assert_eq!(tag.color, "Red");
        assert!(tag.id.is_none());
    }

    #[test]
    fn test_tag_creation_valid_hex_short() {
        let tag = Tag::new("Notice".to_string(), "#F0A".to_string());
        assert!(tag.is_ok());
        let tag = tag.unwrap();
        assert_eq!(tag.color, "#F0A");
    }

    #[test]
    fn test_tag_creation_valid_hex_long() {
        let tag = Tag::new("Alert".to_string(), "#FFAA00".to_string());
        assert!(tag.is_ok());
        let tag = tag.unwrap();
        assert_eq!(tag.color, "#FFAA00");
    }

    #[test]
    fn test_tag_creation_invalid_name() {
        let tag = Tag::new("".to_string(), "Red".to_string());
        assert!(tag.is_err());
        if let Err(e) = tag {
            assert_eq!(e.to_string(), "Invalid tag name: Tag name cannot be empty");
        }
    }

    #[test]
    fn test_tag_creation_invalid_color_empty() {
        let tag = Tag::new("Important".to_string(), "".to_string());
        assert!(tag.is_err());
        if let Err(e) = tag {
            assert_eq!(e.to_string(), "Invalid tag color: Tag color cannot be empty");
        }
    }

    #[test]
    fn test_tag_creation_invalid_color_hex() {
        let tag = Tag::new("Important".to_string(), "#GGG".to_string());
        assert!(tag.is_err());
        if let Err(e) = tag {
            assert_eq!(e.to_string(), "Invalid tag color: Tag color '#GGG' is invalid");
        }
    }

    #[test]
    fn test_tag_creation_invalid_color_name() {
        let tag = Tag::new("Important".to_string(), "NotAColor".to_string());
        assert!(tag.is_err());
        if let Err(e) = tag {
            assert_eq!(e.to_string(), "Invalid tag color: Tag color 'NotAColor' is invalid");
        }
    }

    #[test]
    fn test_tag_creation_invalid_name_and_color() {
        let tag = Tag::new("".to_string(), "".to_string());
        assert!(tag.is_err());
        if let Err(e) = tag {
            assert_eq!(e.to_string(), "Invalid tag name: Tag name cannot be empty");
        }
    }
}
