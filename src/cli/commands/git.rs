use clap::Parser;
use ignore::WalkBuilder;
use std::path::PathBuf;

use crate::{cli, core};

/// The command to manage Git repository synchronization
#[derive(Parser, Debug)]
pub struct Git {
    /// Path to the Git repository or a directory containing Git repositories
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Recursively find and synchronize Git repositories
    #[arg(long)]
    recurse: bool,
}

impl Git {
    /// Execute the command to manage Git repository synchronization
    pub fn execute(&self, cli: &cli::args::Args) -> Result<(), Box<dyn std::error::Error>> {
        if self.recurse {
            self._execute_recursive(cli)
        } else {
            println!("Synchronizing Git history from: {}", self.path.display());
            core::git_sync::sync_history(&self.path, cli.db.as_ref())
        }
    }

    fn _execute_recursive(&self, cli: &cli::args::Args) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "Recursively synchronizing Git history from: {}",
            self.path.display()
        );
        let walk = WalkBuilder::new(&self.path)
            .hidden(false) // Don't ignore hidden files/directories, we need to find .git
            .build();

        for result in walk {
            let entry = result?;
            // Check if it's a directory named ".git"
            if entry.file_type().is_some_and(|ft| ft.is_dir()) && entry.file_name() == ".git" {
                // Get the parent directory of the .git folder, which is the repository root
                if let Some(repo_path) = entry.path().parent() {
                    println!("Found Git repository: {}", repo_path.display());
                    // Synchronize history for this found repository
                    core::git_sync::sync_history(repo_path, cli.db.as_ref())?;
                }
            }
        }
        Ok(())
    }
}
