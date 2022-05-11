use crate::ui;
use chrono::prelude::*;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rayday::get_days_from_month;
use std::{
    error::Error,
    io,
    time::{Duration, Instant}, borrow::Borrow,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{ListState, TableState},
    Terminal,
};

use crate::{
    config::Files,
    calendar::Calendar,
    event::{Event as CalEvent, EventTime as CalEventTime, Today},
};

use chrono::Duration as ChronoDuration;
use pickledb::error::Result as PickleResult;

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

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub struct StatefulDayList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulDayList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}


pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub enhanced_graphics: bool,
    pub files: Files,
    pub events: StatefulList<CalEvent>,
    pub calendar: Calendar,
    pub chosen_date: (u32, u32),
    //pub days: StatefulList<Date<Local>>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        let files = Files::new().unwrap();
        let events = files.events_stateful_list(Local::today());
        let calendar = Calendar::new();
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Calendar", "Todo"]),
            enhanced_graphics,
            files,
            events,
            //days: StatefulList::with_items(calendar.from_today(2)), // 2 weeks
            chosen_date: Calendar::today(),
            calendar,
        }
    }

    pub fn on_up(&mut self) {
        if let Some(day) = self.chosen_date.1.checked_sub(7) {
            self.chosen_date.1 = day;
        } else {
            // TODO add year to calendar state
            let days_in_prev_month = get_days_from_month(2022, self.chosen_date.0.checked_sub(1).unwrap_or(12));
            self.chosen_date.0 -= 1;
            self.chosen_date.1 = days_in_prev_month as u32;
        }
    }

    pub fn on_down(&mut self) {
        let days_in_curr_month = get_days_from_month(2022, self.chosen_date.0);
        if self.chosen_date.1 + 7 <= days_in_curr_month as u32 {
            self.chosen_date.1 += 7;
        } else {
            self.chosen_date.0 += 1;
            self.chosen_date.1 = 1;
        }
    }

    pub fn on_right(&mut self) {
        let days_in_curr_month = get_days_from_month(2022, self.chosen_date.0);
        if self.chosen_date.1 + 1 <= days_in_curr_month as u32 {
            self.chosen_date.1 += 1;
        } else {
            self.chosen_date.0 += 1;
            self.chosen_date.1 = 1;
        }
    }

    pub fn on_left(&mut self) {
        if self.chosen_date.1 - 1 > 0 {
            self.chosen_date.1 -= 1;
        } else {
            let days_in_prev_month = get_days_from_month(2022, self.chosen_date.0.checked_sub(1).unwrap_or(12));
            self.chosen_date.0 -= 1;
            self.chosen_date.1 = days_in_prev_month as u32;
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
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
                self.on_add_item();
            }
            'd' => {
                self.on_rem_item();
            }
            _ => {}
        }
    }

    pub fn on_ctrl_key(&mut self, c: char) {
        match c { 'h' => {
                self.on_left();
            }
            'l' => {
                self.on_right();
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
    }

    pub fn on_add_item(&mut self) {
        match self.tabs.index {
            _ => {
                let event = CalEvent::new(
                    CalEventTime::now(chrono::Duration::minutes(30)),
                    String::from("Test")
                );
                self.events.add(event.clone());
                self
                .files
                .add_event(event.clone())
                .unwrap();
            }
            1 => self.files.add_todo("todo", "TODO").unwrap(),
        }
    }

    pub fn on_rem_item(&mut self) {
        match self.tabs.index {
            0 => {
                let i = self.events.state.selected().unwrap();
                let event = self.events.items.iter().nth(i).unwrap();
                self
                .files
                .remove_event(event.time()).unwrap();
                if i == self.events.items.len() - 1 {
                    self.events.previous();
                }
                self.events.items.remove(i);
            },

            _ => (),
            //1 => self.calendar.add_todo("todo", "TODO").unwrap(),
        };
    }
}

pub fn run(tick_rate: Duration, enhanced_graphics: bool) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new("RayDay", enhanced_graphics);
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) if key.modifiers == KeyModifiers::CONTROL => {
                        app.on_ctrl_key(c)
                    }
                    KeyCode::Char(c) => app.on_key(c),
                    KeyCode::Left => app.on_left(),
                    KeyCode::Up => app.on_up(),
                    KeyCode::Right => app.on_right(),
                    KeyCode::Down => app.on_down(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
        if app.should_quit {
            return Ok(());
        }
    }
}
