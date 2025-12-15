use clap::Parser;
use ignore::WalkBuilder;

use crate::models::FileMetadata;

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

            // Print the metadata
            let metadata = FileMetadata {
                name: entry.file_name().to_string_lossy().to_string(),
                path: entry.path().to_path_buf(),
                size: metadata.len(),
                modified: metadata.modified().ok(),
                created: metadata.created().ok(),
                accessed: metadata.accessed().ok(),
            };
            println!("{metadata:?}");
        }

        Ok(())
    }
}
