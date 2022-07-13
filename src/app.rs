use crate::{ui, widgets::{event_view::EventState, weeks::Weeks}};
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
    config::ConfigFiles,
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

pub enum InputMode {
    Normal,
    AddingTime,
    AddingDescription,
    Selecting,
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub enhanced_graphics: bool,
    pub files: ConfigFiles,
    pub calendar: Calendar,
    pub starting_date: Date<Local>,
    pub chosen_date: Date<Local>,
    pub chosen_event: EventState,
    pub add_event: bool,
    pub input_time: String,
    pub input_description: String,
    pub hint_text: String,
    pub input_mode: InputMode,
    pub first_date: Option<Date<Local>>,
    pub last_date: Option<Date<Local>>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        let files = ConfigFiles::new().unwrap();
        let calendar = Calendar::new();
        let now = Local::now().date();

        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Calendar", "Todo"]),
            enhanced_graphics,
            files,
            starting_date: now,
            chosen_date: now,
            chosen_event: EventState::new(None),
            calendar,
            add_event: false,
            input_time: String::new(),
            input_description: String::new(),
            hint_text: String::new(),
            input_mode: InputMode::Normal,
            first_date: None,
            last_date: None,
        }
    }

    pub fn on_up(&mut self) {
        self.chosen_date = self.chosen_date.checked_sub_signed(ChronoDuration::weeks(1)).unwrap();
        /*
        if self.chosen_date.lt(&self.first_date.unwrap()) {
            self.first_date = self.first_date.unwrap().checked_sub_signed(ChronoDuration::weeks(1));
            self.last_date = self.last_date.unwrap().checked_sub_signed(ChronoDuration::weeks(1));
        }
        self.input_time = self.last_date.unwrap().to_string();
        */
    }

    pub fn on_left(&mut self) {
        self.chosen_date = self.chosen_date.checked_sub_signed(ChronoDuration::days(1)).unwrap();
        /*
        if self.chosen_date.lt(&self.first_date.unwrap()) {
            self.first_date = self.first_date.unwrap().checked_sub_signed(ChronoDuration::weeks(1));
            self.last_date = self.last_date.unwrap().checked_sub_signed(ChronoDuration::weeks(1));
        }
        self.input_time = self.last_date.unwrap().to_string();
        */
    }

    pub fn on_down(&mut self) {
        self.chosen_date = self.chosen_date.checked_add_signed(ChronoDuration::weeks(1)).unwrap();
        /*
        if self.chosen_date.gt(&self.last_date.unwrap()) {
            self.first_date = self.first_date.unwrap().checked_add_signed(ChronoDuration::weeks(1));
            self.last_date = self.last_date.unwrap().checked_add_signed(ChronoDuration::weeks(1));
        }
        self.input_time = self.last_date.unwrap().to_string();
        */
    }

    pub fn on_right(&mut self) {
        self.chosen_date = self.chosen_date.checked_add_signed(ChronoDuration::days(1)).unwrap();
        /*
        if self.chosen_date.gt(&self.last_date.unwrap()) {
            self.first_date = self.first_date.unwrap().checked_add_signed(ChronoDuration::weeks(1));
            self.last_date = self.last_date.unwrap().checked_add_signed(ChronoDuration::weeks(1));
        }
        self.input_time = self.last_date.unwrap().to_string();
        */
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
                //self.on_add_item();
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

    pub fn event_on_key(&mut self, c: char) {
        match c {
            'j' => {

            }
            'k' => {
            }
            _ => {}
        }
    }

    pub fn on_add_item(&mut self) {
        match self.tabs.index {
            _ => {
                let s_e: Vec<&str> = self.input_time.split('-').collect();

                let s_h_m: Vec<&str> = s_e.get(0).unwrap().split(':').collect();
                let e_h_m: Vec<&str> = s_e.get(1).unwrap().split(':').collect();

                let (s_h, s_m) = (s_h_m.get(0).unwrap(), s_h_m.get(1).unwrap());
                let (e_h, e_m) = (e_h_m.get(0).unwrap(), e_h_m.get(1).unwrap());

                let event = CalEvent::new(
                    CalEventTime::new_md(self.chosen_date, (
                            s_h.parse::<u32>().unwrap(),
                            s_m.parse::<u32>().unwrap()
                        ), (
                            e_h.parse::<u32>().unwrap(),
                            e_m.parse::<u32>().unwrap()
                        )
                    ).unwrap(),
                    self.input_description.clone()
                );
                self
                .files
                .add_event(event)
                .unwrap();

                self.input_time = String::new();
                self.input_description = String::new();
            }
            1 => self.files.add_todo("todo", "TODO").unwrap(),
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
                match app.input_mode {
                    InputMode::Normal => {
                        match key.code {
                            KeyCode::Char(c) if key.modifiers == KeyModifiers::CONTROL => {
                                app.on_ctrl_key(c)
                            }
                            KeyCode::Char(c) => app.on_key(c),
                            KeyCode::Left => app.on_left(),
                            KeyCode::Up => app.on_up(),
                            KeyCode::Right => app.on_right(),
                            KeyCode::Down => app.on_down(),
                            KeyCode::Enter => app.input_mode = InputMode::Selecting,
                            _ => {}
                        }
                    },
                    InputMode::AddingTime => match key.code {
                        KeyCode::Enter => {
                            //app.messages.push(app.input.drain(..).collect());
                            //app.on_add_item();
                            app.input_mode = InputMode::AddingDescription;

                        }
                        KeyCode::Char(c) => {
                            app.input_time.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input_time.pop();
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
                            app.input_description.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input_description.pop();
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::AddingTime;
                        }
                        _ => {}
                    },
                    InputMode::Selecting => match key.code {
                        KeyCode::Char(c) => app.event_on_key(c),
                        KeyCode::Esc => app.input_mode = InputMode::Normal,
                        _ => {}
                    }
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
