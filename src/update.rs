use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    widgets::ListState,
};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent, list_state: &mut ListState) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => app.quit(),
        KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.quit();
        }
        KeyCode::Right | KeyCode::Char('j') => list_state.select_next(),
        KeyCode::Left | KeyCode::Char('k') => list_state.select_previous(),
        KeyCode::Char('l') => app.increment_counter(),
        KeyCode::Char('h') => app.decrement_counter(),
        _ => {}
    }
}
