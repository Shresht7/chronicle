use chrono::{DateTime, Utc};
use ignore::WalkBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::path::Path;

use super::helpers::{calculate_sha256, count_lines};
use crate::models::{DirectoryStats, FileMetric, FileTypeStats, Snapshot, SnapshotSummary};

/// Scans a given directory and creates a `Snapshot` of its contents.
///
/// This function walks through the directory, collects metadata for each file,
/// and stores it as `FileMetric` within a `Snapshot`.
/// It respects `.gitignore` files by using `ignore::WalkBuilder`.
pub fn scan_directory(root_path: &Path) -> Result<Snapshot, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    let mut summary = SnapshotSummary {
        total_files: 0,
        total_size: 0,
        total_lines: 0,
        file_type_breakdown: HashMap::new(),
        directory_breakdown: HashMap::new(),
        total_directories: 0,
        total_symlinks: 0,
    };

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

    // Walk the directory collecting the metadata for each file, respecting .gitignore
    for entry in WalkBuilder::new(root_path).build().filter_map(|e| e.ok()) {
        pb.inc(1);
        pb.set_message(format!(
            "Scanning {} - {}",
            root_path.display(),
            entry.path().display()
        ));

        let path = entry.path();
        let file_type = entry.file_type();

        if let Some(ft) = file_type {
            if ft.is_file() || ft.is_symlink() {
                let metadata = entry.metadata()?;
                let modified: DateTime<Utc> = metadata.modified()?.into();
                let created: Option<DateTime<Utc>> =
                    metadata.created().ok().and_then(|t| Some(t.into()));

                let file_type_str = if ft.is_symlink() {
                    "symlink".to_string()
                } else {
                    path.extension()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_string()
                };

                let (symlink_target, symlink_target_exists) = if ft.is_symlink() {
                    let target_path = std::fs::read_link(path).ok();
                    let target_exists = target_path.as_ref().map(|p| p.exists());
                    (target_path, target_exists)
                } else {
                    (None, None)
                };

                let hash = if ft.is_file() {
                    calculate_sha256(path)
                } else {
                    None
                };

                let lines = if ft.is_file() {
                    count_lines(path)
                } else {
                    None
                };

                let file_metric = FileMetric {
                    path: path.strip_prefix(root_path)?.to_path_buf(),
                    size: metadata.len(),
                    modified: Some(modified),
                    created,
                    file_type: file_type_str.clone(),
                    symlink_target,
                    symlink_target_exists,
                    hash,
                    lines,
                };

                // Update summary statistics for files/symlinks
                summary.total_files += 1;
                summary.total_size += file_metric.size;
                if let Some(l) = file_metric.lines {
                    summary.total_lines += l;
                }
                if ft.is_symlink() {
                    summary.total_symlinks += 1;
                }

                // Update file type breakdown
                let file_type_stats = summary
                    .file_type_breakdown
                    .entry(file_type_str)
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

                files.push(file_metric);
            } else if ft.is_dir() {
                summary.total_directories += 1;
            }
        }
    }

    // Calculate directory breakdown after all files are processed
    for file_metric in &files {
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
