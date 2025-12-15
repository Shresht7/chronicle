mod compute_diff;
mod insert_snapshot;

pub use compute_diff::*;
pub use insert_snapshot::*;
use rusqlite::{Connection, Result};
use std::path::Path;

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
