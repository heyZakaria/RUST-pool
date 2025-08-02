use chrono::{Datelike, NaiveDate, Weekday};

pub fn middle_day(year: i32) -> Option<Weekday> {

    let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);

    if is_leap {
        return None;
    }

    let middle_day = NaiveDate::from_yo_opt(year, 183)?;

    Some(middle_day.weekday())
}
