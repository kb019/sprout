use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;
use crate::controller::Actions;
use crate::state::States;
use crate::utils::{is_down_key, is_left_key, is_right_key, is_up_key};

#[allow(clippy::needless_return)]
pub fn handle_app(app: &mut App, key_event: KeyEvent, states: &mut States, actions: &Actions) {
    if app.is_modal_in_focus() {
        return;
    }
    let code = key_event.code;
    let state = &mut states.app_state;

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
            if state.heatmap_tile_state().selected().unwrap_or(0) == 0 {
                app.focus_menu();
            } else {
                state.prev_heatmap_tile(app.habits.len());
            }
        }
        return;
    }

    if app.is_dashboard_in_focus {
        if is_left_key(code) {
            app.focus_menu();
        } else if is_up_key(code) {
            state.prev_dashboard_habit(app.habits.len());
        } else if is_down_key(code) {
            state.next_dashboard_habit(app.habits.len());
        } else if is_right_key(code) {
            app.focus_goal_progress();
        } else if matches!(code, KeyCode::Char('+')) {
            app.show_add_modal();
        } else if matches!(code, KeyCode::Char('e' | 'E')) {
            let current_habit_index = state.dashboard_habits_state().selected();
            if let Some(habit_index) = current_habit_index
                && habit_index < app.habits.len()
            {
                let habit = app.habits[habit_index].clone();
                app.show_edit_modal(habit.id);
                states.modal_state.edit_modal_state_mut().initialize(&habit);
            }
        } else if matches!(code, KeyCode::Enter) {
            //check if the habit is selected or hughlighlted currently
            let current_habit_index = state.dashboard_habits_state().selected();
            if let Some(habit_index) = current_habit_index
                && habit_index < app.habits.len()
            {
                let habit = &app.habits[habit_index];
                let should_show_progress_modal = habit.monthly_goal > 0
                    || habit.yearly_goal > 0
                    || habit.weekly_goal > 0
                    || habit.daily_goal > 0;
                let is_habit_currently_logging = states.log_habit_state.is_habit_logging(habit.id);
                if !should_show_progress_modal && !is_habit_currently_logging {
                    let is_completed = app.completed_habits.contains(&habit.id);
                    actions.log_habit(habit.id, if is_completed { 0 } else { 1 }, 0);
                }
                if should_show_progress_modal && !is_habit_currently_logging {
                    app.show_log_progress_modal(habit.id);
                }
            }
        } else if matches!(code, KeyCode::Char('x' | 'X')) {
            let current_habit_index = state.dashboard_habits_state().selected();
            if let Some(habit_index) = current_habit_index
                && habit_index < app.habits.len()
            {
                let habit = &app.habits[habit_index];
                app.show_delete_modal(habit.id);
            }
        }
        return;
    }

    if app.is_settings_in_focus {
        if is_up_key(code) {
            state.prev_settings(4);
        } else if is_down_key(code) {
            state.next_settings(4);
        } else if is_right_key(code) {
            let row = state.settings_state().selected().unwrap_or(0);
            let len = match row {
                0 => app.themes.len(),
                1 => app.menu.len().saturating_sub(1),
                2 => 2,
                _ => 1,
            };
            if len > 0 {
                state.prev_settings_tile(row, len);
                if row == 0 {
                    app.active_theme = state.active_theme();
                } else if row == 2 {
                    app.cursor_blink_enabled = state.settings_tile_selected(2) == Some(0);
                }
            }
        } else if is_left_key(code) {
            let row = state.settings_state().selected().unwrap_or(0);
            let len = match row {
                0 => app.themes.len(),
                1 => app.menu.len().saturating_sub(1),
                2 => 2,
                _ => 1,
            };
            if len > 0 && state.settings_tile_selected(row) == Some(len - 1) {
                state.clear_settings();
                app.focus_menu();
            } else if len > 0 {
                state.next_settings_tile(row, len);
                if row == 0 {
                    app.active_theme = state.active_theme();
                } else if row == 2 {
                    app.cursor_blink_enabled = state.settings_tile_selected(2) == Some(0);
                }
            }
        } else if matches!(code, KeyCode::Char('m' | 'M')) {
            state.clear_settings();
        }
        return;
    }

    if app.is_goal_progress_in_focus {
        let options_len = app.goal_progress_options.len();
        if is_right_key(code) {
            state.next_goal_progress(options_len);
        } else if is_left_key(code) {
            if state.goal_progress_tile_state().selected().unwrap_or(0) == 0 {
                app.focus_dashboard();
            } else {
                state.prev_goal_progress();
            }
        } else if is_down_key(code) {
            let tab = state.goal_progress_tile_state().selected().unwrap_or(0);
            let len = match tab {
                0 => app.daily_progress.len(),
                1 => app.weekly_progress.len(),
                2 => app.monthly_progress.len(),
                3 => app.yearly_progress.len(),
                _ => 0,
            };
            state.next_goal_progress_row(tab, len);
        } else if is_up_key(code) {
            let tab = state.goal_progress_tile_state().selected().unwrap_or(0);
            state.prev_goal_progress_row(tab);
        }
        return;
    }
}
