#[derive(Debug)]
pub struct FileMetadata {
    /// The name of the file
    pub name: String,
    /// The path to the file
    pub path: std::path::PathBuf,
    /// The size of the file in bytes
    pub size: u64,
    /// The modification time of the file
    pub modified: Option<std::time::SystemTime>,
    /// The creation time of the file
    pub created: Option<std::time::SystemTime>,
    /// The last access time of the file
    pub accessed: Option<std::time::SystemTime>,
}
