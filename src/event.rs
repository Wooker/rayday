use chrono::{prelude::*, Duration};

use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    cmp,
    fmt::{Debug, Display},
    str::FromStr,
};

const PARSE_DATE: &str = "%Y-%m-%d";
const PARSE_TIME: &str = "%H:%M:%S";

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
    IncorrectTime,
    EndBeforeStart,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct EventTime {
    start: NaiveTime,
    end: NaiveTime,
}

impl EventTime {
    pub fn new(start: NaiveTime, end: NaiveTime) -> Result<EventTime, EventTimeError> {
        match start.cmp(&end) {
            cmp::Ordering::Greater => Err(EventTimeError::EndBeforeStart),
            cmp::Ordering::Less => Ok(EventTime { start, end }),
            _ => Err(EventTimeError::Unknown),
        }
    }

    pub fn new_md(
        date: NaiveDate, //Date<Local>,
        start: (u32, u32),
        end: (u32, u32),
    ) -> Result<EventTime, EventTimeError> {
        let start = NaiveTime::from_hms_opt(start.0, start.1, 0).unwrap();
        let end = NaiveTime::from_hms_opt(end.0, end.1, 0).unwrap();

        match start.cmp(&end) {
            cmp::Ordering::Greater => Err(EventTimeError::EndBeforeStart),
            cmp::Ordering::Less => Ok(EventTime { start, end }),
            _ => Err(EventTimeError::Unknown),
        }
    }

    pub fn start_datetime(&self) -> NaiveTime {
        self.start
    }

    pub fn end_datetime(&self) -> NaiveTime {
        self.end
    }

    pub fn to_string(&self) -> String {
        format!("{}-{}", self.start_datetime(), self.end_datetime())
    }
}

impl From<&str> for EventTime {
    fn from(item: &str) -> Self {
        // dbg!(&item);
        let mut v = item.split_once('-').unwrap();
        let start: &str = v.0;
        let end: &str = v.1;

        EventTime {
            start: NaiveTime::parse_from_str(start, PARSE_TIME).unwrap(),
            end: NaiveTime::parse_from_str(end, PARSE_TIME).unwrap(),
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

        let start = NaiveTime::from_hms_opt(hours, minutes, 0).unwrap();
        let (end, seconds) = start.overflowing_add_signed(d);
        EventTime { start, end }
    }

    fn now(d: Duration) -> EventTime {
        let now = Local::now().with_nanosecond(0).unwrap();
        let start = NaiveTime::from_hms_opt(now.hour(), now.minute(), now.second()).unwrap();
        let (end, seconds) = start.overflowing_add_signed(d);

        EventTime { start, end }
    }
}

#[derive(Debug)]
pub enum EventError {
    Parsing,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Event {
    date: NaiveDate,
    time: EventTime,
    description: String,
}

impl Event {
    pub fn new(date: NaiveDate, time: EventTime, description: String) -> Event {
        Event {
            date,
            time,
            description,
        }
    }

    pub fn time(&self) -> EventTime {
        self.time
    }

    pub fn desc(&self) -> String {
        self.description.to_string()
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}|{}|{}",
            self.date,
            self.time.to_string(),
            self.description
        )
    }
}

impl FromStr for Event {
    type Err = EventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // dbg!(&s);
        let mut v = s.split('|').collect::<Vec<&str>>();

        let description = String::from(v.pop().unwrap());
        let time = EventTime::from(v.pop().unwrap());
        let date = NaiveDate::parse_from_str(v.pop().unwrap(), PARSE_DATE).unwrap();

        let event = Event {
            date,
            time,
            description,
        };
        Ok(event)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.time.start < other.time.start {
            Some(std::cmp::Ordering::Less)
        } else if self.time.start == other.time.start && self.time.end < other.time.end {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Greater)
        }
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.time.start.cmp(&other.time.start) == std::cmp::Ordering::Equal {
            // println!("Start the same");
            if self.time.end.cmp(&other.time.end) == std::cmp::Ordering::Equal {
                std::cmp::Ordering::Less
            } else {
                self.time.end.cmp(&other.time.end)
            }
        } else {
            self.time.start.cmp(&other.time.start)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let start = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        let end = NaiveTime::from_hms_opt(12, 30, 0).unwrap();
        let e = EventTime::new(start, end).unwrap();

        let event = Event::new(NaiveDate::from_ymd(2023, 7, 18), e, String::from("Test"));

        let s = event.to_string();
        assert_eq!(event, s.parse::<Event>().unwrap());
    }

    #[test]
    fn event_time_constructor_normal_time_ordering() {
        let start = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        let end = NaiveTime::from_hms_opt(12, 30, 0).unwrap();

        let e = EventTime::new(start, end);
        assert!(e.is_ok());
    }

    #[test]
    fn event_time_constructor_end_before_start() {
        let start = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        let end = NaiveTime::from_hms_opt(10, 0, 0).unwrap();

        let e = EventTime::new(start, end);
        assert!(e.is_err());
    }

    #[test]
    fn event_time_parsing() {
        let time = EventTime::new(
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(1, 0, 0).unwrap(),
        )
        .unwrap();

        assert_eq!(EventTime::from("00:00:00-01:00:00"), time);
    }
}
