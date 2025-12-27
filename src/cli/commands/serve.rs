use clap::Parser;

/// The command to start a web server for visualizations
#[derive(Parser, Debug)]
pub struct Serve {
    /// Port to listen on
    #[arg(short, long, default_value = "8080")]
    port: u16,
}

impl Serve {
    /// Execute the command to start the web server
    pub fn execute(&self, _cli: &crate::cli::args::Args) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting web server on port {}", self.port);
        // TODO: Implement actix-web server here
        Ok(())
    }
}
