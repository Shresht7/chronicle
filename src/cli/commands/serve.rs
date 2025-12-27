use clap::Parser;
use actix_web::{get, web, App, HttpServer, Responder};
use actix_files::Files;
use serde_json::json;

/// The command to start a web server for visualizations
#[derive(Parser, Debug)]
pub struct Serve {
    /// Port to listen on
    #[arg(short, long, default_value = "8080")]
    port: u16,
}

#[get("/api/snapshots")]
async fn get_snapshots() -> impl Responder {
    web::Json(json!([
        { "id": 1, "timestamp": "2024-01-01T12:00:00Z", "files_count": 100, "size_bytes": 102400 },
        { "id": 2, "timestamp": "2024-01-02T12:00:00Z", "files_count": 105, "size_bytes": 105000 },
        { "id": 3, "timestamp": "2024-01-03T12:00:00Z", "files_count": 98, "size_bytes": 98000 }
    ]))
}

impl Serve {
    /// Execute the command to start the web server
    #[actix_web::main]
    pub async fn execute(&self, _cli: &crate::cli::args::Args) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("127.0.0.1:{}", self.port);
        println!("Starting web server on: http://{}", addr);
        println!("Serving static files from: ./web");

        HttpServer::new(|| {
            App::new()
                .service(get_snapshots)
                .service(Files::new("/", "./web").index_file("index.html"))
        })
        .bind(&addr)?
        .run()
        .await?;

        Ok(())
    }
}