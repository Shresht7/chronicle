use crate::models::Snapshot;
use rusqlite::{Connection, Result, params};
use std::time::UNIX_EPOCH;

pub fn insert_snapshot(conn: &mut Connection, snapshot: &Snapshot) -> Result<i64> {
    let tx = conn.transaction()?;

    let timestamp = snapshot
        .timestamp
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Insert Snapshot Row
    tx.execute(
        "INSERT INTO snapshots (root, timestamp, git_commit_hash) VALUES (?1, ?2, ?3)",
        params![
            snapshot.root.to_string_lossy(),
            timestamp,
            snapshot.git_commit_hash
        ],
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
                file.bytes as i64, // Cast u64 to i64 for SQLite
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::initialize_schema;
    use crate::models::FileMetadata;
    use rusqlite::{Connection, params};
    use std::path::PathBuf;
    use std::time::SystemTime;

    fn create_in_memory_db() -> Connection {
        let mut conn = Connection::open_in_memory().unwrap();
        initialize_schema(&mut conn).unwrap();
        conn
    }

    #[test]
    fn test_insert_snapshot() {
        let mut conn = create_in_memory_db();
        let snapshot = Snapshot {
            root: PathBuf::from("/tmp"),
            timestamp: SystemTime::now(),
            files: vec![
                FileMetadata {
                    path: PathBuf::from("file1.txt"),
                    bytes: 123,
                    modified_at: Some(SystemTime::now()),
                    created_at: Some(SystemTime::now()),
                    accessed_at: Some(SystemTime::now()),
                    content_hash: Some("hash1".to_string()),
                },
                FileMetadata {
                    path: PathBuf::from("file2.txt"),
                    bytes: 456,
                    modified_at: Some(SystemTime::now()),
                    created_at: Some(SystemTime::now()),
                    accessed_at: Some(SystemTime::now()),
                    content_hash: Some("hash2".to_string()),
                },
            ],
            git_commit_hash: None,
        };

        let snapshot_id = insert_snapshot(&mut conn, &snapshot).unwrap();
        assert_eq!(snapshot_id, 1);

        let mut stmt = conn
            .prepare("SELECT root, timestamp FROM snapshots WHERE id = ?1")
            .unwrap();
        stmt.query_row(params![snapshot_id], |row| {
            let root: String = row.get(0).unwrap();
            let timestamp: i64 = row.get(1).unwrap();
            assert_eq!(root, "/tmp");
            assert_eq!(
                timestamp,
                snapshot
                    .timestamp
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64
            );
            Ok(())
        })
        .unwrap();

        let mut stmt = conn
            .prepare("SELECT COUNT(*) FROM files WHERE snapshot_id = ?1")
            .unwrap();
        let count: i64 = stmt
            .query_row(params![snapshot_id], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 2);
    }
}
