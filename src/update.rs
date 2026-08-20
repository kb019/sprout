use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;
use crate::state::State;

// Explicit returns ensure that adding code later in this function
// does not accidentally change the control flow of the current focus handling.
#[allow(clippy::needless_return)]
pub fn update(app: &mut App, key_event: KeyEvent, state: &mut State) {
    let code = key_event.code;

    match code {
        KeyCode::Esc | KeyCode::Char('q' | 'Q') => {
            app.quit();
            return;
        }
        KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.quit();
            return;
        }
        _ => {}
    }

    if app.is_menu_in_focus {
        if is_up_key(code) {
            state.prev_menu(app.menu.len());
        } else if is_down_key(code) {
            state.next_menu(app.menu.len());
        } else if is_right_key(code) {
            match state.menu_state().selected() {
                Some(0) => app.focus_dashboard(),
                Some(1) => app.focus_heatmap(),
                Some(3) => app.focus_settings(),
                _ => {}
            }
        }
        return;
    }

    if app.is_heatmap_in_focus {
        if is_right_key(code) {
            state.next_heatmap_tile(app.habits.len());
        } else if is_left_key(code) {
            state.prev_heatmap_tile(app.habits.len());
        } else if matches!(code, KeyCode::Char('m' | 'M')) {
            app.focus_menu();
        }
        return;
    }

    if app.is_dashboard_in_focus {
        if is_left_key(code) {
            app.focus_menu();
        }
        return;
    }

    if app.is_settings_in_focus {
        if is_up_key(code) {
            state.prev_settings(3);
        } else if is_down_key(code) {
            state.next_settings(3);
        } else if is_right_key(code) || is_left_key(code) {
            let row = state.settings_state().selected().unwrap_or(0);
            let len = match row {
                0 => app.themes.len(),
                1 => app.menu.len(),
                _ => 1,
            };
            if len > 0 {
                if is_right_key(code) {
                    state.prev_settings_tile(row, len);
                } else {
                    state.next_settings_tile(row, len);
                }
                if row == 0 {
                    app.active_theme = state.active_theme();
                }
            }
        } else if matches!(code, KeyCode::Char('m' | 'M')) {
            app.focus_menu();
            state.clear_settings();
        }
        return;
    }
}

fn is_right_key(code: KeyCode) -> bool {
    matches!(
        code,
        KeyCode::Char('l') | KeyCode::Char('L') | KeyCode::Right
    )
}

fn is_left_key(code: KeyCode) -> bool {
    matches!(
        code,
        KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Left
    )
}

fn is_up_key(code: KeyCode) -> bool {
    matches!(code, KeyCode::Char('k') | KeyCode::Char('K') | KeyCode::Up)
}

fn is_down_key(code: KeyCode) -> bool {
    matches!(
        code,
        KeyCode::Char('j') | KeyCode::Char('J') | KeyCode::Down
    )
}
