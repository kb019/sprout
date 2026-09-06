pub mod app;
pub mod modal;

use ratatui::crossterm::event::KeyEvent;

use crate::app::App;
use crate::constants::{
    ACTIVE_DAYS_REFRESH_DELAY_TICKS, BEST_STREAK_REFRESH_DELAY_TICKS,
    HEATMAP_YEAR_HABITS_REFRESH_DELAY_TICKS, WEEKLY_AVERAGE_REFRESH_DELAY_TICKS,
};
use crate::controller::Actions;
use crate::event::{
    ActiveDaysEvent, AddHabitEvent, BestStreaksEvent, DailyProgressEvent, DeleteHabitEvent,
    EditHabitEvent, GetStreakEvent, HeatmapDataEvent, HeatmapYearHabitsEvent, LogHabitEvent,
    MonthlyProgressEvent, WeeklyAverageEvent, WeeklyProgressEvent, YearlyProgressEvent,
};
use crate::state::States;
use crate::widgets::notifier::Notifier;
use chrono::Datelike;
use std::collections::HashMap;

pub fn handle(app: &mut App, key_event: KeyEvent, states: &mut States, actions: &Actions) {
    if app.is_modal_in_focus() {
        modal::handle_modal(app, key_event, states, actions);
    } else {
        app::handle_app(app, key_event, states, actions);
    }
}

pub fn handle_add_habit_event(
    app: &mut App,
    event: AddHabitEvent,
    states: &mut States,
    notifier: &mut Notifier,
    _actions: &Actions,
) {
    match event {
        AddHabitEvent::Adding => {
            states
                .modal_state
                .add_modal_state_mut()
                .set_is_adding_habit(true);
        }
        AddHabitEvent::Added(habit) => {
            notifier.notify_success(&format!("{} added successfully", habit.name));
            app.habits.push(habit);
            app.hide_all_modals();
            states
                .modal_state
                .add_modal_state_mut()
                .set_is_adding_habit(false);
            states.modal_state.reset();
            update_heatmap_year_habits_with_delay(app, states);
        }
        AddHabitEvent::Failed(message) => {
            notifier.notify_error(&format!("Failed to add habit: {}", message));
            states
                .modal_state
                .add_modal_state_mut()
                .set_is_adding_habit(false);
        }
    }
}

// Resets the countdown so rapid logs collapse into a single refresh.
// Sets is_fetching immediately so the UI shows a loading state right away.
// The actual fetch fires in the tick handler once the counter reaches zero.
fn update_best_streaks_with_delay(app: &mut App, states: &mut States) {
    app.best_streak_refresh_delay = BEST_STREAK_REFRESH_DELAY_TICKS;
    states.best_streaks_state.start_fetching();
}

fn update_active_days_with_delay(app: &mut App, states: &mut States) {
    app.active_days_refresh_delay = ACTIVE_DAYS_REFRESH_DELAY_TICKS;
    states.active_days_state.start_fetching();
}

fn update_weekly_average_with_delay(app: &mut App, states: &mut States) {
    app.weekly_average_refresh_delay = WEEKLY_AVERAGE_REFRESH_DELAY_TICKS;
    states.weekly_average_state.start_fetching();
}

fn update_heatmap_year_habits_with_delay(app: &mut App, states: &mut States) {
    app.heatmap_year_habits_refresh_delay = HEATMAP_YEAR_HABITS_REFRESH_DELAY_TICKS;
    states.heatmap_year_habits_state.start_fetching();
}

