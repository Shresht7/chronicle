use clap::Parser;

use crate::utils;
use crate::server; // Import the new server module

/// The command to start a web server for visualizations
#[derive(Parser, Debug)]
pub struct Serve {
    /// Port to listen on
    #[arg(short, long, default_value = "8080")]
    port: u16,
}

impl Serve {
    /// Execute the command to start the web server
    #[actix_web::main]
    pub async fn execute(&self, cli: &crate::cli::args::Args) -> Result<(), Box<dyn std::error::Error>> {
        // Get the chronicle DB path
        let db_path = utils::get_chronicle_db_path(cli.db.as_ref())?;

        server::start_server(self.port, db_path).await?;

        Ok(())
    }
}