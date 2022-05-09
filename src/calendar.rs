use chrono::{Date, Local, Duration, Datelike};
use crate::lib::*;

pub struct Calendar {
    date: Date<Local>,
}

impl Calendar {
    pub fn new() -> Self {
        Self {
            date: Local::now().date(),
        }
    }

    pub fn from_today(&self, weeks: u16) -> (Vec<Date<Local>>, (usize, usize)) {
        let curr_monday = self.date.checked_sub_signed(
            Duration::days(self.date.weekday().num_days_from_monday().into())
            ).unwrap();
        let curr_sunday = self.date.checked_add_signed(
            Duration::days(6i64 - self.date.weekday().num_days_from_monday() as i64)
            ).unwrap();
        let mut before = curr_monday.checked_sub_signed(Duration::weeks(weeks as i64)).unwrap();
        let after = curr_sunday.checked_add_signed(Duration::weeks(weeks as i64)).unwrap();

        let mut days: Vec<Date<Local>> = Vec::new();

        let (mut month, mut day): (usize, usize) = (0, 0);

        while before.le(&after) {
            if before.eq(&self.date) {
                (month, day) = (before.month() as usize, before.day() as usize);
            }
            days.push(before);
            before = before.succ();
        }

        (days, (month, day))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_today() {
    }
}
