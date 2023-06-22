extern crate assert_cmd;
extern crate predicates;
extern crate assert_fs;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::{process::Command, alloc::System};
use assert_fs::prelude::*;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("max-grrs")?;
    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert().failure().stderr(predicate::str::contains("Could not read file"));
    
    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("Hello, World!\nThis is a test\nThis is another test\nAnother line")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2 This is a test\n3 This is another test"));

    
    Ok(())
}