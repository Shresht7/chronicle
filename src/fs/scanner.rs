// Library
use chrono::{DateTime, Utc};
use std::path::Path;
use walkdir::WalkDir;

use crate::models::{FileMetric, Snapshot};

/// Scans a given directory and creates a `Snapshot` of its contents.
///
/// This function walks through the directory, collects metadata for each file,
/// and stores it as `FileMetric` within a `Snapshot`.
pub fn scan_directory(root_path: &Path) -> Result<Snapshot, Box<dyn std::error::Error>> {
    let mut files = Vec::new();

    let timestamp = Utc::now();
    let id = uuid::Uuid::new_v4().to_string(); // Placeholder for unique ID

    // Walk the directory collecting the metadata for each file
    for entry in WalkDir::new(root_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let metadata = entry.metadata()?;
            let modified: DateTime<Utc> = metadata.modified()?.into();
            let created: Option<DateTime<Utc>> = metadata.created().ok()
                .and_then(|t| Some(t.into()));

            files.push(FileMetric {
                path: path.strip_prefix(root_path)?.to_path_buf(),
                size: metadata.len(),
                modified: Some(modified),
                created,
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
