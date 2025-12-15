use rusqlite::{Connection, Result, params};
use std::{path::Path, time::UNIX_EPOCH};

use crate::models::Snapshot;

/// Opens (or Creates) the Chronicle database at the given path
pub fn open(path: &Path) -> Result<Connection> {
    let conn = Connection::open(path)?;
    initialize_schema(&conn)?;
    Ok(conn)
}

// Create tables if they don't exist
fn initialize_schema(conn: &Connection) -> Result<()> {
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
