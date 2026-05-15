// src/lib.rs
// Library entry-point — exposes modules for integration tests.
// The `main` binary in main.rs depends on this library crate.

pub mod crypto;
pub mod db;
pub mod models;
pub mod services;
pub mod utils;
