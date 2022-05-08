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

    pub fn from_today(&self, weeks: u16) -> Vec<Date<Local>> {
        let mut before = self.date.checked_sub_signed(Duration::weeks(weeks as i64)).unwrap();
        let after = self.date.checked_add_signed(Duration::weeks(weeks as i64)).unwrap();
        /*
        let (f_year, f_month) = (before.year(), before.month());
        let (l_year, l_month) = (after.year(), after.month());
        while 
        let dayas_in_month = match month {
            2 => {
                if is_leap_year(year as u32) {
                    29
                } else {
                    28
                }
            },
            1 | 3 | 5| 7 | 9 | 11 => 31,
            _ => 30,
        };
        */
        let mut days: Vec<Date<Local>> = Vec::new();

        while before.le(&after) {
            days.push(before);
            before = before.succ();
        }

        days
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_today() {
        let cal = Calendar::new();
        let days = cal.from_today(2);

        dbg!(&days);
        assert_eq!(days.len(), 29);
    }
}
