use std::path::Path;
use std::process::Command;

use crate::{database, models, utils};
use crate::utils::file_lister;

pub fn take_snapshot(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let root = std::fs::canonicalize(path)?;

    if is_git_repository(&root) {
        println!("Git repository detected, creating snapshot from HEAD");
        take_snapshot_from_git(&root)
    } else {
        println!("Scanning directory: {}", root.display());
        take_snapshot_from_fs(&root)
    }
}

fn is_git_repository(path: &Path) -> bool {
    Command::new("git")
        .arg("-C")
        .arg(path.as_os_str())
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
        .map(|output| output.status.success() && String::from_utf8_lossy(&output.stdout).trim() == "true")
        .unwrap_or(false)
}

fn take_snapshot_from_git(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("(Not yet implemented)");
    Ok(())
}

fn take_snapshot_from_fs(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let root = std::fs::canonicalize(path)?;
    let files = file_lister::list_files_with_metadata(&root)?;

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
