use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use walkdir::WalkDir;

fn main() -> std::io::Result<()> {
    let base_path = r"E:\ProgrammingLang\Rust\Books_AnirudhaGaikwad\rust-books\english\rustFundametals\src";

    // Recursively walk through the directory
    for entry in WalkDir::new(base_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
    {
        let path = entry.path();
        println!("Processing: {}", path.display());

        // Read the file content
        let mut content = String::new();
        File::open(path)?.read_to_string(&mut content)?;

        // Find the index of "### See also:"
        if let Some(index) = content.find("### See also:") {
            // Truncate content up to the start of "### See also:"
            content.truncate(index);
            // Remove any trailing whitespace or newlines
            content = content.trim_end().to_string();

            // Write the modified content back to the file
            let mut file = File::create(path)?;
            file.write_all(content.as_bytes())?;
            println!("Updated: {}", path.display());
        } else {
            println!("No '### See also:' section found in: {}", path.display());
        }
    }

    Ok(())
}