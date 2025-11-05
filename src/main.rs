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

        /// The path to the output file
        #[arg(short, long, value_name = "FILE")]
        output: Option<String>,
    },
}

// ----
// MAIN
// ----

// The main entrypoint of the application
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Snapshot { path, output } => {
            let snapshot_result = scanner::scan_directory(Path::new(path));

            match snapshot_result {
                Ok(snapshot) => {
                    let json = serde_json::to_string_pretty(&snapshot).unwrap();
                    match output {
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
    }
}
