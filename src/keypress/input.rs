use chrono::Duration;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log2::info;

use crate::{
    app::{App, InputMode},
    popup::input::PopupInputState,
    widgets::{calendar::CalendarState, event_view::EventViewState},
};

pub fn handle<'a>(key: KeyEvent, mut app: App<'a>) -> App<'a> {
    match key.code {
        KeyCode::Char(c) => on_key(c, app),
        KeyCode::Left => on_left(app),
        KeyCode::Right => on_right(app),
        KeyCode::Enter | KeyCode::Tab => on_next(app),
        KeyCode::BackTab => on_previous(app),
        KeyCode::Tab | KeyCode::Enter => on_next(app),
        KeyCode::Backspace => on_erase(app),
        KeyCode::Esc => on_exit(app),
        _ => app,
    }
}

pub fn on_left<'a>(mut app: App<'a>) -> App<'a> {
    info!("Left in Input InputMode is not implemented");
    app
}

pub fn on_right<'a>(mut app: App<'a>) -> App<'a> {
    info!("Right in Input InputMode is not implemented");
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
            app = on_finish(app);
        }
    }
    app
}

pub fn on_previous<'a>(mut app: App<'a>) -> App<'a> {
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

/// Parse popup input into a new event and saves
/// it in the db. Then clears popup input and loads
/// events for selected date.
pub fn on_finish<'a>(mut app: App<'a>) -> App<'a> {
    if let Some(selected_idx) = app.state_events.selected {
        let date = app.state_calendar.get_selected_date();

        let selected_event_id = app
            .state_events
            .events
            .get(selected_idx)
            .expect("No event selected")
            .id();
        let event = app
            .state_popup
            .input
            .parse(selected_event_id)
            .expect("Could not parse popup input");

        app.input_mode.restore();
        match app.input_mode.current() {
            Some(InputMode::Normal) => app.files.add_event(event).unwrap(),
            Some(InputMode::Select) => app.files.update_event(event).unwrap(),
            _ => {}
        }

        app.state_events = EventViewState::new(
            Some(selected_idx),
            app.files
                .get_events_on_date(app.state_calendar.get_selected_date()),
        );
    }
    app.state_popup.clear();
    app.state_popup.visible = false;
    app
}

pub fn on_exit<'a>(mut app: App<'a>) -> App<'a> {
    app.input_mode.restore();
    app.state_popup.clear();
    app.state_popup.visible = false;
    app
}
