use std::path::PathBuf;
use clap::Parser;

use crate::{core, cli};

/// The command to scan a directory and record a snapshot
#[derive(Parser, Debug)]
pub struct Snapshot {
    /// Path to the directory to scan
    #[arg(default_value = ".")]
    path: PathBuf,
}

impl Snapshot {
    /// Execute the command to scan a directory and record a snapshot
    pub fn execute(&self, cli: &cli::Args) -> Result<(), Box<dyn std::error::Error>> {
        core::snapshot::take_snapshot(&self.path, cli.db.as_ref())
    }
}
