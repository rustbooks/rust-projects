// src/db/conversations.rs

use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;

use crate::db::Database;
use crate::models::Conversation;

pub struct ConversationRepo<'a> {
    pub(super) db: &'a Database,
}

impl ConversationRepo<'_> {
    /// Return all conversations sorted: pinned first, then by most-recent message.
    pub fn list_all(&self) -> Result<Vec<Conversation>> {
        let conn = self.db.conn();
        let mut stmt = conn.prepare(
            r#"SELECT id, contact_name, phone, snippet, last_message_at,
                      unread_count, is_muted, is_pinned, has_attachment
               FROM conversations
               ORDER BY is_pinned DESC, last_message_at DESC"#,
        )?;

        let rows = stmt.query_map([], |r| {
            Ok(Conversation {
                id:              r.get(0)?,
                contact_name:    r.get(1)?,
                phone:           r.get(2)?,
                snippet:         r.get(3)?,
                last_message_at: {
                    let s: String = r.get(4)?;
                    s.parse().unwrap_or_else(|_| Utc::now())
                },
                unread_count:    r.get(5)?,
                is_muted:        r.get::<_, i64>(6)? != 0,
                is_pinned:       r.get::<_, i64>(7)? != 0,
                has_attachment:  r.get::<_, i64>(8)? != 0,
            })
        })?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    /// Full-text search across contact name and phone number.
    pub fn search(&self, query: &str) -> Result<Vec<Conversation>> {
        let pattern = format!("%{query}%");
        let conn = self.db.conn();
        let mut stmt = conn.prepare(
            r#"SELECT id, contact_name, phone, snippet, last_message_at,
                      unread_count, is_muted, is_pinned, has_attachment
               FROM conversations
               WHERE contact_name LIKE ?1 OR phone LIKE ?1
               ORDER BY is_pinned DESC, last_message_at DESC"#,
        )?;

        let rows = stmt.query_map([&pattern], |r| {
            Ok(Conversation {
                id:              r.get(0)?,
                contact_name:    r.get(1)?,
                phone:           r.get(2)?,
                snippet:         r.get(3)?,
                last_message_at: {
                    let s: String = r.get(4)?;
                    s.parse().unwrap_or_else(|_| Utc::now())
                },
                unread_count:    r.get(5)?,
                is_muted:        r.get::<_, i64>(6)? != 0,
                is_pinned:       r.get::<_, i64>(7)? != 0,
                has_attachment:  r.get::<_, i64>(8)? != 0,
            })
        })?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    /// Create a new conversation, returning it.
    pub fn create(&self, phone: &str, name: &str) -> Result<Conversation> {
        let conv = Conversation {
            id:              Uuid::new_v4().to_string(),
            contact_name:    name.to_string(),
            phone:           phone.to_string(),
            snippet:         String::new(),
            last_message_at: Utc::now(),
            unread_count:    0,
            is_muted:        false,
            is_pinned:       false,
            has_attachment:  false,
        };

        let conn = self.db.conn();
        conn.execute(
            r#"INSERT INTO conversations
               (id, contact_name, phone, snippet, last_message_at,
                unread_count, is_muted, is_pinned, has_attachment)
               VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)"#,
            rusqlite::params![
                conv.id, conv.contact_name, conv.phone, conv.snippet,
                conv.last_message_at.to_rfc3339(),
                conv.unread_count as i64,
                conv.is_muted as i64, conv.is_pinned as i64, conv.has_attachment as i64
            ],
        )?;

        Ok(conv)
    }

    /// Update snippet and timestamp after a new message.
    pub fn update_snippet(
        &self,
        id: &str,
        snippet: &str,
        has_attachment: bool,
    ) -> Result<()> {
        let conn = self.db.conn();
        conn.execute(
            r#"UPDATE conversations
               SET snippet = ?2, last_message_at = ?3, has_attachment = ?4
               WHERE id = ?1"#,
            rusqlite::params![
                id, snippet, Utc::now().to_rfc3339(), has_attachment as i64
            ],
        )?;
        Ok(())
    }

    /// Increment unread count for a conversation.
    pub fn increment_unread(&self, id: &str) -> Result<()> {
        let conn = self.db.conn();
        conn.execute(
            "UPDATE conversations SET unread_count = unread_count + 1 WHERE id = ?1",
            [id],
        )?;
        Ok(())
    }

    /// Mark all messages in a conversation as read (reset unread counter).
    pub fn mark_read(&self, id: &str) -> Result<()> {
        let conn = self.db.conn();
        conn.execute(
            "UPDATE conversations SET unread_count = 0 WHERE id = ?1",
            [id],
        )?;
        Ok(())
    }

    /// Delete a conversation and all its messages (CASCADE handles messages).
    pub fn delete(&self, id: &str) -> Result<()> {
        let conn = self.db.conn();
        conn.execute("DELETE FROM conversations WHERE id = ?1", [id])?;
        Ok(())
    }
}
