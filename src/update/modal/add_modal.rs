use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::App,
    constants::{CURSOR_DELAY_THRESHOLD, CURSOR_TYPING_DELAY},
    state::modal::ModalState,
    utils::{
        get_character_from_keycode, is_backspace_code, is_char_code, is_left_key,
        is_numeric_keycode, is_right_key,
    },
};

#[allow(clippy::needless_pass_by_ref_mut)]
#[allow(clippy::needless_return)]
pub fn handle_add_modal(app: &mut App, key_event: KeyEvent, state: &mut ModalState) {
    if !app.display_add_modal {
        return;
    }
    let code = key_event.code;

    let add_modal_state = state.add_modal_state_mut();
    match code {
        KeyCode::Esc => {
            add_modal_state.reset();
        }
        KeyCode::Tab => {
            add_modal_state.focus_next_field();
        }
        KeyCode::BackTab => {
            add_modal_state.focus_prev_field();
        }
        _ => {}
    }

    if add_modal_state.get_current_field_focus() == 0 {
        let habit_name_input_state = add_modal_state.habit_name_input_state_mut();
        let cursur_delay_visibility = habit_name_input_state.get_cursor_visibility_delay();
        if cursur_delay_visibility < CURSOR_DELAY_THRESHOLD {
            habit_name_input_state
                .set_cursor_visibility_delay(cursur_delay_visibility + CURSOR_TYPING_DELAY);
        }

        if is_char_code(code) {
            habit_name_input_state.push_char(get_character_from_keycode(code).unwrap());
        } else if is_backspace_code(code) {
            habit_name_input_state.backspace();
        } else if is_left_key(code) {
            habit_name_input_state.move_cursor_left();
        } else if is_right_key(code) {
            habit_name_input_state.move_cursor_right();
        }

        return;
    }

    if add_modal_state.get_current_field_focus() == 1 {
        let daily_goal_input_state = add_modal_state.daily_goal_input_state_mut();
        let cursur_delay_visibility = daily_goal_input_state.get_cursor_visibility_delay();
        if cursur_delay_visibility < CURSOR_DELAY_THRESHOLD {
            daily_goal_input_state
                .set_cursor_visibility_delay(cursur_delay_visibility + CURSOR_TYPING_DELAY);
        }

        if is_numeric_keycode(code) {
            daily_goal_input_state.push_char(get_character_from_keycode(code).unwrap());
        } else if is_backspace_code(code) {
            daily_goal_input_state.backspace();
        } else if is_left_key(code) {
            daily_goal_input_state.move_cursor_left();
        } else if is_right_key(code) {
            daily_goal_input_state.move_cursor_right();
        }

        return;
    }

    if add_modal_state.get_current_field_focus() == 2 {
        let weekly_goal_input_state = add_modal_state.weekly_goal_input_state_mut();
        let cursur_delay_visibility = weekly_goal_input_state.get_cursor_visibility_delay();
        if cursur_delay_visibility < CURSOR_DELAY_THRESHOLD {
            weekly_goal_input_state
                .set_cursor_visibility_delay(cursur_delay_visibility + CURSOR_TYPING_DELAY);
        }

        if is_numeric_keycode(code) {
            weekly_goal_input_state.push_char(get_character_from_keycode(code).unwrap());
        } else if is_backspace_code(code) {
            weekly_goal_input_state.backspace();
        } else if is_left_key(code) {
            weekly_goal_input_state.move_cursor_left();
        } else if is_right_key(code) {
            weekly_goal_input_state.move_cursor_right();
        }

        return;
    }

    if add_modal_state.get_current_field_focus() == 3 {
        let monthly_goal_input_state = add_modal_state.monthly_goal_input_state_mut();
        let cursur_delay_visibility = monthly_goal_input_state.get_cursor_visibility_delay();
        if cursur_delay_visibility < CURSOR_DELAY_THRESHOLD {
            monthly_goal_input_state
                .set_cursor_visibility_delay(cursur_delay_visibility + CURSOR_TYPING_DELAY);
        }
        if is_numeric_keycode(code) {
            monthly_goal_input_state.push_char(get_character_from_keycode(code).unwrap());
        } else if is_backspace_code(code) {
            monthly_goal_input_state.backspace();
        } else if is_left_key(code) {
            monthly_goal_input_state.move_cursor_left();
        } else if is_right_key(code) {
            monthly_goal_input_state.move_cursor_right();
        }

        return;
    }

    if add_modal_state.get_current_field_focus() == 4 {
        let yearly_goal_input_state = add_modal_state.yearly_goal_input_state_mut();
        let cursur_delay_visibility = yearly_goal_input_state.get_cursor_visibility_delay();
        if cursur_delay_visibility < CURSOR_DELAY_THRESHOLD {
            yearly_goal_input_state
                .set_cursor_visibility_delay(cursur_delay_visibility + CURSOR_TYPING_DELAY);
        }
        if is_numeric_keycode(code) {
            yearly_goal_input_state.push_char(get_character_from_keycode(code).unwrap());
        } else if is_backspace_code(code) {
            yearly_goal_input_state.backspace();
        } else if is_left_key(code) {
            yearly_goal_input_state.move_cursor_left();
        } else if is_right_key(code) {
            yearly_goal_input_state.move_cursor_right();
        }

        return;
    }
}
