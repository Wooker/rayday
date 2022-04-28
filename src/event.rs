use chrono::prelude::*;

use std::{fmt, cmp, borrow::Borrow};
use serde::{Serialize, Deserialize};

enum Periodicity {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug)]
pub enum EventError {
    EndBeforeStart,
    Unknown
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    description: String,
}

impl Event {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>, description: String) -> Result<Event, EventError> {
        match start.cmp(&end) {
            cmp::Ordering::Greater => Err(EventError::EndBeforeStart),
            cmp::Ordering::Less => Ok(Event { start, end, description }),
            _ => Err(EventError::Unknown),

        }
    }

    pub fn date(&self) -> Date<Utc> {
        self.start.date()
    }

    pub fn datetime(&self) -> DateTime<Utc> {
        self.start.with_timezone(&Utc)
    }

    pub fn desc(&self) -> &str {
        self.description.borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_ordering() {
        let start = Utc.ymd(2022, 4, 27).and_hms(12, 0, 0);
        let end = Utc.ymd(2022, 5, 27).and_hms(12, 30, 0);

        let e = Event::new(start, end, String::from(""));
        assert_eq!(e.is_ok(), true);
    }

    #[test]
    fn end_before_start() {
        let start = Utc.ymd(2022, 4, 5).and_hms(12, 0, 0);
        let end = Utc.ymd(2022, 4, 5).and_hms(10, 0, 0);

        let e = Event::new(start, end, String::from(""));
        assert_eq!(e.is_err(), true);
    }
}
