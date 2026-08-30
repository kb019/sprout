pub mod app;
pub mod modal;

use ratatui::crossterm::event::KeyEvent;

use crate::app::App;
use crate::controller::Actions;
use crate::event::{AddHabitEvent, LogHabitEvent};
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
            states.add_habit_state.set_is_adding_habit(true);
        }
        AddHabitEvent::Added(habit) => {
            notifier.notify_success(&format!("{} added successfully", habit.name));
            app.habits.push(habit);
            app.hide_all_modals();

            states.modal_state.reset();
            states.add_habit_state.set_is_adding_habit(false);
        }
        AddHabitEvent::Failed(message) => {
            notifier.notify_error(&format!("Failed to add habit: {}", message));
            states.add_habit_state.set_is_adding_habit(false);
        }
    }
}

pub fn handle_log_habit_event(
    app: &mut App,
    event: LogHabitEvent,
    states: &mut States,
    notifier: &mut Notifier,
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
        }
        LogHabitEvent::Failed(habit_id, message) => {
            notifier.notify_error(&format!("Failed to log habit: {}", message));
            states.log_habit_state.stop_logging(habit_id);
        }
    }
}
