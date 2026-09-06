use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::App,
    constants::{CURSOR_DELAY_THRESHOLD, CURSOR_TYPING_DELAY},
    controller::Actions,
    model::habit::HabitUpdate,
    state::States,
    utils::{get_character_from_keycode, is_backspace_code, is_char_code, is_numeric_keycode},
    widgets::notifier::Notifier,
};

#[allow(clippy::needless_pass_by_ref_mut)]
#[allow(clippy::needless_return)]
pub fn handle_edit_modal(
    app: &mut App,
    key_event: KeyEvent,
    states: &mut States,
    actions: &Actions,
    notifier: &mut Notifier,
) {
    let is_editing = states.modal_state.edit_modal_state_mut().is_editing();
    if app.display_edit_modal.is_none() || is_editing {
        return;
    }
    let code = key_event.code;

    match code {
        KeyCode::Esc => {
            app.hide_all_modals();
            states.modal_state.reset();
            return;
        }
        KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.quit();
            return;
        }
        _ => {}
    }

    if code == KeyCode::Enter {
        let name_is_empty = states
            .modal_state
            .edit_modal_state_mut()
            .habit_name_input_state_mut()
            .get_value()
            .trim()
            .is_empty();
        let goal_field_is_empty = states
            .modal_state
            .edit_modal_state_mut()
            .any_active_goal_field_empty();
        if name_is_empty {
            notifier.notify_error("Habit name should not be empty");
        } else if goal_field_is_empty {
            notifier.notify_error("Goal value should not be empty");
        } else if let Some(update) = build_habit_update(app, states) {
            let created_at = app
                .display_edit_modal
                .and_then(|id| app.habits.iter().find(|h| h.id == id))
                .map(|h| h.created_at.clone())
                .unwrap_or_default();
            states
                .modal_state
                .edit_modal_state_mut()
                .set_is_editing(true);
            actions.edit_habit(update, created_at);
        }
        return;
    }

    let edit_modal_state = states.modal_state.edit_modal_state_mut();

    match code {
        KeyCode::Tab | KeyCode::Down => {
            edit_modal_state.focus_next_field();
            return;
        }
        KeyCode::BackTab | KeyCode::Up => {
            edit_modal_state.focus_prev_field();
            return;
        }
        KeyCode::Left => {
            let field = edit_modal_state.get_current_actual_field();
            let input = match field {
                0 => edit_modal_state.habit_name_input_state_mut(),
                1 => edit_modal_state.daily_goal_input_state_mut(),
                2 => edit_modal_state.weekly_goal_input_state_mut(),
                3 => edit_modal_state.monthly_goal_input_state_mut(),
                _ => edit_modal_state.yearly_goal_input_state_mut(),
            };
            let delay = input.get_cursor_visibility_delay();
            if delay < CURSOR_DELAY_THRESHOLD {
                input.set_cursor_visibility_delay(delay + CURSOR_TYPING_DELAY);
            }
            input.move_cursor_left();
            return;
        }
        KeyCode::Right => {
            let field = edit_modal_state.get_current_actual_field();
            let input = match field {
                0 => edit_modal_state.habit_name_input_state_mut(),
                1 => edit_modal_state.daily_goal_input_state_mut(),
                2 => edit_modal_state.weekly_goal_input_state_mut(),
                3 => edit_modal_state.monthly_goal_input_state_mut(),
                _ => edit_modal_state.yearly_goal_input_state_mut(),
            };
            let delay = input.get_cursor_visibility_delay();
            if delay < CURSOR_DELAY_THRESHOLD {
                input.set_cursor_visibility_delay(delay + CURSOR_TYPING_DELAY);
            }
            input.move_cursor_right();
            return;
        }
        _ => {}
    }

    if edit_modal_state.get_current_actual_field() == 0 {
        let habit_name_input_state = edit_modal_state.habit_name_input_state_mut();
        let cursor_delay = habit_name_input_state.get_cursor_visibility_delay();
        if cursor_delay < CURSOR_DELAY_THRESHOLD {
            habit_name_input_state.set_cursor_visibility_delay(cursor_delay + CURSOR_TYPING_DELAY);
        }
        if is_char_code(code) {
            habit_name_input_state.push_char(get_character_from_keycode(code).unwrap());
        } else if is_backspace_code(code) {
            habit_name_input_state.backspace();
        }
        return;
    }

    if edit_modal_state.get_current_actual_field() == 1 {
        let daily_goal_input_state = edit_modal_state.daily_goal_input_state_mut();
        let cursor_delay = daily_goal_input_state.get_cursor_visibility_delay();
        if cursor_delay < CURSOR_DELAY_THRESHOLD {
            daily_goal_input_state.set_cursor_visibility_delay(cursor_delay + CURSOR_TYPING_DELAY);
        }
        if is_numeric_keycode(code) {
            daily_goal_input_state.push_char(get_character_from_keycode(code).unwrap());
        } else if is_backspace_code(code) {
            daily_goal_input_state.backspace();
        } else if is_char_code(code) {
            notifier.notify_error("Enter only digits");
        }
        return;
    }

    if edit_modal_state.get_current_actual_field() == 2 {
        let weekly_goal_input_state = edit_modal_state.weekly_goal_input_state_mut();
        let cursor_delay = weekly_goal_input_state.get_cursor_visibility_delay();
        if cursor_delay < CURSOR_DELAY_THRESHOLD {
            weekly_goal_input_state.set_cursor_visibility_delay(cursor_delay + CURSOR_TYPING_DELAY);
        }
        if is_numeric_keycode(code) {
            weekly_goal_input_state.push_char(get_character_from_keycode(code).unwrap());
        } else if is_backspace_code(code) {
            weekly_goal_input_state.backspace();
        } else if is_char_code(code) {
            notifier.notify_error("Enter only digits");
        }
        return;
    }

    if edit_modal_state.get_current_actual_field() == 3 {
        let monthly_goal_input_state = edit_modal_state.monthly_goal_input_state_mut();
        let cursor_delay = monthly_goal_input_state.get_cursor_visibility_delay();
        if cursor_delay < CURSOR_DELAY_THRESHOLD {
            monthly_goal_input_state
                .set_cursor_visibility_delay(cursor_delay + CURSOR_TYPING_DELAY);
        }
        if is_numeric_keycode(code) {
            monthly_goal_input_state.push_char(get_character_from_keycode(code).unwrap());
        } else if is_backspace_code(code) {
            monthly_goal_input_state.backspace();
        } else if is_char_code(code) {
            notifier.notify_error("Enter only digits");
        }
        return;
    }

    if edit_modal_state.get_current_actual_field() == 4 {
        let yearly_goal_input_state = edit_modal_state.yearly_goal_input_state_mut();
        let cursor_delay = yearly_goal_input_state.get_cursor_visibility_delay();
        if cursor_delay < CURSOR_DELAY_THRESHOLD {
            yearly_goal_input_state.set_cursor_visibility_delay(cursor_delay + CURSOR_TYPING_DELAY);
        }
        if is_numeric_keycode(code) {
            yearly_goal_input_state.push_char(get_character_from_keycode(code).unwrap());
        } else if is_backspace_code(code) {
            yearly_goal_input_state.backspace();
        } else if is_char_code(code) {
            notifier.notify_error("Enter only digits");
        }
        return;
    }
}

fn build_habit_update(app: &App, states: &mut States) -> Option<HabitUpdate> {
    let habit_id = app.display_edit_modal?;
    let edit_modal_state = states.modal_state.edit_modal_state_mut();
    let name = edit_modal_state
        .habit_name_input_state_mut()
        .get_value()
        .trim()
        .to_string();
    if name.is_empty() {
        return None;
    }
    let daily = edit_modal_state
        .daily_goal_input_state_mut()
        .get_value()
        .parse::<i32>()
        .unwrap_or(0);
    let weekly = edit_modal_state
        .weekly_goal_input_state_mut()
        .get_value()
        .parse::<i32>()
        .unwrap_or(0);
    let monthly = edit_modal_state
        .monthly_goal_input_state_mut()
        .get_value()
        .parse::<i32>()
        .unwrap_or(0);
    let yearly = edit_modal_state
        .yearly_goal_input_state_mut()
        .get_value()
        .parse::<i32>()
        .unwrap_or(0);
    Some(HabitUpdate {
        id: habit_id,
        name,
        daily_goal: daily,
        weekly_goal: weekly,
        monthly_goal: monthly,
        yearly_goal: yearly,
    })
}
