use chrono::{NaiveDate, ParseResult};

pub fn format_naive_date_for_database(date: &NaiveDate) -> String {
    NaiveDate::format(date, "%Y-%m-%d").to_string()
}
pub fn database_date_string_to_naive_date(str: &str) -> ParseResult<NaiveDate> {
    NaiveDate::parse_from_str(str, "%Y-%m-%d")
}