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
        let file_type = entry.file_type();

        // Process only regular files and symlinks
        if let Some(ft) = file_type {
            if ft.is_file() || ft.is_symlink() {
                let metadata = entry.metadata()?;
                let modified: DateTime<Utc> = metadata.modified()?.into();
                let created: Option<DateTime<Utc>> =
                    metadata.created().ok().and_then(|t| Some(t.into()));

                let file_type_str = if ft.is_symlink() {
                    "symlink".to_string()
                } else {
                    path.extension()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_string()
                };

                let (symlink_target, symlink_target_exists) = if ft.is_symlink() {
                    let target_path = std::fs::read_link(path).ok();
                    let target_exists = target_path.as_ref().map(|p| p.exists());
                    (target_path, target_exists)
                } else {
                    (None, None)
                };

                files.push(FileMetric {
                    path: path.strip_prefix(root_path)?.to_path_buf(),
                    size: metadata.len(),
                    modified: Some(modified),
                    created,
                    file_type: file_type_str,
                    symlink_target,
                    symlink_target_exists,
                });
            }
        }
    }

    Ok(Snapshot {
        id,
        timestamp,
        repo_path: root_path.to_path_buf(),
        files,
    })
}
