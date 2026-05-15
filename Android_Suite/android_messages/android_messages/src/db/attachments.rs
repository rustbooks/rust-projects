// src/db/attachments.rs

use anyhow::Result;
use uuid::Uuid;

use crate::db::Database;
use crate::models::{Attachment, AttachmentKind};

pub struct AttachmentRepo<'a> {
    pub(super) db: &'a Database,
}

impl AttachmentRepo<'_> {
    /// Save attachment metadata, return the new record.
    pub fn insert(&self, att: &Attachment) -> Result<()> {
        let conn = self.db.conn();
        conn.execute(
            r#"INSERT OR REPLACE INTO attachments
               (id, file_name, file_path, kind, size, mime_type)
               VALUES (?1,?2,?3,?4,?5,?6)"#,
            rusqlite::params![
                att.id, att.file_name, att.file_path,
                att.kind.as_str(), att.size as i64, att.mime_type
            ],
        )?;
        Ok(())
    }

    /// Retrieve attachment by ID.
    pub fn find(&self, id: &str) -> Result<Option<Attachment>> {
        let conn = self.db.conn();
        let mut stmt = conn.prepare(
            "SELECT id, file_name, file_path, kind, size, mime_type FROM attachments WHERE id = ?1",
        )?;

        let result = stmt.query_row([id], |r| {
            let kind_str: String = r.get(3)?;
            Ok(Attachment {
                id:        r.get(0)?,
                file_name: r.get(1)?,
                file_path: r.get(2)?,
                kind: match kind_str.as_str() {
                    "image" => AttachmentKind::Image,
                    "video" => AttachmentKind::Video,
                    "audio" => AttachmentKind::Audio,
                    _       => AttachmentKind::File,
                },
                size:      r.get::<_, i64>(4)? as u64,
                mime_type: r.get(5)?,
            })
        });

        match result {
            Ok(att) => Ok(Some(att)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Create an Attachment from a file path (for pick-attachment flow).
    pub fn create_from_path(path: &std::path::Path) -> Result<Attachment> {
        let meta = std::fs::metadata(path)?;
        let kind = AttachmentKind::from_path(path);
        let mime = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();

        Ok(Attachment {
            id:        Uuid::new_v4().to_string(),
            file_name: path.file_name()
                          .and_then(|n| n.to_str())
                          .unwrap_or("attachment")
                          .to_string(),
            file_path: path.to_string_lossy().into_owned(),
            kind,
            size:      meta.len(),
            mime_type: mime,
        })
    }
}
