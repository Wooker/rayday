use std::ops::Add;

use anyhow::Result;
use chrono::{Datelike, Duration, Local, NaiveDateTime};
use rusqlite::ToSql;

use crate::event::Event;

pub struct PopupInput {
    pub start_date: String,
    pub start_time: String,
    pub end_date: String,
    pub end_time: String,
    pub description: String,
}

impl Default for PopupInput {
    fn default() -> Self {
        PopupInput {
            start_date: String::new(),
            start_time: String::new(),
            end_date: String::new(),
            end_time: String::new(),
            description: String::new(),
        }
    }
}

impl PopupInput {
    pub fn parse(&self) -> Result<Event> {
        let now = Local::now();
        let date = now.date_naive();
        let time = now.time();
        Ok(Event::new(
            None,
            "Test".to_string(),
            NaiveDateTime::new(date, time),
            NaiveDateTime::new(date, time.add(Duration::hours(1))),
        ))
    }
}
