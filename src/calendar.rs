use chrono::{Datelike, Duration, Local, NaiveDate};
use rayday::*;

#[derive(Debug)]
pub struct Calendar {
    date: NaiveDate,
}

impl Calendar {
    pub fn new() -> Self {
        Self {
            date: Local::now().date_naive(),
        }
    }

    pub fn date_from_today(&self, weeks: u16) -> NaiveDate {
        let curr_monday = self
            .date
            .checked_sub_signed(Duration::days(
                self.date.weekday().num_days_from_monday().into(),
            ))
            .unwrap();

        curr_monday
            .checked_sub_signed(Duration::weeks(weeks as i64))
            .unwrap()
    }

    pub fn from_today(&self, weeks: u16) -> Vec<NaiveDate> {
        let curr_monday = self
            .date
            .checked_sub_signed(Duration::days(
                self.date.weekday().num_days_from_monday().into(),
            ))
            .unwrap();
        let curr_sunday = self
            .date
            .checked_add_signed(Duration::days(
                6i64 - self.date.weekday().num_days_from_monday() as i64,
            ))
            .unwrap();

        let mut before = curr_monday
            .checked_sub_signed(Duration::weeks(weeks as i64))
            .unwrap();
        let after = curr_sunday
            .checked_add_signed(Duration::weeks(weeks as i64))
            .unwrap();

        let mut days: Vec<NaiveDate> = Vec::new();

        while before.le(&after) {
            days.push(before);
            before = before.succ_opt().expect("Last date is reached");
        }

        days
    }

    pub fn today() -> (u32, u32) {
        let d = Local::now().date_naive();
        (d.month(), d.day())
    }

    pub fn get_date(&self) -> NaiveDate {
        self.date
    }
}
