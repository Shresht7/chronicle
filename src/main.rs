use clap::{Parser, Subcommand};

mod commands;
mod database;
mod models;
mod utils;

/// The command-line-interface for the application
#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// The subcommands of the command-line-interface
#[derive(Subcommand, Debug)]
enum Commands {
    /// Scan a directory and record a snapshot
    #[command(alias = "scan")]
    Snapshot(commands::Snapshot),
}

/// The main entrypoint of the application
fn main() {
    // Parse the command line arguments
    let cli = Cli::parse();

    // Run the command-line-interface and handle errors
    if let Err(e) = run(&cli) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

/// Run the command-line-interface
fn run(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    match &cli.command {
        Commands::Snapshot(cmd) => cmd.execute(),
    }
}
