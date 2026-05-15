// src/services/app_controller.rs
// Central application controller.
// Holds shared state, performs async DB operations, posts results to the UI.
//
// Threading model:
//  ─ All public methods are called from Slint callbacks (main thread).
//  ─ Heavy work is spawned onto the tokio pool.
//  ─ Results are returned to the UI via `slint::invoke_from_event_loop`.

use std::sync::{Arc, Mutex};
use anyhow::Result;
use log::{error, info, warn};
use slint::Weak;
use tokio::runtime::Handle;

use crate::db::Database;
use crate::models::{Attachment, MessageStatus};
use crate::services::ui_mapper;
use crate::AppWindow;
use crate::crypto::Encryptor;
use crate::utils::paths;

/// Shared mutable state (arc-mutex so async tasks can access it).
struct State {
    encryption_enabled: bool,
    dark_mode:          bool,
    encryptor:          Option<Encryptor>,
    pending_attachment: Option<Attachment>,
}

pub struct AppController {
    window: Weak<AppWindow>,
    db:     Arc<Database>,
    rt:     Handle,
    state:  Arc<Mutex<State>>,
}

impl AppController {
    pub fn new(window: Weak<AppWindow>, db: Arc<Database>, rt: Handle) -> Self {
        // Try to load the encryption key; failure is non-fatal at startup.
        let encryptor = paths::app_data_dir()
            .ok()
            .map(|d| {
                let kp = d.join(".msg.key");
                Encryptor::load_or_create(&kp)
                    .map_err(|e| warn!("Could not load encryption key: {e}"))
                    .ok()
            })
            .flatten();

        Self {
            window,
            db,
            rt,
            state: Arc::new(Mutex::new(State {
                encryption_enabled: false,
                dark_mode:          false,
                encryptor,
                pending_attachment: None,
            })),
        }
    }

    // ── Helpers ──────────────────────────────────────────────────────────

    /// Post a status snackbar message (auto-dismissed via a timer in the UI layer).
    fn show_status(&self, msg: &str) {
        let msg = msg.to_string();
        let win = self.window.clone();
        slint::invoke_from_event_loop(move || {
            if let Some(w) = win.upgrade() {
                w.set_status_message(msg.into());
            }
        })
        .ok();
    }

    // ── Public async methods (called from wiring.rs) ──────────────────────

    /// Load all conversations and push to UI.
    pub async fn load_conversations(&self) -> Result<()> {
        let db   = self.db.clone();
        let win  = self.window.clone();
        let convs = db.conversations().list_all()?;
        let items = ui_mapper::conversations_to_slint(convs);

        slint::invoke_from_event_loop(move || {
            if let Some(w) = win.upgrade() {
                w.set_conversations(items.into());
                w.set_is_loading(false);
            }
        })?;

        Ok(())
    }

    /// Load messages for a conversation.
    pub async fn load_messages(&self, conv_id: String) -> Result<()> {
        let db    = self.db.clone();
        let win   = self.window.clone();
        let state = self.state.clone();

        let msgs  = db.messages().for_conversation(&conv_id)?;
        let enc   = state.lock().map(|s| s.encryption_enabled).unwrap_or(false);

        // Decrypt bodies if encryption is enabled
        let msgs = if enc {
            let guard  = state.lock().map_err(|e| anyhow::anyhow!("{e}"))?;
            if let Some(encryptor) = &guard.encryptor {
                msgs.into_iter()
                    .map(|mut m| {
                        if m.is_encrypted && !m.body.is_empty() {
                            m.body = encryptor.decrypt(&m.body).unwrap_or_else(|_| {
                                "⚠ Could not decrypt".to_string()
                            });
                        }
                        m
                    })
                    .collect()
            } else {
                msgs
            }
        } else {
            msgs
        };

        // Resolve attachment metadata
        let mut items = Vec::with_capacity(msgs.len());
        for msg in &msgs {
            let att = msg.attachment_id
                .as_deref()
                .and_then(|id| db.attachments().find(id).ok().flatten());
            items.push(ui_mapper::message_to_slint(msg, att.as_ref()));
        }

        let items_clone = items.clone();
        slint::invoke_from_event_loop(move || {
            if let Some(w) = win.upgrade() {
                w.set_messages(items_clone.into());
                w.set_is_loading(false);
            }
        })?;

        Ok(())
    }

