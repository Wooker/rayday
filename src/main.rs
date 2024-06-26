#![allow(unused)]

mod app;
mod calendar;
mod event;
mod files;
mod keypress;
mod popup;
mod runner;
mod ui;
mod widgets;

use crate::app::App;

use anyhow::Result;
use chrono::{Local, NaiveDate};
use log2::*;
use runner::run;
use std::error::Error;

fn main() -> Result<()> {
    let _log2 = log2::open("log.txt")
        .size(100 * 1024 * 1024)
        .rotate(20)
        .tee(false)
        .module(true)
        .level("debug")
        .start();

    let now = Local::now();
    info!("Started Rayday at {} on {}", now.time(), now.date_naive());

    let tick_rate = std::time::Duration::from_secs(5);
    let mut app = App::new("RayDay", true);
    let result = run(app, tick_rate, true);

    info!("Shutdown with result: {:?}", result);
    Ok(())
}
