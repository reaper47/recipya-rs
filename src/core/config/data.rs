use std::fs;
use std::path::{Path, PathBuf};

use directories::BaseDirs;
use tracing::info;

use super::{Error, Result};

/// Stores paths to various application directories.
#[derive(Clone)]
pub struct DataDir {
    backup: PathBuf,
    // TODO: We possibly do not need this folder anymore because of PostgreSQL.
    database: PathBuf,
    images: PathBuf,
    logs: PathBuf,
    thumbnails: PathBuf,
    videos: PathBuf,
}

impl DataDir {
    /// Creates a new `Dir` instance with predefined subdirectories inside the
    /// user's data directory.
    pub fn new() -> Result<DataDir> {
        if let Some(dirs) = BaseDirs::new() {
            let base_dir = dirs.data_dir().join("Recipya");
            let media_dir = base_dir.join("Media");
            let images_dir = media_dir.join("Images");

            let backup = base_dir.join("Backup");
            let database = base_dir.join("Database");
            let logs = base_dir.join("Logs");
            let thumbnails = images_dir.join("Thumbnails");
            let videos = media_dir.join("Videos");

            let paths = [&backup, &database, &logs, &thumbnails, &videos];
            for path in &paths {
                fs::create_dir_all(path)?;
            }

            Ok(Self {
                backup,
                database,
                images: images_dir,
                logs,
                thumbnails,
                videos,
            })
        } else {
            Err(Error::NoValidHomeDir)
        }
    }

    /// Logs the paths.
    pub fn log(&self) {
        info!("File locations:");
        info!("\t- Backups: {:?}", self.backup);
        info!("\t- Database: {:?}", self.database);
        info!("\t- Images: {:?}", self.images);
        info!("\t- Thumbnails: {:?}", self.thumbnails);
        info!("\t- Logs: {:?}", self.logs);
        info!("\t- Videos: {:?}", self.videos);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_new_ok() -> Result<()> {
        let binding = BaseDirs::new().expect("valid home dir");
        let base_dir = binding.data_dir();

        let got = DataDir::new()?;

        let base_dir_str = base_dir.to_str().expect("a path");
        pretty_assertions::assert_eq!(
            got.backup.to_str().expect("a path"),
            format!("{base_dir_str}/Recipya/Backup")
        );
        pretty_assertions::assert_eq!(
            got.database.to_str().expect("a path"),
            format!("{base_dir_str}/Recipya/Database")
        );
        pretty_assertions::assert_eq!(
            got.images.to_str().expect("a path"),
            format!("{base_dir_str}/Recipya/Media/Images")
        );
        pretty_assertions::assert_eq!(
            got.logs.to_str().expect("a path"),
            format!("{base_dir_str}/Recipya/Logs")
        );
        pretty_assertions::assert_eq!(
            got.thumbnails.to_str().expect("a path"),
            format!("{base_dir_str}/Recipya/Media/Images/Thumbnails")
        );
        pretty_assertions::assert_eq!(
            got.videos.to_str().expect("a path"),
            format!("{base_dir_str}/Recipya/Media/Videos")
        );
        Ok(())
    }
}
