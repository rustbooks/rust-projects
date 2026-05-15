// src/model.rs
// ============================================================================
// Data Models — Pure Rust structs, no UI dependency
// ============================================================================
// These types represent the application's core data. They are:
//   - Serializable (serde) for persistence
//   - Clone-able for passing between threads
//   - Completely independent of the UI layer (MVU separation)
// ============================================================================

use chrono::{Datelike, Local, Timelike};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Top-level persisted application state.
/// This is what gets saved to disk and loaded on startup.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppState {
    /// List of configured alarms
    pub alarms: Vec<AlarmModel>,
    /// Timezone strings for world clock (e.g. "America/New_York")
    pub world_clock_cities: Vec<String>,
    /// Last selected tab index (0=Clock, 1=Alarm, 2=Timer, 3=Stopwatch, 4=WorldClock)
    pub last_tab: u8,
    /// User preference: 24-hour format
    pub use_24h: bool,
}

/// An individual alarm configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlarmModel {
    /// Unique identifier — survives reordering
    pub id: String,
    /// User-visible label (e.g. "Wake Up", "Meeting")
    pub label: String,
    /// Hour in 24h format (0–23)
    pub hour: u8,
    /// Minute (0–59)
    pub minute: u8,
    /// Whether the alarm is active
    pub enabled: bool,
    /// Which days to repeat — index 0=Monday, 6=Sunday
    pub repeat_days: [bool; 7],
    /// Whether to vibrate in addition to sound
    pub vibrate: bool,
    /// Sound URI or built-in sound name
    pub sound: String,
}

impl AlarmModel {
    /// Create a new alarm with sensible defaults.
    pub fn new(hour: u8, minute: u8, label: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            label: label.to_string(),
            hour,
            minute,
            enabled: true,
            repeat_days: [false; 7], // No repeat by default (one-shot)
            vibrate: true,
            sound: "default".to_string(),
        }
    }

    /// Human-readable day abbreviations for display in alarm list.
    pub fn format_days(&self) -> String {
        const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
        let active: Vec<&str> = self
            .repeat_days
            .iter()
            .enumerate()
            .filter(|(_, &on)| on)
            .map(|(i, _)| DAYS[i])
            .collect();

        match active.len() {
            0 => "Once".to_string(),
            7 => "Every day".to_string(),
            5 if !self.repeat_days[5] && !self.repeat_days[6] => "Weekdays".to_string(),
            2 if self.repeat_days[5] && self.repeat_days[6] => "Weekends".to_string(),
            _ => active.join(", "),
        }
    }

    /// Check if this alarm should fire right now.
    pub fn should_fire_now(&self) -> bool {
        if !self.enabled {
            return false;
        }
        let now = Local::now();
        if now.hour() as u8 != self.hour || now.minute() as u8 != self.minute {
            return false;
        }
        // Check day of week (chrono: Mon=1, Sun=7; our array: Mon=0, Sun=6)
        let today_idx = now.weekday().num_days_from_monday() as usize;
        let is_repeat_day = self.repeat_days.iter().any(|&d| d);
        if is_repeat_day {
            self.repeat_days[today_idx]
        } else {
            true // One-shot alarm fires any day
        }
    }
}

/// Stopwatch state — ephemeral, not persisted.
#[derive(Debug, Clone, Default)]
pub struct StopwatchState {
    pub elapsed_ms: u64,
    pub running: bool,
    pub laps: Vec<u64>, // lap times in ms
}

/// Timer state — ephemeral, not persisted (resets on app close).
#[derive(Debug, Clone, Default)]
pub struct TimerState {
    pub total_ms: u64,
    pub remaining_ms: u64,
    pub running: bool,
}

/// A lap entry for the stopwatch display.
#[derive(Debug, Clone)]
pub struct LapEntry {
    pub number: u32,
    pub lap_time_ms: u64,
    pub total_time_ms: u64,
}

impl LapEntry {
    pub fn format_time(ms: u64) -> String {
        let cs = (ms / 10) % 100;
        let secs = (ms / 1000) % 60;
        let mins = (ms / 60000) % 60;
        let hours = ms / 3600000;
        if hours > 0 {
            format!("{:02}:{:02}:{:02}.{:02}", hours, mins, secs, cs)
        } else {
            format!("{:02}:{:02}.{:02}", mins, secs, cs)
        }
    }
}

/// Configuration for the application theme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub use_dynamic_color: bool,
    pub dark_mode: bool,
    pub seed_color: u32, // ARGB packed color
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            use_dynamic_color: true, // Material You dynamic theming
            dark_mode: false,
            seed_color: 0xFF6750A4, // Material 3 default purple
        }
    }
}
