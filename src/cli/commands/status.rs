use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use serde_json;

use crate::{database, utils, cli};
use crate::utils::file_lister;

/// Defines the possible output formats for the status command.
#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Text, // Default format
    Json,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Text => write!(f, "text"),
            OutputFormat::Json => write!(f, "json"),
        }
    }
}

/// The command to show the difference between the current directory state and the last snapshot
#[derive(Parser, Debug)]
pub struct Status {
    /// Path to the directory to diff
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Output format
    #[arg(long, default_value_t = OutputFormat::Text)]
    format: OutputFormat,
}

impl Status {
    /// Execute the status command
    pub fn execute(&self, cli: &cli::Args) -> Result<(), Box<dyn std::error::Error>> {
        let root = std::fs::canonicalize(&self.path)?;
        
        let db_path = utils::get_chronicle_db_path(cli.db.as_ref())?;
        let mut conn = database::open(&db_path)?;

        // Get current files metadata
        let current_files = file_lister::list_files_with_metadata(&root)?;

        // Compute the diff against the last snapshot
        let diff = database::compute_diff(&mut conn, &root.to_string_lossy(), &current_files)?;

        match self.format {
            OutputFormat::Json => {
                let json_output = serde_json::to_string_pretty(&diff)?;
                println!("{}", json_output);
            }
            OutputFormat::Text => {
                println!("Computing status for directory: {}", root.display());

                if diff.is_empty() {
                    println!("No changes detected since last snapshot.");
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
            }
        }

        Ok(())
    }
}
