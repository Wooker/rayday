use config::{Config, Map, Source, Value, ConfigError};
use anyhow::{anyhow, Error, Result};
use std::{
    fs,
    collections::HashMap,
    path::{Path, PathBuf},
};


use chrono::prelude::*;

use crate::event::{Event, EventTime, EventTimeError};

use pickledb::{
    PickleDb,
    PickleDbDumpPolicy,
    SerializationMethod,
    PickleDbIterator,
    error::Error as PickleError,
};

const REFORM_YEAR: u32 = 1099;
const MONTHS: usize = 12;
const WEEKDAYS: u32 = 7;

const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "rayday";
const CONFIG_NAME: &str = "config.yml";
const EVENTS_NAME: &str = "events.db";
const TODOS_NAME: &str = "todos.db";

pub struct Calendar {
    config_dir: PathBuf,
    config: Config,
    events: PickleDb,
    todos: PickleDb,
}

impl Calendar {
    pub fn new() -> Result<Calendar, Error> {
        match dirs::home_dir() {
            Some(home) => {
                let path = Path::new(&home);
                let home_config_dir = path.join(CONFIG_DIR);
                let app_config_dir = home_config_dir.join(APP_CONFIG_DIR);

                if !home_config_dir.is_dir() {
                    fs::create_dir(&home_config_dir)?;
                }

                if !app_config_dir.is_dir() {
                    fs::create_dir(&app_config_dir)?;
                }

                let config_file_path = &app_config_dir.join(CONFIG_NAME);
                let events_file_path = &app_config_dir.join(EVENTS_NAME);
                let todos_file_path = &app_config_dir.join(TODOS_NAME);

                if !config_file_path.is_file() {
                    fs::File::create(config_file_path);
                }

                if !events_file_path.is_file() {
                    fs::File::create(events_file_path);
                }

                if !todos_file_path.is_file() {
                    fs::File::create(todos_file_path);
                }

                let mut events_db: PickleDb = PickleDb::new(events_file_path, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json);
                /*
                if !events_file_path.is_file() {
                    events_db = PickleDb::new(events_file_path, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json);
                } else {
                    events_db = PickleDb::load(events_file_path, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json).unwrap();
                }
                */
                //let events_db = PickleDb::load(events_file_path, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json)?;

                let mut todos_db: PickleDb = PickleDb::new(todos_file_path, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json);
                /*
                if !todos_file_path.is_file() {
                    todos_db = PickleDb::new(todos_file_path, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json);
                } else {
                    todos_db = PickleDb::load(todos_file_path, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json).unwrap();
                }
                */
                //let todos_db = PickleDb::load(todos_file_path, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json)?;

                Ok(Calendar {
                    config_dir: app_config_dir,
                    config: Config::builder()
                        .add_source(config::File::from(config_file_path.to_path_buf()))
                        .build()
                        .unwrap(),
                    events: events_db,
                    todos: todos_db,
                })
            }

            None => Err(anyhow!("No $HOME directory found for config")),
        }
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), PickleError> {
        self.events.set(format!("{}|{}", &event.time().start_datetime().to_string(), &event.time().end_datetime().to_string()).as_str(), &event.desc())
    }

    pub fn add_todo(&mut self, key: &str, value: &str) -> Result<()> {
        self.events.set(key, &value)?;
        Ok(())
    }

    pub fn get_events_on_date(&mut self, date: Date<Local>) -> Vec<Event> {
        // Get EventTime as keys from db
        self.events
            .iter()
            .map(|e| {
                Event::new(EventTime::from(e.get_key()), e.get_value::<String>().unwrap())
            })
            .filter(|e| {
                e.time().start_date() == date
            }).collect()
    }

    pub fn get_todo(&mut self, key: &str, value: &str) -> Result<()> {
        self.events.set(key, &value)?;
        Ok(())
    }

    pub fn todos_iter(&self) -> PickleDbIterator {
        self.todos.iter()
    }

    pub fn get_events(&self) -> PickleDbIterator {
        self.events.iter()
    }
}

fn is_leap_year(year: u32) -> bool {
    if year <= REFORM_YEAR {
        return year % 4 == 0;
    }
    (year % 4 == 0) ^ (year % 100 == 0) ^ (year % 400 == 0)
}

fn days_by_year(mut year: u32) -> u32 {
    let mut count: u32 = 0;

    while year > 1 {
        year -= 1;
        if is_leap_year(year) {
            count += 366
        } else {
            count += 365
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use chrono::Duration;
    use config::Source;

    use super::*;

    #[test]
    fn config() {
        let mut cal = Calendar::new().unwrap();
        let mut map = HashMap::new();

        assert_eq!(map, cal.config
            .try_deserialize::<HashMap<String, String>>()
            .unwrap());
    }

    #[test]
    fn add_event() {
        let mut cal = Calendar::new().unwrap();

        let today = Local::today();
        let start = today.and_hms(12, 0, 0);

        let time = EventTime::new(start, start.checked_add_signed(Duration::minutes(40)).unwrap()).unwrap();
        cal.add_event(Event::new(time, "Event today!".to_string()));

        let time = EventTime::new(start.checked_add_signed(Duration::days(5)).unwrap(), start.checked_add_signed(Duration::days(40)).unwrap()).unwrap();
        cal.add_event(Event::new(time, "Another event!".to_string()));

        let events = cal.get_events_on_date(today);
        dbg!(&events);
        assert_eq!(events.is_empty(), false);
        assert_eq!(events.iter().nth(0).unwrap().desc(), "Event today!");
    }
}
