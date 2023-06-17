use std::io::{BufReader};
use std::fs::File;
use anyhow::{Result};
use log::{info};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}


fn main() -> Result<(), Box<dyn std::error::Error>>{
    env_logger::init();
    info!("Started");

    let args = Cli::parse();
    println!("Searching for pattern: '{0}', in {1}", args.pattern, args.path.display());

    let file = File::open(&args.path).expect("Could not read file");
    let reader = BufReader::new(file);
    grrs::find_matches(reader, &args.pattern)?;

    Ok(())
}

