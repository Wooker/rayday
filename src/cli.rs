use centered_interval_tree::CenteredIntervalTree;
use chrono::{Local, NaiveTime};
use clap::{Arg, Parser, Subcommand};

use crate::{app::run, files::Files};

#[derive(Parser)]
#[command(name = "RayDay")]
#[command(author = "Zakhar Semenov <zakhar.semyonov@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Task tracker and calendar combined", long_about = None)]
pub(crate) struct RaydayCli {
    #[command(subcommand)]
    pub command: Option<RaydayCommand>,
}

#[derive(Subcommand)]
pub(crate) enum RaydayCommand {
    Now,
    // Add,
    // Remove,
}

impl RaydayCli {
    pub(crate) fn handle_command(&self) {
        match self.command {
            Some(RaydayCommand::Now) => {
                let files = Files::new().unwrap();
                let now = Local::now().naive_local();
                let events = files.cache_events();

                let mut tree = CenteredIntervalTree::<NaiveTime, String>::new();

                if let Some(events_today) = events.get(&now.date()) {
                    for event in events_today.iter() {
                        let time = event.time();
                        let interval = (time.start_datetime(), time.end_datetime());
                        tree.add(interval, event.desc());
                    }

                    let todo_now = tree.search(now.time());

                    for todo in todo_now {
                        println!("{}", &todo);
                    }
                }
            }
            None | Some(_) => {}
        }
    }
}
