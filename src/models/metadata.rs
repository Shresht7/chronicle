#[derive(Debug)]
pub struct FileMetadata {
    /// The path to the file
    pub path: std::path::PathBuf,
    /// The size of the file in bytes
    pub bytes: u64,
    /// The modification time of the file
    pub modified_at: Option<std::time::SystemTime>,
    /// The creation time of the file
    pub created_at: Option<std::time::SystemTime>,
    /// The last access time of the file
    pub accessed_at: Option<std::time::SystemTime>,
    /// The hash of the file content
    pub content_hash: Option<String>,
}
