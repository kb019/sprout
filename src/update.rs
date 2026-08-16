use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    widgets::ListState,
};

use crate::app::App;

pub fn update(
    app: &mut App,
    key_event: KeyEvent,
    list_state: &mut ListState,
    tile_state: &mut ListState,
) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => app.quit(),
        KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.quit();
        }
        KeyCode::Char('j') | KeyCode::Char('J') => {
            if list_state.selected().is_some() {
                let current_index = list_state.selected().unwrap();
                let next_index = (current_index + 1) % app.menu.len();
                list_state.select(Some(next_index));
            }
        }
        KeyCode::Char('k') | KeyCode::Char('K') => {
            if list_state.selected().is_some() {
                let current_index = list_state.selected().unwrap();
                let prev_index = if current_index == 0 {
                    app.menu.len() - 1
                } else {
                    current_index - 1
                };
                list_state.select(Some(prev_index));
            }
        }
        KeyCode::Up => {
            if tile_state.selected().is_some() {
                let current_index = tile_state.selected().unwrap();
                let prev_index = if current_index == 0 {
                    app.habits.len() - 1
                } else {
                    current_index - 1
                };
                tile_state.select(Some(prev_index));
            }
        }
        KeyCode::Down => {
            if tile_state.selected().is_some() {
                let current_index = tile_state.selected().unwrap();
                let next_index = (current_index + 1) % app.habits.len();
                tile_state.select(Some(next_index));
            }
        }
        KeyCode::Char('l') => app.increment_counter(),
        KeyCode::Char('h') => app.decrement_counter(),
        _ => {}
    }
}
