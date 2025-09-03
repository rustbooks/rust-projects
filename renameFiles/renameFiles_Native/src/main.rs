use std::fs;
use std::path::Path;

fn walk_dir(dir: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            // Recursively walk subdirectories
            walk_dir(&path)?;
        } else if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "markdown" {
                    let new_path = path.with_extension("md");
                    fs::rename(&path, &new_path)?;
                    println!("Renamed: {} -> {}", path.display(), new_path.display());
                }
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let root_dir = r"E:\ProgrammingLang\Rust\Books_AnirudhaGaikwad\JAVA-DSA-BOOKs\java_dsa_beginner\src\problemStatements";
    walk_dir(Path::new(root_dir))?;
    Ok(())
}