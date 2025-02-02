use chrono::NaiveDateTime;

pub fn format_datetime(dt: NaiveDateTime) -> String {
  dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn truncate_to_minute(dt: NaiveDateTime) -> NaiveDateTime {
  let dt_str = dt.format("%Y-%m-%dT%H:%M").to_string();
  NaiveDateTime::parse_from_str(&dt_str, "%Y-%m-%dT%H:%M").unwrap()
}