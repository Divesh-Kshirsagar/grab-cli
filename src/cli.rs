use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
pub struct Cli {
    /// The pattern to find
    pub pattern: String,

    /// The optional path
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// The optional flag for regex search
    #[arg(short, long)]
    pub regex: bool,
}
