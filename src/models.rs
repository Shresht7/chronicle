// Library
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A complete snapshot of repository state at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// Unique identifier for this snapshot
    pub id: String,
    /// When this snapshot was taken
    pub timestamp: DateTime<Utc>,
    /// Path to the repository or directory
    pub repo_path: PathBuf,
    /// All files in this snapshot
    pub files: Vec<FileMetric>,
}

/// Metrics for a single file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetric {
    /// Relative path from repo root
    pub path: PathBuf,
    /// File size in bytes
    pub size: u64,
    /// Last modified time (filesystem snapshots)
    pub modified: Option<DateTime<Utc>>,
    /// Creation time (filesystem snapshots)
    pub created: Option<DateTime<Utc>>,
    /// File extension or type
    pub file_type: String,
    /// For symlinks, the path to the target
    pub symlink_target: Option<PathBuf>,
    /// For symlinks, whether the target exists
    pub symlink_target_exists: Option<bool>,
}
