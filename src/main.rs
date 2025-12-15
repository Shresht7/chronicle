use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Scan a directory and record a snapshot
    #[command(alias = "scan")]
    Snapshot(commands::Snapshot),
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Snapshot(cmd) => cmd.execute(),
    }
}
