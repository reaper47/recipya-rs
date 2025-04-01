use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use derive_more::derive::From;
use diesel::internal::derives::multiconnection::chrono;
use futures_util::future::join_all;
use image::{DynamicImage, ImageReader};
use regex::Regex;
use tokio::process::Command;
use tokio::time::Instant;
use tracing::{error, info};
use uuid::Uuid;
use webp::Encoder;

use crate::core::support::software::is_ffmpeg_installed;
use crate::impl_display_as_debug;

#[allow(unused)]
/// Converts an image to WebP.
pub fn convert_image(input_path: &Path, mut file_name: String, output_dir: &Path) -> Result<()> {
    let image = ImageReader::open(input_path)?.decode()?;
    let webp_data = encode_webp(&image)?;

    file_name.push_str(".webp");
    let output_path = output_dir.join(file_name);
    fs::write(&output_path, webp_data)?;

    Ok(())
}

fn encode_webp(image: &DynamicImage) -> Result<Vec<u8>> {
    Ok(Encoder::from_image(image)
        .map_err(|err| Error::General(err.to_string()))?
        .encode(75.0)
        .to_vec())
}

/// Calculates the duration of video in seconds.
pub async fn calc_video_duration(input_file: &str) -> Result<chrono::Duration> {
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

        Ok(chrono::Duration::seconds(
            (hours * 3600 + minutes * 60 + seconds) as i64,
        ))
    } else {
        Err(Error::Calculate)
    }
}

/// Converts a video to WebM using ffmpeg.
///
/// # Returns
///
/// The duration of the file.
///
pub async fn convert_videos(input_paths: Vec<PathBuf>, output_dir: &Path) -> Result<()> {
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

/// Uploads an image to the user's image directory. Only available outside testing.
#[allow(unused)]
pub fn upload_image(path: &Path, file_name: Uuid, output_path: &Path) {
    #[cfg(not(test))]
    {
        use crate::core::support::fs::convert_image;

        if let Err(err) = convert_image(path, file_name.to_string(), output_path) {
            error!("Error converting image '{file_name}' to WebP: {err}");
        }
    }
}

/// Uploads a video to the user's video directory. Only available outside testing.
#[allow(unused)]
pub fn upload_videos(videos: Vec<PathBuf>, output_path: &Path) {
    #[cfg(not(test))]
    {
        use tokio::task;

        use crate::core::support::fs::convert_videos;

        let output = output_path.to_path_buf();
        let videos = videos.clone();

        task::spawn(async move {
            if let Err(err) = convert_videos(videos, &output).await {
                error!("Error converting videos: {err}");
            }
        });
    }
}

/// Result type for errors related to the file system.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to time.
#[derive(Debug, From)]
pub enum Error {
    Calculate,
    General(String),
    SoftwareNotInstalled,

    #[from(image::error::ImageError)]
    Image,
    #[from(std::io::Error)]
    Io,
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
