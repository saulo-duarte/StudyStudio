use rusqlite::{params, Connection,OptionalExtension};

use crate::{errors::TagError, models::Tag};

pub struct TagRepository;

impl TagRepository {
  pub fn insert_tag(conn: &Connection, tag: &mut Tag) ->Result<(), rusqlite::Error> {

    conn.execute(
      "INSERT INTO tags (tag_name, tag_color) 
        VALUES (?1, ?2)", 
        params![
          tag.name,
          tag.color
        ]
      )?;

      tag.id = Some(conn.last_insert_rowid() as u32);
      Ok(())
  }

  pub fn get_all_tags(conn: &Connection) -> Result<Vec<Tag>, TagError> {
    let mut stmt = conn
        .prepare(
            "SELECT
                id,
                tag_name,
                tag_color
            FROM tags",
        )
        .map_err(|e| TagError::DatabaseError(e.to_string()))?;

    let tags = stmt
        .query_map([], |row| Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
            color: row.get(2)?,
        }))
        .map_err(|e| TagError::DatabaseError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| TagError::DatabaseError(e.to_string()))?;

    Ok(tags)
  }

  pub fn delete_tag(conn: &Connection, tag_id: u32) -> Result<(), TagError> {
    conn.execute(
      "DELETE FROM tags WHERE id = 1?",
          params![tag_id],
      )
      .map_err(|e| TagError::DatabaseError(e.to_string()))?;
    Ok(())
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
    // Remove as associações existentes para essa task
    conn.execute(
        "DELETE FROM task_tags WHERE task_id = ?",
        params![task_id],
    ).map_err(|e| TagError::DatabaseError(e.to_string()))?;

    // Para cada tag, verifica se ela já existe; se não, insere-a
    for tag in tags {
        // Tenta recuperar a tag pelo nome
        let tag_id: Option<u32> = conn.query_row(
            "SELECT id FROM tags WHERE tag_name = ?",
            params![tag.name],
            |row| row.get(0),
        ).optional().map_err(|e| TagError::DatabaseError(e.to_string()))?;

        let tag_id = match tag_id {
            Some(id) => id,
            None => {
                // Insere a nova tag
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

