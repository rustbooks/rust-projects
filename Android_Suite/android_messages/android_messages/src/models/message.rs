// src/models/message.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Delivery / read state of a sent message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageStatus {
    Sending,
    Sent,
    Delivered,
    Read,
    Failed,
}

impl MessageStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sending   => "sending",
            Self::Sent      => "sent",
            Self::Delivered => "delivered",
            Self::Read      => "read",
            Self::Failed    => "failed",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "sending"   => Self::Sending,
            "sent"      => Self::Sent,
            "delivered" => Self::Delivered,
            "read"      => Self::Read,
            _           => Self::Failed,
        }
    }
}

/// A single SMS/MMS message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id:              String,
    pub conversation_id: String,
    /// Plaintext body (decrypted in memory; stored encrypted on disk when enabled)
    pub body:            String,
    pub sent_at:         DateTime<Utc>,
    /// true = outgoing, false = incoming
    pub is_sent:         bool,
    pub is_read:         bool,
    pub status:          MessageStatus,
    /// Optional attachment metadata
    pub attachment_id:   Option<String>,
    /// Whether the stored body was AES-256-GCM encrypted
    pub is_encrypted:    bool,
}

impl Message {
    /// Formatted relative timestamp shown in the UI ("2 min ago", "Mon", "Dec 25", …)
    pub fn formatted_time(&self) -> String {
        let now   = Utc::now();
        let delta = now.signed_duration_since(self.sent_at);

        if delta.num_seconds() < 60 {
            "just now".to_string()
        } else if delta.num_minutes() < 60 {
            format!("{} min ago", delta.num_minutes())
        } else if delta.num_hours() < 24 {
            self.sent_at.format("%H:%M").to_string()
        } else if delta.num_days() < 7 {
            self.sent_at.format("%a").to_string()          // Mon, Tue …
        } else if delta.num_days() < 365 {
            self.sent_at.format("%b %-d").to_string()      // Dec 25
        } else {
            self.sent_at.format("%-d/%-m/%y").to_string()  // 25/12/23
        }
    }
}
