// build.rs - Compiles Slint UI files into Rust code
// This runs at build time, adding zero runtime overhead.
// Slint's compiler inlines the UI into the binary — no external UI files needed at runtime.

fn main() {
    // Compile the main UI entry point.
    // All .slint files imported from main.slint are automatically included.
    let config = slint_build::CompilerConfiguration::new()
        .with_style("material".to_string()); // Use Material 3 style

    slint_build::compile_with_config("ui/main.slint", config)
        .expect("Slint compilation failed — check ui/main.slint for syntax errors");

    // Re-run build script if any .slint file changes
    println!("cargo:rerun-if-changed=ui/");
    println!("cargo:rerun-if-changed=build.rs");
}
