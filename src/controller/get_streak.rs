use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use crate::event::{AppEvent, GetStreakEvent};
use crate::model::habit::HabitDb;

pub struct GetStreakAction {
    sender: mpsc::Sender<AppEvent>,
    db_path: PathBuf,
}

impl GetStreakAction {
    pub fn new(sender: mpsc::Sender<AppEvent>, db_path: PathBuf) -> Self {
        Self { sender, db_path }
    }

    pub fn get_streak(&self, habit_id: i32) {
        let sender = self.sender.clone();
        let path = self.db_path.clone();
        let _ = sender.send(AppEvent::GetStreak(GetStreakEvent::Fetching(habit_id)));
        thread::spawn(move || {
            let habit_db = match HabitDb::new(&path) {
                Ok(h) => h,
                Err(e) => {
                    let _ = sender.send(AppEvent::GetStreak(GetStreakEvent::Failed(
                        habit_id,
                        e.to_string(),
                    )));
                    return;
                }
            };
            match habit_db.get_streak(habit_id) {
                Ok(streak) => {
                    let _ = sender.send(AppEvent::GetStreak(GetStreakEvent::Fetched(
                        habit_id, streak,
                    )));
                }
                Err(e) => {
                    let _ = sender.send(AppEvent::GetStreak(GetStreakEvent::Failed(
                        habit_id,
                        e.to_string(),
                    )));
                }
            }
        });
    }
}
