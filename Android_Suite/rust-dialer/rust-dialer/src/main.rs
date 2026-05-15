// src/main.rs
// ─────────────────────────────────────────────────────────────────────────────
// RustDialer — cross-platform Material You dialer
// Architecture: MVU-inspired (Model=AppState, View=Slint, Update=callbacks)
// ─────────────────────────────────────────────────────────────────────────────

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_state;
mod call_engine;
mod models;
mod storage;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use slint::{ModelRc, SharedString, VecModel};

use app_state::{seed_contacts, AppState};
use call_engine::CallEngine;
use models::{CallDirection, SortMode};

slint::include_modules!();

// ── Mapping helpers ───────────────────────────────────────────────────────────

fn contact_to_slint(c: &models::Contact) -> ContactEntry {
    let argb = c.avatar_argb();
    ContactEntry {
        id:           SharedString::from(c.id.as_str()),
        display_name: SharedString::from(c.display_name().as_str()),
        first_name:   SharedString::from(c.first_name.as_str()),
        last_name:    SharedString::from(c.last_name.as_str()),
        phone:        SharedString::from(c.phone.as_str()),
        initials:     SharedString::from(c.initials().as_str()),
        is_favorite:  c.is_favorite,
        avatar_color: slint::Color::from_argb_u8(
            ((argb >> 24) & 0xFF) as u8,
            ((argb >> 16) & 0xFF) as u8,
            ((argb >>  8) & 0xFF) as u8,
            ( argb        & 0xFF) as u8,
        ),
    }
}

fn call_to_slint(r: &models::CallRecord) -> CallRecord {
    CallRecord {
        id:           SharedString::from(r.id.as_str()),
        contact_name: SharedString::from(r.contact_name.as_str()),
        phone:        SharedString::from(r.phone.as_str()),
        direction:    SharedString::from(r.direction.as_str()),
        duration_sec: r.duration_sec as i32,
        timestamp:    SharedString::from(r.friendly_timestamp().as_str()),
        initials:     SharedString::from(r.initials().as_str()),
    }
}

fn sync_ui(window: &AppWindow, state: &AppState) {
    window.set_filtered_contacts(ModelRc::new(VecModel::from(
        state.view.iter().map(contact_to_slint).collect::<Vec<_>>()
    )));
    window.set_contacts(ModelRc::new(VecModel::from(
        state.contacts.iter().map(contact_to_slint).collect::<Vec<_>>()
    )));
    window.set_favorites(ModelRc::new(VecModel::from(
        state.favorites().iter().map(|c| contact_to_slint(c)).collect::<Vec<_>>()
    )));
    window.set_call_history(ModelRc::new(VecModel::from(
        state.call_history.iter().map(call_to_slint).collect::<Vec<_>>()
    )));
}

fn show_snack(window: &AppWindow, msg: &str) {
    window.set_snack_text(SharedString::from(msg));
    window.set_show_snack(true);
    let handle = window.as_weak();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(2500));
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(w) = handle.upgrade() { w.set_show_snack(false); }
        });
    });
}

