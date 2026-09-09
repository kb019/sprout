use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;
use crate::constants::SETTINGS_SAVE_DELAY_TICKS;
use crate::controller::Actions;
use crate::state::States;
use crate::state::app::AppState;
use crate::state::habit::heatmap_data_state::HeatmapDataState;
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
        } else if matches!(code, KeyCode::Tab) {
            match state.menu_state().selected() {
                Some(0) => app.focus_dashboard(state.dashboard_habits_state_mut()),
                Some(1) => {
                    let year_idx = state.year_list_state().selected().unwrap_or(0);
                    let year = app
                        .heatmap_year_habits
                        .get(year_idx)
                        .map(|(y, _)| *y)
                        .unwrap_or(0);
                    if let Some(tile_state) = state.heatmap_tile_state_for_year_mut(year) {
                        app.focus_heatmap(tile_state);
                    }
                    fetch_heatmap_data_if_needed(
                        app,
                        state,
                        year_idx,
                        year,
                        actions,
                        states.heatmap_year_habits_state.is_fetching(),
                        &states.heatmap_data_state,
                    );
                }
                Some(2) => app.focus_streak_leaderboard(state.streak_leaderboard_state_mut()),
                Some(3) => app.focus_settings(state.settings_state_mut()),
                _ => {}
            }
        } else if matches!(code, KeyCode::BackTab) {
            match state.menu_state().selected() {
                Some(0) => app.focus_best_streaks(state.best_streaks_list_state_mut()),
                Some(1) => {
                    let year_idx = state.year_list_state().selected().unwrap_or(0);
                    let year = app
                        .heatmap_year_habits
                        .get(year_idx)
                        .map(|(y, _)| *y)
                        .unwrap_or(0);
                    if let Some(tile_state) = state.heatmap_tile_state_for_year_mut(year) {
                        app.focus_heatmap(tile_state);
                    }
                    fetch_heatmap_data_if_needed(
                        app,
                        state,
                        year_idx,
                        year,
                        actions,
                        states.heatmap_year_habits_state.is_fetching(),
                        &states.heatmap_data_state,
                    );
                }
                Some(2) => app.focus_streak_leaderboard(state.streak_leaderboard_state_mut()),
                Some(3) => app.focus_settings(state.settings_state_mut()),
                _ => {}
            }
        }
        return;
    }

    if app.is_streak_leaderboard_in_focus {
        if is_down_key(code) {
            state.next_streak_leaderboard(app.habits.len());
        } else if is_up_key(code) {
            state.prev_streak_leaderboard();
        } else if matches!(code, KeyCode::Tab | KeyCode::BackTab) {
            app.focus_menu();
        }
        return;
    }

    if app.is_best_streaks_in_focus {
        if is_down_key(code) {
            state.next_best_streak(app.best_streaks.len());
        } else if is_up_key(code) {
            state.prev_best_streak();
        } else if matches!(code, KeyCode::BackTab) {
            app.focus_goal_progress(state.goal_progress_tile_state_mut());
        } else if matches!(code, KeyCode::Tab) {
            app.focus_menu();
        }
        return;
    }

    if app.is_heatmap_in_focus {
        if states.heatmap_year_habits_state.is_fetching() {
            return;
        }
        let year_idx = state.year_list_state().selected().unwrap_or(0);
        let (year, habit_count) = app
            .heatmap_year_habits
            .get(year_idx)
            .map(|(y, ids)| (*y, ids.len()))
            .unwrap_or((0, 0));
        if is_right_key(code) {
            state.next_heatmap_tile(year, habit_count);
        } else if is_left_key(code) {
            state.prev_heatmap_tile(year);
        } else if is_down_key(code) {
            state.next_year(app.heatmap_year_habits.len());
        } else if is_up_key(code) {
            state.prev_year();
        } else if matches!(code, KeyCode::Tab | KeyCode::BackTab) {
            app.focus_menu();
        }
        if is_right_key(code) || is_left_key(code) || is_down_key(code) || is_up_key(code) {
            let new_yr_idx = state.year_list_state().selected().unwrap_or(0);
            let new_yr = app
                .heatmap_year_habits
                .get(new_yr_idx)
                .map(|(y, _)| *y)
                .unwrap_or(0);
            fetch_heatmap_data_if_needed(
                app,
                state,
                new_yr_idx,
                new_yr,
                actions,
                states.heatmap_year_habits_state.is_fetching(),
                &states.heatmap_data_state,
            );
        }
        return;
    }

    if app.is_dashboard_in_focus {
        if is_up_key(code) {
            state.prev_dashboard_habit(app.habits.len());
        } else if is_down_key(code) {
            state.next_dashboard_habit(app.habits.len());
        } else if matches!(code, KeyCode::Tab) {
            app.focus_goal_progress(state.goal_progress_tile_state_mut());
        } else if matches!(code, KeyCode::BackTab) {
            app.focus_menu();
        } else if matches!(code, KeyCode::Char('a' | 'A')) {
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
        } else if matches!(code, KeyCode::Char('d' | 'D')) {
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
                1 => 2,
                2 => 3,
                _ => 1,
            };
            if len > 0 {
                state.prev_settings_tile(row, len);
                if row == 0 {
                    app.active_theme = state.active_theme();
                } else if row == 1 {
                    app.cursor_blink_enabled = state.settings_tile_selected(1) == Some(0);
                } else if row == 2 {
                    app.notification_level = state.settings_tile_selected(2).unwrap_or(0);
                }
                if row < 3 {
                    app.settings_save_delay = SETTINGS_SAVE_DELAY_TICKS;
                    states.settings_save_state.start_saving();
                }
            }
        } else if is_left_key(code) {
            let row = state.settings_state().selected().unwrap_or(0);
            let len = match row {
                0 => app.themes.len(),
                1 => 2,
                2 => 3,
                _ => 1,
            };
            if len > 0 {
                state.next_settings_tile(row, len);
                if row == 0 {
                    app.active_theme = state.active_theme();
                } else if row == 1 {
                    app.cursor_blink_enabled = state.settings_tile_selected(1) == Some(0);
                } else if row == 2 {
                    app.notification_level = state.settings_tile_selected(2).unwrap_or(0);
                }
                if row < 3 {
                    app.settings_save_delay = SETTINGS_SAVE_DELAY_TICKS;
                    states.settings_save_state.start_saving();
                }
            }
        } else if matches!(code, KeyCode::Enter) {
            let row = state.settings_state().selected().unwrap_or(0);
            if row == 3 {
                app.show_reset_modal();
            }
        } else if matches!(code, KeyCode::Char('m' | 'M')) {
            state.clear_settings();
        } else if matches!(code, KeyCode::Tab | KeyCode::BackTab) {
            app.focus_menu();
        }
        return;
    }

    if app.is_goal_progress_in_focus {
        let tab = state.goal_progress_tile_state().selected().unwrap_or(0);
        let len = match tab {
            0 => app.habits.iter().filter(|h| h.daily_goal > 0).count(),
            1 => app.habits.iter().filter(|h| h.weekly_goal > 0).count(),
            2 => app.habits.iter().filter(|h| h.monthly_goal > 0).count(),
            3 => app.habits.iter().filter(|h| h.yearly_goal > 0).count(),
            _ => 0,
        };
        if is_right_key(code) {
            state.next_goal_progress(4);
        } else if is_left_key(code) {
            state.prev_goal_progress();
        } else if is_down_key(code) {
            state.next_goal_progress_row(tab, len);
        } else if is_up_key(code) {
            state.prev_goal_progress_row(tab);
        } else if matches!(code, KeyCode::Tab) {
            app.focus_best_streaks(state.best_streaks_list_state_mut());
        } else if matches!(code, KeyCode::BackTab) {
            app.focus_dashboard(state.dashboard_habits_state_mut());
        }
        return;
    }
}

/// Dispatch a heatmap-data fetch for the currently selected habit+year if the
/// data isn't already cached and no year-habits fetch is in flight.
fn fetch_heatmap_data_if_needed(
    app: &App,
    app_state: &AppState,
    year_idx: usize,
    year: i32,
    actions: &Actions,
    year_habits_is_fetching: bool,
    heatmap_data_state: &HeatmapDataState,
) {
    if year_habits_is_fetching {
        return;
    }
    let tile_idx = app_state
        .heatmap_tile_state_for_year(year)
        .and_then(|ts| ts.selected())
        .unwrap_or(0);
    let habit_id = app
        .heatmap_year_habits
        .get(year_idx)
        .and_then(|(_, ids)| ids.get(tile_idx))
        .copied();
    if let Some(id) = habit_id
        && !app.heatmap_data.contains_key(&(id, year))
        && !heatmap_data_state.is_fetching(id, year)
    {
        actions.fetch_heatmap_data(id, year);
    }
}
