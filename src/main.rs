mod cli;
mod matcher;
mod printer;

use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use matcher::literal::find_literal_str;
use matcher::regex::find_regex_pattern;
use std::fs;
use std::io;
use std::io::Read;

fn main() -> Result<()> {
    let args = Cli::parse();
    let pattern = args.pattern;

    let mut buffer = String::new();

    if let Some(path) = &args.path {
        if path.exists() {
            buffer = fs::read_to_string(path).context("Unable to read the file in path {path}")?;
        }
    } else {
        io::stdin()
            .read_to_string(&mut buffer)
            .context("Failed to read.")?;
    }

    if args.regex {
        find_regex_pattern(pattern, buffer);
    } else {
        find_literal_str(pattern, buffer);
    }

    Ok(())
}
