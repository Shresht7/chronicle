use std::path::Path;
use std::time::SystemTime;

use gix::bstr::ByteSlice;

use crate::utils::hashing;
use crate::{database, models, utils}; // Added utils back for get_chronicle_db_path

pub fn sync_history(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let repo = gix::open(path)?;
    let head = repo.head_commit()?;

    println!(
        "Starting Git history synchronization from: {}",
        path.display()
    );

    let db_path = utils::get_chronicle_db_path()?;
    let mut conn = database::open(&db_path)?;

    // Iterate through all commits
    let mut rev_walk = head.ancestors().all()?;
    while let Some(commit_id) = rev_walk.next() {
        let commit_id = commit_id?.id(); // Get the actual commit ID
        let commit = repo.find_object(commit_id)?.try_into_commit()?;
        let tree = commit.tree()?;

        // Idempotency check
        if database::snapshot_exists(&mut conn, &path.to_string_lossy(), &commit_id.to_string())? {
            println!("Skipping already synchronized commit: {}", commit_id);
            continue;
        }

        let committer = commit.committer()?;
        let commit_time_str = committer.time;
        let parts: Vec<&str> = commit_time_str.split_whitespace().collect();
        let unix_timestamp_str = parts
            .get(0)
            .ok_or("Failed to parse timestamp from committer.time")?;
        let unix_timestamp = unix_timestamp_str.parse::<u64>()?;
        let timestamp = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(unix_timestamp);

        let mut files = Vec::new();
        let mut recorder = gix::traverse::tree::Recorder::default();
        tree.traverse().breadthfirst(&mut recorder)?;

        for entry in recorder.records {
            if !entry.mode.is_blob() {
                continue;
            }

            let object = repo.find_object(entry.oid)?;
            let blob = object.try_into_blob()?;
            let content_hash = hashing::compute_blake3_hash(&blob.data);

            files.push(models::FileMetadata {
                path: entry.filepath.to_path()?.to_path_buf(),
                bytes: blob.data.len() as u64,
                modified_at: Some(timestamp),
                created_at: None,
                accessed_at: None,
                content_hash: Some(content_hash),
            });
        }
        files.sort_by(|a, b| a.path.cmp(&b.path));

        let snapshot = models::Snapshot {
            root: path.to_path_buf(),
            timestamp,
            git_commit_hash: Some(commit.id().to_string()),
            files,
        };

        // database::store_snapshot expects a mutable connection
        database::store_snapshot(snapshot)?;

        println!("Processed commit: {}", commit_id);
    }

    println!("Git history synchronization completed.");
    Ok(())
}


