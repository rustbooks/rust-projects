// src/world_clock.rs
// ============================================================================
// World Clock Engine — Multi-timezone time display
// ============================================================================
// Uses chrono + chrono-tz for accurate timezone conversions.
// The timezone database is compiled into the binary (~300 KB) — no runtime
// file access needed, works offline.
// ============================================================================

use chrono::{DateTime, Local, Utc};
use chrono_tz::Tz;
use std::str::FromStr;

// Import the Slint-generated WorldClockItem type
use crate::WorldClockItem;
use slint::SharedString;

/// City metadata: display name and timezone string.
struct CityInfo {
    city: &'static str,
    country: &'static str,
    timezone: &'static str,
}

/// Built-in list of popular cities with their IANA timezone identifiers.
/// Users can add/remove from this list in the UI.
const KNOWN_CITIES: &[CityInfo] = &[
    CityInfo {
        city: "New York",
        country: "US",
        timezone: "America/New_York",
    },
    CityInfo {
        city: "Los Angeles",
        country: "US",
        timezone: "America/Los_Angeles",
    },
    CityInfo {
        city: "London",
        country: "GB",
        timezone: "Europe/London",
    },
    CityInfo {
        city: "Paris",
        country: "FR",
        timezone: "Europe/Paris",
    },
    CityInfo {
        city: "Berlin",
        country: "DE",
        timezone: "Europe/Berlin",
    },
    CityInfo {
        city: "Moscow",
        country: "RU",
        timezone: "Europe/Moscow",
    },
    CityInfo {
        city: "Dubai",
        country: "AE",
        timezone: "Asia/Dubai",
    },
    CityInfo {
        city: "Mumbai",
        country: "IN",
        timezone: "Asia/Kolkata",
    },
    CityInfo {
        city: "Singapore",
        country: "SG",
        timezone: "Asia/Singapore",
    },
    CityInfo {
        city: "Tokyo",
        country: "JP",
        timezone: "Asia/Tokyo",
    },
    CityInfo {
        city: "Shanghai",
        country: "CN",
        timezone: "Asia/Shanghai",
    },
    CityInfo {
        city: "Sydney",
        country: "AU",
        timezone: "Australia/Sydney",
    },
    CityInfo {
        city: "Auckland",
        country: "NZ",
        timezone: "Pacific/Auckland",
    },
    CityInfo {
        city: "São Paulo",
        country: "BR",
        timezone: "America/Sao_Paulo",
    },
    CityInfo {
        city: "Chicago",
        country: "US",
        timezone: "America/Chicago",
    },
    CityInfo {
        city: "Toronto",
        country: "CA",
        timezone: "America/Toronto",
    },
];

/// World clock engine — converts timezone strings to display data.
pub struct WorldClockEngine {
    local_offset_secs: i32,
}

impl WorldClockEngine {
    pub fn new() -> Self {
        use chrono::Local;
        let local_offset = Local::now().offset().local_minus_utc();
        Self {
            local_offset_secs: local_offset,
        }
    }

    /// Get current time info for a timezone string.
    /// Returns None if the timezone string is invalid.
    pub fn get_city_info(&self, timezone: &str) -> Option<WorldClockItem> {
        let tz = Tz::from_str(timezone).ok()?;
        let now_utc = Utc::now();
        let now_in_tz: DateTime<Tz> = now_utc.with_timezone(&tz);

        // Find display name from our known cities list
        let (city_name, country) = KNOWN_CITIES
            .iter()
            .find(|c| c.timezone == timezone)
            .map(|c| (c.city, c.country))
            .unwrap_or_else(|| {
                // Fall back to the timezone string if city not found
                let parts: Vec<&str> = timezone.split('/').collect();
                (parts.last().copied().unwrap_or(timezone), "")
            });

        // Calculate UTC offset for display (e.g. "UTC+9")
        let tz_offset = now_in_tz.offset().local_minus_utc();
        let offset_hours = tz_offset / 3600;
        let offset_mins = (tz_offset.abs() % 3600) / 60;
        let offset_str = if offset_mins == 0 {
            format!("UTC{:+}", offset_hours)
        } else {
            format!("UTC{:+}:{:02}", offset_hours, offset_mins)
        };

        // Time difference from local time (for display: "+5h", "-3h", etc.)
        let diff_secs = tz_offset - self.local_offset_secs;
        let diff_hours = diff_secs / 3600;
        let diff_str = if diff_hours == 0 {
            "Same time".to_string()
        } else if diff_hours > 0 {
            format!("+{}h", diff_hours)
        } else {
            format!("{}h", diff_hours)
        };

        use chrono::Timelike;
        let hour = now_in_tz.hour();
        let time_display = format!("{:02}:{:02}", hour, now_in_tz.minute());

        // Determine if it's day or night (for icon display)
        let is_daytime = hour >= 6 && hour < 20;

        // Check if the date in this timezone differs from local date
        use chrono::Datelike;
        let local_date = Local::now().date_naive();
        let tz_date = now_in_tz.date_naive();
        let date_suffix = if tz_date == local_date {
            String::new()
        } else if tz_date > local_date {
            " +1d".to_string()
        } else {
            " -1d".to_string()
        };

        Some(WorldClockItem {
            city: SharedString::from(city_name),
            country: SharedString::from(country),
            timezone: SharedString::from(timezone),
            time_display: SharedString::from(format!("{}{}", time_display, date_suffix)),
            offset_display: SharedString::from(offset_str),
            diff_display: SharedString::from(diff_str),
            is_daytime,
        })
    }

    /// Get the full list of available cities for the add-city dialog.
    pub fn available_cities() -> Vec<(&'static str, &'static str)> {
        KNOWN_CITIES
            .iter()
            .map(|c| (c.city, c.timezone))
            .collect()
    }
}
