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
        println!("Scanning directory: {}", self.path);

        let mut files = Vec::new();

        // Create a walker to scan the directory
        let walker = WalkBuilder::new(&self.path).build();

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

            // Print the metadata;
            let metadata = models::FileMetadata {
                path: entry.path().to_path_buf(),
                bytes: metadata.len(),
                modified_at: metadata.modified().ok(),
                created_at: metadata.created().ok(),
                accessed_at: metadata.accessed().ok(),
                content_hash: utils::hash_file(&entry.path().to_path_buf()).ok(),
            };

            files.push(metadata);
        }

        // Create Snapshot
        let snapshot = models::Snapshot {
            root: std::path::PathBuf::from(&self.path),
            timestamp: std::time::SystemTime::now(),
            files,
        };

        let db_path = utils::get_chronicle_db_path()?;
        let mut conn = database::open(&db_path)?;

        // Check if snapshot has changed
        if !database::snapshot_changed(
            &mut conn,
            &snapshot.root.to_string_lossy(),
            &snapshot.files,
        )? {
            println!("No changes detected");
            return Ok(());
        }

        // Insert Snapshot
        let snapshot_id = database::insert_snapshot(&mut conn, &snapshot)?;

        println!("Snapshot ID: {snapshot_id}");

        Ok(())
    }
}
