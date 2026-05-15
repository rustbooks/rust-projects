// src/contacts.rs
// ContactManager — the core business logic layer.
// Owns the in-memory DataStore and persists changes to disk.
// No Slint types here; conversions live in main.rs.

use std::path::{Path, PathBuf};
use anyhow::{bail, Context, Result};
use chrono::Utc;
use log::{debug, info};
use uuid::Uuid;

use crate::models::{Contact, DataStore, Group, SortKey};
use crate::search::ContactSearch;
use crate::storage;
use crate::vcard;
use crate::csv_io;

// ── ContactManager ────────────────────────────────────────────────────────────

pub struct ContactManager {
    data_dir: PathBuf,
    store: DataStore,
    sort_key: SortKey,
    searcher: ContactSearch,
}

impl ContactManager {
    /// Loads (or creates) a contact store at the given data directory.
    pub fn new(data_dir: PathBuf) -> Result<Self> {
        let store = storage::load(&data_dir)?;
        let searcher = ContactSearch::new(&store.contacts);
        Ok(Self {
            data_dir,
            store,
            sort_key: SortKey::FirstName,
            searcher,
        })
    }

    // ── Query ─────────────────────────────────────────────────────────────────

    /// Returns all contacts sorted by the current sort key.
    pub fn all_sorted(&self) -> Vec<Contact> {
        let mut contacts = self.store.contacts.clone();
        self.sort_in_place(&mut contacts);
        contacts
    }

    /// Full-text search across name, phone, email, org, notes.
    /// Supports Devanagari/Marathi by using Unicode-aware comparison.
    pub fn search(&self, query: &str) -> Vec<Contact> {
        if query.trim().is_empty() {
            return self.all_sorted();
        }
        let mut results = self.searcher.search(query, &self.store.contacts);
        self.sort_in_place(&mut results);
        results
    }

    /// Returns a single contact by ID.
    pub fn get(&self, id: &str) -> Option<&Contact> {
        self.store.contacts.iter().find(|c| c.id == id)
    }

    pub fn groups(&self) -> Vec<Group> {
        // Update counts from actual contacts
        let mut groups = self.store.groups.clone();
        for group in &mut groups {
            group.count = self.store.contacts.iter()
                .filter(|c| c.group == group.name)
                .count();
        }
        groups
    }

    // ── Mutations ─────────────────────────────────────────────────────────────

    /// Creates or updates a contact.
    /// - New contact: generates UUID, sets created_at
    /// - Existing: updates updated_at, preserves created_at
    pub fn upsert(&mut self, mut contact: Contact) -> Result<String> {
        let now = Utc::now().to_rfc3339();

        // Validate: at least one name field required
        if contact.first_name.trim().is_empty() && contact.last_name.trim().is_empty() {
            bail!("Contact must have at least a first or last name");
        }

        // Sanitize input — trim whitespace
        contact.first_name = contact.first_name.trim().to_string();
        contact.last_name = contact.last_name.trim().to_string();
        contact.phone_primary = contact.phone_primary.trim().to_string();
        contact.email = contact.email.trim().to_string();
        contact.organization = contact.organization.trim().to_string();

        // Compute avatar color deterministically
        contact.avatar_color = Contact::compute_avatar_color(
            &contact.first_name, &contact.last_name
        );

        let id = if contact.id.is_empty() {
            // New contact
            let id = Uuid::new_v4().to_string();
            contact.id = id.clone();
            contact.created_at = now.clone();
            contact.updated_at = now;
            debug!("Creating new contact: {} {} (id={})", contact.first_name, contact.last_name, id);
            self.store.contacts.push(contact);
            id
        } else {
            // Update existing
            let id = contact.id.clone();
            contact.updated_at = now;
            if let Some(existing) = self.store.contacts.iter_mut().find(|c| c.id == id) {
                // Preserve original created_at
                let created_at = existing.created_at.clone();
                *existing = contact;
                existing.created_at = created_at;
                debug!("Updated contact id={id}");
            } else {
                // ID provided but not found — treat as new
                contact.created_at = contact.updated_at.clone();
                self.store.contacts.push(contact);
            }
            id
        };

        self.rebuild_search_index();
        self.persist()?;
        Ok(id)
    }

    /// Deletes a contact by ID.
    pub fn delete(&mut self, id: &str) -> Result<()> {
        let before = self.store.contacts.len();
        self.store.contacts.retain(|c| c.id != id);
        if self.store.contacts.len() == before {
            bail!("Contact not found: {id}");
        }
        info!("Deleted contact id={id}");
        self.rebuild_search_index();
        self.persist()
    }

