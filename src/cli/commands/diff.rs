use clap::Parser;
use std::path::PathBuf;

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
}

use crate::{core, database, models, utils};

impl Diff {
    /// Execute the diff command
    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let root = std::fs::canonicalize(&self.path)?;
        let db_path = utils::get_chronicle_db_path()?;
        let conn = database::open(&db_path)?;

        let (files1, rev1_name) = self.get_fileset(&conn, &root, &self.rev1, "HEAD")?;
        let (files2, rev2_name) = self.get_fileset(&conn, &root, &self.rev2, "HEAD~1")?;

        println!("Comparing {} with {}", rev1_name, rev2_name);

        let diff = core::diff::diff_snapshots(&files1, &files2)?;

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

        Ok(())
    }

    fn get_fileset(
        &self,
        conn: &rusqlite::Connection,
        root: &PathBuf,
        rev: &Option<String>,
        default_rev: &str,
    ) -> Result<(Vec<models::FileMetadata>, String), Box<dyn std::error::Error>> {
        let rev = rev.as_deref().unwrap_or(default_rev);

        if rev == "HEAD" {
            let files = utils::file_lister::list_files_with_metadata(root)?;
            return Ok((files, "current state".to_string()));
        }

        let snapshot_id = if rev == "HEAD~1" {
            database::get_penultimate_snapshot_id(conn, &root.to_string_lossy())?
        } else {
            rev.parse::<i64>().ok()
        };

        if let Some(id) = snapshot_id {
            let files = database::get_files_for_snapshot(conn, id)?;
            Ok((files, format!("snapshot {}", id)))
        } else {
            Err(format!("Could not find snapshot for revision: {}", rev).into())
        }
    }
}
