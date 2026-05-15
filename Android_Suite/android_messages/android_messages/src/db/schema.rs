// src/db/schema.rs
// Schema migration system.
// Each migration is a numbered SQL string applied exactly once.
// The `schema_version` PRAGMA tracks the highest applied version.

use anyhow::Result;
use rusqlite::Connection;

/// All migrations in order.  Never modify existing entries — only append.
const MIGRATIONS: &[&str] = &[
    // ── v1: initial schema ───────────────────────────────────────────────
    r#"
    CREATE TABLE IF NOT EXISTS conversations (
        id               TEXT    PRIMARY KEY,
        contact_name     TEXT    NOT NULL,
        phone            TEXT    NOT NULL,
        snippet          TEXT    NOT NULL DEFAULT '',
        last_message_at  TEXT    NOT NULL,   -- ISO-8601 UTC
        unread_count     INTEGER NOT NULL DEFAULT 0,
        is_muted         INTEGER NOT NULL DEFAULT 0,  -- BOOLEAN (0/1)
        is_pinned        INTEGER NOT NULL DEFAULT 0,
        has_attachment   INTEGER NOT NULL DEFAULT 0
    );

    CREATE TABLE IF NOT EXISTS messages (
        id               TEXT    PRIMARY KEY,
        conversation_id  TEXT    NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
        body             TEXT    NOT NULL DEFAULT '',
        sent_at          TEXT    NOT NULL,   -- ISO-8601 UTC
        is_sent          INTEGER NOT NULL DEFAULT 1,
        is_read          INTEGER NOT NULL DEFAULT 0,
        status           TEXT    NOT NULL DEFAULT 'sent',
        attachment_id    TEXT,
        is_encrypted     INTEGER NOT NULL DEFAULT 0
    );

    CREATE TABLE IF NOT EXISTS attachments (
        id        TEXT PRIMARY KEY,
        file_name TEXT NOT NULL,
        file_path TEXT NOT NULL,
        kind      TEXT NOT NULL,
        size      INTEGER NOT NULL DEFAULT 0,
        mime_type TEXT NOT NULL DEFAULT 'application/octet-stream'
    );

    -- Speed up the common query: messages for a conversation, newest first.
    CREATE INDEX IF NOT EXISTS idx_messages_conv_time
        ON messages(conversation_id, sent_at DESC);

    -- Speed up conversation list sort (pinned first, then by time).
    CREATE INDEX IF NOT EXISTS idx_conv_sort
        ON conversations(is_pinned DESC, last_message_at DESC);
    "#,
];

/// Apply any unapplied migrations.  Idempotent — safe to call on every startup.
pub fn migrate(conn: &Connection) -> Result<()> {
    // rusqlite's user_version PRAGMA stores an integer schema version.
    let version: i64 = conn.query_row(
        "PRAGMA user_version",
        [],
        |r| r.get(0),
    )?;

    let pending = MIGRATIONS.iter().enumerate().skip(version as usize);
    for (i, sql) in pending {
        conn.execute_batch(sql)?;
        // Bump version AFTER successful execution so a crash mid-migration
        // is retried on next startup.
        conn.execute_batch(&format!("PRAGMA user_version = {}", i + 1))?;
        log::info!("Applied DB migration v{}", i + 1);
    }

    Ok(())
}
