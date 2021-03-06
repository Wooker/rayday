use chrono::{prelude::*, Duration};

use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, cmp, fmt};

const PARSE_TIME: &str = "%Y-%m-%d %H:%M:%S %:z";

enum Periodicity {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

pub trait Today {
    fn today(hours: u32, minutes: u32, d: Duration) -> EventTime;
    fn now(d: Duration) -> EventTime;
}

#[derive(Debug)]
pub enum EventTimeError {
    EndBeforeStart,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
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

    pub fn new_md(
        date: Date<Local>,
        start: (u32, u32),
        end: (u32, u32),
    ) -> Result<EventTime, EventTimeError> {
        let start = date.and_hms(start.0, start.1, 0);
        let end = date.and_hms(end.0, end.1, 0);

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

    pub fn to_string(&self) -> String {
        format!("{}|{}", self.start_datetime(), self.end_datetime())
    }
}

impl From<&str> for EventTime {
    fn from(item: &str) -> Self {
        let mut v = item.split('|').collect::<Vec<&str>>();
        let end: &str = v.pop().unwrap();
        let start: &str = v.pop().unwrap();

        EventTime {
            start: Local.datetime_from_str(start, PARSE_TIME).unwrap(),
            end: Local.datetime_from_str(end, PARSE_TIME).unwrap(),
        }
    }
}

impl Into<String> for EventTime {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Today for EventTime {
    // duration in minutes
    fn today(hours: u32, minutes: u32, d: Duration) -> EventTime {
        let today = Local::today().and_hms(hours, minutes, 0);
        EventTime {
            start: today,
            end: today.checked_add_signed(d).unwrap(),
        }
    }

    fn now(d: Duration) -> EventTime {
        let now = Local::now().with_nanosecond(0).unwrap();
        EventTime {
            start: now,
            end: now.checked_add_signed(d).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    time: EventTime,
    description: String,
}

impl Event {
    pub fn new(time: EventTime, description: String) -> Event {
        Event { time, description }
    }

    pub fn time(&self) -> EventTime {
        self.time
    }

    pub fn desc(&self) -> String {
        self.description.to_string()
    }

    pub fn to_string(&self) -> String {
        format!("{}: {}", self.time.to_string(), self.description)
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
        let time = EventTime::new(
            Local.ymd(2022, 1, 1).and_hms(0, 0, 0),
            Local.ymd(2022, 1, 1).and_hms(1, 0, 0),
        )
        .unwrap();

        assert_eq!(
            EventTime::from("2022-01-01 00:00:00 +06:00|2022-01-01 01:00:00 +06:00"),
            time
        );
    }
}
