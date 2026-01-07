# `chronicle`

chronicle is a command-line tool that tracks/records structured snapshots of directories on your filesystem and helps visualize how they evolve over time.

It is observational, not a backup tool and certainly not version control.

Chronicle answers questions like:
- *What changed in this folder over time?*
- *When did files appear, disappear, or change?*
- *How is this directory evolving structurally?*

The goal is to collect reliable historical data that can later be queried, diffed, and visualized.

>[!CAUTION]
>
> `chronicle` is currently in _active development_ and intended primarily for personal use. Interfaces and data formats are volatile.

---

## What Chronicle Is (and Is Not)

### Chronicle **is**
- A filesystem observation tool
- A structured snapshot recorder
- A history and diff engine for directories
- Designed for cron / scheduled usage
- Intended for analysis and visualization

### Chronicle **is not**
- A backup system
- A file sync tool
- Version control (Git already exists 🙂)
- A real-time watcher

Chronicle is meant to **observe**, not intervene.

---

## Core Concepts

### Snapshots

A snapshot represents the state of a directory at a point in time.  
Each snapshot records:
- Canonical root directory
- Timestamp
- A set of files with structured metadata

Snapshots are only stored when changes are detected.

---

### File Metadata

For each file, Chronicle records:
- Relative path (from snapshot root)
- Size (bytes)
- Timestamps (created / modified / accessed, when available)
- Content hash (for change detection)

This data is designed to be:
- Stable across runs
- Deterministic
- Suitable for diffing and visualization

---

### Diffs

Chronicle computes diffs between snapshots to determine:
- Added files
- Removed files
- Modified files (content hash changed)

Diffs are the authoritative mechanism for detecting change.

---

## Usage

All commands operate on a specified directory, which defaults to the current directory if not provided.

### Take a snapshot

Scans a directory and records a new snapshot if any changes are detected. If the directory is a Git repository, it will create a snapshot based on the current `HEAD` commit.

```bash
chronicle snapshot /path/to/directory
```

If changes are detected, you'll see a summary:

```
Snapshot detected changes:
  + 2 added files
  - 1 removed files
  * 3 modified files
Snapshot stored with id 23
```

`scan` can be used as an alias for `snapshot`.

### Synchronize Git History

Imports the entire commit history of a Git repository as `chronicle` snapshots.

```bash
chronicle git /path/to/git/repo
```

This command is idempotent; already imported commits will be skipped.

### List snapshots

Lists all recorded snapshots for a directory.

```bash
chronicle list /path/to/directory
```

Output:

```
ID    Timestamp              Files    Size
1     2025-12-22 10:00:00    150      1.2 GB
2     2025-12-23 11:30:00    152      1.3 GB
```

`log` can be used as an alias for `list`. The output format can be changed to JSON with `--format json`.

### Check status

Compares the current state of the directory against the latest snapshot. The output format can be changed to JSON with `--format json`.

```bash
chronicle status /path/to/directory
```

Output:

```
Changes detected:

Added files:
  + new_file.txt

Removed files:
  - old_file.log

Modified files:
  * changed_document.md
```

`st` can be used as an alias for `status`.

### Diff snapshots

Shows the difference between two snapshots or between the current directory state and a snapshot. The output format can be changed to JSON with `--format json`.

**Usage:**

*   `chronicle diff`: Compares the last two snapshots (`HEAD~1` vs `HEAD`).
*   `chronicle diff <rev>`: Compares the current files in the directory to the specified snapshot `<rev>`.
*   `chronicle diff <rev1> <rev2>`: Compares snapshot `<rev1>` and snapshot `<rev2>`.

Revisions can be a snapshot ID (e.g., `123`), `HEAD` (the latest snapshot for the current directory), or `HEAD~1` (the snapshot before the latest).

**Examples:**

*   Compare current directory to the latest snapshot:
    ```bash
    chronicle diff HEAD --path /path/to/directory
    ```
*   Compare a specific snapshot (ID 42) to the latest snapshot:
    ```bash
    chronicle diff 42 HEAD --path /path/to/directory
    ```
*   Compare the last two snapshots:
    ```bash
    chronicle diff --path /path/to/directory
    ```

## Data Storage

Chronicle stores its data locally using **SQLite**.
- The database lives in the OS-appropriate application data directory
  - Windows: `LocalAppData/chronicle`
  - Linux/macOS: `~/.local/share/chronicle`
- All writes are atomic
- One database tracks snapshot for multiple directories
- Each directory is identified by its canonical root path
The database schema is internal and may evolve

### Serve web interface

Starts a local web server to provide a visual interface for exploring snapshots and their history. This command enables interactive visualizations of timelines, treemaps, and other aggregations of the collected data.

```bash
chronicle serve
```

This will typically open a web browser pointing to `http://127.0.0.1:8080` (or similar) where you can interact with the data.

---

## License

This project is licensed under the [MIT license](./LICENSE).
