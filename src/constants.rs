// pub const TICK_RATE_MS: u64 = 250;
pub const TICK_RATE_MS: u64 = 60;

/// How many ticks before the cursor blink toggles.
// pub const CURSOR_BLINK_TICKS: u64 = 2;  // 2 × 250ms = 500ms period
pub const CURSOR_BLINK_TICKS: u64 = 8; // 7 × 60ms  = 420ms period

pub const PROGRESS_LOAD_TICKS: u64 = 10;

/// Delay ticks injected on each keypress to keep the cursor solid while typing.
// pub const CURSOR_TYPING_DELAY: usize = 3;  // 3 × 250ms = 750ms
pub const CURSOR_TYPING_DELAY: usize = 13; // 13 × 60ms = 780ms

/// If the current delay is below this, reset it to CURSOR_TYPING_DELAY.
// pub const CURSOR_DELAY_THRESHOLD: usize = 2;
pub const CURSOR_DELAY_THRESHOLD: usize = 8;

pub const NOTIFICATION_DISPLAY_TICKS: u64 = 70; // 30 × 60ms 
