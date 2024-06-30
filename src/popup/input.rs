use std::ops::Add;

use anyhow::Result;
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use rusqlite::ToSql;

use crate::event::Event;

#[derive(Debug)]
pub enum PopupInputState {
    StartDate,
    StartTime,
    EndDate,
    EndTime,
    Description,
}

#[derive(Debug)]
pub struct PopupInput {
    pub state: PopupInputState,
    pub start_date: String,
    pub start_time: String,
    pub end_date: String,
    pub end_time: String,
    pub description: String,
}

impl Default for PopupInput {
    fn default() -> Self {
        PopupInput {
            state: PopupInputState::StartDate,
            start_date: String::new(),
            start_time: String::new(),
            end_date: String::new(),
            end_time: String::new(),
            description: String::new(),
        }
    }
}

impl PopupInput {
    pub fn parse(&self, id: Option<usize>) -> Result<Event> {
        let start_date = NaiveDate::parse_from_str(self.start_date.as_str(), "%Y-%m-%d")?;
        let end_date = NaiveDate::parse_from_str(self.end_date.as_str(), "%Y-%m-%d")?;
        let start_time = NaiveTime::parse_from_str(self.start_time.as_str(), "%H:%M:%S")?;
        let end_time = NaiveTime::parse_from_str(self.end_time.as_str(), "%H:%M:%S")?;

        Ok(Event::new(
            id,
            self.description.clone(),
            NaiveDateTime::new(start_date, start_time),
            NaiveDateTime::new(end_date, end_time),
        ))
    }

    pub fn load(&mut self, event: &Event) -> Result<()> {
        let start = event.start();
        self.start_date = start.date().to_string();
        self.start_time = start.time().to_string();

        let end = event.end();
        self.end_date = end.date().to_string();
        self.end_time = end.time().to_string();

        self.description = event.desc();

        Ok(())
    }

    pub fn set_date(&mut self, date: NaiveDate) -> Result<()> {
        self.start_date = date.to_string();
        self.end_date = date.to_string();
        Ok(())
    }
}
