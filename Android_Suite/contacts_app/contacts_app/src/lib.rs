// src/lib.rs
// Crate root.
// Slint-generated UI types are included here so both main.rs (desktop)
// and android/src/lib.rs can use them via `contacts_app::AppWindow` etc.

pub mod app;
pub mod contacts;
pub mod crypto;
pub mod csv_io;
pub mod models;
pub mod search;
pub mod storage;
pub mod vcard;

// Include Slint-generated code.
// This must appear exactly once per crate. We put it in lib.rs so both
// the binary (main.rs) and the Android cdylib can use the generated types.
slint::include_modules!();
