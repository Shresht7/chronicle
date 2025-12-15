use clap::Parser;
use std::path::PathBuf;
use chrono::{Local, DateTime};

use crate::{database, utils};

/// The command to list all snapshots for a given directory
#[derive(Parser, Debug)]
pub struct List {
    /// Path to the directory to list snapshots for
    #[arg(default_value = ".")]
    path: PathBuf,
}

impl List {
    /// Execute the command to list all snapshots for a given directory
    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let root = std::fs::canonicalize(&self.path)?;

        let db_path = utils::get_chronicle_db_path()?;
        let conn = database::open(&db_path)?;

        let snapshots = database::list_snapshots_for_root(&conn, &root.to_string_lossy())?;

        if snapshots.is_empty() {
            println!("No snapshots found for directory: {}", root.display());
            return Ok(());
        }

        println!("Snapshots for: {}", root.display());
        println!("------------------------------------");
        for snapshot in snapshots {
            let datetime: DateTime<Local> = snapshot.timestamp.into();
            println!(
                "ID: {:<4} | Timestamp: {:<25} | Root: {}",
                snapshot.id,
                datetime.format("%Y-%m-%d %H:%M:%S"),
                snapshot.root.display()
            );
        }
        println!("------------------------------------");

        Ok(())
    }
}
