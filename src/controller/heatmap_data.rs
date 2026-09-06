use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use crate::event::{AppEvent, HeatmapDataEvent};
use crate::model::habit::HabitDb;

pub struct HeatmapDataAction {
    sender: mpsc::Sender<AppEvent>,
    db_path: PathBuf,
}

impl HeatmapDataAction {
    pub fn new(sender: mpsc::Sender<AppEvent>, db_path: PathBuf) -> Self {
        Self { sender, db_path }
    }

    pub fn fetch(&self, habit_id: i32, year: i32) {
        let sender = self.sender.clone();
        let path = self.db_path.clone();
        let _ = sender.send(AppEvent::HeatmapData(HeatmapDataEvent::Fetching(
            habit_id, year,
        )));
        thread::spawn(move || {
            let habit_db = match HabitDb::new(&path) {
                Ok(h) => h,
                Err(e) => {
                    let _ = sender.send(AppEvent::HeatmapData(HeatmapDataEvent::Failed(
                        habit_id,
                        year,
                        e.to_string(),
                    )));
                    return;
                }
            };
            match habit_db.get_heatmap_data(habit_id, year) {
                Ok(data) => {
                    let _ = sender.send(AppEvent::HeatmapData(HeatmapDataEvent::Fetched(
                        habit_id, year, data,
                    )));
                }
                Err(e) => {
                    let _ = sender.send(AppEvent::HeatmapData(HeatmapDataEvent::Failed(
                        habit_id,
                        year,
                        e.to_string(),
                    )));
                }
            }
        });
    }
}
