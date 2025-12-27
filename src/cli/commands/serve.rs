use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use chrono::{DateTime, Utc};
use clap::Parser;
use serde::Serialize;
use std::env;
use std::path::PathBuf;

use crate::database;
use crate::utils;

/// The command to start a web server for visualizations
#[derive(Parser, Debug)]
pub struct Serve {
    /// Port to listen on
    #[arg(short, long, default_value = "8080")]
    port: u16,
}

// New struct for API response
#[derive(Serialize)]
pub struct ApiSnapshot {
    pub id: i64,
    pub timestamp: String, // ISO 8601 formatted string
    pub files_count: i64,
    pub total_size: i64,
}

#[get("/api/snapshots")]
async fn get_snapshots(db_path: web::Data<PathBuf>) -> impl Responder {
    let db_path_ref: &PathBuf = db_path.get_ref();

    let conn = match database::open(db_path_ref) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error opening database: {}", e);
            return HttpResponse::InternalServerError()
                .body(format!("Error opening database: {}", e));
        }
    };

    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
            return HttpResponse::InternalServerError()
                .body(format!("Error getting current directory: {}", e));
        }
    };

    // Canonicalize the current directory to match the format used in the database
    let canonical_current_dir = match std::fs::canonicalize(&current_dir) {
        Ok(c_dir) => c_dir,
        Err(e) => {
            eprintln!("Error canonicalizing current directory: {}", e);
            return HttpResponse::InternalServerError()
                .body(format!("Error canonicalizing current directory: {}", e));
        }
    };
    let root_str = canonical_current_dir.to_string_lossy().to_string();

    let snapshots = match database::list_snapshots_for_root(&conn, &root_str) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error listing snapshots: {}", e);
            return HttpResponse::InternalServerError()
                .body(format!("Error listing snapshots: {}", e));
        }
    };

    let api_snapshots: Vec<ApiSnapshot> = snapshots
        .into_iter()
        .map(|s| {
            let datetime: DateTime<Utc> = s.timestamp.into(); // Convert SystemTime to DateTime<Utc>
            ApiSnapshot {
                id: s.id,
                timestamp: datetime.to_rfc3339(), // Format as ISO 8601 string
                files_count: s.file_count,
                total_size: s.total_size,
            }
        })
        .collect();

    HttpResponse::Ok().json(api_snapshots)
}

impl Serve {
    /// Execute the command to start the web server
    #[actix_web::main]
    pub async fn execute(
        &self,
        cli: &crate::cli::args::Args,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("127.0.0.1:{}", self.port);
        println!("Starting web server on: http://{}", addr);
        println!("Serving static files from: ./web");

        // Get the chronicle DB path
        let db_path = utils::get_chronicle_db_path(cli.db.as_ref())?;

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(db_path.clone())) // Pass db_path to the app
                .service(get_snapshots)
                .service(Files::new("/", "./web").index_file("index.html"))
        })
        .bind(&addr)?
        .run()
        .await?;

        Ok(())
    }
}
