use clap::Parser;
use ignore::WalkBuilder;

use crate::{database, models, utils};

/// The command to scan a directory and record a snapshot
#[derive(Parser, Debug)]
pub struct Snapshot {
    /// Path to the directory to scan
    #[arg(default_value = ".")]
    path: String,
}

impl Snapshot {
    /// Execute the command to scan a directory and record a snapshot
    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let root = std::fs::canonicalize(&self.path)?;
        println!("Scanning directory: {}", root.display());

        let mut files = Vec::new();

        // Create a walker to scan the directory
        let walker = WalkBuilder::new(&root).build();

        // Iterate over the entries in the directory
        for result in walker {
            let entry = match result {
                Ok(entry) => entry,
                Err(err) => {
                    eprintln!("Walk Error: {err}");
                    continue;
                }
            };

            // Skip directories for now
            if !entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                continue;
            }

            // Get the metadata of the file
            let metadata = entry.metadata()?;

            let full_path = entry.path();
            let relative_path = full_path
                .strip_prefix(&root)
                .unwrap_or(full_path)
                .to_path_buf();

            // Print the metadata
            let metadata = models::FileMetadata {
                path: relative_path,
                bytes: metadata.len(),
                modified_at: metadata.modified().ok(),
                created_at: metadata.created().ok(),
                accessed_at: metadata.accessed().ok(),
                content_hash: utils::hash_file(&entry.path().to_path_buf()).ok(),
            };

            files.push(metadata);
        }

        // Sort files by path to ensure deterministic order
        files.sort_by(|a, b| a.path.cmp(&b.path));

        // Create Snapshot
        let snapshot = models::Snapshot {
            root: root.clone(),
            timestamp: std::time::SystemTime::now(),
            files,
        };

        let db_path = utils::get_chronicle_db_path()?;
        let mut conn = database::open(&db_path)?;

        // Compute Diff
        let diff =
            database::compute_diff(&mut conn, &snapshot.root.to_string_lossy(), &snapshot.files)?;
        if diff.is_empty() {
            println!("No changes detected");
            return Ok(());
        }

        // Print Diff
        if diff.added.is_empty() && diff.removed.is_empty() && diff.modified.is_empty() {
            println!("No changes detected");
            return Ok(());
        }

        // Print summary
        println!("Snapshot detected changes:");
        if !diff.added.is_empty() {
            println!("  + {} added files", diff.added.len());
        }
        if !diff.removed.is_empty() {
            println!("  - {} removed files", diff.removed.len());
        }
        if !diff.modified.is_empty() {
            println!("  * {} modified files", diff.modified.len());
        }

        debug_assert!(
            snapshot.files.iter().all(|f| !f.path.is_absolute()),
            "FileMetadata paths must be relative"
        );

        // Insert Snapshot
        let snapshot_id = database::insert_snapshot(&mut conn, &snapshot)?;
        println!("Snapshot stored with id {}", snapshot_id);

        Ok(())
    }
}
