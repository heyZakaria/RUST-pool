use chrono::{Datelike, NaiveDate, Weekday};

fn middle_day(year: i32) -> Option<Weekday> {

    let is_leap = chrono::naive::is_leap_year(year);

    if is_leap {
        return None;
    }

    let middle_day = NaiveDate::from_yo_opt(year, 183)?;

    Some(middle_day.weekday())
}
