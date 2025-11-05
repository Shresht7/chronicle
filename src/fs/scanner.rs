// Library
use chrono::{DateTime, Utc};
use ignore::WalkBuilder;
use std::path::Path;

use crate::models::{FileMetric, Snapshot};

/// Scans a given directory and creates a `Snapshot` of its contents.
///
/// This function walks through the directory, collects metadata for each file,
/// and stores it as `FileMetric` within a `Snapshot`.
/// It respects `.gitignore` files by using `ignore::WalkBuilder`.
pub fn scan_directory(root_path: &Path) -> Result<Snapshot, Box<dyn std::error::Error>> {
    let mut files = Vec::new();

    let timestamp = Utc::now();
    let id = uuid::Uuid::new_v4().to_string(); // Placeholder for unique ID

    // Walk the directory collecting the metadata for each file, respecting .gitignore
    for entry in WalkBuilder::new(root_path).build().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let metadata = entry.metadata()?;
            let modified: DateTime<Utc> = metadata.modified()?.into();
            let created: Option<DateTime<Utc>> =
                metadata.created().ok().and_then(|t| Some(t.into()));

            let file_type = if metadata.is_symlink() {
                "symlink".to_string()
            } else {
                path.extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string()
            };

            files.push(FileMetric {
                path: path.strip_prefix(root_path)?.to_path_buf(),
                size: metadata.len(),
                modified: Some(modified),
                created,
                file_type,
            });
        }
    }

    Ok(Snapshot {
        id,
        timestamp,
        repo_path: root_path.to_path_buf(),
        files,
    })
}
