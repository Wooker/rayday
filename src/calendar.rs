use config::{Config, Map, Source, Value, ConfigError};
use anyhow::{anyhow, Error, Result};
use std::{
    fs,
    collections::HashMap,
    path::{Path, PathBuf},
};


use chrono::prelude::*;

use crate::event::{Event, EventError};

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
        self.events.set(&event.datetime().to_string(), &event.desc())
    }

    pub fn add_todo(&mut self, key: &str, value: &str) -> Result<()> {
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
    use config::Source;

    use super::*;

    #[test]
    fn leap_year() {
        assert_eq!(is_leap_year(2022), false);
        assert_eq!(is_leap_year(2024), true);
    }

    #[test]
    fn files_exist() {
        match dirs::home_dir() {
            Some(home) => {
                let path = Path::new(&home);
                let home_config_dir = path.join(CONFIG_DIR);
                let app_config_dir = home_config_dir.join(APP_CONFIG_DIR);

                assert_eq!(home_config_dir.is_dir(), true);
                assert_eq!(app_config_dir.is_dir(), true);

                let config_file_path = &app_config_dir.join(CONFIG_NAME);
                let events_file_path = &app_config_dir.join(EVENTS_NAME);
                let todos_file_path = &app_config_dir.join(TODOS_NAME);

                assert_eq!(config_file_path.is_file(), true);
                assert_eq!(events_file_path.is_file(), true);
                assert_eq!(todos_file_path.is_file(), true);
            }
            None => panic!("Home dir does not exist"),
        }
    }

    #[test]
    fn config_create_and_read() {
        use crate::calendar::Calendar;
        use std::collections::HashMap;

        let cal = Calendar::new().unwrap();
        let mut map = HashMap::new();

        //map.insert(String::from("a"), String::from("b"));

        assert_eq!(map, cal.config
            .try_deserialize::<HashMap<String, String>>()
            .unwrap());
    }

    #[test]
    fn eventsdb_set() {
        let cal = Calendar::new().unwrap();
        let events_file_path = cal.config_dir.join(EVENTS_NAME);
        println!("{:?}", events_file_path);
        let mut events_db = PickleDb::new(events_file_path, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json);
        events_db.set::<i32>("my_key", &10).unwrap();
        println!("{:?}", events_db.get_all());

        assert_eq!(10, events_db.get::<i32>("my_key").unwrap());
    }

    #[test]
    fn pickle_test() {
        let mut db = PickleDb::new("example.db", PickleDbDumpPolicy::AutoDump, SerializationMethod::Json);

        // set the value 100 to the key 'key1'
        db.set("key1", &100).unwrap();

        // print the value of key1
        println!("The value of key1 is: {}", db.get::<i32>("key1").unwrap());

        // load the DB from the same file
        let db2 = PickleDb::load("example.db", PickleDbDumpPolicy::DumpUponRequest, SerializationMethod::Json).unwrap();

        // print the value of key1
        let val2 = db2.get::<i32>("key1").unwrap();
        println!("The value of key1 as loaded from file is: {}", val2);
        assert_eq!(val2, 100);
    }

    #[test]
    fn fs_read() {
        let cal = Calendar::new().unwrap();
        let events_file_path = cal.config_dir.join(EVENTS_NAME);
        let content = fs::read(events_file_path).unwrap();
        println!("{:?}", content);
        assert_eq!(String::from("[{},{}]"), String::from_utf8(content).unwrap());

    }

    fn get_events() {
        todo!();
        let mut cal = Calendar::new().unwrap();
        let events = cal.get_events(Utc::now().date());
        assert_eq!(events.is_empty(), true);
    }

    fn add_event() {
        let mut cal = Calendar::new().unwrap();
        cal.events.set("asd", &"qwe").unwrap();
        println!("{:?}", cal.events.get_all());

        let a = cal.add_event(Event::new(Utc.ymd(2022, 4, 27).and_hms(14, 30, 0), Utc.ymd(2022, 4, 27).and_hms(14, 45, 0), "Test".to_string()).unwrap());
        println!("{:?}", a);
        assert_eq!(a.is_ok(), true);
    }
}
