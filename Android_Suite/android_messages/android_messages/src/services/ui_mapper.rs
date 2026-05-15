// src/services/ui_mapper.rs
// Converts domain model types → Slint-generated struct types.
// This file is the only place that knows about both layers, keeping them decoupled.

use slint::SharedString;

use crate::models::{Attachment, Conversation, Message};
// Import the Slint-generated structs (via include_modules! in main.rs)
use crate::{ConversationItem, MessageItem};

/// Map a slice of domain Conversations to a Vec of Slint ConversationItem.
pub fn conversations_to_slint(convs: Vec<Conversation>) -> Vec<ConversationItem> {
    convs.into_iter().map(conv_to_slint).collect()
}

fn conv_to_slint(c: Conversation) -> ConversationItem {
    let color_str = c.avatar_color();
    let color = parse_hex_color(&color_str);

    ConversationItem {
        id:             c.id.into(),
        contact_name:   c.contact_name.into(),
        phone:          c.phone.into(),
        snippet:        c.snippet.into(),
        timestamp:      relative_time(&c.last_message_at).into(),
        unread_count:   c.unread_count as i32,
        avatar_color:   color,
        is_muted:       c.is_muted,
        is_pinned:      c.is_pinned,
        has_attachment: c.has_attachment,
    }
}

/// Map a single domain Message (with optional Attachment) to a Slint MessageItem.
pub fn message_to_slint(m: &Message, att: Option<&Attachment>) -> MessageItem {
    let (att_type, att_path, att_name) = match att {
        Some(a) => (
            SharedString::from(a.kind.as_str()),
            SharedString::from(a.file_path.as_str()),
            SharedString::from(a.file_name.as_str()),
        ),
        None => (
            SharedString::from("none"),
            SharedString::default(),
            SharedString::default(),
        ),
    };

    MessageItem {
        id:              m.id.clone().into(),
        conversation_id: m.conversation_id.clone().into(),
        body:            m.body.clone().into(),
        timestamp:       m.formatted_time().into(),
        is_sent:         m.is_sent,
        is_read:         m.is_read,
        attachment_type: att_type,
        attachment_path: att_path,
        attachment_name: att_name,
        is_encrypted:    m.is_encrypted,
        status:          m.status.as_str().into(),
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Parse a #RRGGBB hex string into a slint::Color.
fn parse_hex_color(s: &str) -> slint::Color {
    let s = s.trim_start_matches('#');
    if s.len() == 6 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&s[0..2], 16),
            u8::from_str_radix(&s[2..4], 16),
            u8::from_str_radix(&s[4..6], 16),
        ) {
            return slint::Color::from_rgb_u8(r, g, b);
        }
    }
    slint::Color::from_rgb_u8(103, 80, 164) // fallback: Material You purple
}

/// Human-readable relative time for conversation list timestamps.
fn relative_time(dt: &chrono::DateTime<chrono::Utc>) -> String {
    let now   = chrono::Utc::now();
    let delta = now.signed_duration_since(*dt);

    if delta.num_seconds() < 60 {
        "now".to_string()
    } else if delta.num_minutes() < 60 {
        format!("{} min", delta.num_minutes())
    } else if delta.num_hours() < 24 {
        dt.format("%H:%M").to_string()
    } else if delta.num_days() < 7 {
        dt.format("%a").to_string()
    } else if delta.num_days() < 365 {
        dt.format("%b %-d").to_string()
    } else {
        dt.format("%-d/%-m/%y").to_string()
    }
}
