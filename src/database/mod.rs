use rusqlite::{Connection, Result};
use std::path::Path;

/// Opens (or Creates) the Chronicle database at the given path
pub fn open(path: &Path) -> Result<Connection> {
    let conn = Connection::open(path)?;
    initialize_schema(&conn)?;
    Ok(conn)
}

// Create tables if they don't exist
fn initialize_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(include_str!("schema.sql"))?;
    Ok(())
}
