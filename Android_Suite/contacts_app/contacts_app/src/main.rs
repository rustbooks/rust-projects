// src/main.rs
// Desktop binary entry point.
// All application logic is in src/app.rs (shared with Android).
// This file is intentionally minimal.

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("contacts_app=info"),
    ).init();
    log::info!("Contacts App v{} starting", env!("CARGO_PKG_VERSION"));
    contacts_app::app::run_app()
}
