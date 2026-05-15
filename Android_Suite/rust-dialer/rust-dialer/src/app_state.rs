// src/app_state.rs — Central application state (Model in MVU)

use crate::models::{CallDirection, CallRecord, Contact, SortMode};
use crate::storage::{Storage, StorageError};

pub struct AppState {
    pub storage:      Storage,
    pub contacts:     Vec<Contact>,
    pub call_history: Vec<CallRecord>,
    pub sort_mode:    SortMode,
    pub search_query: String,
    /// Filtered + sorted contact view — re-computed after every mutation
    pub view:         Vec<Contact>,
}

impl AppState {
    /// Load persisted state from disk.
    pub fn load() -> Result<Self, StorageError> {
        let storage      = Storage::init()?;
        let contacts     = storage.load_contacts()?;
        let call_history = storage.load_call_history()?;
        let sort_mode    = storage.settings().sort_mode;
        let mut s = Self { storage, contacts, call_history, sort_mode,
                           search_query: String::new(), view: Vec::new() };
        s.rebuild_view();
        Ok(s)
    }

    /// Minimal in-memory state for unit tests / last-resort fallback.
    pub fn load_empty() -> Self {
        // Use a process-unique temp dir so parallel tests don't collide
        let tmp = std::env::temp_dir()
            .join(format!("rust-dialer-empty-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&tmp);
        std::env::set_var("ANDROID_DATA_DIR", tmp.to_str().unwrap_or("."));
        Self::load().expect("load_empty: temp dir init failed")
    }

    // ── CRUD ──────────────────────────────────────────────────────────────────

    pub fn add_contact(&mut self, first: &str, last: &str, phone: &str)
        -> Result<String, String>
    {
        let c = Contact::new(first, last, phone);
        c.validate()?;
        let id = c.id.clone();
        self.contacts.push(c);
        self.persist_contacts();
        self.rebuild_view();
        Ok(id)
    }

    pub fn update_contact(&mut self, id: &str, first: &str, last: &str, phone: &str)
        -> Result<(), String>
    {
        let pos = self.contacts.iter().position(|c| c.id == id)
            .ok_or_else(|| format!("Contact not found: {}", id))?;
        let mut tmp = self.contacts[pos].clone();
        tmp.first_name = first.to_string();
        tmp.last_name  = last.to_string();
        tmp.phone      = phone.to_string();
        tmp.validate()?;
        self.contacts[pos] = tmp;
        self.persist_contacts();
        self.rebuild_view();
        Ok(())
    }

    pub fn delete_contact(&mut self, id: &str) -> bool {
        let before = self.contacts.len();
        self.contacts.retain(|c| c.id != id);
        let deleted = self.contacts.len() < before;
        if deleted { self.persist_contacts(); self.rebuild_view(); }
        deleted
    }

    pub fn toggle_favorite(&mut self, id: &str) -> Option<bool> {
        let c = self.contacts.iter_mut().find(|c| c.id == id)?;
        c.is_favorite = !c.is_favorite;
        let val = c.is_favorite;
        self.persist_contacts();
        self.rebuild_view();
        Some(val)
    }

    // ── Call history ──────────────────────────────────────────────────────────

    pub fn record_call(&mut self, name: &str, phone: &str,
                       dir: CallDirection, dur: u32)
    {
        self.call_history.insert(0, CallRecord::new(name, phone, dir, dur));
        self.call_history.truncate(500);
        if let Err(e) = self.storage.save_call_history(&self.call_history) {
            log::error!("save call history: {}", e);
        }
    }

    // ── Search / Sort ─────────────────────────────────────────────────────────

    pub fn set_search(&mut self, q: &str) {
        self.search_query = q.to_string();
        self.rebuild_view();
    }

    pub fn set_sort_mode(&mut self, mode: SortMode) {
        self.sort_mode = mode;
        let mut settings = self.storage.settings().clone();
        settings.sort_mode = mode;
        if let Err(e) = self.storage.save_settings(&settings) {
            log::error!("save settings: {}", e);
        }
        self.rebuild_view();
    }

    // ── Computed views ────────────────────────────────────────────────────────

    pub fn favorites(&self) -> Vec<&Contact> {
        let mut v: Vec<&Contact> = self.contacts.iter().filter(|c| c.is_favorite).collect();
        Self::sort_slice(&mut v, self.sort_mode);
        v
    }

    // ── Private helpers ───────────────────────────────────────────────────────

    fn rebuild_view(&mut self) {
        let q = self.search_query.to_lowercase();
        let mut filtered: Vec<&Contact> = if q.is_empty() {
            self.contacts.iter().collect()
        } else {
            self.contacts.iter().filter(|c|
                c.display_name().to_lowercase().contains(&q)
                || c.phone.contains(&q)
                || c.first_name.to_lowercase().contains(&q)
                || c.last_name.to_lowercase().contains(&q)
            ).collect()
        };
        Self::sort_slice(&mut filtered, self.sort_mode);
        self.view = filtered.into_iter().cloned().collect();
    }

    fn sort_slice(v: &mut Vec<&Contact>, mode: SortMode) {
        v.sort_unstable_by(|a, b| match mode {
            SortMode::FirstName => a.first_name.to_lowercase()
                .cmp(&b.first_name.to_lowercase())
                .then(a.last_name.to_lowercase().cmp(&b.last_name.to_lowercase())),
            SortMode::LastName => a.last_name.to_lowercase()
                .cmp(&b.last_name.to_lowercase())
                .then(a.first_name.to_lowercase().cmp(&b.first_name.to_lowercase())),
            SortMode::PhoneNumber => a.phone.cmp(&b.phone),
        });
    }

    fn persist_contacts(&self) {
        if let Err(e) = self.storage.save_contacts(&self.contacts) {
            log::error!("save contacts: {}", e);
        }
    }
}

/// Demo contacts for first-launch seeding.
pub fn seed_contacts() -> Vec<Contact> {
    vec![
        Contact::new("Alice",   "Anderson",  "+1-555-0101"),
        Contact::new("Bob",     "Brown",     "+1-555-0102"),
        Contact::new("Charlie", "Chen",      "+1-555-0103"),
        Contact::new("Diana",   "Davis",     "+1-555-0104"),
        Contact::new("Ethan",   "Evans",     "+91-98765-43210"),
        Contact::new("Fiona",   "Foster",    "+44-7700-900123"),
        Contact::new("George",  "Garcia",    "+1-555-0107"),
        Contact::new("Hannah",  "Harris",    "+1-555-0108"),
        Contact::new("Ivan",    "Ibrahim",   "+7-916-123-4567"),
        Contact::new("Julia",   "Johnson",   "+1-555-0110"),
        Contact::new("Kevin",   "Kim",       "+82-10-1234-5678"),
        Contact::new("Laura",   "Lee",       "+1-555-0112"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_state() -> AppState {
        let mut s = AppState::load_empty();
        for c in seed_contacts() {
            let _ = s.add_contact(&c.first_name, &c.last_name, &c.phone);
        }
        s
    }

    #[test] fn add_contact_appears_in_view() {
        let mut s = make_state();
        let n = s.view.len();
        s.add_contact("Zara", "Zeta", "+9000000001").unwrap();
        assert_eq!(s.view.len(), n + 1);
    }

    #[test] fn delete_removes_from_view() {
        let mut s = make_state();
        let id = s.contacts[0].id.clone();
        s.delete_contact(&id);
        assert!(!s.view.iter().any(|c| c.id == id));
    }

    #[test] fn search_filters() {
        let mut s = make_state();
        s.set_search("alice");
        assert_eq!(s.view.len(), 1);
        assert_eq!(s.view[0].first_name, "Alice");
    }

    #[test] fn sort_last_name() {
        let mut s = make_state();
        s.set_sort_mode(SortMode::LastName);
        let names: Vec<_> = s.view.iter().map(|c| c.last_name.to_lowercase()).collect();
        let mut sorted = names.clone(); sorted.sort_unstable();
        assert_eq!(names, sorted);
    }
}
