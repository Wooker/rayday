#![deny(warnings)]
#![allow(unused)]

mod app;
mod ui;
mod calendar;
mod event;

use app::run;

use std::{
    error::Error
};
use calendar::Calendar;
use event::Event;
use chrono::{prelude::*, Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let mut calendar = Calendar::new().unwrap();
    /*

    let tick_rate = Duration::from_secs(1);
    run(tick_rate, true)?;

    */
    Ok(())
}
