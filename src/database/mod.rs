mod compute_diff;
mod insert_snapshot;
mod query;

pub use compute_diff::*;
pub use insert_snapshot::*;
pub use query::*;
use rusqlite::{Connection, Result};
use std::path::Path;

use crate::{models, utils}; // Added these imports

/// Opens (or Creates) the Chronicle database at the given path
pub fn open(path: &Path) -> Result<Connection> {
    let mut conn = Connection::open(path)?;
    initialize_schema(&mut conn)?;
    Ok(conn)
}

// Create tables if they don't exist
pub fn initialize_schema(conn: &mut Connection) -> Result<()> {
    conn.execute_batch(include_str!("schema.sql"))?;
    Ok(())
}

pub fn store_snapshot(
    snapshot: models::Snapshot,
    db_path_override: Option<&std::path::PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let db_path = utils::get_chronicle_db_path(db_path_override)?;
    let mut conn = open(&db_path)?;

    // Compute Diff
    let diff = compute_diff(&mut conn, &snapshot.root.to_string_lossy(), &snapshot.files)?;
    if diff.is_empty() {
        println!("No changes detected");
        return Ok(());
    }

    // Print Diff
    if diff.added.is_empty() && diff.removed.is_empty() && diff.modified.is_empty() {
        println!("No changes detected");
        return Ok(());
    }

    // Print summary
    println!("Snapshot detected changes:");
    if !diff.added.is_empty() {
        println!("  + {} added files", diff.added.len());
    }
    if !diff.removed.is_empty() {
        println!("  - {} removed files", diff.removed.len());
    }
    if !diff.modified.is_empty() {
        println!("  * {} modified files", diff.modified.len());
    }

    debug_assert!(
        snapshot.files.iter().all(|f| !f.path.is_absolute()),
        "FileMetadata paths must be relative"
    );

    // Insert Snapshot
    let snapshot_id = insert_snapshot(&mut conn, &snapshot)?;
    println!("Snapshot stored with id {}", snapshot_id);

    Ok(())
}
