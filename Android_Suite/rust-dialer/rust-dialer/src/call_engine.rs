// src/call_engine.rs
// ─────────────────────────────────────────────────────────────────────────────
// Call simulation engine.
//
// On a real device this module would interface with the Android Telecom API
// via the JNI bridge.  On desktop / during development it simulates a call
// with a state machine and a background timer thread.
// ─────────────────────────────────────────────────────────────────────────────

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// ─── Call State ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum CallState {
    Idle,
    Dialing { contact_name: String, phone: String },
    Connected { contact_name: String, phone: String, started_at: Instant },
    Ending,
}

impl CallState {
    pub fn is_active(&self) -> bool {
        !matches!(self, Self::Idle)
    }

    pub fn status_text(&self) -> &'static str {
        match self {
            Self::Idle      => "",
            Self::Dialing { .. }  => "Dialing…",
            Self::Connected { .. } => "Connected",
            Self::Ending    => "Call ended",
        }
    }

    pub fn duration_text(&self) -> String {
        match self {
            Self::Connected { started_at, .. } => {
                let secs = started_at.elapsed().as_secs();
                format!("{}:{:02}", secs / 60, secs % 60)
            }
            _ => String::new(),
        }
    }

    pub fn elapsed_secs(&self) -> u32 {
        match self {
            Self::Connected { started_at, .. } => started_at.elapsed().as_secs() as u32,
            _ => 0,
        }
    }
}

// ─── CallEngine ──────────────────────────────────────────────────────────────

pub struct CallEngine {
    /// Shared state, updated by the timer thread and read by the UI thread
    pub state: Arc<Mutex<CallState>>,
}

impl CallEngine {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(CallState::Idle)),
        }
    }

    /// Initiate a simulated outgoing call.
    /// The call transitions Idle → Dialing → Connected after ~2 seconds.
    pub fn start_call(&self, contact_name: impl Into<String>, phone: impl Into<String>) {
        let name  = contact_name.into();
        let phone = phone.into();
        let state = Arc::clone(&self.state);

        // Transition to Dialing immediately
        {
            let mut s = state.lock().expect("lock poisoned");
            *s = CallState::Dialing { contact_name: name.clone(), phone: phone.clone() };
        }

        // Background thread: simulate ring time → answer
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(2)); // simulate ring

            let mut s = state.lock().expect("lock poisoned");
            if matches!(*s, CallState::Dialing { .. }) {
                *s = CallState::Connected {
                    contact_name: name,
                    phone,
                    started_at: Instant::now(),
                };
            }
        });
    }

    /// Hang up the current call.  Returns (contact_name, phone, duration_secs).
    pub fn hang_up(&self) -> Option<(String, String, u32)> {
        let mut s = self.state.lock().expect("lock poisoned");
        let result = match &*s {
            CallState::Dialing { contact_name, phone } => {
                Some((contact_name.clone(), phone.clone(), 0u32))
            }
            CallState::Connected { contact_name, phone, .. } => {
                Some((contact_name.clone(), phone.clone(), s.elapsed_secs()))
            }
            _ => None,
        };
        *s = CallState::Idle;
        result
    }

    /// Returns current contact name (if in a call)
    pub fn contact_name(&self) -> Option<String> {
        let s = self.state.lock().expect("lock poisoned");
        match &*s {
            CallState::Dialing  { contact_name, .. } |
            CallState::Connected { contact_name, .. } => Some(contact_name.clone()),
            _ => None,
        }
    }

    /// Returns current phone number (if in a call)
    pub fn phone(&self) -> Option<String> {
        let s = self.state.lock().expect("lock poisoned");
        match &*s {
            CallState::Dialing  { phone, .. } |
            CallState::Connected { phone, .. } => Some(phone.clone()),
            _ => None,
        }
    }
}

impl Default for CallEngine {
    fn default() -> Self { Self::new() }
}
