// src/models/mod.rs
// Domain model types shared between DB, crypto and UI layers.
// Using `serde` for JSON serialisation (export/import) and `chrono` for timestamps.

pub mod conversation;
pub mod message;
pub mod attachment;

pub use conversation::Conversation;
pub use message::{Message, MessageStatus};
pub use attachment::{Attachment, AttachmentKind};
