use anyhow::{anyhow, Error as AnyhowError, Result};
//use config::{Config, ConfigError, Map, Source, Value};
use confy::{load_path, store_path};
use rocksdb::{Error as RocksError, DB};
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
    events: DB,
}

impl Files {
    pub fn new() -> Result<Files> {
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

                Ok(Files {
                    config_dir: app_config_dir,
                    config,
                    events: rocksdb::DB::open_default(events_file_path).unwrap(),
                })
            }

            None => Err(anyhow!("No $HOME directory found for config")),
        }
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), RocksError> {
        self.events.put(
            format!(
                "{}|{}-{}",
                &event.date(),
                &event.time().start_datetime().to_string(),
                &event.time().end_datetime().to_string()
            )
            .as_bytes(),
            &event.desc().as_bytes(),
        )
    }

    pub fn remove_event(&mut self, date: NaiveDate, time: EventTime) -> Result<(), RocksError> {
        self.events.delete(
            format!(
                "{}|{}-{}",
                date,
                time.start_datetime().to_string(),
                time.end_datetime().to_string()
            )
            .as_str(),
        )
    }

    pub fn get_event(&self, date: NaiveDate, time: EventTime) -> Option<Event> {
        match self
            .events
            .get(format!("{}|{}", &date, &time.to_string()).as_bytes())
        {
            Ok(Some(ev)) => {
                let s = String::from_utf8(ev.to_vec()).unwrap();
                let event = s.parse::<Event>().unwrap();
                Some(event)
            }
            Ok(None) => None,
            Err(_) => None,
        }
    }

    pub fn get_events_on_date(&self, date: NaiveDate) -> Vec<Event> {
        // Get EventTime as keys from db
        self.events
            .full_iterator(rocksdb::IteratorMode::Start)
            .map(|e| {
                let result = e.unwrap();
                let datetime = String::from_utf8(result.0.to_vec()).unwrap();
                let description = String::from_utf8(result.1.to_vec()).unwrap();

                format!("{}|{}", datetime, description)
                    .parse::<Event>()
                    .unwrap()
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
