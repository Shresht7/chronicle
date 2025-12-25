use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub mod commands;

/// The command-line-interface for the application
#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to the chronicle database file
    #[arg(long, global = true)]
    pub db: Option<PathBuf>,
}

/// The subcommands of the command-line-interface
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scan a directory and record a snapshot
    #[command(alias = "scan")]
    Snapshot(commands::Snapshot),

    /// List all snapshots for a given directory
    #[command(alias = "log")]
    List(commands::List),

    /// Show the difference between the current directory state and the last snapshot
    #[command(alias = "st")]
    Status(commands::Status),

    /// Show the difference between snapshots or the current state
    Diff(commands::Diff),

    /// Synchronize Git history into chronicle
    Sync(commands::Sync),
}

pub fn parse() -> Args {
    Args::parse()
}
