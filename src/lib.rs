use chrono::NaiveDate;

const REFORM_YEAR: u32 = 1099;

pub fn is_leap_year(year: u32) -> bool {
    if year <= REFORM_YEAR {
        return year % 4 == 0;
    }
    (year % 4 == 0) ^ (year % 100 == 0) ^ (year % 400 == 0)
}

pub fn get_days_from_month(year: i32, month: u32) -> i64 {
    NaiveDate::from_ymd_opt(
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
    .expect("Out of range date")
    .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).expect("Out of range date"))
    .num_days()
}
