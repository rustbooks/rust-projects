// src/models/conversation.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A conversation thread (one contact, possibly multiple messages).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    /// UUID v4 primary key
    pub id: String,
    /// Display name (may differ from phone if saved in contacts)
    pub contact_name: String,
    /// E.164-normalised phone number
    pub phone: String,
    /// Preview of the most recent message
    pub snippet: String,
    /// Timestamp of the latest message
    pub last_message_at: DateTime<Utc>,
    /// Number of unread messages
    pub unread_count: i64,
    /// Whether notifications are silenced
    pub is_muted: bool,
    /// Pinned to top of list
    pub is_pinned: bool,
    /// Whether the last message has an attachment
    pub has_attachment: bool,
}

impl Conversation {
    /// Generate a deterministic avatar colour from the contact name.
    /// Returns a hex colour string understood by Slint's `color` type.
    pub fn avatar_color(&self) -> String {
        // Simple djb2 hash → index into a palette of Material You tonal colours.
        let palette = [
            "#6750A4", "#7965AF", "#8B7BB9", "#9D90C4",
            "#0061A4", "#1A6EBB", "#357BCF", "#5089E2",
            "#006C4C", "#1A7B5E", "#358A71", "#4F9984",
            "#984061", "#AC5477", "#C0698E", "#D47FA5",
            "#7E5700", "#916700", "#A57800", "#B98A00",
        ];
        let hash = self.contact_name.bytes().fold(5381u64, |acc, b| {
            acc.wrapping_mul(33).wrapping_add(b as u64)
        });
        palette[(hash as usize) % palette.len()].to_string()
    }
}
