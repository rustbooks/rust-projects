// src/storage.rs
// ============================================================================
// Persistent Storage — Cross-platform file I/O
// ============================================================================
// Uses the `directories` crate to find the correct data directory on each OS:
//   - Linux: ~/.local/share/android-clock/
//   - macOS: ~/Library/Application Support/android-clock/
//   - Windows: C:\Users\<User>\AppData\Roaming\android-clock\
//   - Android: /data/data/dev.slint.androidclock/files/
//
// All I/O is synchronous std::fs — no async needed for small state files.
// Errors are propagated via Result, never panicked.
// ============================================================================

use crate::model::AppState;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

const APP_QUALIFIER: &str = "dev";
const APP_ORGANIZATION: &str = "SlintClock";
const APP_NAME: &str = "android-clock";
const STATE_FILE: &str = "state.json";

/// Application storage manager.
/// Handles finding the right platform directory and reading/writing state.
pub struct AppStorage {
    data_dir: PathBuf,
}

impl AppStorage {
    /// Create a new storage instance, ensuring the data directory exists.
    pub fn new() -> Result<Self> {
        let data_dir = get_data_dir()?;

        // Create the directory if it doesn't exist (first run).
        // create_dir_all is idempotent — safe to call even if it exists.
        fs::create_dir_all(&data_dir)
            .with_context(|| format!("Failed to create data directory: {}", data_dir.display()))?;

        log::info!("Data directory: {}", data_dir.display());
        Ok(Self { data_dir })
    }

    /// Load the saved application state from disk.
    /// Returns Ok(None) if no state file exists (first run).
    /// Returns Err only on I/O or JSON parse errors.
    pub fn load_state(&self) -> Option<AppState> {
        let path = self.data_dir.join(STATE_FILE);

        if !path.exists() {
            log::info!("No saved state found, using defaults");
            return None;
        }

        match self.load_state_inner(&path) {
            Ok(state) => {
                log::info!("Loaded state: {} alarms", state.alarms.len());
                Some(state)
            }
            Err(e) => {
                // If the state file is corrupt, log and use defaults.
                // Do NOT panic — a corrupt save file must not crash the app.
                log::warn!("Failed to load state (using defaults): {}", e);
                None
            }
        }
    }

    fn load_state_inner(&self, path: &PathBuf) -> Result<AppState> {
        let data = fs::read_to_string(path)
            .with_context(|| format!("Failed to read state file: {}", path.display()))?;

        let state: AppState = serde_json::from_str(&data)
            .with_context(|| "Failed to parse state JSON — file may be corrupt")?;

        Ok(state)
    }

    /// Save the application state to disk atomically.
    /// We write to a temp file first, then rename, to avoid partial writes.
    pub fn save_state(&self, state: &AppState) -> Result<()> {
        let path = self.data_dir.join(STATE_FILE);
        let temp_path = self.data_dir.join(format!("{}.tmp", STATE_FILE));

        // Serialize to JSON with pretty-printing for human readability.
        // In production, you might use compact JSON to save space.
        let json = serde_json::to_string_pretty(state)
            .with_context(|| "Failed to serialize state to JSON")?;

        // Write to temp file first
        fs::write(&temp_path, &json)
            .with_context(|| format!("Failed to write temp state file: {}", temp_path.display()))?;

        // Atomic rename — on most filesystems, this is an atomic operation
        // that prevents data loss if the app is killed mid-write.
        fs::rename(&temp_path, &path).with_context(|| {
            format!("Failed to rename temp state file to: {}", path.display())
        })?;

        log::debug!("State saved to {}", path.display());
        Ok(())
    }

    /// Get the data directory path (for display in settings).
    pub fn data_dir(&self) -> &PathBuf {
        &self.data_dir
    }
}

/// Find the platform-appropriate data directory.
fn get_data_dir() -> Result<PathBuf> {
    // On Android, use a hardcoded path relative to the app's files directory.
    // On desktop, use `directories` for OS-standard locations.
    #[cfg(target_os = "android")]
    {
        Ok(PathBuf::from("/data/data/dev.slint.androidclock/files"))
    }

    #[cfg(not(target_os = "android"))]
    {
        let project_dirs =
            ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME).ok_or_else(|| {
                anyhow::anyhow!("Cannot determine data directory for this platform")
            })?;

        Ok(project_dirs.data_dir().to_path_buf())
    }
}
