use std::fs;
use std::path::Path;

fn main() {
    let root_path = "..";
    print!("{}", root_path);
    match enumerate_files(root_path, ".md") {
        Ok(_) => {},
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn enumerate_files<P: AsRef<Path>>(path: P, extension: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.as_ref();
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                enumerate_files(&path, extension)?;
            } else {
                let file_ext = path.extension().and_then(|s| s.to_str());
                if let Some(ext) = file_ext {
                    if ext == extension {
                        println!("{}", path.display());
                        // Perform your task on the file here
                        // For example:
                        // let mut file = fs::File::open(path)?;
                        // let mut contents = String::new();
                        // file.read_to_string(&mut contents)?;
                        // println!("File contents: {}", contents);
                    }
                }
            }
        }
    }
    Ok(())
}