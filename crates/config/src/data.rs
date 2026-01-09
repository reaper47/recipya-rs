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
    pub icon: PathBuf,
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

        let images_icon = images_dir.join("Icon");
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
            &images_icon,
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
                icon: images_icon,
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
        let base_dirs = BaseDirs::new().expect("failed to get base directories");
        let expected_base = base_dirs.data_dir();
        let data_dir = DataDir::new()?;

        for (actual, relative_path) in [
            (&data_dir.backup, "Recipya/Backup"),
            (&data_dir.logs, "Recipya/Logs"),
            (&data_dir.videos, "Recipya/Media/Videos"),
            (&data_dir.images.root, "Recipya/Media/Images"),
            (&data_dir.images.icon, "Recipya/Media/Images/Icon"),
            (&data_dir.images.notes, "Recipya/Media/Images/Notes"),
            (
                &data_dir.images.placeholders,
                "Recipya/Media/Images/Placeholders",
            ),
            (&data_dir.images.timeline, "Recipya/Media/Images/Timelines"),
            (
                &data_dir.images.thumbnails,
                "Recipya/Media/Images/Thumbnails",
            ),
        ] {
            let got = expected_base.join(relative_path);

            pretty_assertions::assert_eq!(
                actual,
                got.as_path(),
                "Path mismatch for {relative_path}"
            );
        }

        Ok(())
    }
}
