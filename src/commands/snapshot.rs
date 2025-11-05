use clap::Args;
use std::path::Path;

use chronicle::fs::scanner;

/// Takes a snapshot of a directory
#[derive(Args)]
pub struct SnapshotCommand {
    /// The path to the directory to snapshot
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: String,

    /// The path to the output file
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<String>,

    /// Pretty-print JSON output (default: false)
    #[arg(long, default_value_t = false)]
    pub pretty: bool,

    /// Do not calculate content hashes (default: false)
    #[arg(long, default_value_t = false)]
    pub no_hash: bool,

    /// Do not count lines of code (default: false)
    #[arg(long, default_value_t = false)]
    pub no_line_count: bool,

    /// Additional ignore patterns (e.g., "*.log", "temp/")
    #[arg(short, long)]
    pub ignore: Vec<String>,

    /// Skip files larger than this size in bytes
    #[arg(long)]
    pub max_size: Option<u64>,

    /// Follow symbolic links (default: false)
    #[arg(long, default_value_t = false)]
    pub follow_symlinks: bool,

    /// Buffer size in bytes for hashing files (default: 1MB)
    #[arg(long, default_value_t = 1048576)]
    pub hash_buffer_size: usize,

    /// Buffer size in bytes for counting lines (default: 1MB)
    #[arg(long, default_value_t = 1048576)]
    pub line_count_buffer_size: usize,
}

pub fn execute(command: &SnapshotCommand) {
    let snapshot_result = scanner::scan_directory(
        Path::new(&command.path),
        command.no_hash,
        command.no_line_count,
        &command.ignore,
        command.max_size,
        command.follow_symlinks,
        Some(command.hash_buffer_size),
        Some(command.line_count_buffer_size),
    );

    match snapshot_result {
        Ok(snapshot) => {
            let json = if command.pretty {
                serde_json::to_string_pretty(&snapshot).unwrap()
            } else {
                serde_json::to_string(&snapshot).unwrap()
            };
            match &command.output {
                Some(file_path) => {
                    if let Err(e) = std::fs::write(file_path, json) {
                        eprintln!("Error writing to file: {}", e);
                    }
                }
                None => {
                    println!("{}", json);
                }
            }
        }
        Err(e) => {
            eprintln!("Error scanning directory: {}", e);
        }
    }
}
