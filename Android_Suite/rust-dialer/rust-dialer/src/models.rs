// src/models.rs
// ─────────────────────────────────────────────────────────────────────────────
// Domain models — plain Rust structs with serde for JSON persistence.
// Separated from UI types to keep business logic independent of the renderer.
// ─────────────────────────────────────────────────────────────────────────────

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─── Contact ─────────────────────────────────────────────────────────────────

/// A single address-book entry.
/// Stored as JSON on disk (optionally AES-256-GCM encrypted).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    /// Stable UUID — never changes even if name/number changes
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub is_favorite: bool,
    /// RGBA color seed for avatar (derived from id hash on first use)
    pub avatar_seed: u32,
    /// ISO-8601 creation timestamp
    pub created_at: DateTime<Utc>,
}

impl Contact {
    /// Create a new contact with a fresh UUID and current timestamp.
    pub fn new(first_name: impl Into<String>, last_name: impl Into<String>, phone: impl Into<String>) -> Self {
        let id = Uuid::new_v4().to_string();
        let avatar_seed = Self::seed_from_id(&id);
        Self {
            id,
            first_name: first_name.into(),
            last_name: last_name.into(),
            phone: phone.into(),
            is_favorite: false,
            avatar_seed,
            created_at: Utc::now(),
        }
    }

    /// Derive a stable color seed from the UUID bytes.
    /// This ensures the same contact always gets the same avatar color
    /// even after app restarts.
    fn seed_from_id(id: &str) -> u32 {
        id.bytes()
            .take(4)
            .fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32))
    }

    /// Full display name (first + last), trimmed.
    pub fn display_name(&self) -> String {
        let full = format!("{} {}", self.first_name, self.last_name);
        full.trim().to_string()
    }

    /// Two-letter initials (up to 2 chars), always uppercase.
    pub fn initials(&self) -> String {
        let f = self.first_name.chars().next().unwrap_or('?').to_uppercase().next().unwrap_or('?');
        let l = self.last_name.chars().next().map(|c| c.to_uppercase().next().unwrap_or('?'));
        match l {
            Some(lc) => format!("{}{}", f, lc),
            None => format!("{}", f),
        }
    }

    /// ARGB color integer for the avatar background, derived from the seed.
    /// Returns one of 12 Material You tonal palette colors.
    pub fn avatar_argb(&self) -> u32 {
        // 12 Material You complementary colors (as ARGB u32)
        const PALETTE: [u32; 12] = [
            0xFF_E8F5E9, // Green 50
            0xFF_E3F2FD, // Blue 50
            0xFF_FCE4EC, // Pink 50
            0xFF_FFF3E0, // Orange 50
            0xFF_EDE7F6, // Deep Purple 50
            0xFF_E0F7FA, // Cyan 50
            0xFF_F3E5F5, // Purple 50
            0xFF_FFF8E1, // Amber 50
            0xFF_E8EAF6, // Indigo 50
            0xFF_FFEBEE, // Red 50
            0xFF_E0F2F1, // Teal 50
            0xFF_F9FBE7, // Lime 50
        ];
        PALETTE[(self.avatar_seed as usize) % PALETTE.len()]
    }

    /// Validate the contact fields.
    /// Returns `Err` with a human-readable message if invalid.
    pub fn validate(&self) -> Result<(), String> {
        if self.first_name.trim().is_empty() && self.last_name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if self.phone.trim().is_empty() {
            return Err("Phone number cannot be empty".to_string());
        }
        // Basic phone sanity: allow +, digits, spaces, dashes, parens
        let cleaned: String = self.phone.chars()
            .filter(|c| c.is_ascii_digit() || *c == '+')
            .collect();
        if cleaned.len() < 3 {
            return Err("Phone number too short".to_string());
        }
        Ok(())
    }
}

// ─── Sort Mode ────────────────────────────────────────────────────────────────

/// Contact list sort mode — mirrors the UI sort chips.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SortMode {
    #[default]
    FirstName,
    LastName,
    PhoneNumber,
}

