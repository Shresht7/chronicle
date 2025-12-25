use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use serde_json;

/// Defines the possible output formats for the diff command.
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

/// The command to show the difference between snapshots or the current state
#[derive(Parser, Debug)]
pub struct Diff {
    /// The first revision to compare (e.g., a snapshot ID). Defaults to the current state.
    rev1: Option<String>,

    /// The second revision to compare (e.g., a snapshot ID). Defaults to the latest snapshot.
    rev2: Option<String>,

    /// Path to the directory the snapshots belong to
    #[arg(long, default_value = ".")]
    path: PathBuf,

    /// Output format
    #[arg(long, default_value_t = OutputFormat::Text)]
    format: OutputFormat,
}

use crate::{core, database, models, utils, cli};
use std::path::Path;

impl Diff {
    /// Execute the diff command
    pub fn execute(&self, cli: &cli::Args) -> Result<(), Box<dyn std::error::Error>> {
        let root = std::fs::canonicalize(&self.path)?;
        let db_path = utils::get_chronicle_db_path(cli.db.as_ref())?;
        let conn = database::open(&db_path)?;

        // Determine which revisions to compare based on the number of arguments
        let (files1, name1, files2, name2) = match (&self.rev1, &self.rev2) {
            // Case: `chronicle diff` (no args) -> compare last two snapshots
            (None, None) => {
                let (f1, n1) =
                    self.resolve_revision_to_fileset(&conn, &root, Some("HEAD~1"))?;
                let (f2, n2) = self.resolve_revision_to_fileset(&conn, &root, Some("HEAD"))?;
                (f1, n1, f2, n2)
            }
            // Case: `chronicle diff <rev>` -> compare working dir vs <rev>
            (Some(r1), None) => {
                let (f1, n1) = self.resolve_revision_to_fileset(&conn, &root, None)?; // None signifies working dir
                let (f2, n2) = self.resolve_revision_to_fileset(&conn, &root, Some(r1.as_str()))?;
                (f1, n1, f2, n2)
            }
            // Case: `chronicle diff <rev1> <rev2>` -> compare two snapshots
            (Some(r1), Some(r2)) => {
                let (f1, n1) = self.resolve_revision_to_fileset(&conn, &root, Some(r1.as_str()))?;
                let (f2, n2) = self.resolve_revision_to_fileset(&conn, &root, Some(r2.as_str()))?;
                (f1, n1, f2, n2)
            }
            // Should not be reachable with current clap config
            (None, Some(_)) => {
                return Err("Invalid combination of arguments".into());
            }
        };

        let diff = core::diff::diff_snapshots(&files1, &files2)?;

        match self.format {
            OutputFormat::Json => {
                let json_output = serde_json::to_string_pretty(&diff)?;
                println!("{}", json_output);
            }
            OutputFormat::Text => {
                println!("Comparing {} with {}", name1, name2);

                if diff.is_empty() {
                    println!("No changes detected.");
                    return Ok(());
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

    /// Takes a revision string and resolves it to a set of files and a display name
    fn resolve_revision_to_fileset(
        &self,
        conn: &rusqlite::Connection,
        root: &Path,
        rev: Option<&str>,
    ) -> Result<(Vec<models::FileMetadata>, String), Box<dyn std::error::Error>> {
        match rev {
            // If no revision is provided, use the current working directory
            None => {
                let files = utils::file_lister::list_files_with_metadata(root)?;
                Ok((files, "current files".to_string()))
            }
            Some(r_str) => {
                let snapshot_id_result = if r_str.eq_ignore_ascii_case("HEAD") {
                    database::get_latest_snapshot_id(conn, &root.to_string_lossy())
                } else if r_str.eq_ignore_ascii_case("HEAD~1") {
                    database::get_penultimate_snapshot_id(conn, &root.to_string_lossy())
                } else {
                    Ok(r_str.parse::<i64>().ok())
                };

                let snapshot_id = snapshot_id_result?.ok_or_else(|| {
                    if r_str.eq_ignore_ascii_case("HEAD~1") {
                        "Not enough snapshots to compare. Only one snapshot exists.".to_string()
                    } else {
                        format!("Could not find a snapshot for revision '{}'", r_str)
                    }
                })?;

                let files = database::get_files_for_snapshot(conn, snapshot_id)?;
                Ok((files, format!("snapshot {}", snapshot_id)))
            }
        }
    }
}
