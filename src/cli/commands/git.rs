use clap::Parser;
use std::path::PathBuf;

use crate::{cli, core};

/// The command to manage Git repository synchronization
#[derive(Parser, Debug)]
pub struct Git {
    /// Path to the Git repository to synchronize
    #[arg(default_value = ".")]
    path: PathBuf,
}

impl Git {
    /// Execute the command to manage Git repository synchronization
    pub fn execute(&self, cli: &cli::args::Args) -> Result<(), Box<dyn std::error::Error>> {
        println!("Synchronizing Git history from: {}", self.path.display());
        core::git_sync::sync_history(&self.path, cli.db.as_ref())
    }
}
