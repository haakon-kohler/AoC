use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_single_line(path: &str) -> io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let bytes_read = reader.read_line(&mut line)?;

    if bytes_read == 0 {
        // EOF reached before any line
        Err(io::Error::new(io::ErrorKind::UnexpectedEof, "file is empty"))
    } else {
        Ok(line)
    }
}

fn main() -> io::Result<()> {
    let line = read_single_line("input.txt")?;
    println!("First line: {}", line.trim_end());
    Ok(())
}
