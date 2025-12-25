use rusqlite::{Connection, OptionalExtension, Result, Row, params};

pub fn snapshot_exists(conn: &Connection, root: &str, git_commit_hash: &str) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM snapshots WHERE root = ?1 AND git_commit_hash = ?2",
        params![root, git_commit_hash],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

pub fn get_penultimate_snapshot_id(conn: &Connection, root: &str) -> Result<Option<i64>> {
    conn.query_row(
        "SELECT id FROM snapshots WHERE root = ?1 ORDER BY timestamp DESC LIMIT 1 OFFSET 1",
        [root],
        |row| row.get(0),
    )
    .optional()
}

use std::path::PathBuf;
use std::time::UNIX_EPOCH;

use crate::models::{FileMetadata, SnapshotMetadata};

pub fn get_files_for_snapshot(conn: &Connection, snapshot_id: i64) -> Result<Vec<FileMetadata>> {
    let mut stmt = conn.prepare(
        "SELECT
            path,
            bytes,
            modified_at,
            created_at,
            accessed_at,
            content_hash
        FROM
            files
        WHERE
            snapshot_id = ?1",
    )?;
    let file_iter = stmt.query_map([snapshot_id], |row| FileMetadata::try_from(row))?;

    let mut files = Vec::new();
    for file in file_iter {
        files.push(file?);
    }
    Ok(files)
}

impl TryFrom<&Row<'_>> for FileMetadata {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self> {
        let modified_at: Option<i64> = row.get(2)?;
        let created_at: Option<i64> = row.get(3)?;
        let accessed_at: Option<i64> = row.get(4)?;

        Ok(FileMetadata {
            path: PathBuf::from(row.get::<_, String>(0)?),
            bytes: row.get::<_, i64>(1)? as u64,
            modified_at: modified_at.map(|t| UNIX_EPOCH + std::time::Duration::from_secs(t as u64)),
            created_at: created_at.map(|t| UNIX_EPOCH + std::time::Duration::from_secs(t as u64)),
            accessed_at: accessed_at.map(|t| UNIX_EPOCH + std::time::Duration::from_secs(t as u64)),
            content_hash: row.get(5)?,
        })
    }
}

pub fn get_latest_snapshot_id(conn: &Connection, root: &str) -> Result<Option<i64>> {
    conn.query_row(
        "SELECT id FROM snapshots WHERE root = ?1 ORDER BY timestamp DESC LIMIT 1",
        [root],
        |row| row.get(0),
    )
    .optional()
}

pub fn list_snapshots_for_root(conn: &Connection, root: &str) -> Result<Vec<SnapshotMetadata>> {
    let mut stmt = conn.prepare(
        "SELECT
            s.id,
            s.root,
            s.timestamp,
            COUNT(f.id),
            SUM(f.bytes)
        FROM
            snapshots s
        JOIN
            files f ON s.id = f.snapshot_id
        WHERE
            s.root = ?1
        GROUP BY
            s.id
        ORDER BY
            s.timestamp DESC",
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
            file_count: row.get(3)?,
            total_size: row.get(4)?,
        })
    }
}
