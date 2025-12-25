use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use blake3::Hasher;

const BUFFER_SIZE: usize = 8192;

/// Compute a BLAKE3 hash of a file, returned as a hex string
pub fn hash_file(path: &PathBuf) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut hasher = Hasher::new();
    let mut buffer = [0u8; BUFFER_SIZE];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Ok(hasher.finalize().to_hex().to_string())
}

/// Compute a BLAKE3 hash of a byte slice, returned as a hex string
pub fn compute_blake3_hash(content: &[u8]) -> String {
    let mut hasher = Hasher::new();
    hasher.update(content);
    hasher.finalize().to_hex().to_string()
}
