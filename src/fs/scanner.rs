use chrono::{DateTime, Utc};
use ignore::WalkBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::Path;

use super::helpers::{calculate_sha256, count_lines};
use crate::models::{
    file_metric::FileMetric,
    snapshot::Snapshot,
    snapshot_summary::{DirectoryStats, FileTypeStats, SnapshotSummary},
};

/// Scans a given directory and creates a `Snapshot` of its contents in parallel.
///
/// This function walks through the directory in parallel, collects metadata for each file,
/// and stores it as `FileMetric` within a `Snapshot`.
/// It respects `.gitignore` files by using `ignore::WalkBuilder`.
pub fn scan_directory(
    root_path: &Path,
    no_hash: bool,
    no_line_count: bool,
    ignore_patterns: &[String],
    max_size: Option<u64>,
    follow_symlinks: bool,
    hash_buffer_size: Option<usize>,
    line_count_buffer_size: Option<usize>,
) -> Result<Snapshot, Box<dyn std::error::Error>> {
    let timestamp = Utc::now();
    let id = uuid::Uuid::new_v4().to_string(); // Generate a unique ID for the snapshot

    // Initialize progress bar for user feedback
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message(format!("Scanning {}", root_path.display()));
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    // Configure the directory walker
    // It respects .gitignore by default and can be configured to follow symlinks
    let mut walk_builder = WalkBuilder::new(root_path);
    walk_builder.follow_links(follow_symlinks);
    // Add any additional ignore patterns provided via CLI
    for pattern in ignore_patterns {
        walk_builder.add_ignore(pattern);
    }

    // Walk the directory in parallel and collect file metrics
    let files: Vec<FileMetric> = walk_builder
        .build()
        .filter_map(|result| result.ok()) // Filter out errors during directory traversal
        .par_bridge() // Bridge to a parallel iterator for performance
        .filter_map(|entry| {
            let path = entry.path();
            let file_type = entry.file_type()?; // Get file type (file, dir, symlink)

            // Process only files and symbolic links
            if file_type.is_file() || file_type.is_symlink() {
                let metadata = entry.metadata().ok()?; // Get file metadata

                // Skip file if its size exceeds the maximum allowed size
                if let Some(max_s) = max_size {
                    if metadata.len() > max_s {
                        return None;
                    }
                }
                let modified: DateTime<Utc> = metadata.modified().ok()?.into();
                let created: Option<DateTime<Utc>> = metadata.created().ok().map(|t| t.into());

                // Determine file type string (extension or "symlink")
                let file_type_str = if file_type.is_symlink() {
                    "symlink".to_string()
                } else {
                    path.extension()
                        .and_then(|s| s.to_str()) // Get extension as string
                        .unwrap_or("") // Default to empty string if no extension
                        .to_string()
                };

                // Handle symlink specific metadata
                let (symlink_target, symlink_target_exists) = if file_type.is_symlink() {
                    let target_path = std::fs::read_link(path).ok(); // Resolve symlink target
                    let target_exists = target_path.as_ref().map(|p| p.exists()); // Check if target exists
                    (target_path, target_exists)
                } else {
                    (None, None)
                };

                // Conditionally calculate SHA-256 hash if not disabled and it's a regular file
                let hash = if !no_hash && file_type.is_file() {
                    calculate_sha256(path, hash_buffer_size)
                } else {
                    None
                };
                // Conditionally count lines if not disabled and it's a regular file
                let lines = if !no_line_count && file_type.is_file() {
                    count_lines(path, line_count_buffer_size)
                } else {
                    None
                };

                Some(FileMetric {
                    path: path.strip_prefix(root_path).ok()?.to_path_buf(), // Store path relative to root
                    size: metadata.len(),
                    modified: Some(modified),
                    created,
                    file_type: file_type_str,
                    symlink_target,
                    symlink_target_exists,
                    hash,
                    lines,
                })
            } else {
                None // Ignore directories, they are handled in summary calculation
            }
        })
        .collect(); // Collect all FileMetrics into a vector

    // Initialize summary statistics
    let mut summary = SnapshotSummary {
        total_files: files.len(),
        total_size: files.iter().map(|f| f.size).sum(),
        total_lines: files.iter().filter_map(|f| f.lines).sum(),
        file_type_breakdown: HashMap::new(),
        directory_breakdown: HashMap::new(),
        total_directories: 0, // Will be calculated from the directory_breakdown
        total_symlinks: files.iter().filter(|f| f.file_type == "symlink").count(),
    };

    // Populate file type and directory breakdown statistics
    for file_metric in &files {
        // Aggregate statistics by file type
        let file_type_stats = summary
            .file_type_breakdown
            .entry(file_metric.file_type.clone())
            .or_insert_with(|| FileTypeStats {
                count: 0,
                total_size: 0,
                total_lines: 0,
            });
        file_type_stats.count += 1;
        file_type_stats.total_size += file_metric.size;
        if let Some(l) = file_metric.lines {
            file_type_stats.total_lines += l;
        }

        // Aggregate statistics for each directory in the file's path
        let mut current_path = file_metric.path.parent();
        while let Some(dir) = current_path {
            if dir.as_os_str().is_empty() {
                break; // Stop if we reach the root of the relative path
            }
            let depth = dir.components().count();
            let dir_stats = summary
                .directory_breakdown
                .entry(dir.to_path_buf())
                .or_insert_with(|| DirectoryStats {
                    file_count: 0,
                    total_size: 0,
                    depth: 0,
                });
            dir_stats.file_count += 1;
            dir_stats.total_size += file_metric.size;
            dir_stats.depth = depth;
            current_path = dir.parent();
        }
    }

    // Finalize total directory count
    summary.total_directories = summary.directory_breakdown.len();

    // Finish progress bar and display summary message
    pb.finish_with_message(format!(
        "Scanned {} files and {} directories.",
        summary.total_files, summary.total_directories
    ));

    // Construct and return the final Snapshot
    Ok(Snapshot {
        id,
        timestamp,
        repo_path: root_path.to_path_buf(),
        files,
        summary,
    })
}
