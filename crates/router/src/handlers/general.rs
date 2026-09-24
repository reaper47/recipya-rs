use std::convert::Infallible;
use std::fmt::Write;
use std::path::Path;

use axum::{
    Json,
    body::Body,
    extract::{Multipart, Query, State},
    http::{Response, StatusCode},
    response::{
        Html, IntoResponse, Redirect, Sse,
        sse::{Event, KeepAlive},
    },
};
use futures::Stream;
use reqwest::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use serde::Serialize;
use serde_json::{Value, json};
use tokio::fs;
use tokio_util::{bytes, io::ReaderStream};
use tracing::{error, warn};
use url::Url;
use uuid::Uuid;

use app::message::{Broadcaster, Toast};
use app::state::AppState;
use models::Recipe;
use models::download::Download;
use models::paper::PaperSize;
use models::params::{DownloadParams, FetchParams, SearchParams};
use models::user::User;
use support::net::{SsrfSafeClient, validate_ip};

use crate::middleware::mw_auth::{OptionalAuth, RequireAuth};
use crate::{Error, Result};

const MAX_IMAGE_SIZE: usize = 10 * 1024 * 1024;

/// Response structure for health checks.
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: Status,
    pub database: bool,
    pub email: Status,
    pub version: &'static str,
}

/// Status of the health check.
#[derive(Serialize, Clone)]
pub enum Status {
    Healthy,
    Degraded,
    Unhealthy,
    Disabled,
}

/// Handles the index page.
pub async fn index_handler(OptionalAuth(user): OptionalAuth) -> Redirect {
    match user {
        Some(_) => Redirect::to("/recipes"),
        None => Redirect::to("/auth/login"),
    }
}

