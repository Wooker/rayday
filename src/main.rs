#![deny(warnings)]
#![allow(unused)]

mod app;
mod calendar;
mod event;
mod ui;

use app::run;

use calendar::Calendar;
use chrono::{prelude::*, Duration};
use event::{Event, EventTime, Today};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut calendar = Calendar::new().unwrap();
    calendar.add_event(Event::new(
        EventTime::today(22, 30, Duration::minutes(30)),
        "Test2".to_string(),
    ));
    calendar.add_event(Event::new(
        EventTime::today(22, 45, Duration::minutes(30)),
        "Test3".to_string(),
    ));

    let tick_rate = std::time::Duration::from_secs(1);
    run(tick_rate, true)?;

    Ok(())
}
