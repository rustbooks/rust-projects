// src/platform.rs
// ============================================================================
// Platform Abstractions — Isolate platform-specific code here
// ============================================================================
// All #[cfg(target_os = "...")] blocks live here, keeping the rest of the
// codebase platform-agnostic. This is the "port" layer in hexagonal architecture.
// ============================================================================

/// Trigger haptic feedback / vibration.
/// On desktop: no-op (no vibration hardware).
/// On Android: calls the Vibrator service via JNI.
pub fn vibrate(duration_ms: u64) {
    #[cfg(target_os = "android")]
    {
        log::debug!("Android: vibrate {}ms", duration_ms);
        // In a full implementation: use ndk to call Vibrator.vibrate()
        // ndk::vibrator::Vibrator::new(activity).vibrate(duration_ms);
    }

    #[cfg(not(target_os = "android"))]
    {
        log::debug!("Desktop: vibrate {}ms (no-op)", duration_ms);
    }
}

/// Play a system notification sound.
pub fn play_alarm_sound(sound_name: &str) {
    log::info!("Playing alarm sound: {}", sound_name);
    // In a full implementation: use rodio to play bundled audio files.
    // let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    // ...
}

/// Post a system notification (for background alarm firing).
pub fn post_notification(title: &str, body: &str) {
    log::info!("Notification: {} — {}", title, body);
    #[cfg(target_os = "android")]
    {
        // Would use the Android Notification API via JNI
    }
    #[cfg(target_os = "linux")]
    {
        // Could use notify-rust for desktop notifications
        // notify_rust::Notification::new().summary(title).body(body).show();
    }
}

/// Get the system's preferred color scheme (dark/light).
pub fn prefers_dark_mode() -> bool {
    #[cfg(target_os = "android")]
    {
        // Read from Android Configuration.uiMode
        false // Placeholder
    }
    #[cfg(not(target_os = "android"))]
    {
        // On desktop, could check environment or system theme
        false
    }
}

/// Returns the screen DPI for responsive layout decisions.
pub fn get_screen_dpi() -> f32 {
    96.0 // Default — Slint handles DPI scaling automatically
}
