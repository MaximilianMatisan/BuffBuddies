use chrono::{Datelike, Days, Duration, Months, NaiveDate, Weekday};
use strum_macros::{Display, EnumIter};
use crate::client::gui::bb_widget::activity::activity::SquareDimensions;

pub const DAYS_PER_WEEK: u32 = 7;
pub const YEAR_SQUARE_DIMENSION: SquareDimensions = SquareDimensions {
    side_length: 10.0,
    spacing: 1.5,
    max_squares_per_col: DAYS_PER_WEEK
};
pub const MONTH_SQUARE_DIMENSION: SquareDimensions = SquareDimensions {
    side_length: 1.5 * YEAR_SQUARE_DIMENSION.side_length,
    spacing: 3.0,
    max_squares_per_col: DAYS_PER_WEEK
};
pub const WEEK_SQUARE_DIMENSION: SquareDimensions = SquareDimensions {
    side_length: 2.5 * YEAR_SQUARE_DIMENSION.side_length,
    spacing: 3.0,
    max_squares_per_col: 1
};
#[derive(Debug, Clone, Copy, EnumIter, Display, Eq, PartialEq)]
pub enum DateScope {
    Year,
    Month,
    Week
}
impl DateScope {
    pub fn dimensions(&self) -> SquareDimensions {
        match self {
            DateScope::Week => WEEK_SQUARE_DIMENSION,
            DateScope::Month => MONTH_SQUARE_DIMENSION,
            DateScope::Year => YEAR_SQUARE_DIMENSION
        }
    }
}
#[derive(Debug, Clone, Copy, EnumIter, Display, Eq, PartialEq)]
pub enum Offset {
    Current,
    Previous,
    BeforePrevious
}
#[derive(Debug)]
pub struct OffsetDates {
    pub current: NaiveDate,
    pub previous: NaiveDate,
    pub before_previous: NaiveDate
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
        DateScope::Year => {
            OffsetDates {
                current: NaiveDate::from_ymd_opt(today.year(),1,1).unwrap(),
                previous: NaiveDate::from_ymd_opt(today.year() - 1, 1, 1).unwrap(),
                before_previous: NaiveDate::from_ymd_opt(today.year() - 2, 1, 1).unwrap()
            }
        },
        DateScope::Month =>{
            OffsetDates {
                current: today.with_day(1).unwrap(),
                previous: (today - Months::new(1)).with_day(1).unwrap(),
                before_previous: (today - Months::new(2)).with_day(1).unwrap(),
            }
        }
        DateScope::Week => {
            let weekday = today.weekday().num_days_from_monday() as i64;
            let monday_cur_week = today - Duration::days(weekday);
            OffsetDates {
                current: monday_cur_week,
                previous: monday_cur_week - Days::new(7),
                before_previous: monday_cur_week - Days::new(14)
            }
        }
    }
}
pub fn get_end_dates_of_offsets(today: NaiveDate, scope: DateScope) -> OffsetDates {
    match scope {
        DateScope::Year => {
            OffsetDates {
                current: NaiveDate::from_ymd_opt(today.year(),12,31).unwrap(),
                previous: NaiveDate::from_ymd_opt(today.year() - 1, 12, 31).unwrap(),
                before_previous: NaiveDate::from_ymd_opt(today.year() - 2, 12, 31).unwrap()
            }
        },
        DateScope::Month => {
            OffsetDates {
                current: (today.with_day(1).unwrap() + Months::new(1)) - Days::new(1),
                previous: today.with_day(1).unwrap() - Days::new(1),
                before_previous: (today.with_day(1).unwrap() - Months::new(1)) - Days::new(1),
            }
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
        Offset::BeforePrevious => offset_dates.before_previous
    }
}
pub fn started_weeks_in_period(start: NaiveDate, end: NaiveDate) -> u32{
    if start > end {
        return 0;
    }
    let mut first_week_monday_finder = start;
    while first_week_monday_finder.weekday() != Weekday::Mon {
        first_week_monday_finder -= Duration::days(1);
    }
    let mut last_week_sunday_finder = end;
    while last_week_sunday_finder.weekday() != Weekday::Sun {
        last_week_sunday_finder += Duration::days(1);
    }

    (((last_week_sunday_finder - first_week_monday_finder).num_days()+1) / 7) as u32
}