pub fn handle_log_habit_event(
    app: &mut App,
    event: LogHabitEvent,
    states: &mut States,
    notifier: &mut Notifier,
    actions: &Actions,
) {
    match event {
        LogHabitEvent::Logging(habit_id) => {
            states.log_habit_state.start_logging(habit_id);
        }
        LogHabitEvent::Logged(log) => {
            if log.completed {
                app.completed_habits.insert(log.habit_id);
            } else {
                app.completed_habits.remove(&log.habit_id);
            }
            if let Some(habit) = app.habits.iter().find(|h| h.id == log.habit_id) {
                let has_goals = habit.daily_goal > 0
                    || habit.weekly_goal > 0
                    || habit.monthly_goal > 0
                    || habit.yearly_goal > 0;
                let message = if has_goals {
                    format!(
                        "{} - {} units updated successfully",
                        habit.name, log.progress
                    )
                } else {
                    let status = if log.completed {
                        "completed"
                    } else {
                        "uncompleted"
                    };
                    format!("{} marked as {}", habit.name, status)
                };
                notifier.notify_success(&message);
            }
            if app.progress_modal_for_habit_id == Some(log.habit_id) {
                app.hide_log_progress_modal();
                states.modal_state.reset();
            }
            states.log_habit_state.stop_logging(log.habit_id);
            actions.get_streak(log.habit_id);
            if let Some(habit) = app.habits.iter().find(|h| h.id == log.habit_id) {
                if habit.daily_goal > 0 {
                    actions.fetch_daily_progress(log.habit_id);
                }
                if habit.weekly_goal > 0 {
                    actions.fetch_weekly_progress(log.habit_id);
                }
                if habit.monthly_goal > 0 {
                    actions.fetch_monthly_progress(log.habit_id);
                }
                if habit.yearly_goal > 0 {
                    actions.fetch_yearly_progress(log.habit_id);
                }
            }
            update_best_streaks_with_delay(app, states);
            update_active_days_with_delay(app, states);
            update_weekly_average_with_delay(app, states);
            let current_year = chrono::Local::now().year();
            app.heatmap_data.remove(&(log.habit_id, current_year));
            actions.fetch_heatmap_data(log.habit_id, current_year);
        }
        LogHabitEvent::Failed(habit_id, message) => {
            notifier.notify_error(&format!("Failed to log habit: {}", message));

            states.log_habit_state.stop_logging(habit_id);
        }
    }
}

pub fn handle_get_streak_event(
    app: &mut App,
    event: GetStreakEvent,
    states: &mut States,
    notifier: &mut Notifier,
) {
    match event {
        GetStreakEvent::Fetching(habit_id) => {
            states.get_streak_state.start_fetching(habit_id);
        }
        GetStreakEvent::Fetched(habit_id, streak) => {
            app.active_streaks.insert(habit_id, streak);
            states.get_streak_state.stop_fetching(habit_id);
            if let Some(habit) = app.habits.iter().find(|h| h.id == habit_id) {
                notifier
                    .notify_success(&format!("{} streak updated to {} days", habit.name, streak));
            }
        }
        GetStreakEvent::Failed(habit_id, message) => {
            states.get_streak_state.stop_fetching(habit_id);
            notifier.notify_error(&format!("Failed to fetch streak: {}", message));
        }
    }
}

pub fn handle_daily_progress_event(
    app: &mut App,
    event: DailyProgressEvent,
    states: &mut States,
    notifier: &mut Notifier,
) {
    match event {
        DailyProgressEvent::Fetching(habit_id) => {
            states.daily_progress_state.start_fetching(habit_id);
        }
        DailyProgressEvent::Fetched(habit_id, progress) => {
            app.daily_progress.insert(habit_id, progress);
            states.daily_progress_state.stop_fetching(habit_id);
            if let Some(habit) = app.habits.iter().find(|h| h.id == habit_id) {
                notifier.notify_success(&format!(
                    "{} - daily progress: {} units",
                    habit.name, progress
                ));
            }
        }
        DailyProgressEvent::Failed(habit_id, message) => {
            states.daily_progress_state.stop_fetching(habit_id);
            notifier.notify_error(&format!("Failed to fetch daily progress: {}", message));
        }
    }
}

pub fn handle_weekly_progress_event(
    app: &mut App,
    event: WeeklyProgressEvent,
    states: &mut States,
    notifier: &mut Notifier,
) {
    match event {
        WeeklyProgressEvent::Fetching(habit_id) => {
            states.weekly_progress_state.start_fetching(habit_id);
        }
        WeeklyProgressEvent::Fetched(habit_id, progress) => {
            app.weekly_progress.insert(habit_id, progress);
            states.weekly_progress_state.stop_fetching(habit_id);
            if let Some(habit) = app.habits.iter().find(|h| h.id == habit_id) {
                notifier.notify_success(&format!(
                    "{} - weekly progress: {} units",
                    habit.name, progress
                ));
            }
        }
        WeeklyProgressEvent::Failed(habit_id, message) => {
            states.weekly_progress_state.stop_fetching(habit_id);
            notifier.notify_error(&format!("Failed to fetch weekly progress: {}", message));
        }
    }
}

