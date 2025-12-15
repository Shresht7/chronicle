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

### Take a snapshot

```bash
chronicle snapshot /path/to/directory
```

if no changes are detected since the last snapshot

```
No changes detected
```

if changes are detected:

```
Snapshot detected changes:
  + 2 added files
  - 1 removed files
  * 3 modified files
Snapshot stored with id 23
```

`scan` can be used as an alias for `snapshot`

## Data Storage

Chronicle stores its data locally using **SQLite**.
- The database lives in the OS-appropriate application data directory
  - Windows: `LocalAppData/chronicle`
  - Linux/macOS: `~/.local/share/chronicle`
- All writes are atomic
- One database tracks snapshot for multiple directories
- Each directory is identified by its canonical root path
The database schema is internal and may evolve

## Planned / Future Work

`chronicle` is intentionally built in layers. Planned additions include:
- JSON export for snapshotting and diffs
- Querying snapshot history
- Visualization pipelines (SVG, graphs, timelines)
- Git repository awareness (using Git metadata instead of file-system scan)
- Additional diff and aggregations views
These will build on the existing snapshot data model

---

## License

This project is licensed under the [MIT license](./LICENSE).
