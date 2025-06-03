use std::collections::HashSet;
use std::env::temp_dir;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_trait::async_trait;
use axum::body::Bytes;
use derive_more::derive::From;
use diesel::internal::derives::multiconnection::chrono::Duration;
use futures_util::future::join_all;
use image::{DynamicImage, ImageReader};
use regex::Regex;
use tokio::process::Command;
use tokio::task;
use tokio::time::Instant;
use tracing::{error, info};
use uuid::Uuid;
use webp::Encoder;

use crate::core::support::software::is_ffmpeg_installed;
use crate::impl_display_as_debug;

/// A trait defining functionality for interacting with the filesystem.
#[async_trait]
pub trait FsSupport: Send + Sync {
    /// Converts an image to WebP.
    fn convert_image(&self, input_path: &Path, file_name: String, output_dir: &Path) -> Result<()>;

    /// Calculates the duration of video in seconds.
    async fn calc_video_duration(&self, input_file: &str) -> Result<Duration>;

    /// Converts a video to WebM using ffmpeg.
    async fn convert_videos(&self, input_paths: Vec<PathBuf>, output_dir: &Path) -> Result<()>;

    /// Checks whether the media file exists in the file system.
    fn is_file_exists(&self, media_file: Uuid, dir: &Path) -> bool;

    /// Collects the paths of all files in the specified directory.
    fn files_in_directory(&self, root: &Path) -> Result<HashSet<PathBuf>>;

    /// Uploads content to the temporary directory.
    async fn upload_to_temp(&self, content: Bytes) -> Result<PathBuf>;

    /// Uploads an image to the user's image directory. Only available outside testing.
    fn upload_image(&self, path: &Path, file_name: Uuid, output_path: &Path);

    /// Uploads a video to the user's video directory. Only available outside testing.
    fn upload_videos(self: Arc<Self>, videos: Vec<PathBuf>, output_path: &Path);
}

/// Creates a struct that implements the FsSupport trait based on the environment.
pub fn new_fs_support() -> Arc<dyn FsSupport> {
    if cfg!(test) {
        Arc::new(MockFs)
    } else {
        Arc::new(AppFs)
    }
}

/// The main object that implements the filesystem trait.
pub struct AppFs;

#[async_trait]
impl FsSupport for AppFs {
    fn convert_image(
        &self,
        input_path: &Path,
        mut file_name: String,
        output_dir: &Path,
    ) -> Result<()> {
        let image = ImageReader::open(input_path)?.decode()?;
        let webp_data = encode_webp(&image)?;

        file_name.push_str(".webp");
        let output_path = output_dir.join(file_name);
        fs::write(&output_path, webp_data)?;

        Ok(())
    }

    async fn calc_video_duration(&self, input_file: &str) -> Result<Duration> {
        if !is_ffmpeg_installed() {
            return Err(Error::SoftwareNotInstalled);
        }

        let output = match Command::new("ffmpeg")
            .args(["-i", input_file])
            .output()
            .await
        {
            Ok(output) => output,
            Err(err) => {
                error!("Failed to run video '{input_file}' through ffmpeg for information: {err}");
                return Err(Error::Calculate);
            }
        };

        let re = Regex::new(r"Duration: (\d+):(\d+):(\d+)").unwrap();

        let stderr = String::from_utf8_lossy(&output.stderr);
        if let Some(caps) = re.captures(&stderr) {
            let hours: u32 = caps[1].parse().map_err(|_| Error::Calculate)?;
            let minutes: u32 = caps[2].parse().map_err(|_| Error::Calculate)?;
            let seconds: u32 = caps[3].parse().map_err(|_| Error::Calculate)?;

            Ok(Duration::seconds(
                (hours * 3600 + minutes * 60 + seconds) as i64,
            ))
        } else {
            Err(Error::Calculate)
        }
    }

