// Library
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::Path;

const MAX_HASH_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100 MB

/// Calculates the SHA-256 hash of a file.
/// Returns `None` if the file is larger than `MAX_HASH_FILE_SIZE` or on error.
pub fn calculate_sha256(path: &Path) -> Option<String> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return None,
    };

    let metadata = match file.metadata() {
        Ok(metadata) => metadata,
        Err(_) => return None,
    };

    if metadata.len() > MAX_HASH_FILE_SIZE {
        return None; // Skip hashing for large files
    }

    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes) => bytes,
            Err(_) => return None,
        };
        hasher.update(&buffer[..bytes_read]);
    }

    Some(format!("{:x}", hasher.finalize()))
}
