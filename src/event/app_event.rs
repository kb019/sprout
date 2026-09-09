use crate::model::habit::{BestStreak, Habit, HabitLog};

#[derive(Clone, Debug)]
pub enum AddHabitEvent {
    Adding,
    Added(Habit),
    Failed(String),
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum EditHabitEvent {
    Editing(i32),
    Edited(Habit),
    Failed(i32, String),
}

#[derive(Clone, Debug)]
pub enum LogHabitEvent {
    Logging(i32),
    Logged(HabitLog),
    Failed(i32, String),
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum DeleteHabitEvent {
    Deleting(i32),
    Deleted(i32),
    Failed(i32, String),
}

#[derive(Clone, Debug)]
pub enum GetStreakEvent {
    Fetching(i32),
    Fetched(i32, i32),
    Failed(i32, String),
}

#[derive(Clone, Debug)]
pub enum DailyProgressEvent {
    Fetching(i32),
    Fetched(i32, i32),
    Failed(i32, String),
}

#[derive(Clone, Debug)]
pub enum WeeklyProgressEvent {
    Fetching(i32),
    Fetched(i32, i32),
    Failed(i32, String),
}

#[derive(Clone, Debug)]
pub enum MonthlyProgressEvent {
    Fetching(i32),
    Fetched(i32, i32),
    Failed(i32, String),
}

#[derive(Clone, Debug)]
pub enum YearlyProgressEvent {
    Fetching(i32),
    Fetched(i32, i32),
    Failed(i32, String),
}

#[derive(Clone, Debug)]
pub enum BestStreaksEvent {
    Fetching,
    Fetched(Vec<BestStreak>),
    Failed(String),
}

#[derive(Clone, Debug)]
pub enum ActiveDaysEvent {
    Fetching,
    Fetched(u32),
    Failed(String),
}

#[derive(Clone, Debug)]
pub enum WeeklyAverageEvent {
    Fetching,
    Fetched([u32; 7]),
    Failed(String),
}

#[derive(Clone, Debug)]
pub enum HeatmapYearHabitsEvent {
    Fetching,
    Fetched(Vec<(i32, Vec<i32>)>),
    Failed(String),
}

#[derive(Clone, Debug)]
pub enum HeatmapDataEvent {
    Fetching(i32, i32),
    Fetched(i32, i32, Vec<(String, bool, i32)>),
    Failed(i32, i32, String),
}

#[derive(Clone, Debug)]
pub enum ResetEvent {
    Resetting,
    Reset,
    Failed(String),
}

#[derive(Clone, Debug)]
pub enum SettingsSaveEvent {
    Saving,
    Saved,
    Failed(String),
}
