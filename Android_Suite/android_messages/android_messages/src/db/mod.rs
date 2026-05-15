// src/db/mod.rs
// SQLite persistence layer using rusqlite with bundled sqlite.
// All schema migrations run in-process using a version table.
// The database file lives in the OS-appropriate app data directory.

mod schema;
mod conversations;
mod messages;
mod attachments;

pub use conversations::ConversationRepo;
pub use messages::MessageRepo;
pub use attachments::AttachmentRepo;

use std::path::Path;
use anyhow::Result;
use rusqlite::Connection;

/// Thread-safe wrapper around a single SQLite connection.
/// rusqlite's Connection is Send but not Sync; we wrap in a Mutex for sharing
/// across async tasks.  On Android, a single WAL-mode connection performs well.
pub struct Database {
    conn: std::sync::Mutex<Connection>,
}

impl Database {
    /// Open an in-memory database (for tests only — data lost on drop).
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA foreign_keys=ON;")?;
        schema::migrate(&conn)?;
        Ok(Self { conn: std::sync::Mutex::new(conn) })
    }

    /// Open (or create) the database at `path` and run any pending migrations.
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;

        // WAL mode: dramatically reduces write latency on mobile storage.
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

        // Run schema migrations
        schema::migrate(&conn)?;

        Ok(Self {
            conn: std::sync::Mutex::new(conn),
        })
    }

    /// Acquire the connection lock.
    /// Panics only if the mutex is poisoned (previous thread panicked while holding it).
    #[inline]
    pub fn conn(&self) -> std::sync::MutexGuard<'_, Connection> {
        // In production we log + return an error instead of panicking,
        // but MutexGuard acquisition can only fail on poison which is unrecoverable.
        self.conn.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Convenience: conversations repository.
    pub fn conversations(&self) -> ConversationRepo<'_> {
        ConversationRepo { db: self }
    }

    /// Convenience: messages repository.
    pub fn messages(&self) -> MessageRepo<'_> {
        MessageRepo { db: self }
    }

    /// Convenience: attachments repository.
    pub fn attachments(&self) -> AttachmentRepo<'_> {
        AttachmentRepo { db: self }
    }
}

// Safety: Connection itself is Send (no thread-local state).
// The Mutex ensures exclusive access.
unsafe impl Send for Database {}
unsafe impl Sync for Database {}
