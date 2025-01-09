use std::fs::File;
use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    let mut file = File::open("example.txt")?; // If this fails, the error propagates automatically
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    println!("File content: {}", content);
    Ok(()) // Indicate success
}
