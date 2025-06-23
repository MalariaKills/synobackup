use std::fs;
use std::io;
use std::path::{Path, PathBuf};

// Cleaned-up recursive copy function that works for any folder
fn copy_folder(source: &Path, destination: &Path) -> Result<(), io::Error> {
    if !source.exists() {
        println!("Source path {:?} doesn't exist, skipping.", source);
        return Ok(());
    }

    if !destination.exists() {
        // Instead of just hoping the parent dir exists, we create everything needed
        fs::create_dir_all(destination)?;
        println!("Created target directory: {:?}", destination);
    }

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let entry_path = entry.path();
        let to_path = destination.join(entry.file_name());

        if entry.file_type()?.is_dir() {
            // Recursive call for nested directories — much simpler than my original version
            copy_folder(&entry_path, &to_path)?;
        } else if entry.file_type()?.is_file() {
            // Only copy actual files — we skip symlinks and we compare metadata size - skipping files if the size hasn't changed.
            let should_copy = match fs::metadata(&to_path) {
                Ok(dest_meta) => {
                    let src_meta = fs::metadata(&entry_path)?;
                    src_meta.len() != dest_meta.len()
                }
                Err(_) => true, // Destination file doesn't exist — definitely copy it
            };

            if should_copy {
                fs::copy(&entry_path, &to_path)?;
                println!("Copied file: {:?} -> {:?}", entry_path, to_path);
            } else {
                println!("Skipped (no changes): {:?}", entry_path);
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    // Source folder to copy
    let source_path = PathBuf::from("/home/marcus/Documents/rust_test");

    // Destination where folder should be replicated
    let destination_path = PathBuf::from("/home/marcus/Desktop");

    // Perform the folder copy
    copy_folder(&source_path, &destination_path)?;

    Ok(())
}
