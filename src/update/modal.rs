mod add_modal;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::App, state::modal::ModalState};

#[allow(clippy::needless_return)]
pub fn handle_modal(app: &mut App, key_event: KeyEvent, state: &mut ModalState) {
    if !app.is_modal_in_focus() {
        return;
    }
    let code = key_event.code;
    add_modal::handle_add_modal(app, key_event, state);
    match code {
        KeyCode::Esc => {
            app.hide_all_modals();
            state.reset(); //reset button state
            return;
        }
        KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.quit();
            return;
        }
        _ => {}
    }
}
