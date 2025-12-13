use chrono::{Datelike, Days, Duration, Months, NaiveDate};
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Copy, EnumIter, Display, Eq, PartialEq)]
pub enum DateScope {
    Year,
    Month,
    Week
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