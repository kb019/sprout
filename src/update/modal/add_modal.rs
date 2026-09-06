use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::App,
    constants::{CURSOR_DELAY_THRESHOLD, CURSOR_TYPING_DELAY},
    controller::Actions,
    model::habit::NewHabit,
    state::States,
    utils::{get_character_from_keycode, is_backspace_code, is_char_code, is_numeric_keycode},
    widgets::notifier::Notifier,
};

#[allow(clippy::needless_pass_by_ref_mut)]
#[allow(clippy::needless_return)]
pub fn handle_add_modal(
    app: &mut App,
    key_event: KeyEvent,
    states: &mut States,
    actions: &Actions,
    notifier: &mut Notifier,
) {
    let is_adding_habit = states.modal_state.add_modal_state_mut().is_adding_habit();
    if !app.display_add_modal || is_adding_habit {
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
            .add_modal_state_mut()
            .habit_name_input_state_mut()
            .get_value()
            .trim()
            .is_empty();
        if name_is_empty {
            notifier.notify_error("Habit name should not be empty");
        } else if let Some(new_habit) = build_new_habit(states) {
            actions.add_habit(new_habit);
        }
        return;
    }

    let add_modal_state = states.modal_state.add_modal_state_mut();

    match code {
        KeyCode::Esc => {
            app.hide_all_modals();
            add_modal_state.reset();
            return;
        }
        KeyCode::Tab | KeyCode::Down => {
            add_modal_state.focus_next_field();
            return;
        }
        KeyCode::BackTab | KeyCode::Up => {
            add_modal_state.focus_prev_field();
            return;
        }
        KeyCode::Left => {
            let field = add_modal_state.get_current_field_focus();
            let input = match field {
                0 => add_modal_state.habit_name_input_state_mut(),
                1 => add_modal_state.daily_goal_input_state_mut(),
                2 => add_modal_state.weekly_goal_input_state_mut(),
                3 => add_modal_state.monthly_goal_input_state_mut(),
                _ => add_modal_state.yearly_goal_input_state_mut(),
            };
            let delay = input.get_cursor_visibility_delay();
            if delay < CURSOR_DELAY_THRESHOLD {
                input.set_cursor_visibility_delay(delay + CURSOR_TYPING_DELAY);
            }
            input.move_cursor_left();
            return;
        }
        KeyCode::Right => {
            let field = add_modal_state.get_current_field_focus();
            let input = match field {
                0 => add_modal_state.habit_name_input_state_mut(),
                1 => add_modal_state.daily_goal_input_state_mut(),
                2 => add_modal_state.weekly_goal_input_state_mut(),
                3 => add_modal_state.monthly_goal_input_state_mut(),
                _ => add_modal_state.yearly_goal_input_state_mut(),
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

    if add_modal_state.get_current_field_focus() == 0 {
        let habit_name_input_state = add_modal_state.habit_name_input_state_mut();
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

    if add_modal_state.get_current_field_focus() == 1 {
        let daily_goal_input_state = add_modal_state.daily_goal_input_state_mut();
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

    if add_modal_state.get_current_field_focus() == 2 {
        let weekly_goal_input_state = add_modal_state.weekly_goal_input_state_mut();
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

    if add_modal_state.get_current_field_focus() == 3 {
        let monthly_goal_input_state = add_modal_state.monthly_goal_input_state_mut();
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

    if add_modal_state.get_current_field_focus() == 4 {
        let yearly_goal_input_state = add_modal_state.yearly_goal_input_state_mut();
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

fn build_new_habit(states: &mut States) -> Option<NewHabit> {
    let add_modal_state = states.modal_state.add_modal_state_mut();
    let name = add_modal_state
        .habit_name_input_state_mut()
        .get_value()
        .trim()
        .to_string();
    if name.is_empty() {
        return None;
    }
    let daily = add_modal_state
        .daily_goal_input_state_mut()
        .get_value()
        .parse::<i32>()
        .unwrap_or(0);
    let weekly = add_modal_state
        .weekly_goal_input_state_mut()
        .get_value()
        .parse::<i32>()
        .unwrap_or(0);
    let monthly = add_modal_state
        .monthly_goal_input_state_mut()
        .get_value()
        .parse::<i32>()
        .unwrap_or(0);
    let yearly = add_modal_state
        .yearly_goal_input_state_mut()
        .get_value()
        .parse::<i32>()
        .unwrap_or(0);
    Some(NewHabit {
        name,
        daily_goal: daily,
        weekly_goal: weekly,
        monthly_goal: monthly,
        yearly_goal: yearly,
    })
}
