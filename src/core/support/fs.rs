use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use derive_more::derive::From;
use uuid::Uuid;

use crate::impl_display_as_debug;

/// Checks whether the media file exists in the file system.
pub fn is_file_exists(media_file: Uuid, dir: &Path) -> bool {
    Path::new(dir).join(media_file.to_string()).exists()
}

/// Collects the paths of all files in the specified directory.
pub fn files_in_directory<P>(root: P) -> Result<HashSet<PathBuf>>
where
    P: AsRef<Path>,
{
    let mut paths = HashSet::new();

    for entry in fs::read_dir(&root)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            paths.insert(path);
        }
    }

    Ok(paths)
}

/// Result type for errors related to the file system.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to time.
#[derive(Debug, From)]
pub enum Error {
    #[from]
    Io(std::io::Error),
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
