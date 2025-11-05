use clap::{Parser, Subcommand};
use std::path::Path;

use chronicle::fs::scanner;

// ---
// CLI
// ---

// Command-Line-Interface
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// SubCommands
#[derive(Subcommand)]
enum Commands {
    /// Takes a snapshot of a directory
    Snapshot {
        /// The path to the directory to snapshot
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
    },
}

// ----
// MAIN
// ----

// The main entrypoint of the application
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Snapshot { path } => {
            println!("Scanning directory: {}", path);
            let snapshot_result = scanner::scan_directory(Path::new(path));

            match snapshot_result {
                Ok(snapshot) => {
                    let json = serde_json::to_string_pretty(&snapshot).unwrap();
                    println!("{}", json);
                }
                Err(e) => {
                    eprintln!("Error scanning directory: {}", e);
                }
            }
        }
    }
}
