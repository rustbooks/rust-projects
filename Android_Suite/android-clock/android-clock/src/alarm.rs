// src/alarm.rs
// ============================================================================
// Alarm Engine — Background alarm checking and notification
// ============================================================================

use crate::model::AlarmModel;
use anyhow::Result;
use std::collections::HashSet;

/// Manages alarm scheduling and firing logic.
/// The actual UI notification is triggered via Slint callbacks.
pub struct AlarmEngine {
    /// Set of alarm IDs that have fired in the current minute.
    /// Prevents the same alarm from firing multiple times per minute.
    fired_this_minute: HashSet<String>,
    last_checked_minute: Option<u32>,
}

impl AlarmEngine {
    pub fn new() -> Self {
        Self {
            fired_this_minute: HashSet::new(),
            last_checked_minute: None,
        }
    }

    /// Check if any alarms should fire now.
    /// Call this every second (or on minute boundaries for efficiency).
    /// Returns a list of alarms that should fire.
    pub fn check_alarms(&mut self, alarms: &[AlarmModel]) -> Vec<String> {
        use chrono::{Local, Timelike};
        let now = Local::now();
        let current_minute = now.minute();

        // Reset the fired set at the start of each new minute
        if self.last_checked_minute != Some(current_minute) {
            self.fired_this_minute.clear();
            self.last_checked_minute = Some(current_minute);
        }

        // Only fire at the start of the minute (second == 0) to avoid
        // firing again if the app was paused and resumed mid-minute.
        if now.second() != 0 {
            return vec![];
        }

        alarms
            .iter()
            .filter(|alarm| {
                alarm.should_fire_now() && !self.fired_this_minute.contains(&alarm.id)
            })
            .map(|alarm| {
                self.fired_this_minute.insert(alarm.id.clone());
                alarm.id.clone()
            })
            .collect()
    }

    /// Dismiss a fired alarm (stop the ringing).
    pub fn dismiss(&mut self, alarm_id: &str) {
        log::info!("Alarm dismissed: {}", alarm_id);
        // Platform-specific: stop audio, cancel vibration
        platform_stop_alarm();
    }

    /// Snooze a fired alarm for N minutes.
    pub fn snooze(&mut self, alarm_id: &str, snooze_minutes: u8) {
        log::info!("Alarm snoozed for {} minutes: {}", snooze_minutes, alarm_id);
        // In a full implementation, we'd schedule a one-shot timer here.
    }
}

/// Platform-specific: stop alarm audio and vibration.
fn platform_stop_alarm() {
    // On real platforms, this would call into the audio subsystem.
    // For now, it's a no-op placeholder.
    log::debug!("Platform: stop alarm audio");
}
