use std::fs;
use std::path::PathBuf;

use support::fs::get_base_dir;
use tracing::info;

use crate::Error;

use super::Result;

/// Stores paths to various application directories.
#[derive(Clone)]
pub struct DataDir {
    pub backup: PathBuf,
    pub debug: PathBuf,
    pub images: ImagesDir,
    pub logs: PathBuf,
    pub videos: PathBuf,
}

#[derive(Clone)]
pub struct ImagesDir {
    pub root: PathBuf,
    // TODO: Clean notes directory
    pub notes: PathBuf,
    pub placeholders: PathBuf,
    // TODO: Clean timeline directory
    pub timeline: PathBuf,
    pub thumbnails: PathBuf,
}

impl DataDir {
    /// Creates a new `Dir` instance with predefined subdirectories inside the
    /// user's data directory.
    pub fn new() -> Result<DataDir> {
        let base_dir = get_base_dir().map_err(|_| Error::NoValidHomeDir)?;
        let media_dir = base_dir.join("Media");
        let images_dir = media_dir.join("Images");

        let images_notes = images_dir.join("Notes");
        let images_placeholders = images_dir.join("Placeholders");
        let images_timelines = images_dir.join("Timelines");
        let images_thumbnails = images_dir.join("Thumbnails");

        let backup = base_dir.join("Backup");
        let debug = base_dir.join("Debug");
        let logs = base_dir.join("Logs");
        let videos = media_dir.join("Videos");

        let paths = [
            &backup,
            &debug,
            &logs,
            &images_notes,
            &images_placeholders,
            &images_timelines,
            &images_thumbnails,
            &videos,
        ];
        for path in &paths {
            fs::create_dir_all(path)?;
        }

        Ok(Self {
            backup,
            debug,
            images: ImagesDir {
                root: images_dir,
                notes: images_notes,
                placeholders: images_placeholders,
                timeline: images_timelines,
                thumbnails: images_thumbnails,
            },
            logs,
            videos,
        })
    }

    /// Logs the data directory paths.
    pub fn log(&self) {
        info!("File locations:");
        info!("\t- Backups: {:?}", self.backup);
        info!("\t- Debug: {:?}", self.debug);
        info!("\t- Images: {:?}", self.images.root);
        info!("\t- Logs: {:?}", self.logs);
        info!("\t- Videos: {:?}", self.videos);
    }
}

#[cfg(test)]
mod tests {
    use directories::BaseDirs;

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
            got.images.root.to_str().expect("a path"),
            format!("{base_dir_str}/Recipya/Media/Images")
        );
        pretty_assertions::assert_eq!(
            got.logs.to_str().expect("a path"),
            format!("{base_dir_str}/Recipya/Logs")
        );
        pretty_assertions::assert_eq!(
            got.images.notes.to_str().expect("a path"),
            format!("{base_dir_str}/Recipya/Media/Images/Notes")
        );
        pretty_assertions::assert_eq!(
            got.images.placeholders.to_str().expect("a path"),
            format!("{base_dir_str}/Recipya/Media/Images/Placeholders")
        );
        pretty_assertions::assert_eq!(
            got.images.timeline.to_str().expect("a path"),
            format!("{base_dir_str}/Recipya/Media/Images/Timelines")
        );
        pretty_assertions::assert_eq!(
            got.images.thumbnails.to_str().expect("a path"),
            format!("{base_dir_str}/Recipya/Media/Images/Thumbnails")
        );
        pretty_assertions::assert_eq!(
            got.videos.to_str().expect("a path"),
            format!("{base_dir_str}/Recipya/Media/Videos")
        );
        Ok(())
    }
}
