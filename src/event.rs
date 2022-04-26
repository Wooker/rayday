use chrono::prelude::*;

use std::{fmt, cmp};

enum Periodicity {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug)]
enum EventError {
    EndBeforeStart,
}

#[derive(Debug)]
struct Event {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl Event {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Event, EventError> {
        match start.cmp(&end) {
            cmp::Ordering::Greater => Err(EventError::EndBeforeStart),
            _ => Ok(Event { start, end, })

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_ordering() {
        let start = Utc.ymd(2022, 4, 5).and_hms(12, 0, 0);
        let end = Utc.ymd(2022, 5, 5).and_hms(12, 0, 0);

        let e = Event::new(start, end);
        assert_eq!(e.is_ok(), true);
    }

    #[test]
    fn end_before_start() {
        let start = Utc.ymd(2022, 4, 5).and_hms(12, 0, 0);
        let end = Utc.ymd(2022, 3, 5).and_hms(12, 0, 0);

        let e = Event::new(start, end);
        assert_eq!(e.is_err(), true);
    }
}
