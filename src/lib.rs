
use std::io::{BufReader, BufRead};
#[cfg(test)]
use tempfile::NamedTempFile;
use std::fs::File;
#[cfg(test)]
use std::io::{Write};
use anyhow::{Context, Result};


pub fn find_matches(reader: BufReader<File>, pattern: &str) -> Result<(), Box<dyn std::error::Error>>{
    for line in reader.lines() {
        let content = line.with_context(|| format!("Could not read file"))?;
        if content.contains(&pattern) {
            println!("{}", content);
        }
    }
    Ok(())
}


#[test]
fn test_find_matches() {
    let input = "Hello, World!\nThis is a test\nAnother line";
    let pattern = "test";

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(input.as_bytes()).unwrap();

    let file = temp_file.into_file();
    let reader = BufReader::new(file);

    find_matches(reader, pattern).unwrap();
}