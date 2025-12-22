use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

use crate::{models, utils};

pub fn list_files_with_metadata(root: &Path) -> Result<Vec<models::FileMetadata>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();

    // Create a walker to scan the directory
    let walker = WalkBuilder::new(root).build();

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
            .strip_prefix(root)
            .unwrap_or(full_path)
            .to_path_buf();

        // Print the metadata
        let metadata = models::FileMetadata {
            path: relative_path,
            bytes: metadata.len(),
            modified_at: metadata.modified().ok(),
            created_at: metadata.created().ok(),
            accessed_at: metadata.accessed().ok(),
            content_hash: utils::hashing::hash_file(&entry.path().to_path_buf()).ok(),
        };

        files.push(metadata);
    }

    // Sort files by path to ensure deterministic order
    files.sort_by(|a, b| a.path.cmp(&b.path));

    Ok(files)
}