    async fn convert_videos(&self, input_paths: Vec<PathBuf>, output_dir: &Path) -> Result<()> {
        if !is_ffmpeg_installed() {
            return Err(Error::SoftwareNotInstalled);
        }

        let tasks = input_paths.clone().into_iter().map(|temp_path| {
            let output_path = output_dir
                .join(temp_path.file_name().unwrap_or_default())
                .with_extension("webm");

            tokio::spawn(async move {
                info!("Converting video '{:?}' to WebM", temp_path);
                let start = Instant::now();

                let input_path = match temp_path.to_str() {
                    Some(path) => path,
                    None => {
                        error!("Failed to convert input path '{:?}' to string", temp_path);
                        return;
                    }
                };

                let output_path = match output_path.to_str() {
                    Some(path) => path,
                    None => {
                        error!(
                            "Failed to convert output path '{:?}' to string",
                            output_path
                        );
                        return;
                    }
                };

                let status = Command::new("ffmpeg")
                    .args([
                        "-i",
                        input_path,
                        "-c:v",
                        "libvpx",
                        "-b:v",
                        "1M",
                        "-c:a",
                        "libvorbis",
                        output_path,
                    ])
                    .status()
                    .await;

                match status {
                    Ok(exit_status) if exit_status.success() => {
                        fs::remove_file(temp_path).ok();
                        info!(
                            "Converted video to WebM ({output_path}) in {:?}",
                            start.elapsed()
                        );
                    }
                    Ok(exit_status) => {
                        error!(
                            "FFmpeg failed for {:?} with exit code {:?}",
                            temp_path, exit_status
                        );
                    }
                    Err(err) => {
                        error!("Error running FFmpeg for {:?}: {:?}", temp_path, err);
                    }
                }
            })
        });

        join_all(tasks).await;

        Ok(())
    }

    fn is_file_exists(&self, media_file: Uuid, dir: &Path) -> bool {
        Path::new(dir).join(media_file.to_string()).exists()
    }

    fn files_in_directory(&self, root: &Path) -> Result<HashSet<PathBuf>> {
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

    async fn upload_to_temp(&self, content: Bytes) -> Result<PathBuf> {
        let file_uuid = Uuid::new_v4();
        let temp = temp_dir().join(file_uuid.to_string());
        tokio::fs::write(&temp, content).await.map_err(|err| {
            error!("Error uploading to temporary directory: {err}");
            Error::UploadFile
        })?;
        Ok(temp)
    }

    fn upload_image(&self, path: &Path, file_name: Uuid, output_path: &Path) {
        if let Err(err) = self.convert_image(path, file_name.to_string(), output_path) {
            error!("Error converting image '{file_name}' to WebP: {err}");
        }
    }

    fn upload_videos(self: Arc<Self>, videos: Vec<PathBuf>, output_path: &Path) {
        let output = output_path.to_path_buf();
        let videos = videos.clone();
        let this = Arc::clone(&self);

        task::spawn(async move {
            if let Err(err) = this.convert_videos(videos, &output).await {
                error!("Error converting videos: {err}");
            }
        });
    }
}

fn encode_webp(image: &DynamicImage) -> Result<Vec<u8>> {
    Ok(Encoder::from_image(image)
        .map_err(|err| Error::General(err.to_string()))?
        .encode(75.0)
        .to_vec())
}

/// A mock that implements the filesystem trait.
pub struct MockFs;

#[async_trait]
impl FsSupport for MockFs {
    fn convert_image(
        &self,
        _input_path: &Path,
        _file_name: String,
        _output_dir: &Path,
    ) -> Result<()> {
        Ok(())
    }

    async fn calc_video_duration(&self, _input_file: &str) -> Result<Duration> {
        Ok(Duration::seconds(60))
    }

    async fn convert_videos(&self, _input_paths: Vec<PathBuf>, _output_dir: &Path) -> Result<()> {
        Ok(())
    }

    fn is_file_exists(&self, media_file: Uuid, _dir: &Path) -> bool {
        media_file != Uuid::nil()
    }

    fn files_in_directory(&self, _root: &Path) -> Result<HashSet<PathBuf>> {
        Ok(HashSet::new())
    }

    async fn upload_to_temp(&self, _content: Bytes) -> Result<PathBuf> {
        Ok(PathBuf::new())
    }

    fn upload_image(&self, _path: &Path, _file_name: Uuid, _output_path: &Path) {}

    fn upload_videos(self: Arc<Self>, _videos: Vec<PathBuf>, _output_path: &Path) {}
}

/// Result type for errors related to the file system.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to time.
#[derive(Debug, From)]
pub enum Error {
    Calculate,
    General(String),
    SoftwareNotInstalled,
    UploadFile,

    #[from(image::error::ImageError)]
    Image,
    #[from(std::io::Error)]
    Io,
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
