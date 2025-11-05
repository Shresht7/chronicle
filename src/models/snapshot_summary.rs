use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

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
