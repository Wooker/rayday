#![deny(warnings)]
#![allow(unused)]

mod app;
mod ui;
mod calendar;
mod event;

use app::run;

use std::{
    time::Duration,
    error::Error
};
use calendar::Calendar;
use event::Event;
use chrono::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut calendar = Calendar::new().unwrap();

    let now = Utc::now().with_nanosecond(0).unwrap();
    let phys = Utc.ymd(2022, 4, 28).and_hms(15, 0, 0);
    calendar.add_event(Event::new(now, now.with_minute(now.minute() + 2).unwrap(), "Test".to_string()).unwrap());
    calendar.add_event(Event::new(phys, phys.with_minute(now.minute() + 30).unwrap(), "Phys202 final".to_string()).unwrap());
    for e in calendar.get_events() {
        println!("{} --- {}", e.get_key(), e.get_value::<String>().unwrap());
    }
    /*

    let tick_rate = Duration::from_secs(1);
    run(tick_rate, true)?;

    */
    Ok(())
}
