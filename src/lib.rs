
use std::io::{BufReader, BufRead};
extern crate tempfile;
#[cfg(test)]
use tempfile::NamedTempFile;
use std::fs::File;
#[cfg(test)]
use std::io::{Write};
extern crate anyhow;
use anyhow::{Context, Result};


pub fn find_matches(reader: BufReader<File>, pattern: &str) -> Result<(), Box<dyn std::error::Error>>{
    for (i, line) in reader.lines().enumerate() {
        let content = line.with_context(|| format!("Could not read file"))?;
        if content.contains(&pattern) {
            println!("{} {}",i , content);
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