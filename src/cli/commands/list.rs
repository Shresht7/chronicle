use clap::Parser;
use std::path::PathBuf;
use chrono::{Local, DateTime};

use crate::{database, models, output_formatter, utils};
use crate::output_formatter::OutputFormatter;

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

        let headers = vec![
            "ID".to_string(),
            "Timestamp".to_string(),
            "Files".to_string(),
            "Size".to_string(),
        ];

        let mut rows = Vec::new();
        for snapshot in snapshots {
            let datetime: DateTime<Local> = snapshot.timestamp.into();
            rows.push(vec![
                snapshot.id.to_string(),
                datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
                snapshot.file_count.to_string(),
                utils::format_size_auto(snapshot.total_size as u64),
            ]);
        }

        let table = models::Table::new(headers, rows);
        let formatter = output_formatter::TsvFormatter;
        println!("{}", formatter.format(&table));

        Ok(())
    }
}
