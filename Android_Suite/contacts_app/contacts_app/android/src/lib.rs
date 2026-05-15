// android/src/lib.rs
// Android cdylib entry point — delegates to shared app logic in contacts_app::app.

#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("ContactsApp"),
    );
    slint::android::init(app).expect("Failed to init Slint Android backend");
    contacts_app::app::run_app().expect("Contacts App crashed");
}
