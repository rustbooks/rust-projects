// src/timer.rs
// ============================================================================
// Timer Engine — Countdown timer logic
// ============================================================================

use anyhow::Result;

/// Manages countdown timer state.
/// The UI drives the timer via Slint's built-in timer callbacks —
/// this engine provides the business logic layer.
pub struct TimerEngine {
    pub total_ms: u64,
    pub remaining_ms: u64,
    pub running: bool,
}

impl TimerEngine {
    pub fn new() -> Self {
        Self {
            total_ms: 0,
            remaining_ms: 0,
            running: false,
        }
    }

    /// Format milliseconds as HH:MM:SS
    pub fn format_time(ms: u64) -> String {
        let total_secs = ms / 1000;
        let h = total_secs / 3600;
        let m = (total_secs % 3600) / 60;
        let s = total_secs % 60;
        format!("{:02}:{:02}:{:02}", h, m, s)
    }

    /// Progress from 0.0 (start) to 1.0 (complete)
    pub fn progress(&self) -> f32 {
        if self.total_ms == 0 {
            return 0.0;
        }
        1.0 - (self.remaining_ms as f32 / self.total_ms as f32)
    }
}
