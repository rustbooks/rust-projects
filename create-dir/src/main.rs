use std::env;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if parent directory path is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <parent_directory_path>", args[0]);
        std::process::exit(1);
    }

    let parent_path = &args[1];
    
    // Verify parent directory exists or create it
    if !Path::new(parent_path).exists() {
        fs::create_dir(parent_path)?;
    }

    // Create directories ch1 through ch18
    for i in 1..=18 {
        let dir_name = format!("ch{}", i);
        let dir_path = Path::new(parent_path).join(dir_name);
        
        // Create directory if it doesn't exist
        if !dir_path.exists() {
            fs::create_dir(&dir_path)?;
            println!("Created directory: {}", dir_path.display());
        } else {
            println!("Directory already exists: {}", dir_path.display());
        }
    }

    Ok(())
}