/// Handles downloading a file from a public URL.
pub async fn download_handler(
    RequireAuth(user): RequireAuth,
    Query(params): Query<DownloadParams>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    use futures_util::StreamExt;

    let token = params.token;

    let file_path = match Download::find_by_token(&state.mm, token).await {
        Ok(Some(dl)) => dl.file_path,
        Ok(_) => {
            warn!("Token '{token}' not found for user '{}'", user.id);
            return StatusCode::NOT_FOUND.into_response();
        }
        Err(err) => {
            error!(user = ?user.id, ?token, ?err, "Failed to find download token");
            Toast::broadcast_error(&state, user.id, "Failed to find download token.").await;
            return StatusCode::NOT_FOUND.into_response();
        }
    };

    let file_name = Path::new(&file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("download");

    let mime_type = mime_guess::from_path(&file_path)
        .first()
        .map(|m| m.to_string())
        .unwrap_or_default();

    let file = match tokio::fs::File::open(&file_path).await {
        Ok(f) => f,
        Err(err) => {
            error!(?file_path, user = ?user.id, ?err, "Failed to open file");
            Toast::broadcast_error(&state, user.id, "Failed to open export file.").await;
            return Error::Fs.into_response();
        }
    };

    let mm = state.mm.clone();
    let file_path = file_path.clone();
    let stream = ReaderStream::new(file).chain(futures::stream::once(async move {
        if let Err(err) = Download::delete_by_token(&mm, token, file_path).await {
            error!(user = ?user.id, ?token, ?err, "Failed to delete download token");
        }
        Ok(bytes::Bytes::new())
    }));

    match Response::builder()
        .header(CONTENT_TYPE, mime_type)
        .header(
            CONTENT_DISPOSITION,
            format!("attachment; filename=\"{file_name}\""),
        )
        .body(Body::from_stream(stream))
    {
        Ok(res) => res,
        Err(err) => {
            error!(user = ?user.id, ?err, "Failed to create response");
            Toast::broadcast_error(&state, user.id, "Failed to create export data response.").await;
            Error::Fs.into_response()
        }
    }
}

/// Handles fetchinmg the content of a public URL.
pub async fn fetch_handler(
    RequireAuth(user): RequireAuth,
    Query(params): Query<FetchParams>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let parsed_url = match Url::parse(&params.url) {
        Ok(u) if matches!(u.scheme(), "http" | "https") => u,
        _ => {
            Toast::broadcast_error(&state, user.id, "Invalid URL").await;
            return StatusCode::BAD_REQUEST.into_response();
        }
    };

    if let Err(err) = validate_ip(&parsed_url) {
        const MESSAGE: &str = "Fetch handler blocked by SSRF guard";
        match err {
            support::net::Error::DNSResolution(url) => {
                warn!(?url, reason = "DNS Resolution", MESSAGE);
            }
            support::net::Error::ForbiddenIP => {
                warn!(?parsed_url, reason = "Forbidden IP", MESSAGE);
            }
            support::net::Error::MissingHost => {
                warn!(?parsed_url, reason = "Missing Host", MESSAGE);
            }
            err => {
                warn!(?parsed_url, ?err, "Failed to validate SSRF");
            }
        }
        Toast::broadcast_error(&state, user.id, "Invalid URL").await;
        return StatusCode::BAD_REQUEST.into_response();
    }

    let client = match SsrfSafeClient::new() {
        Ok(c) => c,
        Err(err) => {
            error!(?err, "Failed to create safe HTTP client");
            Toast::broadcast_error(&state, user.id, "Could not create HTTP client").await;
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let mut res = match client.get(parsed_url).await {
        Ok(r) => r,
        Err(err) => {
            error!(?err, "Failed to fetch URL");
            Toast::broadcast_error(&state, user.id, "Could not fetch URL").await;
            return StatusCode::BAD_REQUEST.into_response();
        }
    };

    let content_type = res.headers_mut().remove(CONTENT_TYPE);

    let body = match res.bytes().await {
        Ok(b) => b,
        Err(err) => {
            error!(?err, "Failed to read response");
            Toast::broadcast_error(&state, user.id, "Could not read response").await;
            return StatusCode::BAD_GATEWAY.into_response();
        }
    };

    let mut res = body.into_response();
    if let Some(ct) = content_type {
        res.headers_mut().insert(CONTENT_TYPE, ct);
    }

    res
}

/// Handler for live health checks.
pub async fn health_live_handler() -> &'static str {
    "OK"
}

/// Handler for ready health checks.
pub async fn health_ready_handler(
    OptionalAuth(_): OptionalAuth,
    State(state): State<AppState>,
) -> (StatusCode, Json<HealthResponse>) {
    let is_db_ok = state.mm.ping().await;

    let email_status = state.email_service.map_or(Status::Disabled, |email| {
        if email.test_connection() {
            Status::Healthy
        } else {
            Status::Unhealthy
        }
    });

    let status = match (is_db_ok, email_status.clone()) {
        (true, Status::Healthy | Status::Disabled) => Status::Healthy,
        (true, Status::Unhealthy) | (false, Status::Healthy) => Status::Unhealthy,
        _ => Status::Degraded,
    };

    let status_code = match status {
        Status::Healthy => StatusCode::OK,
        Status::Degraded | Status::Unhealthy | Status::Disabled => StatusCode::SERVICE_UNAVAILABLE,
    };

    (
        status_code,
        Json(HealthResponse {
            status,
            database: is_db_ok,
            email: email_status,
            version: env!("CARGO_PKG_VERSION"),
        }),
    )
}

/// Handler for fetching paper sizes.
pub async fn paper_sizes_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let papers = match PaperSize::get_all(&state.mm).await {
        Ok(papers) => papers,
        Err(err) => {
            error!(?err, "Failed to get paper sizes");
            Toast::broadcast_error(&state, user.id, "Failed to fetch paper sizes").await;
            return Error::Database.into_response();
        }
    };

    templates::general::render_paper_sizes_table(
        papers
            .iter()
            .flat_map(|(cat, sizes)| sizes.iter().map(|p| (cat.as_str(), p)))
            .enumerate()
            .map(|(idx, (cat, p))| (idx + 1, cat, p))
            .collect::<Vec<_>>()
            .as_slice(),
    )
    .into_response()
}

/// Handles searching for suggestions.
pub async fn search_suggestions_handler(
    RequireAuth(user): RequireAuth,
    Query(params): Query<SearchParams>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    if let Some(q) = params.q {
        let last_token = q.split_whitespace().last().unwrap_or("").to_lowercase();

        let res = if last_token.starts_with("cat:") {
            Recipe::fetch_categories(&state.mm, user.id).await
        } else if last_token.starts_with("cui:") {
            Recipe::fetch_cuisines(&state.mm, user.id).await
        } else if last_token.starts_with("ing:") {
            Recipe::fetch_ingredients(&state.mm, user.id).await
        } else if last_token.starts_with("kw:") {
            Recipe::fetch_keywords(&state.mm, user.id).await
        } else if last_token.starts_with("tool:") {
            Recipe::fetch_tools(&state.mm, user.id).await
        } else if last_token.starts_with("src:") {
            Recipe::fetch_sources(&state.mm, user.id).await
        } else {
            Ok(vec![])
        };

        let options = match res {
            Ok(items) => items
                .iter()
                .fold(String::new(), |mut acc, item| {
                    let full_value = format!("{q}{item}");
                    let _ = write!(
                        acc,
                        r#"<li><a tabindex="0" _="on click
                            set #search-recipes.value to '{}'
                            then add .hidden to #search-suggestions-menu
                            then call #search-recipes.focus()
                            then set #search-recipes.selectionStart to #search-recipes.value.length
                            then set #search-recipes.selectionEnd to #search-recipes.value.length">{}</a></li>"#,
                        full_value.replace('\'', "\\'"),
                        item
                    );
                    acc
                }),
            Err(err) => {
                error!(?q, user = ?user.id, ?err, "(search_suggestions_handler) Error fetching items");
                Toast::broadcast_error(&state, user.id, "Error fetching components.").await;
                return Err(Error::Database);
            }
        };

        Ok(Html(options))
    } else {
        Ok(Html(String::new()))
    }
}

struct ConnectedClientsGuard {
    state: AppState,
    user_id: Uuid,
}

impl ConnectedClientsGuard {
    fn new(state: &AppState, user_id: Uuid) -> Self {
        Self {
            state: state.clone(),
            user_id,
        }
    }
}

impl Drop for ConnectedClientsGuard {
    fn drop(&mut self) {
        let state = self.state.clone();
        let user_id = self.user_id;
        tokio::spawn(async move { state.channels.unsubscribe_client(user_id).await });
    }
}

/// SSE handler for streaming events to the client.
pub async fn sse_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = std::result::Result<Event, Infallible>>> {
    use tokio_stream::StreamExt as _;

    let mut client_rx = tokio_stream::wrappers::BroadcastStream::new(
        state.channels.subscribe_client(user.id).await,
    );
    let mut shutdown_rx = state.channels.shutdown_rx();
    let guard = ConnectedClientsGuard::new(&state, user.id);

    let stream = async_stream::stream! {
        let _guard = guard;
        loop {
            tokio::select! {
                msg = client_rx.next() => {
                    match msg {
                        Some(Ok(msg)) => yield Event::default().data(msg),
                        Some(Err(_)) => {},
                        None => break,
                    }
                }
                _ = shutdown_rx.changed() => {
                    break;
                }
            }
        }
    };

    Sse::new(stream.map(Ok)).keep_alive(KeepAlive::default())
}

/// Handles uploading a note's image to the appropriate data directory.
pub async fn upload_note_image(
    RequireAuth(_): RequireAuth,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() != Some("image") {
            continue;
        }

        let bytes = match field.bytes().await {
            Ok(b) if b.len() <= MAX_IMAGE_SIZE => b,
            Ok(_) => return err_json(StatusCode::PAYLOAD_TOO_LARGE, "fileTooLarge"),
            Err(err) => {
                error!(?err, "Error reading file");
                return err_json(StatusCode::BAD_REQUEST, "importError");
            }
        };

        let kind = infer::get(&bytes);
        let _ = match kind {
            Some(k) if is_image_allowed(k.mime_type()) => (k.extension(), k.mime_type()),
            _ => return err_json(StatusCode::UNSUPPORTED_MEDIA_TYPE, "typeNotAllowed"),
        };

        if let Ok(tmp_path) = state.fs_support.upload_to_temp(bytes).await {
            let filename = Uuid::new_v4();

            state
                .fs_support
                .upload_image(&tmp_path, filename, &state.data_dir.images.notes);

            if let Err(err) = fs::remove_file(tmp_path).await {
                error!(?filename, ?err, "Error removing temporary file");
            }

            let result = json!({
                "data": {
                    "filePath": format!("data/images/Notes/{filename}.webp"),
                }
            });
            return (StatusCode::OK, Json(result));
        }
    }

    err_json(StatusCode::BAD_REQUEST, "noFileGiven")
}

fn err_json(status: StatusCode, code: &str) -> (StatusCode, Json<Value>) {
    (status, Json(json!({"error": code})))
}

fn is_image_allowed(mime: &str) -> bool {
    matches!(
        mime,
        "image/png"
            | "image/jpeg"
            | "image/gif"
            | "image/webp"
            | "image/svg+xml"
            | "image/avif"
            | "image/heic"
            | "image/heif"
    )
}

/// Handles retrieving the user's initials.
pub async fn user_initials_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match User::get_user_by_id(&state.mm, user.id).await {
        Ok(Some(user)) => user
            .email
            .to_uppercase()
            .chars()
            .next()
            .map_or_else(|| "A".into(), |first| first.to_string()),
        Ok(None) => {
            error!(user = ?user.id, "User does not exist");
            "A".into()
        }
        Err(err) => {
            error!("Error getting user: {err}");
            "A".into()
        }
    }
}
