use std::collections::{HashMap, HashSet};

use ratatui::widgets::ListState;

use crate::constants::{CURSOR_BLINK_TICKS, PROGRESS_LOAD_TICKS};
use crate::model::habit::{BestStreak, Habit};
use crate::palette::Palette;
use crate::sprout::{SproutPoints, generate_sprout_points};

/// Application.
#[derive(Debug, Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// counter
    pub counter: u8,
    /// sprout points for each percentage
    pub sprout_percentage_points: SproutPercentagePoints,
    /// menu items
    pub menu: Vec<&'static str>,

    pub habits: Vec<Habit>,

    pub best_streaks: Vec<BestStreak>,

    pub active_days: u32,

    /// Years that have habit log data (descending) with the habit IDs active in each year.
    pub heatmap_year_habits: Vec<(i32, Vec<i32>)>,

    /// Cached heatmap cell data keyed by (habit_id, year) → date → intensity 0-4.
    pub heatmap_data: HashMap<(i32, i32), HashMap<String, u8>>,

    pub weekly_completion: [u32; 7],

    pub completed_habits: HashSet<i32>,

    pub active_streaks: HashMap<i32, i32>,

    pub daily_progress: HashMap<i32, i32>,
    pub weekly_progress: HashMap<i32, i32>,
    pub monthly_progress: HashMap<i32, i32>,
    pub yearly_progress: HashMap<i32, i32>,

    pub themes: Vec<&'static str>,

    pub goal_progress_options: Vec<&'static str>,

    pub active_theme: usize,

    pub is_menu_in_focus: bool,

    pub is_dashboard_in_focus: bool,

    pub is_heatmap_in_focus: bool,

    pub is_settings_in_focus: bool,

    pub is_goal_progress_in_focus: bool,

    pub is_best_streaks_in_focus: bool,

    pub is_streak_leaderboard_in_focus: bool,

    pub display_add_modal: bool,

    pub display_delete_modal: Option<i32>,

    pub display_edit_modal: Option<i32>,

    pub display_reset_modal: bool,

    pub best_streak_refresh_delay: u32,
    pub active_days_refresh_delay: u32,
    pub weekly_average_refresh_delay: u32,
    pub heatmap_year_habits_refresh_delay: u32,

    // 0 = All, 1 = Errors only, 2 = Off
    pub notification_level: usize,

    pub tick_count: u64,

    pub progress_tick_count: u64,

    pub cursor_blink_enabled: bool,

    pub progress_modal_for_habit_id: Option<i32>,
}

#[derive(Debug, Default)]
pub struct SproutPercentagePoints {
    pub five_percent_points: Option<SproutPoints>,
    pub ten_percent_points: Option<SproutPoints>,
    pub fifteen_percent_points: Option<SproutPoints>,
    pub twenty_percent_points: Option<SproutPoints>,
    pub twenty_five_percent_points: Option<SproutPoints>,
    pub thirty_percent_points: Option<SproutPoints>,
    pub thirty_five_percent_points: Option<SproutPoints>,
    pub forty_percent_points: Option<SproutPoints>,
    pub forty_five_percent_points: Option<SproutPoints>,
    pub fifty_percent_points: Option<SproutPoints>,
    pub fifty_five_percent_points: Option<SproutPoints>,
    pub sixty_percent_points: Option<SproutPoints>,
    pub sixty_five_percent_points: Option<SproutPoints>,
    pub seventy_percent_points: Option<SproutPoints>,
    pub seventy_five_percent_points: Option<SproutPoints>,
    pub eighty_percent_points: Option<SproutPoints>,
    pub eighty_five_percent_points: Option<SproutPoints>,
    pub ninety_percent_points: Option<SproutPoints>,
    pub ninety_five_percent_points: Option<SproutPoints>,
    pub hundred_percent_points: Option<SproutPoints>,
}

