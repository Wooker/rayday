#![deny(warnings)]
#![allow(unused)]

mod app;
mod config;
mod calendar;
mod event;
mod ui;
mod widgets;
mod lib;

use app::run;

use crate::config::Files;
use chrono::{prelude::*, Duration};
use event::{Event, EventTime, Today};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let tick_rate = std::time::Duration::from_secs(1);
    run(tick_rate, true)?;

    Ok(())
}
