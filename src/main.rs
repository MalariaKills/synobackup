use std::fs;
use std::fs::read_dir;
use std::path::PathBuf;

//helper function - this function will be called inside main to actually walk through and copy all of the files inside each dir
fn copy_folder(
    source: &PathBuf,
    destination: &PathBuf,
    root: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();

        let relative_path = source_path.strip_prefix(source)?;
        let destination_path = destination.join(relative_path);

        if source_path.is_file() {
            if let Some(parent) = destination_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&source_path, &destination_path)?;
            println!(
                "File copied successfully from {:?} to {:?}",
                source_path, destination_path
            );
        } else if source_path.is_dir() {
            fs::create_dir_all(&destination_path)?;
            println!("Created Directory: {:?}", destination_path);
            copy_folder(&source_path, &destination_path, root)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from("/home/marcus/Documents/rust_test");
    let destination = PathBuf::from("/home/marcus/Desktop");

    for entry in read_dir(&root)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let folder_name = entry.file_name();
            let dest_path = destination.join(&folder_name);
            fs::create_dir_all(&dest_path)?;

            copy_folder(&path, &dest_path, &path)?;
        }
    }

    Ok(())
}
