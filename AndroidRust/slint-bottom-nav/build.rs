fn main() {
    // Explicitly specify the path to app.slint
    slint_build::compile("app.slint").expect("Failed to compile app.slint");
}
