use rusqlite::{Connection, params, OptionalExtension};
use crate::{models::Tag, errors::TagError};

pub struct TagRepository;

impl TagRepository {
    pub fn create(conn: &Connection, tag: &mut Tag) -> Result<(), TagError> {
        conn.execute(
            "INSERT INTO tags (tag_name, tag_color) VALUES (?1, ?2)",
            params![tag.name, tag.color],
        )
        .map_err(|e| TagError::DatabaseError(e.to_string()))?;

        tag.id = Some(conn.last_insert_rowid() as u32);
        Ok(())
    }

    pub fn update_tag(conn: &Connection, id: u32, tag_name: String) -> Result<(), TagError> {
        conn.execute(
            "UPDATE tags SET tag_name = ?1 WHERE id = ?2",
            &[&tag_name as &str, &id.to_string()],
        )
        .map_err(|e| TagError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }

    pub fn find_by_id(conn: &Connection, id: u32) -> Result<Option<Tag>, TagError> {
        let mut stmt = conn.prepare("SELECT id, name, color FROM tags WHERE id = ?1")
            .map_err(|e| TagError::DatabaseError(e.to_string()))?;
        
        let tag = stmt.query_row(params![id], |row| {
            Ok(Tag {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                color: row.get(2)?,
            })
        })
        .optional()
        .map_err(|e| TagError::DatabaseError(e.to_string()))?;
        
        Ok(tag)
    }
    
    pub fn list_all(conn: &Connection) -> Result<Vec<Tag>, TagError> {
        let mut stmt = conn.prepare("SELECT id, tag_name, tag_color FROM tags")
            .map_err(|e| TagError::DatabaseError(e.to_string()))?;
        
        let tag_iter = stmt
            .query_map([], |row| {
                Ok(Tag {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    color: row.get(2)?,
                })
            })
            .map_err(|e| TagError::DatabaseError(e.to_string()))?;
        
        let mut tags = Vec::new();
        for tag in tag_iter {
            tags.push(tag.map_err(|e| TagError::DatabaseError(e.to_string()))?);
        }
        Ok(tags)
    }

    pub fn delete_by_id(conn: &Connection, id: u32) -> Result<bool, TagError> {
        let rows_affected = conn
            .execute("DELETE FROM tags WHERE id = ?1", params![id])
            .map_err(|e| TagError::DatabaseError(e.to_string()))?;

        Ok(rows_affected > 0)
    }

    pub fn get_task_tags(conn: &Connection, task_id: u32) -> Result<Option<Vec<Tag>>, TagError> {
        let mut stmt = conn
            .prepare(
                "SELECT
                    t.id,
                    t.tag_name,
                    t.tag_color
                FROM tags t
                JOIN task_tags tt ON t.id = tt.tag_id
                WHERE tt.task_id = ?"
            )
            .map_err(|e| TagError::DatabaseError(e.to_string()))?;
        
        let tag_iter = stmt
            .query_map(params![task_id], |row| {
                Ok(Tag {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    color: row.get(2)?,
                })
            })
            .map_err(|e| TagError::DatabaseError(e.to_string()))?;
        
        let tags: Result<Vec<Tag>, _> = tag_iter.collect();
        let tags = tags.map_err(|e| TagError::DatabaseError(e.to_string()))?;
        
        if tags.is_empty() {
            Ok(None)
        } else {
            Ok(Some(tags))
        }
    }

    pub fn update_task_tags(conn: &Connection, task_id: u32, tags: &[Tag]) -> Result<(), TagError> {
    conn.execute(
        "DELETE FROM task_tags WHERE task_id = ?",
        params![task_id],
    ).map_err(|e| TagError::DatabaseError(e.to_string()))?;

    for tag in tags {
        let tag_id: Option<u32> = conn.query_row(
            "SELECT id FROM tags WHERE tag_name = ?",
            params![tag.name],
            |row| row.get(0),
        ).optional().map_err(|e| TagError::DatabaseError(e.to_string()))?;

        let tag_id = match tag_id {
            Some(id) => id,
            None => {
                conn.execute(
                    "INSERT INTO tags (tag_name, tag_color) VALUES (?, ?)",
                    params![tag.name, tag.color],
                ).map_err(|e| TagError::DatabaseError(e.to_string()))?;
                conn.last_insert_rowid() as u32
            }
        };

        conn.execute(
            "INSERT INTO task_tags (task_id, tag_id) VALUES (?, ?)",
            params![task_id, tag_id],
        ).map_err(|e| TagError::DatabaseError(e.to_string()))?;
    }
        Ok(())
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    
    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                color TEXT NOT NULL
            )",
            [],
        )
        .unwrap();
        conn
    }
    
    #[test]
    fn test_create_tag() {
        let conn = setup_db();
        let mut tag = Tag::new("Tech".to_string(), "#FF5733".to_string()).unwrap();
        
        assert!(TagRepository::create(&conn, &mut tag).is_ok());
        assert!(tag.id.is_some());
        
        let stored_tag: Tag = conn
            .query_row(
                "SELECT id, name, color FROM tags WHERE id = ?1",
                [tag.id.unwrap()],
                |row| {
                    Ok(Tag {
                        id: Some(row.get(0)?),
                        name: row.get(1)?,
                        color: row.get(2)?,
                    })
                },
            )
            .unwrap();
            
        assert_eq!(stored_tag.name, "Tech");
        assert_eq!(stored_tag.color, "#FF5733");
    }
    
    #[test]
    fn test_find_by_id() {
        let conn = setup_db();
        let mut tag = Tag::new("Art".to_string(), "#00FF00".to_string()).unwrap();
        TagRepository::create(&conn, &mut tag).unwrap();
        
        let found_tag = TagRepository::find_by_id(&conn, tag.id.unwrap()).unwrap();
        assert!(found_tag.is_some());
        let found_tag = found_tag.unwrap();
        assert_eq!(found_tag.name, "Art");
        assert_eq!(found_tag.color, "#00FF00");
    }
    
    #[test]
    fn test_list_all() {
        let conn = setup_db();
        let mut tag1 = Tag::new("Tag1".to_string(), "#123456".to_string()).unwrap();
        let mut tag2 = Tag::new("Tag2".to_string(), "#654321".to_string()).unwrap();
        TagRepository::create(&conn, &mut tag1).unwrap();
        TagRepository::create(&conn, &mut tag2).unwrap();
        
        let tags = TagRepository::list_all(&conn).unwrap();
        assert_eq!(tags.len(), 2);
        
        let names: Vec<String> = tags.into_iter().map(|t| t.name).collect();
        assert!(names.contains(&"Tag1".to_string()));
        assert!(names.contains(&"Tag2".to_string()));
    }
}
