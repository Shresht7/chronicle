use std::path::PathBuf;

use actix_files::Files;
use actix_web::{App, HttpServer, web};

pub mod models;
pub mod routes;
pub mod tree;

pub async fn start_server(port: u16, db_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("127.0.0.1:{port}");
    println!("Starting web server on: http://{addr}");
    println!("Serving static files from: ./web");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_path.clone()))
            .service(routes::get_snapshots)
            .service(routes::get_latest_snapshot_tree)
            .service(Files::new("/", "./web").index_file("index.html"))
    })
    .bind(&addr)?
    .run()
    .await?;

    Ok(())
}
