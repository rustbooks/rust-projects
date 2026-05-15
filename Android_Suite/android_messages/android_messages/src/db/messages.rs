// src/db/messages.rs

use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;

use crate::db::Database;
use crate::models::{Message, MessageStatus};

pub struct MessageRepo<'a> {
    pub(super) db: &'a Database,
}

impl MessageRepo<'_> {
    /// Retrieve all messages for a conversation, oldest first.
    pub fn for_conversation(&self, conv_id: &str) -> Result<Vec<Message>> {
        let conn = self.db.conn();
        let mut stmt = conn.prepare(
            r#"SELECT id, conversation_id, body, sent_at, is_sent, is_read,
                      status, attachment_id, is_encrypted
               FROM messages
               WHERE conversation_id = ?1
               ORDER BY sent_at ASC"#,
        )?;

        let rows = stmt.query_map([conv_id], |r| {
            Ok(Message {
                id:              r.get(0)?,
                conversation_id: r.get(1)?,
                body:            r.get(2)?,
                sent_at: {
                    let s: String = r.get(3)?;
                    s.parse().unwrap_or_else(|_| Utc::now())
                },
                is_sent:      r.get::<_, i64>(4)? != 0,
                is_read:      r.get::<_, i64>(5)? != 0,
                status:       MessageStatus::from_str(&r.get::<_, String>(6)?),
                attachment_id: r.get(7)?,
                is_encrypted: r.get::<_, i64>(8)? != 0,
            })
        })?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    /// Insert a new message, returning it.
    pub fn insert(
        &self,
        conv_id:       &str,
        body:          &str,
        is_sent:       bool,
        attachment_id: Option<&str>,
        is_encrypted:  bool,
    ) -> Result<Message> {
        let msg = Message {
            id:              Uuid::new_v4().to_string(),
            conversation_id: conv_id.to_string(),
            body:            body.to_string(),
            sent_at:         Utc::now(),
            is_sent,
            is_read:         is_sent, // own messages are implicitly read
            status:          if is_sent { MessageStatus::Sending } else { MessageStatus::Delivered },
            attachment_id:   attachment_id.map(|s| s.to_string()),
            is_encrypted,
        };

        let conn = self.db.conn();
        conn.execute(
            r#"INSERT INTO messages
               (id, conversation_id, body, sent_at, is_sent, is_read,
                status, attachment_id, is_encrypted)
               VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)"#,
            rusqlite::params![
                msg.id, msg.conversation_id, msg.body,
                msg.sent_at.to_rfc3339(),
                msg.is_sent as i64, msg.is_read as i64,
                msg.status.as_str(), msg.attachment_id,
                msg.is_encrypted as i64
            ],
        )?;

        Ok(msg)
    }

    /// Update message delivery status.
    pub fn update_status(&self, id: &str, status: MessageStatus) -> Result<()> {
        let conn = self.db.conn();
        conn.execute(
            "UPDATE messages SET status = ?2 WHERE id = ?1",
            rusqlite::params![id, status.as_str()],
        )?;
        Ok(())
    }

    /// Delete a single message.
    pub fn delete(&self, id: &str) -> Result<()> {
        let conn = self.db.conn();
        conn.execute("DELETE FROM messages WHERE id = ?1", [id])?;
        Ok(())
    }
}
