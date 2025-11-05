# Chronicle

A Rust-based tool to record, analyze, and visualize how files and projects evolve over time using both git history and filesystem snapshots.

**Chronicle** tracks the evolution of your code, notes, and projects - preserving the story of how they grow and change.

## Vision

Chronicle's core mission is to empower users to understand the historical evolution of their digital workspaces. By taking regular, efficient snapshots of specified directories, Chronicle builds a rich timeline of changes—documenting file additions, deletions, modifications, and structural shifts. This historical data then serves as the foundation for powerful analysis and visualization tools, allowing users to:

*   **Track Growth**: Monitor how projects, codebases, or personal notes grow in terms of file count, size, and lines of code over days, weeks, or months.
*   **Identify Trends**: Spot patterns in development, such as periods of high activity, refactoring efforts, or shifts in technology usage.
*   **Visualize Evolution**: Generate interactive graphs and charts that illustrate changes in directory structure, file types, and content over time.
*   **Optimize Workflows**: Gain insights into personal or team productivity by understanding where changes are concentrated and how different parts of a project evolve.

Ultimately, Chronicle aims to provide a "time machine" for your filesystem, enabling you to revisit past states, understand the journey of your projects, and make informed decisions about their future.

## Current Status

**Phase 0: Filesystem Snapshots - COMPLETE!**

This phase focused on building a robust filesystem snapshot capability, which is now fully implemented. The `snapshot` command has been enhanced to not only capture the current state of any directory but also to intelligently record and persist these snapshots, only storing new data when changes are detected. This lays the groundwork for efficient, automated historical tracking.

## Usage

### `chronicle snapshot <path> [options]`

Scans a directory and generates a JSON snapshot of its contents. By default, it outputs to `stdout`.

-   `<path>`: The target directory to scan. Defaults to the current directory (`.`).

**Options:**

-   `--output <FILE>`: Specifies an output file for the JSON snapshot. If omitted, the JSON is printed to `stdout`.
-   `--pretty`: Pretty-prints the JSON output for better readability. By default, the JSON is minified.
-   `--no-hash`: Disables SHA-256 content hashing for files. This can speed up scanning, especially for large files.
-   `--no-line-count`: Disables line counting for text files. This can also improve performance.
-   `--ignore <PATTERNS>`: Provides additional glob patterns to ignore during scanning (e.g., `"*.log"`, `"temp/"`). These patterns are applied in addition to any `.gitignore` files.
-   `--max-size <BYTES>`: Skips files larger than the specified size in bytes. For example, `--max-size 10485760` skips files larger than 10MB.
-   `--follow-symlinks`: Follows symbolic links during scanning. By default, symlinks are not followed.

**Examples:**

```sh
# Take a snapshot of the current directory and print to console (minified JSON)
cargo run -- snapshot .

# Take a snapshot of a specific directory and save to a pretty-printed JSON file
cargo run -- snapshot ~/my_project --output project_snapshot.json --pretty

# Snapshot without calculating hashes or line counts, ignoring 'target/' directory
cargo run -- snapshot . --no-hash --no-line-count --ignore "target/"

# Snapshot, skipping files larger than 50MB
cargo run -- snapshot . --max-size 52428800

# Snapshot, following symbolic links
cargo run -- snapshot . --follow-symlinks
```

## Output Format

The `snapshot` command outputs a JSON object representing the entire directory state. The top-level structure is a `Snapshot`:

```json
{
  "id": "<uuid>",
  "timestamp": "<ISO 8601 datetime>",
  "repo_path": "<path_to_scanned_directory>",
  "files": [
    { /* FileMetric object */ },
    { /* FileMetric object */ }
  ],
  "summary": { /* SnapshotSummary object */ }
}
```

### `FileMetric` Object

Each file is represented by a `FileMetric` object:

```json
{
  "path": "src/main.rs",
  "size": 1234,
  "modified": "2025-11-05T18:00:00Z",
  "created": "2025-11-05T14:30:00Z",
  "file_type": "rs",
  "symlink_target": null,
  "symlink_target_exists": null,
  "hash": "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
  "lines": 50
}
```

-   `path`: Relative path of the file from the scanned root.
-   `size`: Size of the file in bytes.
-   `modified`: Last modification timestamp (ISO 8601 format).
-   `created`: Creation timestamp (ISO 8601 format).
-   `file_type`: File extension (e.g., "rs", "txt"). "symlink" for symbolic links.
-   `symlink_target`: If a symlink, the path it points to. `null` otherwise.
-   `symlink_target_exists`: If a symlink, whether its target exists. `null` otherwise.
-   `hash`: SHA-256 hash of the file's content. `null` if `--no-hash` is used or file is too large.
-   `lines`: Number of lines in the file. `null` if `--no-line-count` is used, file is binary, or file is too large.

### `SnapshotSummary` Object

Provides aggregated statistics for the entire snapshot:

```json
{
  "total_files": 100,
  "total_size": 1024000,
  "total_lines": 5000,
  "file_type_breakdown": {
    "rs": {"count": 10, "total_size": 50000, "total_lines": 2000},
    "txt": {"count": 50, "total_size": 100000, "total_lines": 3000}
  },
  "directory_breakdown": {
    "src": {"file_count": 20, "total_size": 150000, "depth": 1},
    "src/utils": {"file_count": 5, "total_size": 20000, "depth": 2}
  },
  "total_directories": 5,
  "total_symlinks": 2
}
```

-   `total_files`: Total number of files scanned.
-   `total_size`: Total size of all files in bytes.
-   `total_lines`: Total lines of code across all text files.
-   `file_type_breakdown`: Statistics grouped by file extension.
-   `directory_breakdown`: Statistics for each directory.
-   `total_directories`: Total number of directories scanned.
-   `total_symlinks`: Total number of symbolic links found.