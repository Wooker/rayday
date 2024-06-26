use crate::{
    popup::input::PopupInput,
    ui,
    widgets::{calendar::CalendarState, event_view::EventViewState},
};
use chrono::prelude::*;
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

use crate::{
    event::{Event as CalEvent, EventTime as CalEventTime},
    files::Files,
};

use chrono::Duration as ChronoDuration;

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
    Selecting,
}

pub(crate) struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub state_tabs: TabsState<'a>,
    pub enhanced_graphics: bool,
    pub files: Files,
    pub state_calendar: CalendarState, //Date<Local>,
    pub state_events: EventViewState,
    pub popup_input: PopupInput,
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
            popup_input: PopupInput::default(),
            hint_text: String::new(),
            input_mode: InputMode::Normal,
        }
    }

    pub fn on_up(&mut self) {
        match self.input_mode {
            InputMode::Normal => {
                self.state_calendar = CalendarState::new(
                    self.state_calendar
                        .get_selected_date()
                        .checked_sub_signed(ChronoDuration::weeks(1))
                        .unwrap(),
                );
                self.state_events = EventViewState::new(
                    None,
                    self.files
                        .get_events_on_date(self.state_calendar.get_selected_date()),
                );
            }
            InputMode::Selecting => {
                self.state_events.selected = if let Some(sel) = self.state_events.selected {
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

                    debug!(
                        "Events on date {}: {:?}",
                        date,
                        self.state_events
                            .events
                            .iter()
                            .map(|e| format!("{}", e))
                            .collect::<Vec<String>>()
                    );

                    let selected_event = self.state_events.events.remove(selected_idx);

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

                    self.files.remove_event(selected_event.id().unwrap());
                }
            }
            _ => {}
        }
    }

    // Handler for adding a new event
    pub fn on_add_item(&mut self) {
        match self.state_tabs.index {
            0 => {
                let event = self
                    .popup_input
                    .parse()
                    .expect("Could not parse popup input");
                // s_e == start-end

                // let s_e: Vec<&str> = self.popup_input.start_date.split('-').collect();

                // let s_h_m: Vec<&str> = s_e.get(0).unwrap().split(':').collect();
                // let e_h_m: Vec<&str> = s_e.get(1).unwrap().split(':').collect();

                // let (s_h, s_m) = (s_h_m.get(0).unwrap(), s_h_m.get(1).unwrap());
                // let (e_h, e_m) = (e_h_m.get(0).unwrap(), e_h_m.get(1).unwrap());

                // let date = self.state_calendar.get_selected_date();

                // let event = CalEvent::new(
                //     None,
                //     self.input_description.clone(),
                //     NaiveDateTime::parse_from_str(
                //         format!("{} {}", date, s_e.get(0).unwrap()).as_str(),
                //         "%Y-%m-%d %H:%M:%S",
                //     )
                //     .unwrap(),
                //     NaiveDateTime::parse_from_str(
                //         format!("{} {}", date, s_e.get(1).unwrap()).as_str(),
                //         "%Y-%m-%d %H:%M:%S",
                //     )
                //     .unwrap(),
                // );
                self.files.add_event(event).unwrap();

                self.popup_input = PopupInput::default();
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

    debug!("Result: {:?}", res);
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
        debug!("TICK");

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char(c) if key.modifiers == KeyModifiers::CONTROL => {
                            app.on_ctrl_key(c)
                        }
                        KeyCode::Char(c) => app.on_key(c),
                        KeyCode::Left => app.on_left(),
                        KeyCode::Up => app.on_up(),
                        KeyCode::Right => app.on_right(),
                        KeyCode::Down => app.on_down(),
                        KeyCode::Enter => {
                            if app.state_events.events.len() > 0 {
                                app.input_mode = InputMode::Selecting;
                                app.state_events.select(Some(0));
                            }
                        }
                        _ => {}
                    },
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
                    InputMode::Selecting => match key.code {
                        KeyCode::Char(c) => app.event_on_key(c),
                        KeyCode::Left => app.on_left(),
                        KeyCode::Up => app.on_up(),
                        KeyCode::Right => app.on_right(),
                        KeyCode::Down => app.on_down(),
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.state_events.select(None);
                        }
                        _ => {}
                    },
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
