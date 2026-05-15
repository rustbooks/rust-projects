// src/models/attachment.rs

use serde::{Deserialize, Serialize};

/// Media type of an attachment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttachmentKind {
    Image,
    Video,
    Audio,
    File,
}

impl AttachmentKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Video => "video",
            Self::Audio => "audio",
            Self::File  => "file",
        }
    }

    /// Detect kind from file extension using `mime_guess`.
    pub fn from_path(path: &std::path::Path) -> Self {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        match mime.type_().as_str() {
            "image" => Self::Image,
            "video" => Self::Video,
            "audio" => Self::Audio,
            _       => Self::File,
        }
    }
}

/// Metadata for a file attachment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id:        String,
    pub file_name: String,
    /// Absolute path to the stored file (internal storage or user-selected)
    pub file_path: String,
    pub kind:      AttachmentKind,
    /// File size in bytes
    pub size:      u64,
    /// MIME type string (e.g. "image/jpeg")
    pub mime_type: String,
}
