use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use crate::event::{AppEvent, WeeklyAverageEvent};
use crate::model::habit::HabitDb;

pub struct WeeklyAverageAction {
    sender: mpsc::Sender<AppEvent>,
    db_path: PathBuf,
}

impl WeeklyAverageAction {
    pub fn new(sender: mpsc::Sender<AppEvent>, db_path: PathBuf) -> Self {
        Self { sender, db_path }
    }

    pub fn fetch(&self) {
        let sender = self.sender.clone();
        let path = self.db_path.clone();
        let _ = sender.send(AppEvent::WeeklyAverage(WeeklyAverageEvent::Fetching));
        thread::spawn(move || {
            let habit_db = match HabitDb::new(&path) {
                Ok(h) => h,
                Err(e) => {
                    let _ = sender.send(AppEvent::WeeklyAverage(WeeklyAverageEvent::Failed(
                        e.to_string(),
                    )));
                    return;
                }
            };
            match habit_db.get_weekly_completion_by_day() {
                Ok(days) => {
                    let _ = sender.send(AppEvent::WeeklyAverage(WeeklyAverageEvent::Fetched(days)));
                }
                Err(e) => {
                    let _ = sender.send(AppEvent::WeeklyAverage(WeeklyAverageEvent::Failed(
                        e.to_string(),
                    )));
                }
            }
        });
    }
}