fn main() {
    #[cfg(feature = "logging")]
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info"),
    ).init();

    log::info!("RustDialer v{} starting", env!("CARGO_PKG_VERSION"));

    // Load state, with temp-dir fallback if storage fails
    let mut state = AppState::load().unwrap_or_else(|e| {
        log::error!("Storage load failed: {} — using temp fallback", e);
        let tmp = std::env::temp_dir().join("rust-dialer-fallback");
        let _ = std::fs::create_dir_all(&tmp);
        std::env::set_var("ANDROID_DATA_DIR", tmp.to_str().unwrap_or("."));
        AppState::load().expect("Temp fallback also failed")
    });

    if state.contacts.is_empty() {
        for c in seed_contacts() {
            let _ = state.add_contact(&c.first_name, &c.last_name, &c.phone);
        }
    }

    let window = AppWindow::new().expect("Failed to create AppWindow");
    sync_ui(&window, &state);
    window.set_sort_mode(state.sort_mode.to_int());

    let state  = Rc::new(RefCell::new(state));
    let engine = Rc::new(CallEngine::new());

    // Add Contact
    {
        let (s, w) = (Rc::clone(&state), window.as_weak());
        window.on_add_contact(move |first, last, phone| {
            let mut st = s.borrow_mut();
            let win = w.upgrade().unwrap();
            match st.add_contact(first.as_str(), last.as_str(), phone.as_str()) {
                Ok(_)  => { sync_ui(&win, &st); show_snack(&win, "Contact added"); }
                Err(e) => { show_snack(&win, &format!("Error: {}", e)); }
            }
        });
    }

    // Update Contact
    {
        let (s, w) = (Rc::clone(&state), window.as_weak());
        window.on_update_contact(move |id, first, last, phone| {
            let mut st = s.borrow_mut();
            let win = w.upgrade().unwrap();
            match st.update_contact(id.as_str(), first.as_str(), last.as_str(), phone.as_str()) {
                Ok(_)  => { sync_ui(&win, &st); show_snack(&win, "Contact updated"); }
                Err(e) => { show_snack(&win, &format!("Error: {}", e)); }
            }
        });
    }

    // Delete Contact
    {
        let (s, w) = (Rc::clone(&state), window.as_weak());
        window.on_delete_contact(move |id| {
            let mut st = s.borrow_mut();
            let win = w.upgrade().unwrap();
            st.delete_contact(id.as_str());
            sync_ui(&win, &st);
            show_snack(&win, "Contact deleted");
        });
    }

    // Toggle Favorite
    {
        let (s, w) = (Rc::clone(&state), window.as_weak());
        window.on_toggle_favorite(move |id| {
            let mut st = s.borrow_mut();
            let win = w.upgrade().unwrap();
            let fav = st.toggle_favorite(id.as_str()).unwrap_or(false);
            sync_ui(&win, &st);
            show_snack(&win, if fav { "Added to favorites ⭐" } else { "Removed from favorites" });
        });
    }

    // Search
    {
        let (s, w) = (Rc::clone(&state), window.as_weak());
        window.on_search_changed(move |q| {
            let mut st = s.borrow_mut();
            st.set_search(q.as_str());
            let win = w.upgrade().unwrap();
            sync_ui(&win, &st);
        });
    }

    // Sort
    {
        let (s, w) = (Rc::clone(&state), window.as_weak());
        window.on_sort_changed(move |mode| {
            let mut st = s.borrow_mut();
            st.set_sort_mode(SortMode::from_int(mode));
            let win = w.upgrade().unwrap();
            sync_ui(&win, &st);
        });
    }

    // Make Call
    {
        let eng = Rc::clone(&engine);
        let w   = window.as_weak();
        window.on_make_call(move |name, phone| {
            {
                let cs = eng.state.lock().expect("lock");
                if cs.is_active() {
                    show_snack(&w.upgrade().unwrap(), "Already in a call");
                    return;
                }
            }
            eng.start_call(name.as_str(), phone.as_str());
            let win = w.upgrade().unwrap();
            win.set_in_call(true);
            win.set_call_contact_name(name);
            win.set_call_phone(phone);
            win.set_call_status(SharedString::from("Dialing…"));
            win.set_call_duration(SharedString::from(""));

            let engine_state = Arc::clone(&eng.state);
            let handle       = w.clone();
            thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_secs(1));
                    let (status, duration, active) = {
                        let cs = engine_state.lock().expect("lock");
                        (SharedString::from(cs.status_text()),
                         SharedString::from(cs.duration_text()),
                         cs.is_active())
                    };
                    let (s2, d2, h2) = (status.clone(), duration.clone(), handle.clone());
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(w) = h2.upgrade() {
                            w.set_call_status(s2);
                            w.set_call_duration(d2);
                        }
                    });
                    if !active { break; }
                }
            });
        });
    }

    // Hang Up
    {
        let eng = Rc::clone(&engine);
        let s   = Rc::clone(&state);
        let w   = window.as_weak();
        window.on_hang_up_call(move || {
            if let Some((name, phone, dur)) = eng.hang_up() {
                let mut st = s.borrow_mut();
                st.record_call(&name, &phone, CallDirection::Outgoing, dur);
                let win = w.upgrade().unwrap();
                win.set_in_call(false);
                sync_ui(&win, &st);
                show_snack(&win, "Call ended");
            }
        });
    }

    log::info!("Entering Slint event loop");
    window.run().expect("Event loop error");
    log::info!("Shutdown complete");
}
