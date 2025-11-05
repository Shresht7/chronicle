use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    /// Summary statistics
    pub summary: SnapshotSummary,
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
    /// Content hash (for detecting renames/moves)
    pub hash: Option<String>,
    /// Number of lines (if text file)
    pub lines: Option<usize>,
}

/// Aggregated summary statistics for a snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotSummary {
    pub total_files: usize,
    pub total_size: u64,
    pub total_lines: usize,
    pub file_type_breakdown: HashMap<String, FileTypeStats>,
    pub directory_breakdown: HashMap<PathBuf, DirectoryStats>,
    pub total_directories: usize,
    pub total_symlinks: usize,
}

/// Statistics for a particular file type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeStats {
    pub count: usize,
    pub total_size: u64,
    pub total_lines: usize,
}

/// Statistics for a directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryStats {
    pub file_count: usize,
    pub total_size: u64,
    pub depth: usize,
}
