use clap::{Parser, Subcommand};
use ignore::WalkBuilder;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Scan a directory and record a snapshot
    #[command(alias = "scan")]
    Snapshot {
        /// Path to the directory to scan
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Snapshot { path } => {
            run_snapshot(&path);
        }
    }
}

fn run_snapshot(path: &str) {
    let walker = WalkBuilder::new(path).build();

    for result in walker {
        let entry = match result {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Walk Error: {err}");
                continue;
            }
        };

        // Skip directories for now
        if !entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            continue;
        }

        println!("{}", entry.path().display())
    }
}
