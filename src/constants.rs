// pub const TICK_RATE_MS: u64 = 250;
pub const TICK_RATE_MS: u64 = 100;

/// How many ticks before the cursor blink toggles.
// pub const CURSOR_BLINK_TICKS: u64 = 2;  // 2 × 250ms = 500ms period
pub const CURSOR_BLINK_TICKS: u64 = 5; // 9 × 60ms  = 540ms period

pub const PROGRESS_LOAD_TICKS: u64 = 10;

/// Delay ticks injected on each keypress to keep the cursor solid while typing.
// pub const CURSOR_TYPING_DELAY: usize = 3;  // 3 × 250ms = 750ms
pub const CURSOR_TYPING_DELAY: usize = 5; // 5 × 100ms = 500ms

/// If the current delay is below this, reset it to CURSOR_TYPING_DELAY.
// pub const CURSOR_DELAY_THRESHOLD: usize = 2;
pub const CURSOR_DELAY_THRESHOLD: usize = 3;

pub const NOTIFICATION_DISPLAY_TICKS: u64 = 40;

/// Ticks to wait after the last log before refreshing best streaks (20 × 100ms = 2s).
pub const BEST_STREAK_REFRESH_DELAY_TICKS: u32 = 20;

/// Ticks to wait after the last log before refreshing active days (20 × 100ms = 2s).
pub const ACTIVE_DAYS_REFRESH_DELAY_TICKS: u32 = 20;

/// Ticks to wait after the last log before refreshing weekly average (20 × 100ms = 2s).
pub const WEEKLY_AVERAGE_REFRESH_DELAY_TICKS: u32 = 20;

/// Ticks to wait after add/delete before refreshing heatmap year-habit data (20 × 100ms = 2s).
pub const HEATMAP_YEAR_HABITS_REFRESH_DELAY_TICKS: u32 = 20;
