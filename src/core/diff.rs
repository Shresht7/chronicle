use std::collections::{HashMap, HashSet};

use crate::models::{Diff, FileMetadata};

pub fn diff_snapshots(
    files1: &[FileMetadata],
    files2: &[FileMetadata],
) -> Result<Diff, Box<dyn std::error::Error>> {
    let files1_map: HashMap<String, Option<String>> = files1
        .iter()
        .map(|f| (f.path.to_string_lossy().to_string(), f.content_hash.clone()))
        .collect();

    let files2_map: HashMap<String, Option<String>> = files2
        .iter()
        .map(|f| (f.path.to_string_lossy().to_string(), f.content_hash.clone()))
        .collect();

    let files1_paths: HashSet<String> = files1_map.keys().cloned().collect();
    let files2_paths: HashSet<String> = files2_map.keys().cloned().collect();

    let added: Vec<String> = files2_paths
        .difference(&files1_paths)
        .cloned()
        .collect();

    let removed: Vec<String> = files1_paths
        .difference(&files2_paths)
        .cloned()
        .collect();

    let modified: Vec<String> = files1_paths
        .intersection(&files2_paths)
        .filter(|&path| {
            let hash1 = files1_map.get(path).unwrap();
            let hash2 = files2_map.get(path).unwrap();
            hash1 != hash2
        })
        .cloned()
        .collect();

    Ok(Diff {
        added,
        removed,
        modified,
    })
}