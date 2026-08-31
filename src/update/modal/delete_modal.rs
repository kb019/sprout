use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::App, controller::Actions, state::States};

#[allow(clippy::needless_pass_by_ref_mut)]
#[allow(clippy::needless_return)]
pub fn handle_delete_modal(
    app: &mut App,
    key_event: KeyEvent,
    states: &mut States,
    actions: &Actions,
) {
    let Some(habit_id) = app.display_delete_modal else {
        return;
    };

    if states.modal_state.delete_modal_state_mut().is_deleting() {
        return;
    }

    let code = key_event.code;

    match code {
        KeyCode::Esc => {
            app.hide_delete_modal();
            states.modal_state.reset();
            return;
        }
        KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.quit();
            return;
        }
        _ => {}
    }

    // Left/Right navigate Confirm / Cancel (right-to-left layout).
    if matches!(code, KeyCode::Left) {
        states.modal_state.delete_modal_state_mut().next_button();
        return;
    }
    if matches!(code, KeyCode::Right) {
        states.modal_state.delete_modal_state_mut().prev_button();
        return;
    }

    if code == KeyCode::Enter {
        let selected = states
            .modal_state
            .delete_modal_state_mut()
            .selected_button();
        if selected == 1 {
            // Cancel
            app.hide_delete_modal();
            states.modal_state.reset();
        } else {
            // Confirm delete
            states
                .modal_state
                .delete_modal_state_mut()
                .set_is_deleting(true);
            actions.delete_habit(habit_id);
        }
    }
}
