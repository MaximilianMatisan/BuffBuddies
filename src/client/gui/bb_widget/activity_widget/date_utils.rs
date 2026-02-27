use crate::client::gui::bb_widget::activity_widget::activity::SquareDimensions;
use chrono::{Datelike, Days, Duration, Months, NaiveDate, Weekday};
use std::collections::HashSet;
use strum_macros::{Display, EnumIter};

pub const DAYS_PER_WEEK: u32 = 7;
pub const YEAR_SQUARE_DIMENSION: SquareDimensions = SquareDimensions {
    side_length: 10.0,
    spacing: 1.5,
    max_squares_per_col: DAYS_PER_WEEK,
};
pub const MONTH_SQUARE_DIMENSION: SquareDimensions = SquareDimensions {
    side_length: 1.5 * YEAR_SQUARE_DIMENSION.side_length,
    spacing: 3.0,
    max_squares_per_col: DAYS_PER_WEEK,
};
pub const WEEK_SQUARE_DIMENSION: SquareDimensions = SquareDimensions {
    side_length: 2.5 * YEAR_SQUARE_DIMENSION.side_length,
    spacing: 3.0,
    max_squares_per_col: 1,
};
#[derive(Debug, Clone, Copy, EnumIter, Display, Eq, PartialEq)]
pub enum DateScope {
    Year,
    Month,
    Week,
}
impl DateScope {
    pub fn dimensions(&self) -> SquareDimensions {
        match self {
            DateScope::Week => WEEK_SQUARE_DIMENSION,
            DateScope::Month => MONTH_SQUARE_DIMENSION,
            DateScope::Year => YEAR_SQUARE_DIMENSION,
        }
    }
}
#[derive(Debug, Clone, Copy, EnumIter, Display, Eq, PartialEq)]
pub enum Offset {
    Current,
    Previous,
    BeforePrevious,
}
#[derive(Debug)]
pub struct OffsetDates {
    pub current: NaiveDate,
    pub previous: NaiveDate,
    pub before_previous: NaiveDate,
}
/// Returns the current, previous and before-previous date of the given time-scope.
/// The returned dates are aligned to the start of their period
/// - Years are dated back to January 1st of the year.
/// - Months are dated back to the 1st of the month.
/// - Weeks are dated back to the monday of the week.
///
/// By dating back unnecessary date components we can ensure to use `unwrap()` only on valid dates
/// and not worry about leap years and day differences of individual months.
///
/// In week: (`NaiveDate` - `NaiveDate`) always returns a valid date.
/// Only problem would be if the user has a local date outside the representable time frame of
/// `NaiveDate`
pub fn get_start_dates_of_offsets(today: NaiveDate, scope: DateScope) -> OffsetDates {
    match scope {
        DateScope::Year => OffsetDates {
            current: NaiveDate::from_ymd_opt(today.year(), 1, 1).unwrap(),
            previous: NaiveDate::from_ymd_opt(today.year() - 1, 1, 1).unwrap(),
            before_previous: NaiveDate::from_ymd_opt(today.year() - 2, 1, 1).unwrap(),
        },
        DateScope::Month => OffsetDates {
            current: today.with_day(1).unwrap(),
            previous: (today - Months::new(1)).with_day(1).unwrap(),
            before_previous: (today - Months::new(2)).with_day(1).unwrap(),
        },
        DateScope::Week => {
            let weekday = today.weekday().num_days_from_monday() as i64;
            let monday_cur_week = today - Duration::days(weekday);
            OffsetDates {
                current: monday_cur_week,
                previous: monday_cur_week - Days::new(7),
                before_previous: monday_cur_week - Days::new(14),
            }
        }
    }
}
pub fn get_end_dates_of_offsets(today: NaiveDate, scope: DateScope) -> OffsetDates {
    match scope {
        DateScope::Year => OffsetDates {
            current: NaiveDate::from_ymd_opt(today.year(), 12, 31).unwrap(),
            previous: NaiveDate::from_ymd_opt(today.year() - 1, 12, 31).unwrap(),
            before_previous: NaiveDate::from_ymd_opt(today.year() - 2, 12, 31).unwrap(),
        },
        DateScope::Month => OffsetDates {
            current: (today.with_day(1).unwrap() + Months::new(1)) - Days::new(1),
            previous: today.with_day(1).unwrap() - Days::new(1),
            before_previous: (today.with_day(1).unwrap() - Months::new(1)) - Days::new(1),
        },
        DateScope::Week => {
            let days_until_sunday = 6 - today.weekday().num_days_from_monday() as i64;
            let sunday_cur_week = today + Duration::days(days_until_sunday);
            OffsetDates {
                current: sunday_cur_week,
                previous: sunday_cur_week - Days::new(7),
                before_previous: sunday_cur_week - Days::new(14),
            }
        }
    }
}
pub fn get_date_by_offset(offset_dates: OffsetDates, offset: Offset) -> NaiveDate {
    match offset {
        Offset::Current => offset_dates.current,
        Offset::Previous => offset_dates.previous,
        Offset::BeforePrevious => offset_dates.before_previous,
    }
}
pub fn started_weeks_in_period(start: NaiveDate, end: NaiveDate) -> u32 {
    if start > end {
        return 0;
    }
    let monday_of_first_week = get_monday_of_week_belonging_to_date(start);
    let sunday_of_last_week = get_sunday_of_week_belonging_to_date(end);

    (((sunday_of_last_week - monday_of_first_week).num_days() + 1) / 7) as u32
}

