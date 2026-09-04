use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use crate::event::{AppEvent, HeatmapYearHabitsEvent};
use crate::model::habit::HabitDb;

pub struct HeatmapYearHabitsAction {
    sender: mpsc::Sender<AppEvent>,
    db_path: PathBuf,
}

impl HeatmapYearHabitsAction {
    pub fn new(sender: mpsc::Sender<AppEvent>, db_path: PathBuf) -> Self {
        Self { sender, db_path }
    }

    pub fn fetch(&self) {
        let sender = self.sender.clone();
        let path = self.db_path.clone();
        let _ = sender.send(AppEvent::HeatmapYearHabits(
            HeatmapYearHabitsEvent::Fetching,
        ));
        thread::spawn(move || {
            let habit_db = match HabitDb::new(&path) {
                Ok(h) => h,
                Err(e) => {
                    let _ = sender.send(AppEvent::HeatmapYearHabits(
                        HeatmapYearHabitsEvent::Failed(e.to_string()),
                    ));
                    return;
                }
            };
            match habit_db.get_heatmap_year_habits() {
                Ok(data) => {
                    let _ = sender.send(AppEvent::HeatmapYearHabits(
                        HeatmapYearHabitsEvent::Fetched(data),
                    ));
                }
                Err(e) => {
                    let _ = sender.send(AppEvent::HeatmapYearHabits(
                        HeatmapYearHabitsEvent::Failed(e.to_string()),
                    ));
                }
            }
        });
    }
}
