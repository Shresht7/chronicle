use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Scan a directory and record a snapshot
    Snapshot {
        /// Path to the directory to scan
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();
    println!("{cli:#?}");
}
