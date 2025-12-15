use rusqlite::{Connection, Result, params};
use std::{collections::HashMap, path::Path, time::UNIX_EPOCH};

use crate::models::{FileMetadata, Snapshot};

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
    let timestamp = snapshot
        .timestamp
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Insert Snapshot Row
    conn.execute(
        "INSERT INTO snapshots (root, timestamp) VALUES (?1, ?2)",
        params![snapshot.root.to_string_lossy(), timestamp],
    )?;

    let snapshot_id = conn.last_insert_rowid();

    // Insert Files
    let tx = conn.transaction()?;
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

/// Returns true if the new snapshot differs from the last one
pub fn snapshot_changed(conn: &mut Connection, root: &str, files: &[FileMetadata]) -> Result<bool> {
    // Get last snapshot_id
    let last_id: Option<i64> = conn.query_row(
        "SELECT id FROM snapshots WHERE root = ?1 ORDER BY timestamp DESC LIMIT 1",
        [root],
        |row| row.get(0),
    )?;

    let Some(last_id) = last_id else {
        // No previous snapshot, must insert
        return Ok(true);
    };

    // Load previous files
    let mut stmt = conn.prepare("SELECT path, content_hash FROM files WHERE snapshot_id = ?1")?;
    let previous_files: HashMap<String, Option<String>> = stmt
        .query_map([last_id], |row| {
            let path: String = row.get(0)?;
            let content_hash: Option<String> = row.get(1)?;
            Ok((path, content_hash))
        })?
        .collect::<Result<HashMap<_, _>>>()?;

    // Compare
    if previous_files.len() != files.len() {
        return Ok(true);
    }

    for file in files {
        match previous_files.get(&file.path.to_string_lossy().to_string()) {
            Some(old_hash) if old_hash == &file.content_hash => continue,
            _ => return Ok(true), // file added, removed or changed
        }
    }

    Ok(false)
}
