pub const TICK_RATE_MS: u64 = 250;

/// How many ticks before the cursor blink toggles (2 ticks × 250ms = 500ms period).
pub const CURSOR_BLINK_TICKS: u64 = 2;

/// Delay ticks injected on each keypress to keep the cursor solid while typing.
pub const CURSOR_TYPING_DELAY: usize = 3;

/// If the current delay is below this, reset it to CURSOR_TYPING_DELAY.
pub const CURSOR_DELAY_THRESHOLD: usize = 2;
