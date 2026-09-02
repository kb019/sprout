use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use crate::event::{AppEvent, BestStreaksEvent};
use crate::model::habit::HabitDb;

pub struct BestStreaksAction {
    sender: mpsc::Sender<AppEvent>,
    db_path: PathBuf,
}

impl BestStreaksAction {
    pub fn new(sender: mpsc::Sender<AppEvent>, db_path: PathBuf) -> Self {
        Self { sender, db_path }
    }

    pub fn fetch(&self) {
        let sender = self.sender.clone();
        let path = self.db_path.clone();
        let _ = sender.send(AppEvent::BestStreaks(BestStreaksEvent::Fetching));
        thread::spawn(move || {
            let habit_db = match HabitDb::new(&path) {
                Ok(h) => h,
                Err(e) => {
                    let _ = sender.send(AppEvent::BestStreaks(BestStreaksEvent::Failed(
                        e.to_string(),
                    )));
                    return;
                }
            };
            match habit_db.get_best_streaks() {
                Ok(best) => {
                    let _ = sender.send(AppEvent::BestStreaks(BestStreaksEvent::Fetched(best)));
                }
                Err(e) => {
                    let _ = sender.send(AppEvent::BestStreaks(BestStreaksEvent::Failed(
                        e.to_string(),
                    )));
                }
            }
        });
    }
}
