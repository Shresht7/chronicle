use std::path::{Path, PathBuf};

use gix::bstr::ByteSlice;

use crate::utils::file_lister;
use crate::{database, models, utils};

pub fn take_snapshot(path: &Path, db_path_override: Option<&PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let root = std::fs::canonicalize(path)?;

    if is_git_repository(&root) {
        println!("Git repository detected, creating snapshot from HEAD");
        take_snapshot_from_git(&root, db_path_override)
    } else {
        println!("Scanning directory: {}", root.display());
        take_snapshot_from_fs(&root, db_path_override)
    }
}

fn is_git_repository(path: &Path) -> bool {
    gix::discover(path).is_ok()
}

fn take_snapshot_from_git(root: &Path, db_path_override: Option<&PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let repo = gix::open(root)?;
    let head = repo.head_commit()?;
    let tree = head.tree()?;

    let committer = head.committer()?;
    let commit_time_str = committer.time; // This is a &str

    // Parse the Unix timestamp from the string. Example: "1766575549 +0530"
    let parts: Vec<&str> = commit_time_str.split_whitespace().collect();
    let unix_timestamp_str = parts
        .get(0)
        .ok_or("Failed to parse timestamp from committer.time")?;
    let unix_timestamp = unix_timestamp_str.parse::<u64>()?;

    let timestamp = std::time::UNIX_EPOCH + std::time::Duration::from_secs(unix_timestamp);

    let mut files = Vec::new();
    let mut recorder = gix::traverse::tree::Recorder::default();

    tree.traverse().breadthfirst(&mut recorder)?;

    for entry in recorder.records {
        if !entry.mode.is_blob() {
            continue;
        }

        let object = repo.find_object(entry.oid)?;
        let blob = object.try_into_blob()?;
        let content_hash = utils::hashing::compute_blake3_hash(&blob.data);

        files.push(models::FileMetadata {
            path: entry.filepath.to_path()?.to_path_buf(),
            bytes: blob.data.len() as u64,
            modified_at: Some(timestamp),
            created_at: None,
            accessed_at: None,
            content_hash: Some(content_hash),
        });
    }

    // Sort files by path to ensure deterministic order, same as the fs version
    files.sort_by(|a, b| a.path.cmp(&b.path));

    let snapshot = models::Snapshot {
        root: root.to_path_buf(),
        timestamp,
        git_commit_hash: Some(head.id().to_string()),
        files,
    };

    database::store_snapshot(snapshot, db_path_override)
}

fn take_snapshot_from_fs(root: &Path, db_path_override: Option<&PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let files = file_lister::list_files_with_metadata(root)?;

    // Create Snapshot
    let snapshot = models::Snapshot {
        root: root.to_path_buf(),
        timestamp: std::time::SystemTime::now(),
        git_commit_hash: None,
        files,
    };

    database::store_snapshot(snapshot, db_path_override)
}