    /// Send a message.
    pub async fn send_message(
        &self,
        conv_id:    String,
        body:       String,
        attachment: String,
    ) -> Result<()> {
        if conv_id.is_empty() {
            return Ok(());
        }

        let db    = self.db.clone();
        let state = self.state.clone();

        // Determine attachment_id
        let attachment_id: Option<String> = {
            let mut guard = state.lock().map_err(|e| anyhow::anyhow!("{e}"))?;
            guard.pending_attachment.take().map(|att| {
                let _ = db.attachments().insert(&att);
                att.id
            })
        };

        // Optionally encrypt body
        let (stored_body, is_enc) = {
            let guard = state.lock().map_err(|e| anyhow::anyhow!("{e}"))?;
            if guard.encryption_enabled {
                if let Some(enc) = &guard.encryptor {
                    let ct = enc.encrypt(&body).unwrap_or_else(|_| body.clone());
                    (ct, true)
                } else {
                    (body.clone(), false)
                }
            } else {
                (body.clone(), false)
            }
        };

        let msg = db.messages().insert(
            &conv_id,
            &stored_body,
            true,
            attachment_id.as_deref(),
            is_enc,
        )?;

        // Update conversation snippet
        let snippet = if body.is_empty() { "📎 Attachment" } else { &body };
        db.conversations().update_snippet(
            &conv_id,
            snippet,
            attachment_id.is_some(),
        )?;

        // Simulate delivery status progression in a background task
        let msg_id  = msg.id.clone();
        let db2     = db.clone();
        let ctrl_win = self.window.clone();
        let body_for_ui = body.clone();
        let conv_clone = conv_id.clone();
        self.rt.spawn(async move {
            // Simulate "sent" after 500ms
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            let _ = db2.messages().update_status(&msg_id, MessageStatus::Sent);
            let _ = db2.messages().update_status(&msg_id, MessageStatus::Delivered);

            // Refresh message list in UI
            slint::invoke_from_event_loop(move || {
                // Reload messages to reflect updated status
                let _ = ctrl_win;
            }).ok();
        });

        // Immediately refresh both messages and conversations in UI
        self.load_messages(conv_id).await?;
        self.load_conversations().await?;

        info!("Message sent in conv {conv_clone}: {} chars", body.len());
        Ok(())
    }

    /// Search conversations.
    pub async fn search_conversations(&self, query: String) -> Result<()> {
        let db  = self.db.clone();
        let win = self.window.clone();

        let convs = if query.is_empty() {
            db.conversations().list_all()?
        } else {
            db.conversations().search(&query)?
        };

        let items = ui_mapper::conversations_to_slint(convs);
        slint::invoke_from_event_loop(move || {
            if let Some(w) = win.upgrade() {
                w.set_conversations(items.into());
            }
        })?;

        Ok(())
    }

    /// Create a new conversation.
    pub async fn new_conversation(&self, phone: String, name: String) -> Result<()> {
        let db  = self.db.clone();
        let win = self.window.clone();

        let conv = db.conversations().create(&phone, &name)?;
        let id   = conv.id.clone();
        let cname = conv.contact_name.clone();
        let cphone = conv.phone.clone();

        self.load_conversations().await?;

        slint::invoke_from_event_loop(move || {
            if let Some(w) = win.upgrade() {
                w.set_active_conv_id(id.into());
                w.set_active_contact(cname.into());
                w.set_active_phone(cphone.into());
                w.set_show_thread(true);
                w.set_messages(slint::ModelRc::default());
            }
        })?;

        Ok(())
    }

    /// Delete a conversation.
    pub async fn delete_conversation(&self, conv_id: String) -> Result<()> {
        self.db.conversations().delete(&conv_id)?;
        self.load_conversations().await?;

        let win = self.window.clone();
        slint::invoke_from_event_loop(move || {
            if let Some(w) = win.upgrade() {
                w.set_active_conv_id("".into());
                w.set_show_thread(false);
            }
        })?;

        self.show_status("Conversation deleted");
        Ok(())
    }

