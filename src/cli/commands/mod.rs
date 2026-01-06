use clap::Subcommand;

mod diff;
mod git;
mod list;
mod snapshot;
mod status;

mod serve;

/// The subcommands of the command-line-interface
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Scan a directory and record a snapshot
    #[command(alias = "scan")]
    Snapshot(snapshot::Snapshot),

    /// List all snapshots for a given directory
    #[command(alias = "log")]
    List(list::List),

    /// Show the difference between the current directory state and the last snapshot
    #[command(alias = "st")]
    Status(status::Status),

    /// Show the difference between snapshots or the current state
    Diff(diff::Diff),

    /// Synchronize Git history into chronicle
    Git(git::Git),

    /// Start a web server for visualizations
    Serve(serve::Serve),
}
