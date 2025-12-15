use rusqlite::{Connection, Result, Row};
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

use crate::models::SnapshotMetadata;

pub fn list_snapshots_for_root(
    conn: &Connection,
    root: &str,
) -> Result<Vec<SnapshotMetadata>> {
    let mut stmt = conn.prepare(
        "SELECT id, root, timestamp FROM snapshots WHERE root = ?1 ORDER BY timestamp DESC",
    )?;
    let snapshot_iter = stmt.query_map([root], |row| SnapshotMetadata::try_from(row))?;

    let mut snapshots = Vec::new();
    for snapshot_metadata in snapshot_iter {
        snapshots.push(snapshot_metadata?);
    }
    Ok(snapshots)
}

impl TryFrom<&Row<'_>> for SnapshotMetadata {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self> {
        let timestamp_secs: i64 = row.get(2)?;
        let timestamp = UNIX_EPOCH + std::time::Duration::from_secs(timestamp_secs as u64);

        Ok(SnapshotMetadata {
            id: row.get(0)?,
            root: PathBuf::from(row.get::<_, String>(1)?),
            timestamp,
        })
    }
}