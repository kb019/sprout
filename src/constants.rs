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

pub const NOTIFICATION_DISPLAY_TICKS: u64 = 60; // 30 × 60ms 