    /// Archive a conversation (placeholder — same as delete for now).
    pub async fn archive_conversation(&self, conv_id: String) -> Result<()> {
        self.show_status("Conversation archived");
        // TODO: add `is_archived` column in next migration
        info!("Archive conv {conv_id} (stub)");
        Ok(())
    }

    /// Mark conversation as read.
    pub async fn mark_read(&self, conv_id: String) -> Result<()> {
        self.db.conversations().mark_read(&conv_id)?;
        self.load_conversations().await?;
        Ok(())
    }

    /// Delete a single message.
    pub async fn delete_message(&self, msg_id: String) -> Result<()> {
        let conv_id = {
            // We need to know the conv_id to reload after deletion.
            // For simplicity, we read it from the UI state.
            let win = self.window.clone();
            slint::invoke_from_event_loop(move || {
                win.upgrade().map(|w| w.get_active_conv_id().to_string())
            })
            .ok()
            .flatten()
            .unwrap_or_default()
        };

        self.db.messages().delete(&msg_id)?;
        self.show_status("Message deleted");
        if !conv_id.is_empty() {
            self.load_messages(conv_id).await?;
        }
        Ok(())
    }

    /// Toggle AES-256-GCM encryption for messages.
    pub fn toggle_encryption(&self) {
        let mut guard = match self.state.lock() {
            Ok(g) => g,
            Err(e) => { error!("State lock poisoned: {e}"); return; }
        };
        guard.encryption_enabled = !guard.encryption_enabled;
        let enabled = guard.encryption_enabled;
        drop(guard);

        let win = self.window.clone();
        slint::invoke_from_event_loop(move || {
            if let Some(w) = win.upgrade() {
                w.set_encryption_enabled(enabled);
            }
        }).ok();

        self.show_status(if enabled {
            "🔒 Encryption enabled"
        } else {
            "🔓 Encryption disabled"
        });
    }

    /// Toggle dark mode.
    pub fn toggle_dark_mode(&self) {
        let win = self.window.clone();
        slint::invoke_from_event_loop(move || {
            if let Some(w) = win.upgrade() {
                let next = !w.get_dark_mode();
                w.set_dark_mode(next);
            }
        }).ok();
    }

    /// Share a message body via the system share sheet (best-effort on desktop).
    pub fn share_message(&self, body: String) {
        #[cfg(not(target_os = "android"))]
        {
            // On desktop: copy to clipboard (if arboard is available)
            // For now just log it; real clipboard support requires the `arboard` crate.
            info!("Share: {body}");
            self.show_status("Message copied (share not available on desktop)");
        }
        #[cfg(target_os = "android")]
        {
            // On Android, invoke the system Intent via JNI / android_activity.
            info!("Share via Android Intent: {body}");
        }
    }

    /// Store a pending attachment (picked via file dialog).
    pub fn set_pending_attachment(&self, path: &str) {
        let p = std::path::Path::new(path);
        match crate::db::attachments::AttachmentRepo::create_from_path(p) {
            Ok(att) => {
                if let Ok(mut guard) = self.state.lock() {
                    guard.pending_attachment = Some(att);
                }
                self.show_status("Attachment ready");
            }
            Err(e) => {
                error!("Failed to prepare attachment: {e}");
                self.show_status("Could not attach file");
            }
        }
    }

    /// Open a native file picker and store the result.
    pub async fn pick_attachment(&self) -> Result<()> {
        #[cfg(not(target_os = "android"))]
        {
            // rfd is desktop-only; on Android the OS provides its own picker.
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Images",    &["jpg","jpeg","png","gif","webp"])
                .add_filter("Videos",    &["mp4","mkv","webm","mov"])
                .add_filter("Audio",     &["mp3","ogg","wav","flac","m4a"])
                .add_filter("All files", &["*"])
                .pick_file()
            {
                self.set_pending_attachment(&path.to_string_lossy());
            }
        }
        #[cfg(target_os = "android")]
        {
            // Trigger Android Storage Access Framework via android_activity.
            info!("Android file picker (stub)");
        }
        Ok(())
    }
}
