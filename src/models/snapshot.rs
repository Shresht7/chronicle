use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::{file_metric::FileMetric, snapshot_summary::SnapshotSummary};

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
