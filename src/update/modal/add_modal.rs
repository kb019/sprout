use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::App,
    state::modal::ModalState,
    utils::{
        get_character_from_keycode, is_backspace_code, is_char_code, is_left_key, is_right_key,
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
}
