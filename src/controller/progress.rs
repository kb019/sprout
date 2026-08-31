use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use crate::event::{
    AppEvent, DailyProgressEvent, MonthlyProgressEvent, WeeklyProgressEvent, YearlyProgressEvent,
};
use crate::model::habit::HabitDb;

pub struct DailyProgressAction {
    sender: mpsc::Sender<AppEvent>,
    db_path: PathBuf,
}

impl DailyProgressAction {
    pub fn new(sender: mpsc::Sender<AppEvent>, db_path: PathBuf) -> Self {
        Self { sender, db_path }
    }

    pub fn fetch(&self, habit_id: i32) {
        let sender = self.sender.clone();
        let path = self.db_path.clone();
        let _ = sender.send(AppEvent::DailyProgress(DailyProgressEvent::Fetching(
            habit_id,
        )));
        thread::spawn(move || {
            let habit_db = match HabitDb::new(&path) {
                Ok(h) => h,
                Err(e) => {
                    let _ = sender.send(AppEvent::DailyProgress(DailyProgressEvent::Failed(
                        habit_id,
                        e.to_string(),
                    )));
                    return;
                }
            };
            match habit_db.get_daily_progress() {
                Ok(map) => {
                    let progress = map.get(&habit_id).copied().unwrap_or(0);
                    let _ = sender.send(AppEvent::DailyProgress(DailyProgressEvent::Fetched(
                        habit_id, progress,
                    )));
                }
                Err(e) => {
                    let _ = sender.send(AppEvent::DailyProgress(DailyProgressEvent::Failed(
                        habit_id,
                        e.to_string(),
                    )));
                }
            }
        });
    }
}

pub struct WeeklyProgressAction {
    sender: mpsc::Sender<AppEvent>,
    db_path: PathBuf,
}

impl WeeklyProgressAction {
    pub fn new(sender: mpsc::Sender<AppEvent>, db_path: PathBuf) -> Self {
        Self { sender, db_path }
    }

    pub fn fetch(&self, habit_id: i32) {
        let sender = self.sender.clone();
        let path = self.db_path.clone();
        let _ = sender.send(AppEvent::WeeklyProgress(WeeklyProgressEvent::Fetching(
            habit_id,
        )));
        thread::spawn(move || {
            let habit_db = match HabitDb::new(&path) {
                Ok(h) => h,
                Err(e) => {
                    let _ = sender.send(AppEvent::WeeklyProgress(WeeklyProgressEvent::Failed(
                        habit_id,
                        e.to_string(),
                    )));
                    return;
                }
            };
            match habit_db.get_weekly_progress() {
                Ok(map) => {
                    let progress = map.get(&habit_id).copied().unwrap_or(0);
                    let _ = sender.send(AppEvent::WeeklyProgress(WeeklyProgressEvent::Fetched(
                        habit_id, progress,
                    )));
                }
                Err(e) => {
                    let _ = sender.send(AppEvent::WeeklyProgress(WeeklyProgressEvent::Failed(
                        habit_id,
                        e.to_string(),
                    )));
                }
            }
        });
    }
}

pub struct MonthlyProgressAction {
    sender: mpsc::Sender<AppEvent>,
    db_path: PathBuf,
}

impl MonthlyProgressAction {
    pub fn new(sender: mpsc::Sender<AppEvent>, db_path: PathBuf) -> Self {
        Self { sender, db_path }
    }

    pub fn fetch(&self, habit_id: i32) {
        let sender = self.sender.clone();
        let path = self.db_path.clone();
        let _ = sender.send(AppEvent::MonthlyProgress(MonthlyProgressEvent::Fetching(
            habit_id,
        )));
        thread::spawn(move || {
            let habit_db = match HabitDb::new(&path) {
                Ok(h) => h,
                Err(e) => {
                    let _ = sender.send(AppEvent::MonthlyProgress(MonthlyProgressEvent::Failed(
                        habit_id,
                        e.to_string(),
                    )));
                    return;
                }
            };
            match habit_db.get_monthly_progress() {
                Ok(map) => {
                    let progress = map.get(&habit_id).copied().unwrap_or(0);
                    let _ = sender.send(AppEvent::MonthlyProgress(MonthlyProgressEvent::Fetched(
                        habit_id, progress,
                    )));
                }
                Err(e) => {
                    let _ = sender.send(AppEvent::MonthlyProgress(MonthlyProgressEvent::Failed(
                        habit_id,
                        e.to_string(),
                    )));
                }
            }
        });
    }
}

pub struct YearlyProgressAction {
    sender: mpsc::Sender<AppEvent>,
    db_path: PathBuf,
}

impl YearlyProgressAction {
    pub fn new(sender: mpsc::Sender<AppEvent>, db_path: PathBuf) -> Self {
        Self { sender, db_path }
    }

    pub fn fetch(&self, habit_id: i32) {
        let sender = self.sender.clone();
        let path = self.db_path.clone();
        let _ = sender.send(AppEvent::YearlyProgress(YearlyProgressEvent::Fetching(
            habit_id,
        )));
        thread::spawn(move || {
            let habit_db = match HabitDb::new(&path) {
                Ok(h) => h,
                Err(e) => {
                    let _ = sender.send(AppEvent::YearlyProgress(YearlyProgressEvent::Failed(
                        habit_id,
                        e.to_string(),
                    )));
                    return;
                }
            };
            match habit_db.get_yearly_progress() {
                Ok(map) => {
                    let progress = map.get(&habit_id).copied().unwrap_or(0);
                    let _ = sender.send(AppEvent::YearlyProgress(YearlyProgressEvent::Fetched(
                        habit_id, progress,
                    )));
                }
                Err(e) => {
                    let _ = sender.send(AppEvent::YearlyProgress(YearlyProgressEvent::Failed(
                        habit_id,
                        e.to_string(),
                    )));
                }
            }
        });
    }
}
