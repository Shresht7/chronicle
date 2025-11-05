use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    /// Content hash (for detecting renames/moves)
    pub hash: Option<String>,
    /// Number of lines (if text file)
    pub lines: Option<usize>,
}
