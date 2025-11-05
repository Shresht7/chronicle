use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Default buffer size for hashing (1MB)
const DEFAULT_HASH_BUFFER_SIZE: usize = 1024 * 1024;
/// Default buffer size for line counting (1MB)
const DEFAULT_LINE_COUNT_BUFFER_SIZE: usize = 1024 * 1024;

/// Calculates the SHA-256 hash of a file.
/// Skips files larger than 100MB for performance reasons.
pub fn calculate_sha256(path: &Path, buffer_size: Option<usize>) -> Option<String> {
    if let Ok(metadata) = path.metadata() {
        if metadata.len() > 100 * 1024 * 1024 {
            // 100MB limit
            return None;
        }
    }

    let mut file = File::open(path).ok()?;
    let mut hasher = Sha256::new();
    let buf_size = buffer_size.unwrap_or(DEFAULT_HASH_BUFFER_SIZE);
    let mut buffer = vec![0; buf_size];

    loop {
        let bytes_read = io::Read::read(&mut file, &mut buffer).ok()?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Some(format!("{:x}", hasher.finalize()))
}

/// Counts the number of lines in a file.
/// Skips files larger than 1MB for performance reasons.
/// Returns None if the file appears to be binary.
pub fn count_lines(path: &Path, buffer_size: Option<usize>) -> Option<usize> {
    let buf_size = buffer_size.unwrap_or(DEFAULT_LINE_COUNT_BUFFER_SIZE);
    if let Ok(metadata) = path.metadata() {
        if metadata.len() > buf_size as u64 {
            // Use the configurable buffer size here
            return None;
        }
    }

    let file = File::open(path).ok()?;
    // Create BufReader with the specified buffer size
    let reader = BufReader::with_capacity(buf_size, file);

    let mut line_count = 0;
    for line in reader.lines() {
        let line = line.ok()?;
        // Check for null bytes to detect binary files
        if line.as_bytes().contains(&0) {
            return None;
        }
        line_count += 1;
    }

    Some(line_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_calculate_sha256() {
        let mut file = NamedTempFile::new().unwrap();
        let content = b"hello world";
        file.write_all(content).unwrap();

        let hash = calculate_sha256(file.path(), None);
        let expected_hash =
            Some("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9".to_string());

        assert_eq!(hash, expected_hash);
    }

    #[test]
    fn test_count_lines() {
        let mut file = NamedTempFile::new().unwrap();
        let content = b"hello\nworld\n";
        file.write_all(content).unwrap();

        let lines = count_lines(file.path(), None);
        assert_eq!(lines, Some(2));
    }

    #[test]
    fn test_count_lines_no_newline_at_end() {
        let mut file = NamedTempFile::new().unwrap();
        let content = b"hello\nworld";
        file.write_all(content).unwrap();

        let lines = count_lines(file.path(), None);
        assert_eq!(lines, Some(2));
    }

    #[test]
    fn test_count_lines_binary_file() {
        let mut file = NamedTempFile::new().unwrap();
        let content = b"\x00\x01\x02\x03";
        file.write_all(content).unwrap();

        let lines = count_lines(file.path(), None);
        assert_eq!(lines, None);
    }
}
