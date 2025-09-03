use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // Define the root directory
    let root_dir = r"E:\ProgrammingLang\Rust\Books_AnirudhaGaikwad\JAVA-DSA-BOOKs\java_dsa_beginner\src\problemStatements";

    // Walk through the directory and its subfolders
    for entry in walkdir::WalkDir::new(root_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "markdown" {
                    // Construct new file path
                    let new_path = path.with_extension("md");
                    
                    // Rename the file
                    fs::rename(&path, &new_path)?;
                    println!("Renamed: {} -> {}", path.display(), new_path.display());
                }
            }
        }
    }
    
    Ok(())
}