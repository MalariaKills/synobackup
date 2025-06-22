use std::fs;
use std::fs::read_dir;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let source = PathBuf::from("/home/marcus/Documents/rust_test/2025-06-19");
    let destination = PathBuf::from("/home/marcus/Desktop");

    for entry in read_dir(&source)? {
        let entry = entry?;
        let source_path = entry.path();
        let file_name = entry.file_name();
        let destination_path = destination.join(file_name);

        fs::copy(&source_path, &destination_path)?;

        println!(
            "File copied successfully from {:?} to {:?}",
            source, destination
        );
    }

    Ok(())
}