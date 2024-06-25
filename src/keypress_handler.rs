use chrono::{Duration, NaiveDate};

use crate::{
    app::{App, InputMode},
    widgets::{calendar::CalendarState, event_view::EventViewState},
};

enum Direction {
    Before,
    After,
}

pub struct KeyPressHandler {
    pub input_mode: InputMode,
}

/*
fn update_state_calendar(date: NaiveDate, dir: Direction, dur: Duration) -> CalendarState {
    match dir {
        Direction::Before => {
            CalendarState::new(date.checked_sub_signed(Duration::weeks(1)).unwrap())
        }
        Direction::After => {
            CalendarState::new(date.checked_add_signed(Duration::weeks(1)).unwrap())
        }
    }
}
impl KeyPressHandler {
    pub fn on_up(&self, date: NaiveDate) {
        match self.input_mode {
            InputMode::Normal => {
                // update calendar state
                let new_state_calendar =
                    update_state_calendar(date, Direction::Before, Duration::weeks(1));
                // update events
                let new_state_events = &mut EventViewState::new(
                    None,
                    files.get_events_on_date(state_calendar.get_selected_date()),
                );
            }
            InputMode::Selecting => {
                state_events.selected = if let Some(sel) = state_events.selected {
                    Some(sel.saturating_sub(1))
                } else {
                    Some(0)
                }
            }
            _ => {}
        }
    }

    pub fn on_left(&mut self) {
        match self.input_mode {
            InputMode::Normal => {
                self.state_calendar = CalendarState::new(
                    self.state_calendar
                        .get_selected_date()
                        .checked_sub_signed(ChronoDuration::days(1))
                        .unwrap(),
                );
                self.state_events = EventViewState::new(
                    None,
                    self.files
                        .get_events_on_date(self.state_calendar.get_selected_date()),
                );
            }
            _ => {}
        }
    }

    pub fn on_down(&mut self) {
        let events = self
            .files
            .get_events_on_date(self.state_calendar.get_selected_date());

        match self.input_mode {
            InputMode::Normal => {
                self.state_calendar = CalendarState::new(
                    self.state_calendar
                        .get_selected_date()
                        .checked_add_signed(ChronoDuration::weeks(1))
                        .unwrap(),
                );
                self.state_events = EventViewState::new(None, events);
            }
            InputMode::Selecting => {
                self.state_events.selected = if let Some(sel) = self.state_events.selected {
                    if sel == events.len() - 1 {
                        Some(sel)
                    } else {
                        Some(sel.saturating_add(1))
                    }
                } else {
                    Some(0)
                }
            }
            _ => {}
        }
    }

    pub fn on_right(&mut self) {
        match self.input_mode {
            InputMode::Normal => {
                self.state_calendar = CalendarState::new(
                    self.state_calendar
                        .get_selected_date()
                        .checked_add_signed(ChronoDuration::days(1))
                        .unwrap(),
                );
                self.state_events = EventViewState::new(
                    None,
                    self.files
                        .get_events_on_date(self.state_calendar.get_selected_date()),
                );
            }
            _ => {}
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
                //dbg!(&self.last_date);
                //std::thread::sleep(Duration::from_secs(5));
            }
            'j' => {
                self.on_down();
            }
            'k' => {
                self.on_up();
            }
            'h' => {
                self.on_left();
            }
            'l' => {
                self.on_right();
            }
            'a' => {
                self.input_mode = InputMode::AddingTime;
            }
            _ => {}
        }
    }

    pub fn on_ctrl_key(&mut self, c: char) {
        match c {
            'h' => {
                self.on_left();
            }
            'l' => {
                self.on_right();
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        info!("On tick");
        self.state_events.events = self
            .files
            .get_events_on_date(self.state_calendar.get_selected_date());

        info!("App events after tick: {:?}", self.state_events.events);
    }

    pub fn event_on_key(&mut self, c: char) {
        match c {
            'k' => self.on_up(),
            'j' => self.on_down(),
            'q' => {
                self.input_mode = InputMode::Normal;
                self.state_events.select(None);
            }
            'd' => {
                if let Some(selected_idx) = self.state_events.selected {
                    let date = self.state_calendar.get_selected_date();

                    self.state_events.events.remove(selected_idx);
                    debug!("Events on date {}: {:?}", date, self.state_events.events);
                    let selected_event = self.state_events.events.get(selected_idx).unwrap();
                    //self.files.remove_event(selected_event.id().unwrap());

                    if let Some(upper) = self.state_events.events.get(selected_idx) {
                        self.state_events.selected = Some(selected_idx);
                    } else if let Some(lower) =
                        self.state_events.events.get(selected_idx.saturating_sub(1))
                    {
                        self.state_events.selected = Some(selected_idx - 1);
                    } else {
                        self.input_mode = InputMode::Normal;
                        self.state_events.selected = None;
                    }
                }
            }
            _ => {}
        }
    }

    // Handler for adding a new event
    pub fn on_add_item(&mut self) {
        match self.state_tabs.index {
            0 => {
                // s_e == start-end
                let s_e: Vec<&str> = self.input_time.split('-').collect();

                let s_h_m: Vec<&str> = s_e.get(0).unwrap().split(':').collect();
                let e_h_m: Vec<&str> = s_e.get(1).unwrap().split(':').collect();

                let (s_h, s_m) = (s_h_m.get(0).unwrap(), s_h_m.get(1).unwrap());
                let (e_h, e_m) = (e_h_m.get(0).unwrap(), e_h_m.get(1).unwrap());

                let date = self.state_calendar.get_selected_date();

                let event = CalEvent::new(
                    None,
                    self.input_description.clone(),
                    NaiveDateTime::parse_from_str(
                        format!("{} {}", date, s_e.get(0).unwrap()).as_str(),
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .unwrap(),
                    NaiveDateTime::parse_from_str(
                        format!("{} {}", date, s_e.get(1).unwrap()).as_str(),
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .unwrap(),
                );
                self.files.add_event(event).unwrap();

                self.input_time = String::new();
                self.input_description = String::new();
                self.state_events = EventViewState::new(
                    None,
                    self.files
                        .get_events_on_date(self.state_calendar.get_selected_date()),
                );
            }
            _ => {}
        }
    }
}
*/
