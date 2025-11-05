use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

const MAX_HASH_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100 MB
const MAX_LINE_COUNT_FILE_SIZE: u64 = 5 * 1024 * 1024; // 5 MB

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

/// Counts the number of lines in a file.
/// Returns `None` if the file is binary, larger than `MAX_LINE_COUNT_FILE_SIZE`, or on error.
pub fn count_lines(path: &Path) -> Option<usize> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return None,
    };

    let metadata = match file.metadata() {
        Ok(metadata) => metadata,
        Err(_) => return None,
    };

    if metadata.len() > MAX_LINE_COUNT_FILE_SIZE {
        return None; // Skip line counting for large files
    }

    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1024];

    // Check for null bytes in the first 1KB to guess if it's a binary file
    let bytes_read = match reader.read(&mut buffer) {
        Ok(bytes) => bytes,
        Err(_) => return None,
    };

    if buffer[..bytes_read].contains(&0) {
        return None; // It's likely a binary file
    }

    // We need to create a new reader because the previous one consumed the first 1KB
    let file = File::open(path).ok()?;
    let reader = BufReader::new(file);

    Some(reader.lines().count())
}
