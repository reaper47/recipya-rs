use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use axum::http::{StatusCode, Uri, header};
use axum::response::IntoResponse;
use rust_embed::Embed;
use tracing::error;

use crate::{Error, Result};

#[derive(Embed)]
#[folder = "../../web/public/"]
struct Asset;

/// Wrapper type for serving static files in the web application.
pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> axum::response::Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();

                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}

/// Serves static files for requests with a URI prefixed by `/public/`.
pub async fn static_files_handler(uri: Uri) -> impl IntoResponse {
    let path = uri
        .path()
        .trim_start_matches("/public/")
        .trim_start_matches('/');

    StaticFile(path).into_response()
}

/// Copies an embedded asset over to the file system.
pub fn copy_to_fs(src: &str, dest: PathBuf) -> Result<()> {
    let Some(asset) = Asset::get(src) else {
        return Err(Error::AssetCouldNotCopy);
    };

    let mut file = File::create_new(dest).map_err(|_| Error::FileExists)?;
    file.write_all(asset.data.trim_ascii()).map_err(|err| {
        error!("Failed to write content to file: {err}");
        Error::Fs
    })?;

    Ok(())
}
