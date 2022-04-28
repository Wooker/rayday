use chrono::prelude::*;

use std::{fmt, cmp, borrow::Borrow};
use serde::{Serialize, Deserialize};

const PARSE_TIME: &str = "%Y-%m-%d %H:%M:%S %:z";

enum Periodicity {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug)]
pub enum EventTimeError {
    EndBeforeStart,
    Unknown
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EventTime {
    start: DateTime<Local>,
    end: DateTime<Local>,
}

impl EventTime {
    pub fn new(start: DateTime<Local>, end: DateTime<Local>) -> Result<EventTime, EventTimeError> {
        match start.cmp(&end) {
            cmp::Ordering::Greater => Err(EventTimeError::EndBeforeStart),
            cmp::Ordering::Less => Ok(EventTime { start, end }),
            _ => Err(EventTimeError::Unknown),

        }
    }

    pub fn start_date(&self) -> Date<Local> {
        self.start.date()
    }

    pub fn start_datetime(&self) -> DateTime<Local> {
        self.start.with_timezone(&Local)
    }

    pub fn end_date(&self) -> Date<Local> {
        self.end.date()
    }

    pub fn end_datetime(&self) -> DateTime<Local> {
        self.end.with_timezone(&Local)
    }
}

impl From<&str> for EventTime {
    fn from(item: &str) -> Self {
        let mut v = item.split('|').collect::<Vec<&str>>();
        let end: &str = v.pop().unwrap();
        let start: &str = v.pop().unwrap();
        println!("{}|{}", start, end);

        EventTime {
            start: Local.datetime_from_str(start, PARSE_TIME).unwrap(),
            end: Local.datetime_from_str(end, PARSE_TIME).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    time: EventTime,
    description: String,
}

impl Event {
    pub fn new(time: EventTime, description: String) -> Event {
        Event {
            time,
            description,
        }
    }

    pub fn time(&self) -> &EventTime {
        &self.time
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
        let start = Local.ymd(2022, 4, 27).and_hms(12, 0, 0);
        let end = Local.ymd(2022, 5, 27).and_hms(12, 30, 0);

        let e = EventTime::new(start, end);
        assert_eq!(e.is_ok(), true);
    }

    #[test]
    fn end_before_start() {
        let start = Local.ymd(2022, 4, 5).and_hms(12, 0, 0);
        let end = Local.ymd(2022, 4, 5).and_hms(10, 0, 0);

        let e = EventTime::new(start, end);
        assert_eq!(e.is_err(), true);
    }

    #[test]
    fn time_from_str() {
        let time = EventTime::new(Local.ymd(2022, 1, 1).and_hms(0, 0, 0), Local.ymd(2022, 1, 1).and_hms(1, 0, 0)).unwrap();

        assert_eq!(EventTime::from("2022-01-01 00:00:00 +06:00|2022-01-01 01:00:00 +06:00"), time);
    }
}
