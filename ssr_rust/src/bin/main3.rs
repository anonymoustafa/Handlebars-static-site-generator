use std::io;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::error::Error;
use std::fs::File;
fn main() -> Result<(), Box<dyn Error>> {
    let root_path = ".";
    enumerate_files(root_path)?;
    Ok(())
}

fn enumerate_files<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
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