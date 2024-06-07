use std::io;
use std::io::Write;
use std::fs;
use std::fs::File;
fn main() -> io::Result<()> {
    let file_path = "kirkhar.md";
    let contents = fs::read_to_string(file_path)?;

    // Print the file contents
    

    // Open a file in write-only mode, creating it if it doesn't exist
    let mut file = File::create("output.html")?;

    // Use write! macro to write formatted string to the file
    writeln!(file, "{}", markdown::to_html(&contents))?;
    
    // If everything went well, return Ok
    Ok(())
}