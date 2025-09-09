use android_activity::AndroidApp;

slint::include_modules!();

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)] // Suppress FFI warning for AndroidApp
pub unsafe extern "C" fn android_main(app: AndroidApp) {
    slint::android::init(app).expect("Failed to initialize Slint Android backend");
    let ui = App::new().expect("Failed to create UI");
    ui.run().expect("Failed to run UI");
}