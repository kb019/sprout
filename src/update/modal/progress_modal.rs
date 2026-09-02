use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::App,
    constants::{CURSOR_DELAY_THRESHOLD, CURSOR_TYPING_DELAY},
    controller::Actions,
    state::States,
    utils::{get_character_from_keycode, is_backspace_code, is_numeric_keycode},
};

pub fn handle_progress_modal(
    app: &mut App,
    key_event: KeyEvent,
    states: &mut States,
    actions: &Actions,
) {
    let Some(habit_id) = app.progress_modal_for_habit_id else {
        return;
    };

    // Block all input while the log request is in-flight.
    if states.log_habit_state.is_habit_logging(habit_id) {
        return;
    }

    let code = key_event.code;

    match code {
        KeyCode::Esc => {
            app.hide_log_progress_modal();
            states.modal_state.reset();
            return;
        }
        KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.quit();
            return;
        }
        _ => {}
    }

    // Left/Right navigate the button row (right-to-left layout: Log on right, Cancel on left).
    if matches!(code, KeyCode::Left) {
        states.modal_state.progress_modal_state_mut().next_button();
        return;
    }
    if matches!(code, KeyCode::Right) {
        states.modal_state.progress_modal_state_mut().prev_button();
        return;
    }

    if code == KeyCode::Enter {
        let selected = states
            .modal_state
            .progress_modal_state_mut()
            .selected_button();
        if selected == 1 {
            // Cancel
            app.hide_log_progress_modal();
            states.modal_state.reset();
        } else {
            // Log
            let progress_value = states
                .modal_state
                .progress_modal_state_mut()
                .progress_input_state_mut()
                .get_value()
                .parse::<i32>()
                .unwrap_or(0);
            let daily_goal = app
                .habits
                .iter()
                .find(|h| h.id == habit_id)
                .map(|h| h.daily_goal)
                .unwrap_or(0);
            let completed = if daily_goal > 0 {
                if progress_value >= daily_goal { 1 } else { 0 }
            } else {
                if progress_value > 0 { 1 } else { 0 }
            };
            actions.log_habit(habit_id, completed, progress_value);
        }
        return;
    }

    // Forward numeric keypresses to the progress input.
    let progress_input = states
        .modal_state
        .progress_modal_state_mut()
        .progress_input_state_mut();
    let cursor_delay = progress_input.get_cursor_visibility_delay();
    if cursor_delay < CURSOR_DELAY_THRESHOLD {
        progress_input.set_cursor_visibility_delay(cursor_delay + CURSOR_TYPING_DELAY);
    }
    if is_numeric_keycode(code) {
        if let Some(c) = get_character_from_keycode(code) {
            progress_input.push_char(c);
        }
    } else if is_backspace_code(code) {
        progress_input.backspace();
    }
}
