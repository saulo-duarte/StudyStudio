use chrono::NaiveDateTime;

pub fn format_datetime(dt: NaiveDateTime) -> String {
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn truncate_to_minute(dt: NaiveDateTime) -> NaiveDateTime {
    let dt_str = dt.format("%Y-%m-%dT%H:%M").to_string();
    NaiveDateTime::parse_from_str(&dt_str, "%Y-%m-%dT%H:%M").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_datetime() {
        let dt = NaiveDateTime::parse_from_str("2023-10-15 14:30:45", "%Y-%m-%d %H:%M:%S").unwrap();
        let formatted = format_datetime(dt);
        assert_eq!(formatted, "2023-10-15 14:30:45");

        let dt = NaiveDateTime::parse_from_str("2025-03-01 09:15:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let formatted = format_datetime(dt);
        assert_eq!(formatted, "2025-03-01 09:15:00");
    }

    #[test]
    fn test_truncate_to_minute() {
        let dt = NaiveDateTime::parse_from_str("2023-10-15 14:30:45.123", "%Y-%m-%d %H:%M:%S%.f").unwrap();
        let truncated = truncate_to_minute(dt);
        let expected = NaiveDateTime::parse_from_str("2023-10-15 14:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!(truncated, expected);

        let dt = NaiveDateTime::parse_from_str("2025-03-01 09:15:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let truncated = truncate_to_minute(dt);
        let expected = NaiveDateTime::parse_from_str("2025-03-01 09:15:00", "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!(truncated, expected);

        let dt = NaiveDateTime::parse_from_str("2024-12-31 23:59:59", "%Y-%m-%d %H:%M:%S").unwrap();
        let truncated = truncate_to_minute(dt);
        let expected = NaiveDateTime::parse_from_str("2024-12-31 23:59:00", "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!(truncated, expected);
    }

    #[test]
    fn test_format_and_truncate_combined() {
        let dt = NaiveDateTime::parse_from_str("2023-10-15 14:30:45.789", "%Y-%m-%d %H:%M:%S%.f").unwrap();
        let truncated = truncate_to_minute(dt);
        let formatted = format_datetime(truncated);
        assert_eq!(formatted, "2023-10-15 14:30:00");
    }
}