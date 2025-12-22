use std::path::PathBuf;
use std::time::SystemTime;

/// Represents the metadata of a snapshot, without the file list
#[derive(Debug, serde::Serialize)]
pub struct SnapshotMetadata {
    pub id: i64,
    pub root: PathBuf,
    pub timestamp: SystemTime,
    pub file_count: i64,
    pub total_size: i64,
}
