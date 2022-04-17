#![deny(warnings)]
#![allow(unused)]
mod app;
mod ui;
mod calendar;

use app::run;

use std::{
    time::Duration,
    error::Error
};

fn main() -> Result<(), Box<dyn Error>> {
    let calendar = calendar::Calendar::new().unwrap();
    let tick_rate = Duration::from_secs(1);
    run(tick_rate, true)?;

    Ok(())
}