pub fn handle_monthly_progress_event(
    app: &mut App,
    event: MonthlyProgressEvent,
    states: &mut States,
    notifier: &mut Notifier,
) {
    match event {
        MonthlyProgressEvent::Fetching(habit_id) => {
            states.monthly_progress_state.start_fetching(habit_id);
        }
        MonthlyProgressEvent::Fetched(habit_id, progress) => {
            app.monthly_progress.insert(habit_id, progress);
            states.monthly_progress_state.stop_fetching(habit_id);
            if let Some(habit) = app.habits.iter().find(|h| h.id == habit_id) {
                notifier.notify_success(&format!(
                    "{} - monthly progress: {} units",
                    habit.name, progress
                ));
            }
        }
        MonthlyProgressEvent::Failed(habit_id, message) => {
            states.monthly_progress_state.stop_fetching(habit_id);
            notifier.notify_error(&format!("Failed to fetch monthly progress: {}", message));
        }
    }
}

pub fn handle_yearly_progress_event(
    app: &mut App,
    event: YearlyProgressEvent,
    states: &mut States,
    notifier: &mut Notifier,
) {
    match event {
        YearlyProgressEvent::Fetching(habit_id) => {
            states.yearly_progress_state.start_fetching(habit_id);
        }
        YearlyProgressEvent::Fetched(habit_id, progress) => {
            app.yearly_progress.insert(habit_id, progress);
            states.yearly_progress_state.stop_fetching(habit_id);
            if let Some(habit) = app.habits.iter().find(|h| h.id == habit_id) {
                notifier.notify_success(&format!(
                    "{} - yearly progress: {} units",
                    habit.name, progress
                ));
            }
        }
        YearlyProgressEvent::Failed(habit_id, message) => {
            states.yearly_progress_state.stop_fetching(habit_id);
            notifier.notify_error(&format!("Failed to fetch yearly progress: {}", message));
        }
    }
}

pub fn handle_best_streaks_event(
    app: &mut App,
    event: BestStreaksEvent,
    states: &mut States,
    notifier: &mut Notifier,
) {
    match event {
        BestStreaksEvent::Fetching => {
            states.best_streaks_state.start_fetching();
        }
        BestStreaksEvent::Fetched(best) => {
            app.best_streaks = best;
            states.best_streaks_state.stop_fetching();
            notifier.notify_success("Best streaks loaded");
        }
        BestStreaksEvent::Failed(message) => {
            states.best_streaks_state.stop_fetching();
            notifier.notify_error(&format!("Failed to load best streaks: {}", message));
        }
    }
}

pub fn handle_edit_habit_event(
    app: &mut App,
    event: EditHabitEvent,
    states: &mut States,
    notifier: &mut Notifier,
) {
    match event {
        EditHabitEvent::Editing(_) => {
            states
                .modal_state
                .edit_modal_state_mut()
                .set_is_editing(true);
        }
        EditHabitEvent::Edited(habit) => {
            notifier.notify_success(&format!("{} updated successfully", habit.name));
            if let Some(pos) = app.habits.iter().position(|h| h.id == habit.id) {
                app.habits[pos] = habit;
            }
            app.hide_all_modals();
            states
                .modal_state
                .edit_modal_state_mut()
                .set_is_editing(false);
            states.modal_state.reset();
        }
        EditHabitEvent::Failed(_, message) => {
            notifier.notify_error(&format!("Failed to update habit: {}", message));
            states
                .modal_state
                .edit_modal_state_mut()
                .set_is_editing(false);
        }
    }
}

pub fn handle_delete_habit_event(
    app: &mut App,
    event: DeleteHabitEvent,
    states: &mut States,
    notifier: &mut Notifier,
    _actions: &Actions,
) {
    match event {
        DeleteHabitEvent::Deleting(_) => {
            states
                .modal_state
                .delete_modal_state_mut()
                .set_is_deleting(true);
        }
        DeleteHabitEvent::Deleted(habit_id) => {
            if let Some(pos) = app.habits.iter().position(|h| h.id == habit_id) {
                let name = app.habits[pos].name.clone();
                app.habits.remove(pos);
                app.completed_habits.remove(&habit_id);
                notifier.notify_success(&format!("{} deleted successfully", name));
            }
            app.hide_delete_modal();
            states
                .modal_state
                .delete_modal_state_mut()
                .set_is_deleting(false);
            states.modal_state.reset();
            update_heatmap_year_habits_with_delay(app, states);
        }
        DeleteHabitEvent::Failed(_, message) => {
            notifier.notify_error(&format!("Failed to delete habit: {}", message));
            states
                .modal_state
                .delete_modal_state_mut()
                .set_is_deleting(false);
        }
    }
}

