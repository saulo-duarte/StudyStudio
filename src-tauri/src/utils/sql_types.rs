use rusqlite::types::{FromSql, FromSqlResult, ValueRef, FromSqlError};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SqliteDateTime(pub NaiveDateTime);

impl FromSql for SqliteDateTime {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Text(s) => {
                let s = std::str::from_utf8(s)
                    .map_err(|_| FromSqlError::Other("Invalid UTF-8 in date string".into()))?;
                let dt = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M")
                    .map_err(|e| FromSqlError::Other(Box::new(e)))?;
                Ok(SqliteDateTime(dt))
            },
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl From<SqliteDateTime> for NaiveDateTime {
    fn from(sqlite_dt: SqliteDateTime) -> Self {
        sqlite_dt.0
    }
}
