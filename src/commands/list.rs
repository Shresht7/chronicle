use clap::Parser;
use std::path::PathBuf;

/// The command to list all snapshots for a given directory
#[derive(Parser, Debug)]
pub struct List {
    /// Path to the directory to list snapshots for
    #[arg(default_value = ".")]
    path: PathBuf,
}

impl List {
    /// Execute the command to list all snapshots for a given directory
    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Listing snapshots for: {:?}", self.path);
        Ok(())
    }
}
