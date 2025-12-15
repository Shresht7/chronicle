use rusqlite::{Connection, Result, params};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
    time::UNIX_EPOCH,
};

use crate::models::{Diff, FileMetadata, Snapshot};

/// Opens (or Creates) the Chronicle database at the given path
pub fn open(path: &Path) -> Result<Connection> {
    let mut conn = Connection::open(path)?;
    initialize_schema(&mut conn)?;
    Ok(conn)
}

// Create tables if they don't exist
fn initialize_schema(conn: &mut Connection) -> Result<()> {
    conn.execute_batch(include_str!("schema.sql"))?;
    Ok(())
}

pub fn insert_snapshot(conn: &mut Connection, snapshot: &Snapshot) -> Result<i64> {
    let tx = conn.transaction()?;

    let timestamp = snapshot
        .timestamp
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Insert Snapshot Row
    tx.execute(
        "INSERT INTO snapshots (root, timestamp) VALUES (?1, ?2)",
        params![snapshot.root.to_string_lossy(), timestamp],
    )?;

    let snapshot_id = tx.last_insert_rowid();

    // Insert Files
    for file in &snapshot.files {
        let modified = file
            .modified_at
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64);
        let created = file
            .created_at
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64);
        let accessed = file
            .accessed_at
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64);

        tx.execute(
            "INSERT INTO files
            (snapshot_id, path, bytes, modified_at, created_at, accessed_at, content_hash)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                snapshot_id,
                file.path.to_string_lossy(),
                file.bytes,
                modified,
                created,
                accessed,
                file.content_hash
            ],
        )?;
    }

    tx.commit()?;

    Ok(snapshot_id)
}

pub fn compute_diff(conn: &mut Connection, root: &str, files: &[FileMetadata]) -> Result<Diff> {
    use rusqlite::OptionalExtension;

    // Get last snapshot
    let last_id: Option<i64> = conn
        .query_row(
            "SELECT id FROM snapshots WHERE root = ?1 ORDER BY timestamp DESC LIMIT 1",
            [root],
            |row| row.get(0),
        )
        .optional()?;

    let last_id = match last_id {
        Some(id) => id,
        None => {
            // No previous snapshots -> everything is new
            return Ok(Diff {
                added: files
                    .iter()
                    .map(|f| f.path.to_string_lossy().to_string())
                    .collect(),
                removed: vec![],
                modified: vec![],
            });
        }
    };

    // Load previous files: path -> content_hash
    let mut stmt = conn.prepare("SELECT path, content_hash FROM files WHERE snapshot_id = ?1")?;
    let previous_files: HashMap<String, Option<String>> = stmt
        .query_map([last_id], |row| {
            let path: String = row.get(0)?;
            let content_hash: Option<String> = row.get(1)?;
            Ok((path, content_hash))
        })?
        .collect::<Result<_>>()?;

    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut modified = Vec::new();

    let current_paths: HashSet<String> = files
        .iter()
        .map(|f| f.path.to_string_lossy().to_string())
        .collect();
    let previous_paths: HashSet<String> = previous_files.keys().cloned().collect();

    // Added files
    for f in current_paths.difference(&previous_paths) {
        added.push(f.clone());
    }

    // Removed files
    for f in previous_paths.difference(&current_paths) {
        removed.push(f.clone());
    }

    // Modified files (present in both, different hash)
    for f in current_paths.intersection(&previous_paths) {
        let new_file = files
            .iter()
            .find(|x| x.path.to_string_lossy() == *f)
            .unwrap();
        let old_hash = previous_files.get(f).unwrap();
        if old_hash != &new_file.content_hash {
            modified.push(f.clone());
        }
    }

    Ok(Diff {
        added,
        removed,
        modified,
    })
}
