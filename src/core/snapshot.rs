use crate::core::git_sync;
use crate::core::scan; // Added this
use crate::{database, models};
use std::path::{Path, PathBuf}; // Added this // Added this

pub fn take_snapshot(
    path: &Path,
    db_path_override: Option<&PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = std::fs::canonicalize(path)?;

    if is_git_repository(&root) {
        println!("Git repository detected, synchronizing history up to HEAD...");
        git_sync::sync_history(&root, db_path_override) // Changed to git_sync::sync_history
    } else {
        println!("Scanning directory: {}", root.display());
        take_snapshot_from_fs(&root, db_path_override)
    }
}

fn is_git_repository(path: &Path) -> bool {
    gix::discover(path).is_ok()
}

fn take_snapshot_from_fs(
    root: &Path,
    db_path_override: Option<&PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let files = scan::scan(root)?;

    // Create Snapshot
    let snapshot = models::Snapshot {
        root: root.to_path_buf(),
        timestamp: std::time::SystemTime::now(),
        git_commit_hash: None,
        files,
    };

    database::store_snapshot(snapshot, db_path_override)
}
