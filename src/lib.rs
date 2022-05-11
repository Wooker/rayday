use chrono::NaiveDate;

const REFORM_YEAR: u32 = 1099;
const MONTHS: usize = 12;
const WEEKDAYS: u32 = 7;

pub fn is_leap_year(year: u32) -> bool {
    if year <= REFORM_YEAR {
        return year % 4 == 0;
    }
    (year % 4 == 0) ^ (year % 100 == 0) ^ (year % 400 == 0)
}

fn days_by_year(mut year: u32) -> u32 {
    let mut count: u32 = 0;

    while year > 1 {
        year -= 1;
        if is_leap_year(year) {
            count += 366
        } else {
            count += 365
        }
    }
    count
}

pub fn get_days_from_month(year: i32, month: u32) -> i64 {
    NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days()
}
