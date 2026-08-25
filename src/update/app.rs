use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;
use crate::state::app::AppState;
use crate::utils::{is_down_key, is_left_key, is_right_key, is_up_key};

// Explicit returns ensure that adding code later in this function
// does not accidentally change the control flow of the current focus handling.
#[allow(clippy::needless_return)]
pub fn handle_app(app: &mut App, key_event: KeyEvent, state: &mut AppState) {
    if app.is_modal_in_focus() {
        return;
    }
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
        KeyCode::Char('m' | 'M') => {
            app.focus_menu();
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
                Some(0) => {
                    app.focus_dashboard();
                }
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
            if state.heatmap_tile_state().selected().unwrap_or(0) == 0 {
                app.focus_menu();
            } else {
                state.prev_heatmap_tile(app.habits.len());
            }
            state.prev_heatmap_tile(app.habits.len());
        }
        return;
    }

    if app.is_dashboard_in_focus {
        if is_left_key(code) {
            app.focus_menu();
        } else if is_up_key(code) {
            state.prev_dashboard_habit(3);
        } else if is_down_key(code) {
            state.next_dashboard_habit(3);
        } else if is_right_key(code) {
            app.focus_goal_progress();
        } else if matches!(code, KeyCode::Char('+')) {
            app.show_add_modal();
        }
        return;
    }

    if app.is_settings_in_focus {
        if is_up_key(code) {
            state.prev_settings(3);
        } else if is_down_key(code) {
            state.next_settings(3);
        }
        //The lft and right key logic can be combined to one and logic seems repetitive, but separating will make it easier to read and understand the logic.
        else if is_right_key(code) {
            let row = state.settings_state().selected().unwrap_or(0);
            let len = match row {
                0 => app.themes.len(),
                1 => app.menu.len().saturating_sub(1),
                _ => 1,
            };
            if len > 0 {
                state.prev_settings_tile(row, len);
                if row == 0 {
                    app.active_theme = state.active_theme();
                }
            }
        } else if is_left_key(code) {
            let row = state.settings_state().selected().unwrap_or(0);
            let len = match row {
                0 => app.themes.len(),
                1 => app.menu.len().saturating_sub(1),
                _ => 1,
            };
            if len > 0 && state.settings_tile_selected(row) == Some(len - 1) {
                state.clear_settings();
                app.focus_menu();
            } else if len > 0 {
                state.next_settings_tile(row, len);
                if row == 0 {
                    app.active_theme = state.active_theme();
                }
            }
        } else if matches!(code, KeyCode::Char('m' | 'M')) {
            state.clear_settings();
        }
        return;
    }

    if app.is_goal_progress_in_focus {
        if is_right_key(code) {
            state.next_goal_progress(app.goal_progress_options.len());
        } else if is_left_key(code) {
            if state.goal_progress_tile_state().selected().unwrap_or(0) == 0 {
                app.focus_dashboard();
            } else {
                state.prev_goal_progress();
            }
        } else if is_down_key(code) {
            let tab = state.goal_progress_tile_state().selected().unwrap_or(0);
            state.next_goal_progress_row(tab, app.goal_progress_options.len());
        } else if is_up_key(code) {
            let tab = state.goal_progress_tile_state().selected().unwrap_or(0);
            state.prev_goal_progress_row(tab);
        }
        return;
    }
}
