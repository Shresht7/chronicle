use clap::{Parser, Subcommand};

pub mod commands;

/// The command-line-interface for the application
#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
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
    Diff(commands::Diff),
}

pub fn parse() -> Args {
    Args::parse()
}
