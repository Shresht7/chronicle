mod cli;
mod core;
mod database;
mod models;
mod output_formatter;
mod utils;

/// The main entrypoint of the application
fn main() {
    // Parse the command line arguments
    let args = cli::parse();

    // Run the command-line-interface and handle errors
    if let Err(e) = run(&args) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

/// Run the command-line-interface
fn run(cli: &cli::Args) -> Result<(), Box<dyn std::error::Error>> {
    match &cli.command {
        cli::Commands::Snapshot(cmd) => cmd.execute(),
        cli::Commands::List(cmd) => cmd.execute(),
    }
}
