use anyhow::{anyhow, Error, Result};
//use config::{Config, ConfigError, Map, Source, Value};
use confy::{load_path, store_path};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    default::Default,
};

use chrono::prelude::*;
use serde_derive::{Serialize, Deserialize};

use crate::{
    event::{Event, EventTime, EventTimeError, Today},
    app::StatefulList,
};

use pickledb::{
    error::{Error as PickleError, Result as PickleResult},
    PickleDb, PickleDbDumpPolicy, PickleDbIterator,
    SerializationMethod,
};

const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "rayday";
const CONFIG_NAME: &str = "config.yml";
const EVENTS_NAME: &str = "events.db";
const TODOS_NAME: &str = "todos.db";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub color: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            color: String::from("blue")
        }
    }
}

pub struct Files {
    config_dir: PathBuf,
    pub config: Config,
    events: PickleDb,
    todos: PickleDb,
}


impl Files {
    pub fn new() -> Result<Files, Error> {
        match dirs::home_dir() {
            Some(home) => {
                let path = Path::new(&home);
                let home_config_dir = path.join(CONFIG_DIR);
                let app_config_dir = home_config_dir.join(APP_CONFIG_DIR);

                let config_file_path = &app_config_dir.join(CONFIG_NAME);
                let events_file_path = &app_config_dir.join(EVENTS_NAME);
                let todos_file_path = &app_config_dir.join(TODOS_NAME);

                if !home_config_dir.is_dir() {
                    fs::create_dir(&home_config_dir)?;
                }

                let mut config: Config;
                let mut events_db: PickleDb;
                let mut todos_db: PickleDb;
                if !app_config_dir.is_dir() {
                    fs::create_dir(&app_config_dir)?;
                    //fs::File::create(config_file_path);
                    config = Config::default();
                    events_db = PickleDb::new(
                        events_file_path,
                        PickleDbDumpPolicy::AutoDump,
                        SerializationMethod::Json,
                    );
                    todos_db = PickleDb::new(
                        todos_file_path,
                        PickleDbDumpPolicy::AutoDump,
                        SerializationMethod::Json,
                    );
                } else {
                    config = load_path(config_file_path).unwrap();
                    events_db = PickleDb::load(
                        events_file_path,
                        PickleDbDumpPolicy::AutoDump,
                        SerializationMethod::Json,
                    )
                    .unwrap();
                    todos_db = PickleDb::load(
                        todos_file_path,
                        PickleDbDumpPolicy::AutoDump,
                        SerializationMethod::Json,
                    )
                    .unwrap();
                }

                Ok(Files {
                    config_dir: app_config_dir,
                    config,
                    events: events_db,
                    todos: todos_db,
                })
            }

            None => Err(anyhow!("No $HOME directory found for config")),
        }
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), PickleError> {
        self.events.set(
            format!(
                "{}|{}",
                &event.time().start_datetime().to_string(),
                &event.time().end_datetime().to_string()
            )
            .as_str(),
            &event.desc(),
        )
    }

    pub fn remove_event(&mut self, time: EventTime) -> PickleResult<bool> {
        self.events.rem(
            format!(
                "{}|{}",
                time.start_datetime().to_string(),
                time.end_datetime().to_string()
            )
            .as_str()
        )
    }

    pub fn add_todo(&mut self, key: &str, value: &str) -> Result<()> {
        self.events.set(key, &value)?;
        Ok(())
    }

    pub fn get_event(&self, time: EventTime) -> Option<Event> {
        self.events.get(format!("{}|{}", time.start_datetime().to_string(), time.end_datetime().to_string()).as_str())
    }

    pub fn get_events_on_date(&self, date: Date<Local>) -> Vec<Event> {
        // Get EventTime as keys from db
        self.events
            .iter()
            .map(|e| {
                Event::new(
                    EventTime::from(e.get_key()),
                    e.get_value::<String>().unwrap(),
                )
            })
            .filter(|e| e.time().start_date() == date)
            .collect()
    }

    pub fn events_stateful_list(&self, date: Date<Local>) -> StatefulList<Event> {
        // Get EventTime as keys from db
        StatefulList::with_items(
            self.events
                .iter()
                .map(|e| {
                    Event::new(
                        EventTime::from(e.get_key()),
                        e.get_value::<String>().unwrap(),
                    )
                })
                .filter(|e| e.time().start_date() == date)
                .collect()
        )
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

#[cfg(test)]
mod tests {
    use chrono::Duration;
    use config::Source;

    use super::*;

    #[test]
    fn config() {
        let mut cal = Files::new().unwrap();
        let mut map = HashMap::new();

        assert_eq!(
            map,
            cal.config
                .try_deserialize::<HashMap<String, String>>()
                .unwrap()
        );
    }

    #[test]
    fn add_event() {
        let mut cal = Files::new().unwrap();


        cal.add_event(Event::new(EventTime::today(12, 0, Duration::minutes(30)), "Event today!".to_string()));
        cal.add_event(Event::new(EventTime::today(12, 0, Duration::minutes(35)), "Another event!".to_string()));

        let events = cal.get_events_on_date(Local::today());
        dbg!(&events);
        assert_eq!(events.is_empty(), false);
        assert_eq!(events.iter().nth(0).unwrap().desc(), "Event today!");

        cal.add_event(Event::new(EventTime::today(12, 0, Duration::minutes(40)), "Yet another event!".to_string()));
    }
}
