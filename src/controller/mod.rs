mod active_days;
mod add_habit;
mod best_streaks;
mod delete_habit;
mod edit_habit;
mod get_streak;
mod heatmap_data;
mod heatmap_year_habits;
mod log_habit;
mod progress;
mod weekly_average;

use std::path::PathBuf;
use std::sync::mpsc;

use crate::event::AppEvent;
use crate::model::habit::{HabitUpdate, NewHabit};
use active_days::ActiveDaysAction;
use add_habit::AddHabitAction;
use best_streaks::BestStreaksAction;
use delete_habit::DeleteHabitAction;
use edit_habit::EditHabitAction;
use get_streak::GetStreakAction;
use heatmap_data::HeatmapDataAction;
use heatmap_year_habits::HeatmapYearHabitsAction;
use log_habit::LogHabitAction;
use progress::{
    DailyProgressAction, MonthlyProgressAction, WeeklyProgressAction, YearlyProgressAction,
};
use weekly_average::WeeklyAverageAction;

pub struct Actions {
    active_days_action: ActiveDaysAction,
    heatmap_data_action: HeatmapDataAction,
    heatmap_year_habits_action: HeatmapYearHabitsAction,
    weekly_average_action: WeeklyAverageAction,
    add_habit_action: AddHabitAction,
    best_streaks_action: BestStreaksAction,
    edit_habit_action: EditHabitAction,
    log_habit_action: LogHabitAction,
    delete_habit_action: DeleteHabitAction,
    get_streak_action: GetStreakAction,
    daily_progress_action: DailyProgressAction,
    weekly_progress_action: WeeklyProgressAction,
    monthly_progress_action: MonthlyProgressAction,
    yearly_progress_action: YearlyProgressAction,
}

impl Actions {
    pub fn new(sender: mpsc::Sender<AppEvent>, db_path: PathBuf) -> Self {
        Self {
            active_days_action: ActiveDaysAction::new(sender.clone(), db_path.clone()),
            heatmap_data_action: HeatmapDataAction::new(sender.clone(), db_path.clone()),
            heatmap_year_habits_action: HeatmapYearHabitsAction::new(
                sender.clone(),
                db_path.clone(),
            ),
            weekly_average_action: WeeklyAverageAction::new(sender.clone(), db_path.clone()),
            add_habit_action: AddHabitAction::new(sender.clone(), db_path.clone()),
            best_streaks_action: BestStreaksAction::new(sender.clone(), db_path.clone()),
            edit_habit_action: EditHabitAction::new(sender.clone(), db_path.clone()),
            log_habit_action: LogHabitAction::new(sender.clone(), db_path.clone()),
            delete_habit_action: DeleteHabitAction::new(sender.clone(), db_path.clone()),
            get_streak_action: GetStreakAction::new(sender.clone(), db_path.clone()),
            daily_progress_action: DailyProgressAction::new(sender.clone(), db_path.clone()),
            weekly_progress_action: WeeklyProgressAction::new(sender.clone(), db_path.clone()),
            monthly_progress_action: MonthlyProgressAction::new(sender.clone(), db_path.clone()),
            yearly_progress_action: YearlyProgressAction::new(sender, db_path),
        }
    }

    pub fn add_habit(&self, new_habit: NewHabit) {
        self.add_habit_action.add_habit(new_habit);
    }

    pub fn edit_habit(&self, update: HabitUpdate, created_at: String) {
        self.edit_habit_action.edit_habit(update, created_at);
    }

    pub fn log_habit(&self, habit_id: i32, completed: i32, progress: i32) {
        self.log_habit_action
            .log_habit(habit_id, completed, progress);
    }

    pub fn delete_habit(&self, habit_id: i32) {
        self.delete_habit_action.delete_habit(habit_id);
    }

    pub fn get_streak(&self, habit_id: i32) {
        self.get_streak_action.get_streak(habit_id);
    }

    pub fn fetch_daily_progress(&self, habit_id: i32) {
        self.daily_progress_action.fetch(habit_id);
    }

    pub fn fetch_weekly_progress(&self, habit_id: i32) {
        self.weekly_progress_action.fetch(habit_id);
    }

    pub fn fetch_monthly_progress(&self, habit_id: i32) {
        self.monthly_progress_action.fetch(habit_id);
    }

    pub fn fetch_yearly_progress(&self, habit_id: i32) {
        self.yearly_progress_action.fetch(habit_id);
    }

    pub fn fetch_best_streaks(&self) {
        self.best_streaks_action.fetch();
    }

    pub fn fetch_active_days(&self) {
        self.active_days_action.fetch();
    }

    pub fn fetch_weekly_average(&self) {
        self.weekly_average_action.fetch();
    }

    pub fn fetch_heatmap_year_habits(&self) {
        self.heatmap_year_habits_action.fetch();
    }

    pub fn fetch_heatmap_data(&self, habit_id: i32, year: i32) {
        self.heatmap_data_action.fetch(habit_id, year);
    }
}
