#![allow(unused)]

mod app;
mod calendar;
mod event;
mod files;
mod keypress_handler;
mod ui;
mod widgets;

use app::run;
use chrono::{Local, NaiveDate};
use log2::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let weeks = Weeks::new(Local.ymd(2023, 5, 31), 36, 20);
    // for spans in weeks.content() {
    //     for span in spans.0 {
    //         print!("{}", span.content)
    //     }
    //     println!();
    // }

    // let files = files::Files::new().unwrap();
    // let events = files.get_events_on_date(NaiveDate::from_ymd(2023, 7, 18));
    // dbg!(&events);

    let _log2 = log2::open("log.txt")
        .size(100 * 1024 * 1024)
        .rotate(20)
        .tee(false)
        .module(true)
        .level("info")
        .start();

    let now = Local::now();
    info!("Started Rayday at {} on {}", now.time(), now.date_naive());

    let tick_rate = std::time::Duration::from_secs(5);
    run(tick_rate, true)?;

    Ok(())
}
