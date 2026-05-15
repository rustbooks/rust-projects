// build.rs
// This build script compiles all .slint UI definition files into Rust code
// at compile time. This means zero runtime parsing overhead — the UI is
// compiled directly into the binary, keeping startup time under 300ms.

fn main() {
    // Configure Slint compiler with optimal settings
    let config = slint_build::CompilerConfiguration::new()
        .with_style("material".into()); // Use Material 3 design system

    // Compile the main UI entry point.
    // All other .slint files are imported via @import in app.slint.
    slint_build::compile_with_config("ui/app.slint", config)
        .expect("Failed to compile Slint UI files. Check ui/app.slint for syntax errors.");

    // Tell Cargo to re-run this build script if any .slint file changes.
    println!("cargo:rerun-if-changed=ui/");
    println!("cargo:rerun-if-changed=build.rs");
}
