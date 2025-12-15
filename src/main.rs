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
    Snapshot(Snapshot),
}

#[derive(Parser, Debug)]
struct Snapshot {
    /// Path to the directory to scan
    #[arg(default_value = ".")]
    path: String,
}

impl Snapshot {
    fn execute(&self) {
        // Create a walker to scan the directory
        let walker = WalkBuilder::new(&self.path).build();

        // Iterate over the entries in the directory
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

            // Print the path to the file
            println!("{}", entry.path().display())
        }
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Snapshot(cmd) => cmd.execute(),
    }
}
