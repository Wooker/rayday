use crate::{
    event::{Event as CalEvent, EventTime as CalEventTime},
    files::Files,
    popup::{input::PopupInput, state::PopupState},
    ui,
    widgets::{calendar::CalendarState, event_view::EventViewState},
};
use anyhow::Result;
use chrono::prelude::*;
use chrono::Duration as ChronoDuration;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log2::{debug, info};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub enum InputMode {
    Normal,
    AddingTime,
    AddingDescription,
    Select,
}

pub(crate) struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub enhanced_graphics: bool,
    pub files: Files,
    pub state_tabs: TabsState<'a>,
    pub state_calendar: CalendarState,
    pub state_events: EventViewState,
    pub state_popup: PopupState,
    pub hint_text: String,
    pub input_mode: InputMode,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        let files = Files::new().unwrap();
        let selected_date = Local::now().naive_local().date();
        let events = files.get_events_on_date(selected_date);

        App {
            title,
            should_quit: false,
            state_tabs: TabsState::new(vec!["Calendar"]),
            enhanced_graphics,
            files,
            state_calendar: CalendarState::new(selected_date),
            state_events: EventViewState::new(None, events),
            state_popup: PopupState::new(PopupInput::default()),
            hint_text: String::new(),
            input_mode: InputMode::Normal,
        }
    }

    pub fn on_tick(&mut self) {
        info!("On tick");
        self.state_events.events = self
            .files
            .get_events_on_date(self.state_calendar.get_selected_date());

        info!("App events after tick: {:?}", self.state_events.events);
    }
}
