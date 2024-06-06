use std::fs;
use std::path::Path;

fn main() {
    let root_path = "..";
    print!("{}",root_path);
    match enumerate_files(root_path) {
        Ok(_) => {},
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn enumerate_files<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.as_ref();
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                enumerate_files(&path)?;
            } else {
                println!("{}", path.display());
            }
        }
    }
    Ok(())
}
