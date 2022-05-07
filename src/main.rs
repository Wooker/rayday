#![deny(warnings)]
#![allow(unused)]

mod app;
mod calendar;
mod event;
mod ui;
mod widgets;

use app::run;

use calendar::Calendar;
use chrono::{prelude::*, Duration};
use event::{Event, EventTime, Today};
use crate::widgets::calendar::{List, ListItem};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", 8 % 7);

    let tick_rate = std::time::Duration::from_secs(1);
    run(tick_rate, true)?;

    Ok(())
}
