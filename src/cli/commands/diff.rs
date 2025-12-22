use clap::Parser;
use std::path::PathBuf;

use crate::{database, utils};
use crate::utils::file_lister; // Import the new file_lister module

/// The command to show the difference between the current directory state and the last snapshot
#[derive(Parser, Debug)]
pub struct Diff {
    /// Path to the directory to diff
    #[arg(default_value = ".")]
    path: PathBuf,
}

impl Diff {
    /// Execute the diff command
    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let root = std::fs::canonicalize(&self.path)?;
        println!("Computing diff for directory: {}", root.display());

        // Get current files metadata
        let current_files = file_lister::list_files_with_metadata(&root)?;

        // Open the database
        let db_path = utils::get_chronicle_db_path()?;
        let mut conn = database::open(&db_path)?;

        // Compute the diff against the last snapshot
        let diff = database::compute_diff(&mut conn, &root.to_string_lossy(), &current_files)?;

        if diff.is_empty() {
            println!("No changes detected between current state and last snapshot.");
            return Ok(())
        }

        println!("Changes detected:");
        if !diff.added.is_empty() {
            println!("\nAdded files:");
            for file in diff.added {
                println!("  + {}", file);
            }
        }

        if !diff.removed.is_empty() {
            println!("\nRemoved files:");
            for file in diff.removed {
                println!("  - {}", file);
            }
        }

        if !diff.modified.is_empty() {
            println!("\nModified files:");
            for file in diff.modified {
                println!("  * {}", file);
            }
        }

        Ok(())
    }
}
