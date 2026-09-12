mod cli;

use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use std::fs;
use std::io;
use std::io::Read;

fn main() -> Result<()> {
    let args = Cli::parse();
    let pattern = args.pattern;

    let mut buffer = String::new();

    if let Some(path) = &args.path {
        if path.exists() {
            fs::read_to_string(path).context("Unable to read the file in path {path}")?;
        }
    } else {
        io::stdin()
            .read_to_string(&mut buffer)
            .context("Failed to read.")?;
    }

    for (idx, line) in buffer.lines().enumerate() {
        let line_num = idx + 1;

        if line.contains(&pattern) {
            println!("Line {line_num} at {line}");
        }
    }

    Ok(())
}
