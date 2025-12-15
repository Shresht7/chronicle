use std::path::PathBuf;
use std::time::SystemTime;

/// Represents the metadata of a snapshot, without the file list
#[derive(Debug)]
pub struct SnapshotMetadata {
    pub id: i64,
    pub root: PathBuf,
    pub timestamp: SystemTime,
}
