use chrono::{DateTime, Utc};
use ignore::WalkBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::Path;

use super::helpers::{calculate_sha256, count_lines};
use crate::models::{DirectoryStats, FileMetric, FileTypeStats, Snapshot, SnapshotSummary};

/// Scans a given directory and creates a `Snapshot` of its contents in parallel.
///
/// This function walks through the directory in parallel, collects metadata for each file,
/// and stores it as `FileMetric` within a `Snapshot`.
/// It respects `.gitignore` files by using `ignore::WalkBuilder`.
pub fn scan_directory(
    root_path: &Path,
    no_hash: bool,
    no_line_count: bool,
) -> Result<Snapshot, Box<dyn std::error::Error>> {
    let timestamp = Utc::now();
    let id = uuid::Uuid::new_v4().to_string(); // Placeholder for unique ID

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message(format!("Scanning {}", root_path.display()));
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    // Walk the directory and bridge to a parallel iterator
    let files: Vec<FileMetric> = WalkBuilder::new(root_path)
        .build()
        .filter_map(|result| result.ok())
        .par_bridge()
        .filter_map(|entry| {
            let path = entry.path();
            let file_type = entry.file_type()?;

            if file_type.is_file() || file_type.is_symlink() {
                let metadata = entry.metadata().ok()?;
                let modified: DateTime<Utc> = metadata.modified().ok()?.into();
                let created: Option<DateTime<Utc>> = metadata.created().ok().map(|t| t.into());

                let file_type_str = if file_type.is_symlink() {
                    "symlink".to_string()
                } else {
                    path.extension()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_string()
                };

                let (symlink_target, symlink_target_exists) = if file_type.is_symlink() {
                    let target_path = std::fs::read_link(path).ok();
                    let target_exists = target_path.as_ref().map(|p| p.exists());
                    (target_path, target_exists)
                } else {
                    (None, None)
                };

                let hash = if !no_hash && file_type.is_file() {
                    calculate_sha256(path)
                } else {
                    None
                };
                let lines = if !no_line_count && file_type.is_file() {
                    count_lines(path)
                } else {
                    None
                };

                Some(FileMetric {
                    path: path.strip_prefix(root_path).ok()?.to_path_buf(),
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
                None
            }
        })
        .collect();

    // Now, calculate the summary from the collected files
    let mut summary = SnapshotSummary {
        total_files: files.len(),
        total_size: files.iter().map(|f| f.size).sum(),
        total_lines: files.iter().filter_map(|f| f.lines).sum(),
        file_type_breakdown: HashMap::new(),
        directory_breakdown: HashMap::new(),
        total_directories: 0, // This will be calculated from the directory_breakdown
        total_symlinks: files.iter().filter(|f| f.file_type == "symlink").count(),
    };

    for file_metric in &files {
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

        let mut current_path = file_metric.path.parent();
        while let Some(dir) = current_path {
            if dir.as_os_str().is_empty() {
                break;
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

    summary.total_directories = summary.directory_breakdown.len();

    pb.finish_with_message(format!(
        "Scanned {} files and {} directories.",
        summary.total_files, summary.total_directories
    ));

    Ok(Snapshot {
        id,
        timestamp,
        repo_path: root_path.to_path_buf(),
        files,
        summary,
    })
}
