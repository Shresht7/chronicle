use clap::Parser;
use std::path::PathBuf;

use crate::{core, cli};

/// The command to synchronize Git history into chronicle
#[derive(Parser, Debug)]
pub struct Sync {
    /// Path to the Git repository to synchronize
    #[arg(default_value = ".")]
    path: PathBuf,
}

impl Sync {
    /// Execute the command to synchronize Git history
    pub fn execute(&self, cli: &cli::Args) -> Result<(), Box<dyn std::error::Error>> {
        println!("Synchronizing Git history from: {}", self.path.display());
        core::git_sync::sync_history(&self.path, cli.db.as_ref())
    }
}
