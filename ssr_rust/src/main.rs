use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Open a file in write-only mode, creating it if it doesn't exist
    let mut file = File::create("output.txt")?;

    // Use write! macro to write formatted string to the file
    writeln!(file, "Hello, world!")?;
    writeln!(file, "This is another line.")?;
    
    // If everything went well, return Ok
    Ok(())
}
