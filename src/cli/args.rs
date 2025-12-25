use clap::Parser;
use std::path::PathBuf;

use crate::cli::commands::Command;

/// The command-line-interface for the application
#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,

    /// Path to the chronicle database file
    #[arg(long, global = true)]
    pub db: Option<PathBuf>,
}

pub fn parse() -> Args {
    Args::parse()
}
