// Library
use clap::{Parser, Subcommand};

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
            println!("Snapshot Target: {}", path);
        }
    }
}
