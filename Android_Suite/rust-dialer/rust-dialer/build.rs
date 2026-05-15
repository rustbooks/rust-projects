// build.rs — Invoked automatically by Cargo before compilation.
// Compiles all .slint files into Rust code that is linked into the binary.
// Slint's build step validates UI syntax, generates type-safe bindings,
// and embeds the compiled bytecode — zero runtime parsing overhead.

fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_style("material".into()); // Use Material You theme

    slint_build::compile_with_config("ui/dialer.slint", config)
        .expect("Slint compilation failed — check ui/dialer.slint for syntax errors");

    // Tell Cargo to re-run this build script if any .slint file changes
    println!("cargo:rerun-if-changed=ui/");
}
