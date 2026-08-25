pub mod app;
pub mod modal;

use ratatui::crossterm::event::KeyEvent;

use crate::app::App;
use crate::state::app::AppState;
use crate::state::modal::ModalState;

pub fn handle(
    app: &mut App,
    key_event: KeyEvent,
    app_state: &mut AppState,
    modal_state: &mut ModalState,
) {
    if app.is_modal_in_focus() {
        modal::handle_modal(app, key_event, modal_state);
    } else {
        app::handle_app(app, key_event, app_state);
    }
}
