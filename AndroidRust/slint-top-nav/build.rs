fn main() {
    if let Err(e) = slint_build::compile("ui/main.slint") {
        eprintln!("Slint compilation failed: {:?}", e);
        std::process::exit(1);
    }
}