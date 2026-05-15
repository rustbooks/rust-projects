// src/storage.rs
// Handles reading and writing the contacts data store to disk.
// Uses JSON for portability + human readability + easy debugging.
//
// File location strategy:
//   - Linux/macOS: ~/.local/share/contacts-app/ (XDG_DATA_HOME)
//   - Windows:     %APPDATA%\contacts-app\
//   - Android:     /data/data/com.contacts.app/files/ (internal app storage)
// No root/admin required on any platform.

use std::path::{Path, PathBuf};
use std::fs;

use anyhow::{Context, Result};
use log::{debug, info, warn};

use crate::models::DataStore;

/// Returns the app-specific data directory, creating it if needed.
pub fn app_data_dir() -> PathBuf {
    let dir = resolve_data_dir();
    if !dir.exists() {
        if let Err(e) = fs::create_dir_all(&dir) {
            warn!("Could not create data dir {}: {e}", dir.display());
        }
    }
    dir
}

fn resolve_data_dir() -> PathBuf {
    #[cfg(target_os = "android")]
    {
        // Android internal app storage — no permission needed
        PathBuf::from("/data/data/com.contacts.app/files")
    }

    #[cfg(target_os = "windows")]
    {
        // Windows: %APPDATA%\contacts-app\
        std::env::var_os("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."))
            .join("contacts-app")
    }

    #[cfg(all(not(target_os = "android"), not(target_os = "windows")))]
    {
        // Linux / macOS: respect XDG_DATA_HOME, fallback to ~/.local/share
        let base = std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                std::env::var_os("HOME")
                    .map(PathBuf::from)
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join(".local")
                    .join("share")
            });
        base.join("contacts-app")
    }
}

// ── File Paths ────────────────────────────────────────────────────────────────

pub fn store_path(data_dir: &Path) -> PathBuf {
    data_dir.join("contacts.json")
}

// ── Load ──────────────────────────────────────────────────────────────────────

/// Loads the DataStore from disk. Returns a fresh empty store if not found.
pub fn load(data_dir: &Path) -> Result<DataStore> {
    let path = store_path(data_dir);
    if !path.exists() {
        info!("No data store found at {}; starting fresh", path.display());
        return Ok(DataStore::new());
    }
    load_json(&path)
}

fn load_json(path: &Path) -> Result<DataStore> {
    debug!("Loading contacts from {}", path.display());
    let bytes = fs::read(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    let store: DataStore = serde_json::from_slice(&bytes)
        .with_context(|| format!("Failed to parse {}", path.display()))?;
    info!("Loaded {} contacts, {} groups", store.contacts.len(), store.groups.len());
    Ok(store)
}

// ── Save ──────────────────────────────────────────────────────────────────────

/// Atomically saves the DataStore to disk.
/// Write-to-temp + rename prevents data corruption on power loss.
pub fn save(data_dir: &Path, store: &DataStore) -> Result<()> {
    let json_path = store_path(data_dir);

    #[cfg(debug_assertions)]
    let bytes = serde_json::to_vec_pretty(store).context("Failed to serialize DataStore")?;
    #[cfg(not(debug_assertions))]
    let bytes = serde_json::to_vec(store).context("Failed to serialize DataStore")?;

    // Write to temp first, then atomically rename
    let tmp_path = json_path.with_extension("tmp");
    fs::write(&tmp_path, &bytes)
        .with_context(|| format!("Failed to write temp file {}", tmp_path.display()))?;
    fs::rename(&tmp_path, &json_path)
        .with_context(|| format!("Failed to rename temp → {}", json_path.display()))?;

    debug!("Saved {} contacts", store.contacts.len());
    Ok(())
}

// ── Backup ────────────────────────────────────────────────────────────────────

/// Creates a timestamped backup. Called before bulk import operations.
pub fn backup(data_dir: &Path) -> Result<PathBuf> {
    let src = store_path(data_dir);
    if !src.exists() { return Ok(src); }

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_path = data_dir.join(format!("contacts_backup_{timestamp}.json"));
    fs::copy(&src, &backup_path)
        .with_context(|| format!("Failed to create backup at {}", backup_path.display()))?;
    info!("Backup created: {}", backup_path.display());
    Ok(backup_path)
}
