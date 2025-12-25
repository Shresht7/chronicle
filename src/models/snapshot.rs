use super::FileMetadata;

/// Represents a single snapshot of a directory at a point of time
#[derive(Debug)]
pub struct Snapshot {
    /// Root directory being observed
    pub root: std::path::PathBuf,
    /// When the snapshot was taken
    pub timestamp: std::time::SystemTime,
    /// The Git commit hash this snapshot corresponds to (if applicable)
    pub git_commit_hash: Option<String>,
    /// Files discovered in the snapshot
    pub files: Vec<FileMetadata>,
}
