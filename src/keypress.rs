use anyhow::Result;
use chrono::Duration;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::{App, InputMode},
    widgets::{calendar::CalendarState, event_view::EventViewState},
};

mod normal;
mod select;

pub fn handle<'a>(key: KeyEvent, mut app: App<'a>) -> Result<App<'a>> {
    // TODO: implement handlers to return Result<App<'a>> insted of App<'a>
    // to get rid of `let app ...`
    let app = match app.input_mode {
        InputMode::Normal => normal::handle_normal(key, app),
        InputMode::Select => select::handle(key, app),
        /*
                InputMode::AddingTime => match key.code {
                    KeyCode::Enter => {
                        //app.messages.push(app.input.drain(..).collect());
                        //app.on_add_item();
                        app.input_mode = InputMode::AddingDescription;
                    }
                    KeyCode::Char(c) => {
                        // app.input_time.push(c);
                    }
                    KeyCode::Backspace => {
                        // app.input_time.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
                InputMode::AddingDescription => match key.code {
                    KeyCode::Enter => {
                        app.on_add_item();
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char(c) => {
                        // app.input_description.push(c);
                    }
                    KeyCode::Backspace => {
                        // app.input_description.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::AddingTime;
                    }
                    _ => {}
                },
        */
        _ => app,
    };

    Ok(app)
}
