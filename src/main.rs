#![deny(warnings)]
#![allow(unused)]

mod app;
mod calendar;
mod config;
mod event;
mod ui;
mod widgets;

use app::run;
use chrono::{Local, TimeZone};
use std::error::Error;
use widgets::weeks::Weeks;

fn main() -> Result<(), Box<dyn Error>> {
    // let weeks = Weeks::new(Local.ymd(2023, 5, 31), 36, 20);
    // for spans in weeks.content() {
    //     for span in spans.0 {
    //         print!("{}", span.content)
    //     }
    //     println!();
    // }

    let tick_rate = std::time::Duration::from_secs(1);
    run(tick_rate, true)?;

    Ok(())
}
