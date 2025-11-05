use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Takes a snapshot of a directory
    Snapshot(commands::snapshot::SnapshotCommand),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Snapshot(command) => {
            commands::snapshot::execute(&command);
        }
    }
}
