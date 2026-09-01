pub mod app;
pub mod modal;

use ratatui::crossterm::event::KeyEvent;

use crate::app::App;
use crate::controller::Actions;
use crate::event::{
    AddHabitEvent, DailyProgressEvent, DeleteHabitEvent, EditHabitEvent, GetStreakEvent,
    LogHabitEvent, MonthlyProgressEvent, WeeklyProgressEvent, YearlyProgressEvent,
};
use crate::state::States;
use crate::widgets::notifier::Notifier;

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
            app.streaks.insert(habit_id, streak);
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
