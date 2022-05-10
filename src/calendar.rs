use chrono::{Date, Local, Duration, Datelike};
use crate::lib::*;

#[derive(Debug)]
pub struct Calendar {
    date: Date<Local>,
}

impl Calendar {
    pub fn new() -> Self {
        Self {
            date: Local::now().date(),
        }
    }

    pub fn from_today(&self, weeks: u16) -> Vec<Date<Local>> {
        let curr_monday = self.date.checked_sub_signed(
            Duration::days(self.date.weekday().num_days_from_monday().into())
            ).unwrap();
        let curr_sunday = self.date.checked_add_signed(
            Duration::days(6i64 - self.date.weekday().num_days_from_monday() as i64)
            ).unwrap();
        let mut before = curr_monday.checked_sub_signed(Duration::weeks(weeks as i64)).unwrap();
        let after = curr_sunday.checked_add_signed(Duration::weeks(weeks as i64)).unwrap();

        let mut days: Vec<Date<Local>> = Vec::new();

        while before.le(&after) {
            days.push(before);
            before = before.succ();
        }

        days
    }

    pub fn today() -> (u32, u32) {
        let d = Local::now().date();
        (d.month(), d.day())
    }

    pub fn get_date(&self) -> Date<Local> {
        self.date
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_today() {
    }
}
