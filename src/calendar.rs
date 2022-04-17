use config::Config;
use anyhow::{anyhow, Error, Result};
use std::{
    fs,
    collections::HashMap,
    path::{Path, PathBuf},
};

use pickledb::{
    PickleDb,
    PickleDbDumpPolicy,
    SerializationMethod,
    PickleDbIterator
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
    config: Config,
    events: PickleDb,
    todos: PickleDb,
}

impl Calendar {
    pub fn new() -> Result<Calendar> {
        match dirs::home_dir() {
            Some(home) => {
                let path = Path::new(&home);
                let home_config_dir = path.join(CONFIG_DIR);
                let app_config_dir = home_config_dir.join(APP_CONFIG_DIR);

                if !home_config_dir.exists() {
                    fs::create_dir(&home_config_dir)?;
                }

                if !app_config_dir.exists() {
                    fs::create_dir(&app_config_dir)?;
                }

                let config_file_path = &app_config_dir.join(CONFIG_NAME);
                let events_file_path = &app_config_dir.join(EVENTS_NAME);
                let todos_file_path = &app_config_dir.join(TODOS_NAME);

                if !config_file_path.exists() {
                    fs::File::create(config_file_path);
                }

                let events_db: PickleDb;
                if !events_file_path.exists() {
                    events_db = PickleDb::new(events_file_path, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json)
                } else {
                    events_db = PickleDb::load(events_file_path, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json).unwrap()
                }

                let todos_db: PickleDb;
                if !todos_file_path.exists() {
                    todos_db = PickleDb::new(todos_file_path, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json)
                } else {
                    todos_db = PickleDb::load(todos_file_path, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json).unwrap()
                }

                Ok(Calendar {
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

    pub fn add_event(&mut self, key: &str, value: &str) -> Result<()> {
        self.events.set(key, &value)?;
        Ok(())
    }

    pub fn add_todo(&mut self, key: &str, value: &str) -> Result<()> {
        self.events.set(key, &value)?;
        Ok(())
    }

    pub fn todos_iter(&self) -> PickleDbIterator {
        self.todos.iter()
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
    use crate::calendar::{is_leap_year, days_by_year, CONFIG_NAME};

    #[test]
    fn leap_year() {
        assert_eq!(is_leap_year(2022), false);
        assert_eq!(is_leap_year(2024), true);
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
        use crate::calendar::Calendar;

        let mut cal = Calendar::new().unwrap();
        //cal.events.set("key", &100).unwrap();

        assert_eq!(100, cal.events.get::<i32>("key").unwrap());
    }
}
