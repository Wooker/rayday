use anyhow::{anyhow, Error, Result};
//use config::{Config, ConfigError, Map, Source, Value};
use confy::{load_path, store_path};
use std::{
    collections::HashMap,
    default::Default,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};
use serde_yaml::*;
use tui::style::Color;

use crate::event::{Event, EventTime, EventTimeError, Today};

use pickledb::{
    error::{Error as PickleError, Result as PickleResult},
    PickleDb, PickleDbDumpPolicy, PickleDbIterator, SerializationMethod,
};

const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "rayday";
const CONFIG_NAME: &str = "config.yml";
const EVENTS_NAME: &str = "events.db";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub highlight_color: Color, //TODO tui feature "serde"
    pub event_color: Color,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            highlight_color: Color::LightBlue,
            event_color: Color::LightBlue,
        }
    }
}

pub struct Files {
    config_dir: PathBuf,
    config: Config,
    events: PickleDb,
}

impl Files {
    pub fn new() -> Result<Files, Error> {
        match dirs::home_dir() {
            Some(home) => {
                let path = Path::new(&home);
                let home_config_dir = path.join(CONFIG_DIR);
                let app_config_dir = home_config_dir.join(APP_CONFIG_DIR);

                let config_file_path = app_config_dir.join(CONFIG_NAME);
                let events_file_path = app_config_dir.join(EVENTS_NAME);

                if !home_config_dir.is_dir() {
                    fs::create_dir(home_config_dir)?;
                }

                let config: Config;
                let events_db: PickleDb;
                let todos_db: PickleDb;
                if !app_config_dir.is_dir() {
                    fs::create_dir(&app_config_dir)?;
                }
                if !config_file_path.exists() {
                    config = Config::default();
                    fs::File::create(config_file_path)?
                        .write(serde_yaml::to_string(&config)?.as_bytes());
                } else {
                    config = load_path(config_file_path).unwrap();
                }

                if !events_file_path.exists() {
                    events_db = PickleDb::new(
                        events_file_path,
                        PickleDbDumpPolicy::AutoDump,
                        SerializationMethod::Yaml,
                    );
                } else {
                    events_db = PickleDb::load(
                        events_file_path,
                        PickleDbDumpPolicy::AutoDump,
                        SerializationMethod::Yaml,
                    )
                    .unwrap();
                }

                Ok(Files {
                    config_dir: app_config_dir,
                    config,
                    events: events_db,
                })
            }

            None => Err(anyhow!("No $HOME directory found for config")),
        }
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), PickleError> {
        self.events.set(
            format!(
                "{}|{}|{}",
                &event.date(),
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
            .as_str(),
        )
    }

    pub fn get_event(&self, time: EventTime) -> Option<Event> {
        self.events.get(
            format!(
                "{}|{}",
                time.start_datetime().to_string(),
                time.end_datetime().to_string()
            )
            .as_str(),
        )
    }

    pub fn get_events_on_date(&self, date: NaiveDate) -> Vec<Event> {
        // Get EventTime as keys from db
        self.events
            .iter()
            .map(|e| {
                let mut key = e.get_key().split('|').collect::<Vec<&str>>();

                let end_str = key.pop().unwrap();
                let start_str = key.pop().unwrap();
                let date_str = key.pop().unwrap();

                let time_format = "%H:%M:%S";

                Event::new(
                    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap(),
                    EventTime::new(
                        NaiveTime::parse_from_str(start_str, time_format).unwrap(),
                        NaiveTime::parse_from_str(end_str, time_format).unwrap(),
                    )
                    .unwrap(),
                    e.get_value::<String>().unwrap(),
                )
            })
            .filter(|e| e.date() == date)
            .collect()
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {}