/// Calculates the date of monday for the week the given `date` belongs to
pub fn get_monday_of_week_belonging_to_date(date: NaiveDate) -> NaiveDate {
    let mut monday_finder = date;

    while monday_finder.weekday() != Weekday::Mon {
        monday_finder -= Duration::days(1);
    }

    monday_finder
}

/// Calculates the date of sunday for the week the given `date` belongs to
pub fn get_sunday_of_week_belonging_to_date(date: NaiveDate) -> NaiveDate {
    let mut sunday_finder = date;

    while sunday_finder.weekday() != Weekday::Sun {
        sunday_finder += Duration::days(1);
    }

    sunday_finder
}

/// Returns the dates of the week in which the given `date` falls
pub fn get_dates_of_week_belonging_to_date(date: NaiveDate) -> HashSet<NaiveDate> {
    let mut dates: HashSet<NaiveDate> = HashSet::new();

    let mut monday_date_week = get_monday_of_week_belonging_to_date(date);
    let sunday_date_week = get_sunday_of_week_belonging_to_date(date);

    while monday_date_week <= sunday_date_week {
        dates.insert(monday_date_week);
        monday_date_week += Duration::days(1);
    }

    dates
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn basic_year_scope() {
        let start_dates = get_start_dates_of_offsets(
            NaiveDate::from_ymd_opt(2026, 1, 27).unwrap(),
            DateScope::Year,
        );
        assert_eq!(
            start_dates.current,
            NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()
        );
        assert_eq!(
            start_dates.previous,
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()
        );
        assert_eq!(
            start_dates.before_previous,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
        );
        let end_dates = get_end_dates_of_offsets(
            NaiveDate::from_ymd_opt(2026, 1, 27).unwrap(),
            DateScope::Year,
        );
        assert_eq!(
            end_dates.current,
            NaiveDate::from_ymd_opt(2026, 12, 31).unwrap()
        );
        assert_eq!(
            end_dates.previous,
            NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()
        );
        assert_eq!(
            end_dates.before_previous,
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()
        );
    }
    #[test]
    fn week_scope_across_years() {
        let start_dates = get_start_dates_of_offsets(
            NaiveDate::from_ymd_opt(2026, 1, 2).unwrap(),
            DateScope::Week,
        );
        assert_eq!(
            start_dates.current,
            NaiveDate::from_ymd_opt(2025, 12, 29).unwrap()
        );
        assert_eq!(
            start_dates.previous,
            NaiveDate::from_ymd_opt(2025, 12, 22).unwrap()
        );
        assert_eq!(
            start_dates.before_previous,
            NaiveDate::from_ymd_opt(2025, 12, 15).unwrap()
        );

        let end_dates = get_end_dates_of_offsets(
            NaiveDate::from_ymd_opt(2026, 1, 2).unwrap(),
            DateScope::Week,
        );
        assert_eq!(
            end_dates.current,
            NaiveDate::from_ymd_opt(2026, 1, 4).unwrap()
        );
        assert_eq!(
            end_dates.previous,
            NaiveDate::from_ymd_opt(2025, 12, 28).unwrap()
        );
        assert_eq!(
            end_dates.before_previous,
            NaiveDate::from_ymd_opt(2025, 12, 21).unwrap()
        );
    }
    #[test]
    fn month_scope_across_years() {
        let start_dates = get_start_dates_of_offsets(
            NaiveDate::from_ymd_opt(2026, 1, 27).unwrap(),
            DateScope::Month,
        );
        assert_eq!(
            start_dates.current,
            NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()
        );
        assert_eq!(
            start_dates.previous,
            NaiveDate::from_ymd_opt(2025, 12, 1).unwrap()
        );
        assert_eq!(
            start_dates.before_previous,
            NaiveDate::from_ymd_opt(2025, 11, 1).unwrap()
        );

        let end_dates = get_end_dates_of_offsets(
            NaiveDate::from_ymd_opt(2026, 1, 27).unwrap(),
            DateScope::Month,
        );
        assert_eq!(
            end_dates.current,
            NaiveDate::from_ymd_opt(2026, 1, 31).unwrap()
        );
        assert_eq!(
            end_dates.previous,
            NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()
        );
        assert_eq!(
            end_dates.before_previous,
            NaiveDate::from_ymd_opt(2025, 11, 30).unwrap()
        );
    }
    #[test]
    fn month_scope_leap_year_february() {
        // 2024 was a leap year
        let end_dates = get_end_dates_of_offsets(
            NaiveDate::from_ymd_opt(2024, 2, 12).unwrap(),
            DateScope::Month,
        );
        assert_eq!(
            end_dates.current,
            NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()
        );
    }

    #[test]
    fn test_started_weeks_2025() {
        //2025 had 53 started weeks
        let weeks = started_weeks_in_period(
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
        );
        assert_eq!(weeks, 53);
    }

    #[test]
    fn test_started_weeks_2021_feb() {
        //2021 feb had 28 days with exactly 4 started weeks
        let weeks = started_weeks_in_period(
            NaiveDate::from_ymd_opt(2021, 2, 1).unwrap(),
            NaiveDate::from_ymd_opt(2021, 2, 28).unwrap(),
        );
        assert_eq!(weeks, 4);
    }

    #[test]
    fn end_date_before_start_date() {
        let weeks = started_weeks_in_period(
            NaiveDate::from_ymd_opt(2026, 12, 1).unwrap(),
            NaiveDate::from_ymd_opt(2020, 7, 1).unwrap(),
        );
        assert_eq!(weeks, 0);
    }

    #[test]
    fn test_started_weeks_one_day() {
        let weeks = started_weeks_in_period(
            NaiveDate::from_ymd_opt(2026, 1, 27).unwrap(),
            NaiveDate::from_ymd_opt(2026, 1, 27).unwrap(),
        );
        assert_eq!(weeks, 1);
    }
    #[test]
    fn monday_of_week_if_date_is_monday() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 23).unwrap();

        assert_eq!(get_monday_of_week_belonging_to_date(date), date)
    }

    #[test]
    fn monday_of_week_if_date_is_sunday() {
        let searched_monday = NaiveDate::from_ymd_opt(2026, 2, 23).unwrap();
        let date = NaiveDate::from_ymd_opt(2026, 3, 1).unwrap();

        assert_eq!(get_monday_of_week_belonging_to_date(date), searched_monday)
    }

    #[test]
    fn sunday_of_week_if_date_is_sunday() {
        let date = NaiveDate::from_ymd_opt(2026, 3, 1).unwrap();

        assert_eq!(get_sunday_of_week_belonging_to_date(date), date)
    }

    #[test]
    fn sunday_of_week_if_date_is_monday() {
        let searched_sunday = NaiveDate::from_ymd_opt(2026, 3, 1).unwrap();
        let date = NaiveDate::from_ymd_opt(2026, 2, 23).unwrap();

        assert_eq!(get_sunday_of_week_belonging_to_date(date), searched_sunday)
    }
    #[test]
    fn days_of_week_equals_seven() {
        let date = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
        let dates_hash_set = get_dates_of_week_belonging_to_date(date);

        assert_eq!(dates_hash_set.len(), 7);
    }

    #[test]
    fn dates_of_week_check() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 27).unwrap();
        let dates_hash_set = get_dates_of_week_belonging_to_date(date);

        let real_dates_of_week = [
            NaiveDate::from_ymd_opt(2026, 2, 23).unwrap(),
            NaiveDate::from_ymd_opt(2026, 2, 24).unwrap(),
            NaiveDate::from_ymd_opt(2026, 2, 25).unwrap(),
            NaiveDate::from_ymd_opt(2026, 2, 26).unwrap(),
            NaiveDate::from_ymd_opt(2026, 2, 27).unwrap(),
            NaiveDate::from_ymd_opt(2026, 2, 28).unwrap(),
            NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
        ];

        assert_eq!(dates_hash_set.len(), 7);

        for real_date in real_dates_of_week {
            assert!(dates_hash_set.contains(&real_date));
        }
    }
}
