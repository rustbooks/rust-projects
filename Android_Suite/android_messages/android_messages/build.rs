// build.rs — compile all .slint files at build time.
// This runs before rustc so the generated Rust code is available as
// include!() macros in main.rs. Doing this at build time means the
// entire UI grammar is checked at compile time with zero runtime parsing.

fn main() {
    // Compile the root UI entry-point; it `import`s all sub-components.
    slint_build::compile("ui/app.slint")
        .expect("Slint compilation failed — check ui/app.slint for syntax errors");
}
