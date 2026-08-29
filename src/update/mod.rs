pub mod app;
pub mod modal;

use ratatui::crossterm::event::KeyEvent;

use crate::app::App;
use crate::controller::Actions;
use crate::event::AddHabitEvent;
use crate::state::States;
use crate::widgets::notifier::Notifier;

pub fn handle(app: &mut App, key_event: KeyEvent, states: &mut States, actions: &Actions) {
    if app.is_modal_in_focus() {
        modal::handle_modal(app, key_event, states, actions);
    } else {
        app::handle_app(app, key_event, states);
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