pub fn handle_weekly_average_event(
    app: &mut App,
    event: WeeklyAverageEvent,
    states: &mut States,
    notifier: &mut Notifier,
) {
    match event {
        WeeklyAverageEvent::Fetching => {
            states.weekly_average_state.start_fetching();
        }
        WeeklyAverageEvent::Fetched(days) => {
            app.weekly_completion = days;
            states.weekly_average_state.stop_fetching();
            notifier.notify_success("Updated weekly average");
        }
        WeeklyAverageEvent::Failed(message) => {
            states.weekly_average_state.stop_fetching();
            notifier.notify_error(&format!("Failed to fetch weekly average: {}", message));
        }
    }
}

pub fn handle_heatmap_year_habits_event(
    app: &mut App,
    event: HeatmapYearHabitsEvent,
    states: &mut States,
    notifier: &mut Notifier,
) {
    match event {
        HeatmapYearHabitsEvent::Fetching => {
            states.heatmap_year_habits_state.start_fetching();
        }
        HeatmapYearHabitsEvent::Fetched(year_habits) => {
            app.heatmap_year_habits = year_habits;
            states
                .app_state
                .update_heatmap_tile_states(&app.heatmap_year_habits);
            states.heatmap_year_habits_state.stop_fetching();
            notifier.notify_success("Heatmap data refreshed");
        }
        HeatmapYearHabitsEvent::Failed(message) => {
            states.heatmap_year_habits_state.stop_fetching();
            notifier.notify_error(&format!("Failed to refresh heatmap data: {}", message));
        }
    }
}

pub fn handle_heatmap_data_event(
    app: &mut App,
    event: HeatmapDataEvent,
    states: &mut States,
    notifier: &mut Notifier,
) {
    match event {
        HeatmapDataEvent::Fetching(habit_id, year) => {
            states.heatmap_data_state.start_fetching(habit_id, year);
        }
        HeatmapDataEvent::Fetched(habit_id, year, rows) => {
            let max_p = rows.iter().map(|(_, _, p)| *p).max().unwrap_or(0);
            let min_p = rows.iter().map(|(_, _, p)| *p).min().unwrap_or(0);
            let intensity_map: HashMap<String, u8> = rows
                .into_iter()
                .map(|(date, completed, progress)| {
                    let intensity = if max_p == 0 || max_p == min_p {
                        if completed { 4 } else { 0 }
                    } else {
                        ((progress - min_p) as f32 / (max_p - min_p) as f32 * 4.0).min(4.0) as u8
                    };
                    (date, intensity)
                })
                .collect();
            app.heatmap_data.insert((habit_id, year), intensity_map);
            states.heatmap_data_state.stop_fetching(habit_id, year);
            let habit_name = app
                .habits
                .iter()
                .find(|h| h.id == habit_id)
                .map(|h| h.name.as_str())
                .unwrap_or("unknown");
            notifier.notify_success(&format!("Updated heatmap for {}-{}", habit_name, year));
        }
        HeatmapDataEvent::Failed(habit_id, year, message) => {
            states.heatmap_data_state.stop_fetching(habit_id, year);
            notifier.notify_error(&format!("Failed to load heatmap data: {}", message));
        }
    }
}

pub fn handle_active_days_event(
    app: &mut App,
    event: ActiveDaysEvent,
    states: &mut States,
    notifier: &mut Notifier,
) {
    match event {
        ActiveDaysEvent::Fetching => {
            states.active_days_state.start_fetching();
        }
        ActiveDaysEvent::Fetched(count) => {
            app.active_days = count;
            states.active_days_state.stop_fetching();
            notifier.notify_success(&format!("Active days: {}", count));
        }
        ActiveDaysEvent::Failed(message) => {
            states.active_days_state.stop_fetching();
            notifier.notify_error(&format!("Failed to fetch active days: {}", message));
        }
    }
}
