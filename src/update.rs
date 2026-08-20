use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;
use crate::state::State;

pub fn update(app: &mut App, key_event: KeyEvent, state: &mut State) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => app.quit(),
        KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => app.quit(),
        KeyCode::Char('j') | KeyCode::Char('J') => state.next_menu(app.menu.len()),
        KeyCode::Char('k') | KeyCode::Char('K') => state.prev_menu(app.menu.len()),
        KeyCode::Down => match state.menu_state().selected() {
            Some(1) => state.next_tile(app.habits.len()),
            Some(3) => state.next_settings(3),
            _ => {}
        },
        KeyCode::Up => match state.menu_state().selected() {
            Some(1) => state.prev_tile(app.habits.len()),
            Some(3) => state.prev_settings(3),
            _ => {}
        },
        KeyCode::Right => {
            if state.menu_state().selected() == Some(3) {
                let row = state.settings_state().selected().unwrap_or(0);
                let len = match row {
                    0 => app.themes.len(),
                    1 => app.menu.len(),
                    _ => 1,
                };
                if len > 0 {
                    state.prev_settings_tile(row, len);
                    if row == 0 {
                        app.active_theme = state.active_theme();
                    }
                }
            }
        }
        KeyCode::Left => {
            if state.menu_state().selected() == Some(3) {
                let row = state.settings_state().selected().unwrap_or(0);
                let len = match row {
                    0 => app.themes.len(),
                    1 => app.menu.len(),
                    _ => 1,
                };
                if len > 0 {
                    state.next_settings_tile(row, len);
                    if row == 0 {
                        app.active_theme = state.active_theme();
                    }
                }
            }
        }
        KeyCode::Char('l') => app.increment_counter(),
        KeyCode::Char('h') => app.decrement_counter(),
        _ => {}
    }
}
