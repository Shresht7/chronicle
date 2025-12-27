use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::env;
use std::path::{Path, PathBuf};

// Removed unused rusqlite::Connection import
// Removed unused utils import

use crate::{database, models};

// New struct for API response (Timeline)
#[derive(Serialize)]
pub struct ApiSnapshot {
    pub id: i64,
    pub timestamp: String, // ISO 8601 formatted string
    pub files_count: i64,
    pub total_size: i64,
}

// Struct for tree visualization nodes
#[derive(Debug, Serialize)]
pub struct ApiNode {
    name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    children: Vec<ApiNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<u64>, // For files, represents bytes
}

// A more robust helper function to build a tree structure from file metadata
fn build_file_tree(files: Vec<models::FileMetadata>) -> ApiNode {
    let mut root_node = ApiNode {
        name: ".".to_string(),
        children: Vec::new(),
        size: None,
    };

    // Helper to insert a file's path components into the tree
    fn insert_into_tree(current_node: &mut ApiNode, path_components: &[&Path], file_bytes: Option<u64>) {
        if path_components.is_empty() {
            return;
        }

        let component = path_components[0];
        let is_last_component = path_components.len() == 1;
        let component_name = component.to_string_lossy().to_string();

        // Try to find an existing child
        let mut found_child_index = None;
        for (i, child) in current_node.children.iter_mut().enumerate() {
            if child.name == component_name {
                found_child_index = Some(i);
                break;
            }
        }

        match found_child_index {
            Some(idx) => {
                let child = &mut current_node.children[idx];
                if is_last_component && file_bytes.is_some() {
                    child.size = file_bytes; // Update size for file
                }
                insert_into_tree(child, &path_components[1..], file_bytes);
            },
            None => {
                // Create new child
                let mut new_child = ApiNode {
                    name: component_name,
                    children: Vec::new(),
                    size: if is_last_component { file_bytes } else { None },
                };
                insert_into_tree(&mut new_child, &path_components[1..], file_bytes);
                current_node.children.push(new_child);
            }
        }
    }

    // Sort files to ensure stable order
    let mut sorted_files = files;
    sorted_files.sort_by(|a, b| a.path.cmp(&b.path));

    for file_meta in sorted_files {
        let path_components: Vec<&Path> = file_meta.path.iter().map(Path::new).collect(); // FIXED THIS LINE
        insert_into_tree(&mut root_node, &path_components, Some(file_meta.bytes));
    }

    root_node
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

#[get("/api/latest_snapshot_tree")]
async fn get_latest_snapshot_tree(db_path: web::Data<PathBuf>) -> impl Responder {
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

    let canonical_current_dir = match std::fs::canonicalize(&current_dir) {
        Ok(c_dir) => c_dir,
        Err(e) => {
            eprintln!("Error canonicalizing current directory: {}", e);
            return HttpResponse::InternalServerError()
                .body(format!("Error canonicalizing current directory: {}", e));
        }
    };
    let root_str = canonical_current_dir.to_string_lossy().to_string();

    let latest_snapshot_id = match database::get_latest_snapshot_id(&conn, &root_str) { // Corrected path
        Ok(Some(id)) => id,
        Ok(None) => return HttpResponse::NotFound().body("No latest snapshot found for this directory."),
        Err(e) => {
            eprintln!("Error getting latest snapshot ID: {}", e);
            return HttpResponse::InternalServerError()
                .body(format!("Error getting latest snapshot ID: {}", e));
        }
    };

    let files = match database::get_files_for_snapshot(&conn, latest_snapshot_id) { // Corrected path
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error getting files for snapshot: {}", e);
            return HttpResponse::InternalServerError()
                .body(format!("Error getting files for snapshot: {}", e));
        }
    };

    let file_tree = build_file_tree(files);

    HttpResponse::Ok().json(file_tree)
}


pub async fn start_server(port: u16, db_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("127.0.0.1:{}", port);
    println!("Starting web server on: http://{}", addr);
    println!("Serving static files from: ./web");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_path.clone()))
            .service(get_snapshots)
            .service(get_latest_snapshot_tree) // Added this service
            .service(Files::new("/", "./web").index_file("index.html"))
    })
    .bind(&addr)?
    .run()
    .await?;

    Ok(())
}