use chrono::Datelike;

pub type UtcDateTime = chrono::DateTime<chrono::Utc>;

pub fn date_from_parts(year: i32, month: u32, day: u32) -> UtcDateTime {
    let date = UtcDateTime::default();
    let date = date.with_year(year).unwrap_or_default();
    let date = date.with_month(month).unwrap_or_default();
    let date = date.with_day(day).unwrap_or_default();
    date
}
