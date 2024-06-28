use chrono::Duration;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log2::debug;

use crate::{
    app::{App, InputMode},
    popup::input::PopupInputState,
    widgets::{calendar::CalendarState, event_view::EventViewState},
};

use super::normal::on_add_event;

pub fn handle<'a>(key: KeyEvent, mut app: App<'a>) -> App<'a> {
    match key.code {
        // KeyCode::Char(c) if key.modifiers == KeyModifiers::CONTROL => app.on_ctrl_key(c),
        KeyCode::Char(c) => on_key(c, app),
        KeyCode::Left => on_left(app),
        KeyCode::Right => on_right(app),
        KeyCode::Enter | KeyCode::Tab => on_next(app),
        KeyCode::Tab if key.modifiers == KeyModifiers::SHIFT => on_previous(app),
        KeyCode::Enter => on_next(app),
        KeyCode::Backspace => on_erase(app),
        KeyCode::Esc => on_exit(app),
        _ => app,
    }
}

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

pub fn on_key<'a>(c: char, mut app: App<'a>) -> App<'a> {
    match app.state_popup.input.state {
        PopupInputState::StartDate => app.state_popup.input.start_date.push(c),
        PopupInputState::StartTime => app.state_popup.input.start_time.push(c),
        PopupInputState::EndDate => app.state_popup.input.end_date.push(c),
        PopupInputState::EndTime => app.state_popup.input.end_time.push(c),
        PopupInputState::Description => app.state_popup.input.description.push(c),
    }
    app
}

pub fn on_erase<'a>(mut app: App<'a>) -> App<'a> {
    match app.state_popup.input.state {
        PopupInputState::StartDate => app.state_popup.input.start_date.pop(),
        PopupInputState::StartTime => app.state_popup.input.start_time.pop(),
        PopupInputState::EndDate => app.state_popup.input.end_date.pop(),
        PopupInputState::EndTime => app.state_popup.input.end_time.pop(),
        PopupInputState::Description => app.state_popup.input.description.pop(),
    };
    app
}

pub fn on_next<'a>(mut app: App<'a>) -> App<'a> {
    match app.state_popup.input.state {
        PopupInputState::StartDate => app.state_popup.input.state = PopupInputState::StartTime,
        PopupInputState::StartTime => app.state_popup.input.state = PopupInputState::EndDate,
        PopupInputState::EndDate => app.state_popup.input.state = PopupInputState::EndTime,
        PopupInputState::EndTime => app.state_popup.input.state = PopupInputState::Description,
        PopupInputState::Description => {
            app.state_popup.input.state = PopupInputState::StartDate;
            app.input_mode = InputMode::Normal;
            app = crate::keypress::normal::on_key('a', app);
            app = on_add_event(app);
            app = on_exit(app);
        }
    }
    app
}

pub fn on_previous<'a>(mut app: App<'a>) -> App<'a> {
    debug!("On Prev");
    match app.state_popup.input.state {
        PopupInputState::StartDate => {
            app = on_exit(app);
        }
        PopupInputState::StartTime => app.state_popup.input.state = PopupInputState::StartDate,
        PopupInputState::EndDate => app.state_popup.input.state = PopupInputState::StartTime,
        PopupInputState::EndTime => app.state_popup.input.state = PopupInputState::EndDate,
        PopupInputState::Description => app.state_popup.input.state = PopupInputState::EndTime,
    }
    app
}
pub fn on_exit<'a>(mut app: App<'a>) -> App<'a> {
    app.input_mode = InputMode::Normal;
    app.state_popup.visible = false;
    app
}
