mod cli;
mod core;
mod database;
mod models;
mod output_formatter;
mod utils;

/// The main entrypoint of the application
fn main() {
    // Parse the command line arguments
    let args = cli::args::parse();

    // Run the command-line-interface and handle errors
    if let Err(e) = run(&args) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

/// Run the command-line-interface
fn run(cli: &cli::args::Args) -> Result<(), Box<dyn std::error::Error>> {
    match &cli.command {
        cli::commands::Command::Snapshot(cmd) => cmd.execute(cli),
        cli::commands::Command::List(cmd) => cmd.execute(cli),
        cli::commands::Command::Status(cmd) => cmd.execute(cli),
        cli::commands::Command::Diff(cmd) => cmd.execute(cli),
        cli::commands::Command::Sync(cmd) => cmd.execute(cli),
    }
}
