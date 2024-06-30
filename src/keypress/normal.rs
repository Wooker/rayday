use chrono::Duration;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::{App, InputMode},
    widgets::{calendar::CalendarState, event_view::EventViewState},
};

/// Main handler for the normal input mode
pub fn handle<'a>(key: KeyEvent, mut app: App<'a>) -> App<'a> {
    match key.code {
        KeyCode::Char(c) => on_key(c, app),
        KeyCode::Up => on_up(app),
        KeyCode::Down => on_down(app),
        KeyCode::Left => on_left(app),
        KeyCode::Right => on_right(app),
        KeyCode::Enter => on_key('s', app),
        _ => app,
    }
}

/// Update calendar state by one week earlier
pub fn on_up<'a>(mut app: App<'a>) -> App<'a> {
    app.state_calendar = CalendarState::new(
        app.state_calendar
            .get_selected_date()
            .checked_sub_signed(Duration::weeks(1))
            .unwrap(),
    );
    app.state_events = EventViewState::new(
        None,
        app.files
            .get_events_on_date(app.state_calendar.get_selected_date()),
    );
    app
}

/// Update calendar state by one week further
pub fn on_down<'a>(mut app: App<'a>) -> App<'a> {
    app.state_calendar = CalendarState::new(
        app.state_calendar
            .get_selected_date()
            .checked_add_signed(Duration::weeks(1))
            .unwrap(),
    );

    let events = app
        .files
        .get_events_on_date(app.state_calendar.get_selected_date());
    app.state_events = EventViewState::new(None, events);
    app
}

/// Update calendar state by one day earlier
pub fn on_left<'a>(mut app: App<'a>) -> App<'a> {
    app.state_calendar = CalendarState::new(
        app.state_calendar
            .get_selected_date()
            .checked_sub_signed(Duration::days(1))
            .unwrap(),
    );
    app.state_events = EventViewState::new(
        None,
        app.files
            .get_events_on_date(app.state_calendar.get_selected_date()),
    );
    app
}

/// Update calendar state by one day further
pub fn on_right<'a>(mut app: App<'a>) -> App<'a> {
    app.state_calendar = CalendarState::new(
        app.state_calendar
            .get_selected_date()
            .checked_add_signed(Duration::days(1))
            .unwrap(),
    );
    app.state_events = EventViewState::new(
        None,
        app.files
            .get_events_on_date(app.state_calendar.get_selected_date()),
    );
    app
}

/// Handle key presses
pub fn on_key<'a>(c: char, mut app: App<'a>) -> App<'a> {
    match c {
        'q' => {
            app.should_quit = true;
            app
        }
        'j' => on_down(app),
        'k' => on_up(app),
        'h' => on_left(app),
        'l' => on_right(app),
        /// Go into select mode
        's' => {
            if app.state_events.events.len() > 0 {
                app.input_mode.store(InputMode::Select);
                app.state_events.select(Some(0));
            }
            app
        }
        /// Go into input mode
        'a' => {
            app.input_mode.store(InputMode::Input);
            app.state_popup
                .input
                .set_date(app.state_calendar.get_selected_date());
            app.state_popup.visible = true;
            app
        }
        _ => app,
    }
}
