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

    pub habits: Vec<String>,

    pub themes: [&'static str; 3],

    pub active_theme: usize,
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
        app.menu = vec!["Dashboard", "Heatmap", "Stats", "Settings"];
        app.habits = vec![
            "All habits".to_string(),
            "Reading".to_string(),
            "Pushups".to_string(),
            "Meditation".to_string(),
            "loooooooooooooooooo".to_string(),
        ];
        app.themes = ["Sprout", "Amber", "Mono"];
        app
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

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
