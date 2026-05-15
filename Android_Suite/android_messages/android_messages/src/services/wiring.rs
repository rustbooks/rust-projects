// src/services/wiring.rs
// Wires every Slint callback to an AppController method.
// Kept as a separate file so main.rs stays clean and callback bindings are
// all visible in one place.

use std::sync::Arc;
use log::error;

use crate::AppWindow;
use super::app_controller::AppController;

/// Wire all UI callbacks.  Called once from main() after window creation.
pub fn wire_callbacks(window: &AppWindow, ctrl: Arc<AppController>) {
    // ── Load messages ─────────────────────────────────────────────────────
    {
        let c = ctrl.clone();
        window.on_load_messages(move |conv_id| {
            let ctrl = c.clone();
            let id   = conv_id.to_string();
            tokio::spawn(async move {
                if let Err(e) = ctrl.load_messages(id).await {
                    error!("load_messages: {e}");
                }
            });
        });
    }

    // ── Send message ──────────────────────────────────────────────────────
    {
        let c = ctrl.clone();
        window.on_send_message(move |conv_id, body, attachment| {
            let ctrl = c.clone();
            let ci = conv_id.to_string();
            let b  = body.to_string();
            let a  = attachment.to_string();
            tokio::spawn(async move {
                if let Err(e) = ctrl.send_message(ci, b, a).await {
                    error!("send_message: {e}");
                }
            });
        });
    }

    // ── Delete conversation ───────────────────────────────────────────────
    {
        let c = ctrl.clone();
        window.on_delete_conversation(move |conv_id| {
            let ctrl = c.clone();
            let id   = conv_id.to_string();
            tokio::spawn(async move {
                if let Err(e) = ctrl.delete_conversation(id).await {
                    error!("delete_conversation: {e}");
                }
            });
        });
    }

    // ── Archive conversation ──────────────────────────────────────────────
    {
        let c = ctrl.clone();
        window.on_archive_conversation(move |conv_id| {
            let ctrl = c.clone();
            let id   = conv_id.to_string();
            tokio::spawn(async move {
                if let Err(e) = ctrl.archive_conversation(id).await {
                    error!("archive_conversation: {e}");
                }
            });
        });
    }

    // ── Search conversations ──────────────────────────────────────────────
    {
        let c = ctrl.clone();
        window.on_search_conversations(move |query| {
            let ctrl = c.clone();
            let q    = query.to_string();
            tokio::spawn(async move {
                if let Err(e) = ctrl.search_conversations(q).await {
                    error!("search_conversations: {e}");
                }
            });
        });
    }

    // ── New conversation ──────────────────────────────────────────────────
    {
        let c = ctrl.clone();
        window.on_new_conversation(move |phone, name| {
            let ctrl = c.clone();
            let p    = phone.to_string();
            let n    = name.to_string();
            tokio::spawn(async move {
                if let Err(e) = ctrl.new_conversation(p, n).await {
                    error!("new_conversation: {e}");
                }
            });
        });
    }

    // ── Pick attachment ───────────────────────────────────────────────────
    {
        let c = ctrl.clone();
        window.on_pick_attachment(move || {
            let ctrl = c.clone();
            tokio::spawn(async move {
                if let Err(e) = ctrl.pick_attachment().await {
                    error!("pick_attachment: {e}");
                }
            });
        });
    }

    // ── Share message ─────────────────────────────────────────────────────
    {
        let c = ctrl.clone();
        window.on_share_message(move |body| {
            c.share_message(body.to_string());
        });
    }

    // ── Toggle encryption ─────────────────────────────────────────────────
    {
        let c = ctrl.clone();
        window.on_toggle_encryption(move || {
            c.toggle_encryption();
        });
    }

    // ── Toggle dark mode ──────────────────────────────────────────────────
    {
        let c = ctrl.clone();
        window.on_toggle_dark_mode(move || {
            c.toggle_dark_mode();
        });
    }

    // ── Mark read ─────────────────────────────────────────────────────────
    {
        let c = ctrl.clone();
        window.on_mark_read(move |conv_id| {
            let ctrl = c.clone();
            let id   = conv_id.to_string();
            tokio::spawn(async move {
                if let Err(e) = ctrl.mark_read(id).await {
                    error!("mark_read: {e}");
                }
            });
        });
    }

    // ── Delete message ────────────────────────────────────────────────────
    {
        let c = ctrl.clone();
        window.on_delete_message(move |msg_id| {
            let ctrl = c.clone();
            let id   = msg_id.to_string();
            tokio::spawn(async move {
                if let Err(e) = ctrl.delete_message(id).await {
                    error!("delete_message: {e}");
                }
            });
        });
    }

    // ── Refresh conversations ─────────────────────────────────────────────
    {
        let c = ctrl.clone();
        window.on_refresh_conversations(move || {
            let ctrl = c.clone();
            tokio::spawn(async move {
                if let Err(e) = ctrl.load_conversations().await {
                    error!("refresh_conversations: {e}");
                }
            });
        });
    }

    // ── Open / close settings ─────────────────────────────────────────────
    {
        let win = window.as_weak();
        window.on_open_settings(move || {
            if let Some(w) = win.upgrade() {
                w.set_show_settings(true);
            }
        });
    }
    {
        let win = window.as_weak();
        window.on_close_settings(move || {
            if let Some(w) = win.upgrade() {
                w.set_show_settings(false);
            }
        });
    }
}
