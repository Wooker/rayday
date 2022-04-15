use config::Config;
use anyhow::{anyhow, Error, Result};
use std::{
    fs,
    collections::HashMap,
    path::{Path, PathBuf},
};

const REFORM_YEAR: u32 = 1099;
const MONTHS: usize = 12;
const WEEKDAYS: u32 = 7;

const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "rayday";
const CONFIG_NAME: &str = "config.yml";

struct Calendar {
    config: Config,
}

impl Calendar {
    fn new(path: &str) -> Result<Calendar> {
        match dirs::home_dir() {
            Some(home) => {
                let path = Path::new(&home);
                let home_config_dir = path.join(CONFIG_DIR);
                let app_config_dir = home_config_dir.join(APP_CONFIG_DIR);

                if !home_config_dir.exists() {
                    fs::create_dir(&home_config_dir)?;
                }

                Ok(Calendar {
                    config: Config::builder()
                        .add_source(config::File::with_name(CONFIG_NAME))
                        .build()
                        .unwrap()
                })
            }

            None => Err(anyhow!("No $HOME directory found for config")),
        }
    }

    fn show(self) {
        println!("{:?}", self.config.try_deserialize::<HashMap<String, String>>().unwrap());
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
    fn config() {
        use crate::calendar::Calendar;
        use std::collections::HashMap;

        let cal = Calendar::new(CONFIG_NAME).unwrap();
        let mut map = HashMap::new();

        map.insert(String::from("a"), String::from("b"));

        assert_eq!(map, cal.config
            .try_deserialize::<HashMap<String, String>>()
            .unwrap());
    }
}
