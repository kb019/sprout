use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::App,
    state::State,
    utils::{is_left_key, is_right_key},
};

pub fn handle_modal(app: &mut App, key_event: KeyEvent, state: &mut State) {
    let code = key_event.code;

    match code {
        KeyCode::Esc => {
            app.hide_all_modals();
            return;
        }
        KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.quit();
            return;
        }
        _ => {}
    }

    if is_left_key(code) {
        state.next_modal_button();
    } else if is_right_key(code) {
        state.prev_modal_button();
    }
}
