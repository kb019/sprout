pub mod app;
pub mod habit;
pub mod input;
pub mod modal;

use self::app::AppState;
use self::habit::active_days::ActiveDaysState;
use self::habit::best_streaks::BestStreaksState;
use self::habit::get_streak::GetStreakState;
use self::habit::log_habit::LogHabitState;
use self::habit::progress::{
    DailyProgressState, MonthlyProgressState, WeeklyProgressState, YearlyProgressState,
};
use self::habit::weekly_average::WeeklyAverageState;
use self::modal::ModalState;

pub struct States {
    pub app_state: AppState,
    pub modal_state: ModalState,
    pub log_habit_state: LogHabitState,
    pub get_streak_state: GetStreakState,
    pub daily_progress_state: DailyProgressState,
    pub weekly_progress_state: WeeklyProgressState,
    pub monthly_progress_state: MonthlyProgressState,
    pub yearly_progress_state: YearlyProgressState,
    pub best_streaks_state: BestStreaksState,
    pub active_days_state: ActiveDaysState,
    pub weekly_average_state: WeeklyAverageState,
}

impl States {
    pub fn new() -> Self {
        Self {
            app_state: AppState::new(),
            modal_state: ModalState::new(),
            log_habit_state: LogHabitState::new(),
            get_streak_state: GetStreakState::new(),
            daily_progress_state: DailyProgressState::new(),
            weekly_progress_state: WeeklyProgressState::new(),
            monthly_progress_state: MonthlyProgressState::new(),
            yearly_progress_state: YearlyProgressState::new(),
            best_streaks_state: BestStreaksState::new(),
            active_days_state: ActiveDaysState::new(),
            weekly_average_state: WeeklyAverageState::new(),
        }
    }
}

impl Default for States {
    fn default() -> Self {
        Self::new()
    }
}
