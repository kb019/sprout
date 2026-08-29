mod add_modal;

use crossterm::event::KeyEvent;

use crate::app::App;
use crate::controller::Actions;
use crate::state::States;

#[allow(clippy::needless_return)]
pub fn handle_modal(app: &mut App, key_event: KeyEvent, states: &mut States, actions: &Actions) {
    if !app.is_modal_in_focus() {
        return;
    }
    add_modal::handle_add_modal(app, key_event, states, actions);
}
