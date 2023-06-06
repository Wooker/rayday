#![allow(unused)]

mod app;
mod calendar;
mod event;
mod files;
mod ui;
mod widgets;

use app::run;
use std::error::Error;

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
