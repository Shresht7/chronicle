use std::path::PathBuf;

/// Retrieves the local application data directory for the current user.
///
/// This function attempts to obtain the path to the local application data directory
/// using the `dirs_next` crate. If the directory cannot be found, it returns an
/// `std::io::Error` with a `NotFound` kind and a message indicating that the
/// LocalAppData was not found.
///
/// # Errors
///
/// Returns an error if the local application data directory cannot be determined.
///
/// # Example
///
/// ```rust
/// let base_dir = dirs_next::data_local_dir().ok_or(std::io::Error::new(
///     std::io::ErrorKind::NotFound,
///     "LocalAppData Not Found!",
/// ))?;
/// ```
fn get_chronicle_dir() -> std::io::Result<PathBuf> {
    // Determine base data directory
    #[cfg(target_os = "windows")]
    let base_dir = dirs_next::data_local_dir().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "LocalAppData Not Found!",
    ))?;

    #[cfg(not(target_os = "windows"))]
    let base_dir = dirs_next::data_dir()
        .unwrap_or_else(|| dirs_next::home_dir().unwrap().join(".local/share"))?;

    // Chronicle Specific Folder
    let chronicle_dir = base_dir.join(env!("CARGO_PKG_NAME"));

    // Create folder if it doesn't exist
    std::fs::create_dir_all(&chronicle_dir)?;

    Ok(chronicle_dir)
}