impl SortMode {
    pub fn from_int(v: i32) -> Self {
        match v {
            1 => Self::LastName,
            2 => Self::PhoneNumber,
            _ => Self::FirstName,
        }
    }

    pub fn to_int(self) -> i32 {
        match self {
            Self::FirstName   => 0,
            Self::LastName    => 1,
            Self::PhoneNumber => 2,
        }
    }
}

// ─── Call Direction ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CallDirection {
    Incoming,
    Outgoing,
    Missed,
}

impl CallDirection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Incoming => "incoming",
            Self::Outgoing => "outgoing",
            Self::Missed   => "missed",
        }
    }
}

// ─── Call Record ─────────────────────────────────────────────────────────────

/// A single entry in the call history log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallRecord {
    pub id: String,
    /// Contact display name at time of call (snapshot — contact may be deleted)
    pub contact_name: String,
    pub phone: String,
    pub direction: CallDirection,
    /// Duration in seconds (0 for missed / unanswered)
    pub duration_sec: u32,
    pub timestamp: DateTime<Utc>,
}

impl CallRecord {
    pub fn new(
        contact_name: impl Into<String>,
        phone: impl Into<String>,
        direction: CallDirection,
        duration_sec: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            contact_name: contact_name.into(),
            phone: phone.into(),
            direction,
            duration_sec,
            timestamp: Utc::now(),
        }
    }

    /// Human-friendly timestamp (e.g. "Today 14:23" or "Mon 09:01")
    pub fn friendly_timestamp(&self) -> String {
        let now = Utc::now();
        let ts  = self.timestamp;
        let diff = now.signed_duration_since(ts);

        if diff.num_hours() < 24 {
            ts.format("Today %H:%M").to_string()
        } else if diff.num_days() < 7 {
            ts.format("%a %H:%M").to_string()
        } else {
            ts.format("%d %b %H:%M").to_string()
        }
    }

    /// Initials derived from contact_name
    pub fn initials(&self) -> String {
        let mut parts = self.contact_name.split_whitespace();
        let first = parts.next().and_then(|s| s.chars().next()).unwrap_or('?').to_uppercase().next().unwrap_or('?');
        let last  = parts.next().and_then(|s| s.chars().next()).map(|c| c.to_uppercase().next().unwrap_or('?'));
        match last {
            Some(l) => format!("{}{}", first, l),
            None    => format!("{}", first),
        }
    }
}

// ─── App Settings ─────────────────────────────────────────────────────────────

/// Persisted user preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub sort_mode: SortMode,
    /// Whether contacts file is AES-256-GCM encrypted at rest
    pub encryption_enabled: bool,
    /// Base64-encoded 32-byte AES key (only present if user enabled encryption)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_key_b64: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            sort_mode: SortMode::FirstName,
            encryption_enabled: false,
            encrypted_key_b64: None,
        }
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contact_display_name_both() {
        let c = Contact::new("John", "Doe", "+1234567890");
        assert_eq!(c.display_name(), "John Doe");
    }

    #[test]
    fn contact_display_name_only_first() {
        let c = Contact::new("Alice", "", "+1111111111");
        assert_eq!(c.display_name(), "Alice");
    }

    #[test]
    fn contact_initials() {
        let c = Contact::new("Bob", "Smith", "555");
        assert_eq!(c.initials(), "BS");
    }

    #[test]
    fn contact_validate_empty_name() {
        let c = Contact::new("", "", "+9999999999");
        assert!(c.validate().is_err());
    }

    #[test]
    fn contact_validate_ok() {
        let c = Contact::new("Jane", "Doe", "+9876543210");
        assert!(c.validate().is_ok());
    }

    #[test]
    fn sort_mode_round_trip() {
        for i in 0..3i32 {
            assert_eq!(SortMode::from_int(i).to_int(), i);
        }
    }

    #[test]
    fn call_record_initials() {
        let r = CallRecord::new("Ada Lovelace", "+0", CallDirection::Outgoing, 30);
        assert_eq!(r.initials(), "AL");
    }
}
