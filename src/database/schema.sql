-- SNAPSHOTS
CREATE TABLE IF NOT EXISTS snapshots (
    id INTEGER PRIMARY KEY,
    root TEXT NOT NULL,
    timestamp INTEGER NOT NULL
);

-- FILES
CREATE TABLE IF NOT EXISTS files (
    id INTEGER PRIMARY KEY,
    snapshot_id INTEGER NOT NULL,
    path TEXT NOT NULL,
    bytes INTEGER NOT NULL,
    modified_at INTEGER,
    created_at INTEGER,
    accessed_at INTEGER,
    content_hash TEXT,
    FOREIGN KEY(snapshot_id) REFERENCES snapshots(id)
);

-- INDEXES
CREATE INDEX IF NOT EXISTS idx_files_snapshot ON files(snapshot_id);
CREATE INDEX IF NOT EXISTS idx_snapshots_root ON snapshots(root);
