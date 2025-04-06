use std::collections::HashSet;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tracing::{error, info};
use uuid::Uuid;

use super::Result;
use crate::core::config::DataDir;
use crate::core::repository::pool::PgPooledConn;
use crate::core::repository::{ModelManager, schema};
use crate::core::support::fs::FsSupport;

/// Removes files from the Media folder that do not belong to any recipe or cookbook.
pub async fn clean_media(
    mm: Arc<ModelManager>,
    data_dir: Arc<DataDir>,
    fs_support: Arc<dyn FsSupport + Sync + Send>,
) -> Result<()> {
    let mut conn = mm.pool.get().await?;
    let videos = fetch_videos(&mut conn, &data_dir).await?;
    let images = fetch_images(&mut conn, &data_dir).await?;

    let (num_images_deleted, images_space_reclaimed_bytes) =
        clean_files(&data_dir.images, &images, true, Arc::clone(&fs_support)).await?;

    let (num_videos_deleted, videos_space_reclaimed_bytes) =
        clean_files(&data_dir.videos, &videos, false, fs_support).await?;

    info!(
        "CleanMedia: Removed {num_images_deleted} images and {num_videos_deleted} videos. Reclaimed {:.2} MB.",
        ((images_space_reclaimed_bytes + videos_space_reclaimed_bytes) as f64) / 1_000_000.0
    );

    Ok(())
}

async fn fetch_videos(
    conn: &mut PgPooledConn<'_>,
    data_dir: &Arc<DataDir>,
) -> Result<HashSet<PathBuf>> {
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    Ok(schema::videos_recipes::table
        .distinct()
        .order_by(schema::videos_recipes::video.asc())
        .select(schema::videos_recipes::video)
        .load::<Uuid>(conn)
        .await
        .unwrap_or_else(|err| {
            error!("CleanMedia: Error fetching distinct video images: {err}");
            Vec::new()
        })
        .into_iter()
        .map(|uuid| PathBuf::from(format!("{:?}/{uuid}.webp", &data_dir.videos)))
        .collect())
}

async fn fetch_images(
    conn: &mut PgPooledConn<'_>,
    data_dir: &Arc<DataDir>,
) -> Result<HashSet<PathBuf>> {
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    Ok(schema::recipes::table
        .select(schema::recipes::image)
        .distinct()
        .union(
            schema::cookbooks::table
                .select(schema::cookbooks::image)
                .distinct(),
        )
        .load::<Option<Uuid>>(conn)
        .await
        .unwrap_or_else(|err| {
            error!("CleanMedia: Error fetching distinct recipe and cookbook images: {err}");
            Vec::new()
        })
        .into_iter()
        .flatten()
        .map(|uuid| PathBuf::from(format!("{:?}/{uuid}.webp", &data_dir.images)))
        .collect())
}

async fn clean_files(
    dir: &PathBuf,
    files_to_keep: &HashSet<PathBuf>,
    is_delete_thumbnails: bool,
    fs_support: Arc<dyn FsSupport + Sync + Send>,
) -> Result<(u64, u64)> {
    let mut num_files_deleted = 0u64;
    let mut space_reclaimed_bytes = 0u64;

    match fs_support.files_in_directory(dir) {
        Ok(all_files) => {
            all_files
                .difference(files_to_keep)
                .for_each(|path| match fs::metadata(path) {
                    Ok(metadata) => {
                        if fs::remove_file(path).is_ok() {
                            num_files_deleted += 1;
                            space_reclaimed_bytes += metadata.len();

                            if is_delete_thumbnails {
                                if let Ok(bytes) = process_thumbnail(&path) {
                                    space_reclaimed_bytes += bytes;
                                }
                            }
                        }
                    }
                    Err(err) => {
                        error!("CleanMedia: Failed to delete file {:?}: {err}", path);
                    }
                })
        }
        Err(err) => {
            error!(
                "CleanMedia: Failed to collect all paths in {:?}: {err}",
                dir
            );
        }
    }

    Ok((num_files_deleted, space_reclaimed_bytes))
}

fn process_thumbnail(path: &Path) -> Result<u64> {
    if let Some(thumbnail) = generate_thumbnail_path(path) {
        match fs::metadata(&thumbnail) {
            Ok(metadata) => match fs::remove_file(&thumbnail) {
                Ok(_) => Ok(metadata.size()),
                Err(err) => {
                    error!(
                        "CleanMedia: Failed to remove thumbnail {:?}: {err}",
                        thumbnail
                    );
                    Err(err.into())
                }
            },
            Err(err) => {
                error!(
                    "CleanMedia: Failed to read metadata of thumbnail {:?}: {err}",
                    thumbnail
                );
                Err(err.into())
            }
        }
    } else {
        Err("CleanMedia: Thumbnail does not exist".into())
    }
}

fn generate_thumbnail_path(path: &Path) -> Option<PathBuf> {
    path.to_str()
        .and_then(|buf_str| buf_str.replace("Images", "Images/Thumbnails").into())
        .map(PathBuf::from)
}
