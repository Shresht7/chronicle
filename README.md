# Chronicle

A Rust-based tool to record, analyze, and visualize how files and projects evolve over time using both git history and filesystem snapshots.

**Chronicle** tracks the evolution of your code, notes, and projects - preserving the story of how they grow and change.

## Current Status

This project is in the early stages of development. The current focus is on building a robust filesystem snapshot capability as the foundation for more advanced analysis and visualization features.

## Usage

The primary command available right now is `snapshot`, which scans a directory and outputs a JSON representation of its contents.

```sh
cargo run -- snapshot <path>
```

-   `<path>`: The directory to scan. Defaults to the current directory (`.`).

### Example Output

The command will produce a JSON object representing the snapshot. Each file is represented by a `FileMetric` object, which looks like this:

```json
{
  "path": "src/main.rs",
  "size": 1234,
  "modified": "2025-11-05T18:00:00Z",
  "created": "2025-11-05T14:30:00Z",
  "file_type": "rs",
  "symlink_target": null,
  "symlink_target_exists": null
}
```