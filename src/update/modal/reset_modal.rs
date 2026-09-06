use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::App, controller::Actions, state::States, widgets::notifier::Notifier};

#[allow(clippy::needless_pass_by_ref_mut)]
#[allow(clippy::needless_return)]
pub fn handle_reset_modal(
    app: &mut App,
    key_event: KeyEvent,
    states: &mut States,
    actions: &Actions,
    _notifier: &mut Notifier,
) {
    if !app.display_reset_modal {
        return;
    }

    if states.modal_state.reset_modal_state_mut().is_resetting() {
        return;
    }

    let code = key_event.code;

    match code {
        KeyCode::Esc => {
            app.hide_reset_modal();
            states.modal_state.reset();
            return;
        }
        KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.quit();
            return;
        }
        _ => {}
    }

    if matches!(code, KeyCode::Left) {
        states.modal_state.reset_modal_state_mut().next_button();
        return;
    }
    if matches!(code, KeyCode::Right) {
        states.modal_state.reset_modal_state_mut().prev_button();
        return;
    }

    if code == KeyCode::Enter {
        let selected = states.modal_state.reset_modal_state_mut().selected_button();
        if selected == 1 {
            app.hide_reset_modal();
            states.modal_state.reset();
        } else {
            states
                .modal_state
                .reset_modal_state_mut()
                .set_is_resetting(true);
            actions.reset_all();
        }
    }
}
