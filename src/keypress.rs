use anyhow::Result;
use chrono::Duration;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::{App, InputMode},
    widgets::{calendar::CalendarState, event_view::EventViewState},
};

mod input;
mod normal;
mod select;

pub fn handle<'a>(key: KeyEvent, mut app: App<'a>) -> Result<App<'a>> {
    // TODO: implement handlers to return Result<App<'a>> insted of App<'a>
    // to get rid of `let app ...`
    let app = match app.input_mode.current() {
        Some(m) => match m {
            InputMode::Normal => normal::handle(key, app),
            InputMode::Select => select::handle(key, app),
            InputMode::Input => input::handle(key, app),
            _ => app,
        },
        None => app,
    };

    Ok(app)
}
