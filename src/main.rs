#![deny(warnings)]
#![allow(unused)]
#![feature(int_roundings)]

mod app;
mod calendar;
mod config;
mod event;
mod lib;
mod ui;
mod widgets;

use app::run;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let tick_rate = std::time::Duration::from_secs(1);
    run(tick_rate, true)?;

    Ok(())
}
