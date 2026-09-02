use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use crate::event::{ActiveDaysEvent, AppEvent};
use crate::model::habit::HabitDb;

pub struct ActiveDaysAction {
    sender: mpsc::Sender<AppEvent>,
    db_path: PathBuf,
}

impl ActiveDaysAction {
    pub fn new(sender: mpsc::Sender<AppEvent>, db_path: PathBuf) -> Self {
        Self { sender, db_path }
    }

    pub fn fetch(&self) {
        let sender = self.sender.clone();
        let path = self.db_path.clone();
        let _ = sender.send(AppEvent::ActiveDays(ActiveDaysEvent::Fetching));
        thread::spawn(move || {
            let habit_db = match HabitDb::new(&path) {
                Ok(h) => h,
                Err(e) => {
                    let _ =
                        sender.send(AppEvent::ActiveDays(ActiveDaysEvent::Failed(e.to_string())));
                    return;
                }
            };
            match habit_db.get_active_days_count() {
                Ok(count) => {
                    let _ = sender.send(AppEvent::ActiveDays(ActiveDaysEvent::Fetched(count)));
                }
                Err(e) => {
                    let _ =
                        sender.send(AppEvent::ActiveDays(ActiveDaysEvent::Failed(e.to_string())));
                }
            }
        });
    }
}
