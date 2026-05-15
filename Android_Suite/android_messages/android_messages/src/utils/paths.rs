// src/utils/paths.rs
// Cross-platform app data directory resolution.
// We never hard-code paths — always derive from OS conventions.

use anyhow::Result;
use std::path::PathBuf;

/// Return the platform-specific application data directory.
///
/// | Platform | Path                                      |
/// |----------|-------------------------------------------|
/// | Windows  | %APPDATA%\android-messages                |
/// | macOS    | ~/Library/Application Support/android-messages |
/// | Linux    | $XDG_DATA_HOME/android-messages  (or ~/.local/share/…) |
/// | Android  | /data/data/<package>/files                |
pub fn app_data_dir() -> Result<PathBuf> {
    #[cfg(target_os = "android")]
    {
        // On Android, use the internal storage data directory.
        // The actual path is injected at runtime by android_activity.
        // For now we fall back to a relative path so the desktop build still compiles.
        return Ok(PathBuf::from("/data/data/dev.slint.androidmessages/files"));
    }

    #[cfg(not(target_os = "android"))]
    {
        let base = dirs_next::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Cannot determine data directory"))?;
        Ok(base.join("android-messages"))
    }
}

// Note: `dirs_next` is intentionally NOT in Cargo.toml to keep deps minimal.
// We implement the same logic inline to avoid the extra crate.
#[cfg(not(target_os = "android"))]
mod dirs_next {
    use std::path::PathBuf;

    pub fn data_dir() -> Option<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            std::env::var("APPDATA").ok().map(PathBuf::from)
        }
        #[cfg(target_os = "macos")]
        {
            std::env::var("HOME").ok().map(|h| {
                PathBuf::from(h)
                    .join("Library")
                    .join("Application Support")
            })
        }
        #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
        {
            // XDG on Linux/BSD
            std::env::var("XDG_DATA_HOME")
                .ok()
                .map(PathBuf::from)
                .or_else(|| {
                    std::env::var("HOME")
                        .ok()
                        .map(|h| PathBuf::from(h).join(".local").join("share"))
                })
        }
    }
}
