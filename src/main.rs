use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let source = PathBuf::from("/home/marcus/Documents/rust_test/test_file.txt");
    let destination = PathBuf::from("/home/marcus/Desktop/test_file.txt");

    fs::copy(&source, &destination)?;
    println!("File copied successfully from {:?} to {:?}", source, destination);

    Ok(())
}