impl App {
    pub const APP_NAME: &'static str = "Sprout";

    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let mut app = Self::default();
        for pct in (5u8..=100).step_by(5) {
            app.set_sprout_points(pct, generate_sprout_points(pct));
        }
        app.is_menu_in_focus = true;
        app.menu = vec!["Dashboard", "Heatmap", "Stats", "Settings"];
        app.habits = vec![];
        app.themes = vec!["Sprout", "Amber", "Mono"];
        app.goal_progress_options = vec!["Daily", "Weekly", "Monthly", "Yearly"];
        app.cursor_blink_enabled = true;
        app.notification_level = 1;
        app
    }

    pub fn show_log_progress_modal(&mut self, habit_id: i32) {
        self.progress_modal_for_habit_id = Some(habit_id);
    }

    pub fn is_modal_in_focus(&self) -> bool {
        self.display_add_modal
            || self.display_delete_modal.is_some()
            || self.display_edit_modal.is_some()
            || self.display_reset_modal
            || self.progress_modal_for_habit_id.is_some()
    }

    pub fn hide_log_progress_modal(&mut self) {
        self.progress_modal_for_habit_id = None;
    }

    pub fn show_edit_modal(&mut self, habit_id: i32) {
        self.display_edit_modal = Some(habit_id);
    }

    pub fn hide_edit_modal(&mut self) {
        self.display_edit_modal = None;
    }

    pub fn show_delete_modal(&mut self, habit_id: i32) {
        self.display_delete_modal = Some(habit_id);
    }

    pub fn hide_delete_modal(&mut self) {
        self.display_delete_modal = None;
    }

    pub fn show_reset_modal(&mut self) {
        self.display_reset_modal = true;
    }

    pub fn hide_reset_modal(&mut self) {
        self.display_reset_modal = false;
    }

    pub fn show_add_modal(&mut self) {
        self.display_add_modal = true;
    }

    pub fn hide_add_modal(&mut self) {
        self.display_add_modal = false;
    }

    pub fn hide_all_modals(&mut self) {
        self.display_add_modal = false;
        self.display_delete_modal = None;
        self.progress_modal_for_habit_id = None;
        self.display_edit_modal = None;
        self.display_reset_modal = false;
    }

    pub fn focus_menu(&mut self) {
        self.remove_all_focus();
        self.is_menu_in_focus = true;
    }
    /// Sets the focus to the dashboard.
    pub fn focus_dashboard(&mut self, list_state: &mut ListState) {
        self.remove_all_focus();
        self.is_dashboard_in_focus = true;
        if list_state.selected().is_none() {
            list_state.select(Some(0));
        }
    }

    /// Sets the focus to the heatmap.
    pub fn focus_heatmap(&mut self, list_state: &mut ListState) {
        self.remove_all_focus();
        self.is_heatmap_in_focus = true;
        if list_state.selected().is_none() {
            list_state.select(Some(0));
        }
    }

    /// Sets the focus to the settings.
    pub fn focus_settings(&mut self, list_state: &mut ListState) {
        self.remove_all_focus();
        self.is_settings_in_focus = true;
        if list_state.selected().is_none() {
            list_state.select(Some(0));
        }
    }

    /// Sets the focus to the goal progress.
    pub fn focus_goal_progress(&mut self, list_state: &mut ListState) {
        self.remove_all_focus();
        self.is_goal_progress_in_focus = true;
        if list_state.selected().is_none() {
            list_state.select(Some(0));
        }
    }

    pub fn focus_best_streaks(&mut self, list_state: &mut ListState) {
        self.remove_all_focus();
        self.is_best_streaks_in_focus = true;
        if list_state.selected().is_none() {
            list_state.select(Some(0));
        }
    }

    pub fn focus_streak_leaderboard(&mut self, list_state: &mut ListState) {
        self.remove_all_focus();
        self.is_streak_leaderboard_in_focus = true;
        if list_state.selected().is_none() {
            list_state.select(Some(0));
        }
    }

    pub fn remove_all_focus(&mut self) {
        self.is_dashboard_in_focus = false;
        self.is_heatmap_in_focus = false;
        self.is_settings_in_focus = false;
        self.is_menu_in_focus = false;
        self.is_goal_progress_in_focus = false;
        self.is_best_streaks_in_focus = false;
        self.is_streak_leaderboard_in_focus = false;
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.tick_count = self.tick_count.saturating_add(1);
        self.tick_count %= CURSOR_BLINK_TICKS;
        self.progress_tick_count = self.progress_tick_count.saturating_add(1);
        self.progress_tick_count %= PROGRESS_LOAD_TICKS;
    }

    pub fn palette(&self) -> Palette {
        Palette::from_index(self.active_theme)
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(5) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(5) {
            self.counter = res;
        }
    }

    /// Get cached sprout points for a given percentage, if they have been set.
    pub fn get_sprout_points(&self, percentage: u8) -> Option<&SproutPoints> {
        let opt = match percentage {
            5 => &self.sprout_percentage_points.five_percent_points,
            10 => &self.sprout_percentage_points.ten_percent_points,
            15 => &self.sprout_percentage_points.fifteen_percent_points,
            20 => &self.sprout_percentage_points.twenty_percent_points,
            25 => &self.sprout_percentage_points.twenty_five_percent_points,
            30 => &self.sprout_percentage_points.thirty_percent_points,
            35 => &self.sprout_percentage_points.thirty_five_percent_points,
            40 => &self.sprout_percentage_points.forty_percent_points,
            45 => &self.sprout_percentage_points.forty_five_percent_points,
            50 => &self.sprout_percentage_points.fifty_percent_points,
            55 => &self.sprout_percentage_points.fifty_five_percent_points,
            60 => &self.sprout_percentage_points.sixty_percent_points,
            65 => &self.sprout_percentage_points.sixty_five_percent_points,
            70 => &self.sprout_percentage_points.seventy_percent_points,
            75 => &self.sprout_percentage_points.seventy_five_percent_points,
            80 => &self.sprout_percentage_points.eighty_percent_points,
            85 => &self.sprout_percentage_points.eighty_five_percent_points,
            90 => &self.sprout_percentage_points.ninety_percent_points,
            95 => &self.sprout_percentage_points.ninety_five_percent_points,
            _ => &self.sprout_percentage_points.hundred_percent_points,
        };
        opt.as_ref()
    }

    pub fn set_sprout_points(
        &mut self,
        percentage: u8,
        points: SproutPoints,
    ) -> Option<&SproutPoints> {
        match percentage {
            5 => self.sprout_percentage_points.five_percent_points = Some(points),
            10 => self.sprout_percentage_points.ten_percent_points = Some(points),
            15 => self.sprout_percentage_points.fifteen_percent_points = Some(points),
            20 => self.sprout_percentage_points.twenty_percent_points = Some(points),
            25 => self.sprout_percentage_points.twenty_five_percent_points = Some(points),
            30 => self.sprout_percentage_points.thirty_percent_points = Some(points),
            35 => self.sprout_percentage_points.thirty_five_percent_points = Some(points),
            40 => self.sprout_percentage_points.forty_percent_points = Some(points),
            45 => self.sprout_percentage_points.forty_five_percent_points = Some(points),
            50 => self.sprout_percentage_points.fifty_percent_points = Some(points),
            55 => self.sprout_percentage_points.fifty_five_percent_points = Some(points),
            60 => self.sprout_percentage_points.sixty_percent_points = Some(points),
            65 => self.sprout_percentage_points.sixty_five_percent_points = Some(points),
            70 => self.sprout_percentage_points.seventy_percent_points = Some(points),
            75 => self.sprout_percentage_points.seventy_five_percent_points = Some(points),
            80 => self.sprout_percentage_points.eighty_percent_points = Some(points),
            85 => self.sprout_percentage_points.eighty_five_percent_points = Some(points),
            90 => self.sprout_percentage_points.ninety_percent_points = Some(points),
            95 => self.sprout_percentage_points.ninety_five_percent_points = Some(points),
            _ => self.sprout_percentage_points.hundred_percent_points = Some(points),
        }
        self.get_sprout_points(percentage)
    }
}
