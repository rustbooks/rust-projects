#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(app: android_activity::AndroidApp) {
    use slint::platform::Platform;
    slint::platform::set_platform(Box::new(slint::platform::android::AndroidPlatform::new(app)))
        .expect("Failed to set Android platform");
    
    // Initialize logger for Android
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(log::Level::Info)
            .with_tag("slint-top-nav"),
    );
    
    // Run the main application
    main::main().expect("Failed to run main application");
}

#[cfg(not(target_os = "android"))]
fn main() {
    // For non-Android platforms (e.g., desktop development)
    main::main().expect("Failed to run main application");
}

// Re-export main module to avoid duplication
mod main;
mod theme;
mod strings;
mod colors;
mod components;