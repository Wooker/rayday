use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, InputMode};

pub fn handle<'a>(key: KeyEvent, mut app: App<'a>) -> App<'a> {
    match key.code {
        KeyCode::Up => on_up(app),
        KeyCode::Down => on_down(app),
        KeyCode::Char(c) => on_key(c, app),
        KeyCode::Esc => on_key('q', app),
        _ => app,
    }
}

pub fn on_up<'a>(mut app: App<'a>) -> App<'a> {
    app.state_events.selected = if let Some(sel) = app.state_events.selected {
        Some(sel.saturating_sub(1))
    } else {
        Some(0)
    };

    app
}

pub fn on_down<'a>(mut app: App<'a>) -> App<'a> {
    let events = &app.state_events.events;
    app.state_events.selected = if let Some(sel) = app.state_events.selected {
        if sel == events.len() - 1 {
            Some(sel)
        } else {
            Some(sel.saturating_add(1))
        }
    } else {
        Some(0)
    };
    app
}

pub fn on_delete<'a>(mut app: App<'a>) -> App<'a> {
    if let Some(selected_idx) = app.state_events.selected {
        let date = app.state_calendar.get_selected_date();

        let selected_event = app.state_events.events.remove(selected_idx);

        if let Some(upper) = app.state_events.events.get(selected_idx) {
            app.state_events.selected = Some(selected_idx);
        } else if let Some(lower) = app.state_events.events.get(selected_idx.saturating_sub(1)) {
            app.state_events.selected = Some(selected_idx - 1);
        } else {
            app.input_mode = InputMode::Normal;
            app.state_events.selected = None;
        }

        app.files.remove_event(selected_event.id().unwrap());
    }
    app
}

pub fn on_key<'a>(c: char, mut app: App<'a>) -> App<'a> {
    match c {
        'q' => {
            app.input_mode = InputMode::Normal;
            app
        }
        'j' => on_down(app),
        'k' => on_up(app),
        'd' => on_delete(app),
        _ => app,
    }
}
