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

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::types::ValueRef;

    #[test]
    fn test_from_sql_valid_datetime() {
        let input = "2023-10-15T14:30".as_bytes();
        let result = SqliteDateTime::column_result(ValueRef::Text(input));
        assert!(result.is_ok());

        let sqlite_dt = result.unwrap();
        let expected = NaiveDateTime::parse_from_str("2023-10-15 14:30", "%Y-%m-%d %H:%M").unwrap();
        assert_eq!(sqlite_dt.0, expected);
    }

    #[test]
    fn test_from_sql_invalid_format() {
        let input = "2023-10-15 14:30:00".as_bytes();
        let result = SqliteDateTime::column_result(ValueRef::Text(input));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FromSqlError::Other(_)));
    }

    #[test]
    fn test_from_sql_invalid_type() {
        let result = SqliteDateTime::column_result(ValueRef::Integer(123));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FromSqlError::InvalidType));
    }

    #[test]
    fn test_from_sql_invalid_utf8() {
        let input = &[0xFF, 0xFF, 0xFF];
        let result = SqliteDateTime::column_result(ValueRef::Text(input));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FromSqlError::Other(_)));
    }

    #[test]
    fn test_from_sqlite_datetime_to_naive_datetime() {
        let dt = NaiveDateTime::parse_from_str("2023-10-15 14:30", "%Y-%m-%d %H:%M").unwrap();
        let sqlite_dt = SqliteDateTime(dt.clone());
        let converted: NaiveDateTime = sqlite_dt.into();
        assert_eq!(converted, dt);
    }

    #[test]
    fn test_sqlite_datetime_debug() {
        let dt = NaiveDateTime::parse_from_str("2023-10-15 14:30", "%Y-%m-%d %H:%M").unwrap();
        let sqlite_dt = SqliteDateTime(dt);
        let debug_output = format!("{:?}", sqlite_dt);
        assert!(debug_output.contains("SqliteDateTime"));
        assert!(debug_output.contains("2023-10-15T14:30:00"));
    }
}