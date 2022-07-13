#![deny(warnings)]
#![allow(unused)]

#![feature(int_roundings)]

mod app;
mod config;
mod calendar;
mod event;
mod ui;
mod widgets;
mod lib;

use app::run;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let tick_rate = std::time::Duration::from_secs(1);
    run(tick_rate, true)?;

    Ok(())
}
