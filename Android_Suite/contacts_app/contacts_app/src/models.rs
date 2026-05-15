// src/models.rs
// Core data models.
// These are plain Rust structs — no Slint dependency here.
// They serialize to JSON for persistence and convert to/from Slint types in main.rs.

use serde::{Deserialize, Serialize};

// ── Contact ───────────────────────────────────────────────────────────────────

/// The canonical contact record.
/// All string fields default to "" (never null) to simplify UI code.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    /// UUID v4 — stable identifier, never changes after creation
    pub id: String,

    // Name fields — Unicode, supports Devanagari and all other scripts
    pub first_name: String,
    pub last_name: String,

    // Phone — stored as-is (user enters local or international format)
    pub phone_primary: String,
    pub phone_secondary: String,

    // Email
    pub email: String,

    // Work
    pub organization: String,
    pub job_title: String,
    pub website: String,

    // Misc
    pub notes: String,
    pub group: String,

    // UI state
    pub is_favorite: bool,

    /// RGB color encoded as 0xRRGGBB — derived deterministically from name
    pub avatar_color: u32,

    // ISO 8601 timestamps — set by ContactManager, not the UI
    pub created_at: String,
    pub updated_at: String,
}

impl Contact {
    /// Returns the character(s) shown in the avatar circle.
    /// Prefers first letter of first_name, falls back to last_name, then "?".
    /// Works correctly for Devanagari (takes the first Unicode char, not byte).
    pub fn avatar_letter(&self) -> String {
        // `chars().next()` handles multi-byte Unicode correctly
        let letter = self.first_name.chars().next()
            .or_else(|| self.last_name.chars().next())
            .unwrap_or('?');
        letter.to_uppercase().to_string()
    }

    /// Full display name: "FirstName LastName" (trimmed).
    pub fn display_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name).trim().to_string()
    }

    /// Computes a deterministic avatar color from the contact's name.
    /// Uses the same palette defined in theme.slint.
    pub fn compute_avatar_color(first: &str, last: &str) -> u32 {
        // 12 Material You tonal colors — same as AppTheme.avatar-colors in Slint
        const PALETTE: &[u32] = &[
            0x1565C0, 0xC62828, 0x2E7D32, 0x6A1B9A,
            0xE65100, 0x00695C, 0x37474F, 0xAD1457,
            0x0277BD, 0x558B2F, 0x4527A0, 0xF57F17,
        ];

        // Simple hash: sum of Unicode code points
        let hash: u32 = first.chars().chain(last.chars())
            .fold(0u32, |acc, c| acc.wrapping_add(c as u32));

        PALETTE[(hash as usize) % PALETTE.len()]
    }
}

impl Default for Contact {
    fn default() -> Self {
        Self {
            id: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            phone_primary: String::new(),
            phone_secondary: String::new(),
            email: String::new(),
            organization: String::new(),
            job_title: String::new(),
            website: String::new(),
            notes: String::new(),
            group: String::new(),
            is_favorite: false,
            avatar_color: 0x1565C0,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }
}

// ── Group ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Group {
    pub id: String,
    pub name: String,
    /// Number of contacts in this group (denormalized count, for display)
    pub count: usize,
    /// Display color as 0xRRGGBB
    pub color: u32,
}

// ── Sort Key ──────────────────────────────────────────────────────────────────

/// How the contacts list is sorted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortKey {
    #[default]
    FirstName,
    LastName,
    Phone,
}

// ── Persisted Store ───────────────────────────────────────────────────────────

/// The full data structure written to disk as JSON.
/// Using a versioned envelope enables future migrations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataStore {
    /// Schema version — increment if the format changes
    pub version: u32,
    pub contacts: Vec<Contact>,
    pub groups: Vec<Group>,
}

impl DataStore {
    pub fn new() -> Self {
        Self { version: 1, contacts: Vec::new(), groups: Vec::new() }
    }
}
