use anyhow::{anyhow, Error as AnyhowError, Result};
//use config::{Config, ConfigError, Map, Source, Value};
use confy::{load_path, store_path};
use log2::{debug, info};
use rusqlite::{params, Connection, Params};
use std::{
    collections::HashMap,
    default::Default,
    fs,
    io::Write,
    ops::Add,
    os::fd::AsFd,
    path::{Path, PathBuf},
};

use chrono::{prelude::*, Duration};
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
    db: Connection,
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

                let db_file = PathBuf::from(app_config_dir.join("events.db"));

                let db = Connection::open(db_file.clone()).expect("Could not connect to db");
                db.execute(
                    r"create table if not exists events(
                        id integer primary key,
                        description text not null,
                        start datetime not null,
                        end datetime not null
                    )",
                    params![],
                )?;
                Ok(Files {
                    config_dir: app_config_dir,
                    config,
                    db, // rocksdb::DB::open_default(events_file_path).unwrap(),
                })
            }
            None => Err(anyhow!("No $HOME directory found for config")),
        }
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), Error> {
        self.db.execute(
            "insert into events (description, start, end) values (?1, ?2, ?3)",
            params![event.desc(), event.start(), event.end(),],
        );

        info!("Added event {}", event.to_string());
        Ok(())
    }

    pub fn remove_event(&mut self, id: usize) -> Result<(), Error> {
        self.db
            .execute("delete from events where id=?1", params![id])
            .expect("Could not remove event from db");

        Ok(())
    }

    pub fn get_event(&self, id: usize) -> Option<Event> {
        let mut stmt = self
            .db
            .prepare("select * from events where id = ?1")
            .expect("Could not prepare statement");
        let event = stmt.query([id]).expect("Could not query statement");
        info!("Event with id: {}", id);

        None
    }

    pub fn get_events_on_date(&self, date: NaiveDate) -> Vec<Event> {
        // Get EventTime as keys from db
        let mut stmt = self
            .db
            .prepare("select * from events where start > ?1 and end < ?2")
            .expect("Could not prepare statement");

        // Query rows and parse Events
        let event_iter = stmt
            .query_map([date, date.add(Duration::days(1))], |row| {
                let id = Some(row.get::<usize, usize>(0).unwrap());
                Ok(Event::new(
                    id,
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                ))
            })
            .expect("Could not query rows");

        // Collect events
        let events = event_iter.map(|e| e.unwrap()).collect();

        debug!("Events on date {}: {:?}", date, events);
        events
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {}
