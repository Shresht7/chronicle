use std::env;
use std::path::PathBuf;

use actix_web::{HttpResponse, Responder, get, web};
use chrono::{DateTime, Utc};

use super::models::ApiSnapshot;
use super::tree::build_file_tree;
use crate::database;

#[get("/api/snapshots")]
pub async fn get_snapshots(db_path: web::Data<PathBuf>) -> impl Responder {
    let db_path_ref: &PathBuf = db_path.get_ref();

    let conn = match database::open(db_path_ref) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error opening database: {e}");
            return HttpResponse::InternalServerError()
                .body(format!("Error opening database: {e}"));
        }
    };

    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error getting current directory: {e}");
            return HttpResponse::InternalServerError()
                .body(format!("Error getting current directory: {e}"));
        }
    };

    let canonical_current_dir = match std::fs::canonicalize(&current_dir) {
        Ok(c_dir) => c_dir,
        Err(e) => {
            eprintln!("Error canonicalizing current directory: {e}");
            return HttpResponse::InternalServerError()
                .body(format!("Error canonicalizing current directory: {e}"));
        }
    };
    let root_str = canonical_current_dir.to_string_lossy().to_string();

    let snapshots = match database::list_snapshots_for_root(&conn, &root_str) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error listing snapshots: {e}");
            return HttpResponse::InternalServerError()
                .body(format!("Error listing snapshots: {e}"));
        }
    };

    let api_snapshots: Vec<ApiSnapshot> = snapshots
        .into_iter()
        .map(|s| {
            let datetime: DateTime<Utc> = s.timestamp.into();
            ApiSnapshot {
                id: s.id,
                timestamp: datetime.to_rfc3339(),
                files_count: s.file_count,
                total_size: s.total_size,
            }
        })
        .collect();

    HttpResponse::Ok().json(api_snapshots)
}

#[get("/api/latest_snapshot_tree")]
pub async fn get_latest_snapshot_tree(db_path: web::Data<PathBuf>) -> impl Responder {
    let db_path_ref: &PathBuf = db_path.get_ref();

    let conn = match database::open(db_path_ref) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error opening database: {e}");
            return HttpResponse::InternalServerError()
                .body(format!("Error opening database: {e}"));
        }
    };

    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error getting current directory: {e}");
            return HttpResponse::InternalServerError()
                .body(format!("Error getting current directory: {e}"));
        }
    };

    let canonical_current_dir = match std::fs::canonicalize(&current_dir) {
        Ok(c_dir) => c_dir,
        Err(e) => {
            eprintln!("Error canonicalizing current directory: {e}");
            return HttpResponse::InternalServerError()
                .body(format!("Error canonicalizing current directory: {e}"));
        }
    };
    let root_str = canonical_current_dir.to_string_lossy().to_string();

    let latest_snapshot_id = match database::get_latest_snapshot_id(&conn, &root_str) {
        Ok(Some(id)) => id,
        Ok(None) => {
            return HttpResponse::NotFound().body("No latest snapshot found for this directory.");
        }
        Err(e) => {
            eprintln!("Error getting latest snapshot ID: {e}");
            return HttpResponse::InternalServerError()
                .body(format!("Error getting latest snapshot ID: {e}"));
        }
    };

    let files = match database::get_files_for_snapshot(&conn, latest_snapshot_id) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error getting files for snapshot: {e}");
            return HttpResponse::InternalServerError()
                .body(format!("Error getting files for snapshot: {e}"));
        }
    };

    let file_tree = build_file_tree(files);

    HttpResponse::Ok().json(file_tree)
}