    /// Toggles the favorite flag. Returns the new value.
    pub fn toggle_favorite(&mut self, id: &str) -> Result<bool> {
        let contact = self.store.contacts.iter_mut()
            .find(|c| c.id == id)
            .with_context(|| format!("Contact not found: {id}"))?;
        contact.is_favorite = !contact.is_favorite;
        let new_val = contact.is_favorite;
        debug!("Toggled favorite for id={id}: {new_val}");
        self.persist()?;
        Ok(new_val)
    }

    // ── Sort ──────────────────────────────────────────────────────────────────

    pub fn set_sort(&mut self, key: SortKey) {
        self.sort_key = key;
    }

    fn sort_in_place(&self, contacts: &mut Vec<Contact>) {
        match self.sort_key {
            SortKey::FirstName => contacts.sort_by(|a, b| {
                // Unicode-aware case-insensitive sort — works for Devanagari
                a.first_name.to_lowercase().cmp(&b.first_name.to_lowercase())
                    .then(a.last_name.to_lowercase().cmp(&b.last_name.to_lowercase()))
            }),
            SortKey::LastName => contacts.sort_by(|a, b| {
                a.last_name.to_lowercase().cmp(&b.last_name.to_lowercase())
                    .then(a.first_name.to_lowercase().cmp(&b.first_name.to_lowercase()))
            }),
            SortKey::Phone => contacts.sort_by(|a, b| {
                a.phone_primary.cmp(&b.phone_primary)
            }),
        }
    }

    // ── Groups ────────────────────────────────────────────────────────────────

    pub fn upsert_group(&mut self, mut group: Group) -> Result<()> {
        if group.name.trim().is_empty() {
            bail!("Group name cannot be empty");
        }
        group.name = group.name.trim().to_string();

        if let Some(existing) = self.store.groups.iter_mut().find(|g| g.id == group.id) {
            *existing = group;
        } else {
            if group.id.is_empty() { group.id = Uuid::new_v4().to_string(); }
            self.store.groups.push(group);
        }
        self.persist()
    }

    pub fn delete_group(&mut self, id: &str) -> Result<()> {
        let before = self.store.groups.len();
        self.store.groups.retain(|g| g.id != id);
        if self.store.groups.len() == before {
            bail!("Group not found: {id}");
        }
        self.persist()
    }

    // ── Import / Export ───────────────────────────────────────────────────────

    /// Imports contacts from a file. Detects format by extension.
    /// Returns the number of contacts imported.
    pub fn import_file(&mut self, path: &Path) -> Result<usize> {
        // Backup before bulk import
        storage::backup(&self.data_dir)?;

        let ext = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let imported: Vec<Contact> = match ext.as_str() {
            "vcf" => vcard::import(path)?,
            "csv" => csv_io::import(path)?,
            "json" => {
                let bytes = std::fs::read(path)?;
                let store: DataStore = serde_json::from_slice(&bytes)?;
                store.contacts
            }
            _ => bail!("Unsupported import format: .{ext}"),
        };

        let count = imported.len();
        for mut contact in imported {
            // Merge: update existing (by phone), add new
            let key = contact.phone_primary.clone();
            if !key.is_empty() {
                if let Some(existing) = self.store.contacts.iter_mut()
                    .find(|c| c.phone_primary == key)
                {
                    // Update but keep our ID and timestamps
                    let id = existing.id.clone();
                    let created_at = existing.created_at.clone();
                    *existing = contact;
                    existing.id = id;
                    existing.created_at = created_at;
                    existing.updated_at = Utc::now().to_rfc3339();
                    continue;
                }
            }
            // New contact
            if contact.id.is_empty() { contact.id = Uuid::new_v4().to_string(); }
            let now = Utc::now().to_rfc3339();
            if contact.created_at.is_empty() { contact.created_at = now.clone(); }
            contact.updated_at = now;
            contact.avatar_color = Contact::compute_avatar_color(
                &contact.first_name, &contact.last_name
            );
            self.store.contacts.push(contact);
        }

        self.rebuild_search_index();
        self.persist()?;
        info!("Imported {count} contacts");
        Ok(count)
    }

    /// Exports all contacts to a vCard file.
    pub fn export_vcf(&self, path: &Path) -> Result<usize> {
        let count = vcard::export(&self.store.contacts, path)?;
        info!("Exported {count} contacts to {}", path.display());
        Ok(count)
    }

    /// Exports all contacts to a CSV file.
    pub fn export_csv(&self, path: &Path) -> Result<usize> {
        csv_io::export(&self.store.contacts, path)
    }

    // ── Internal ─────────────────────────────────────────────────────────────

    fn persist(&self) -> Result<()> {
        storage::save(&self.data_dir, &self.store)
    }

    fn rebuild_search_index(&mut self) {
        self.searcher = ContactSearch::new(&self.store.contacts);
    }
}
