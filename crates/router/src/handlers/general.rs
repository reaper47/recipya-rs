use std::fmt::Write;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::path::Path;

use axum::Json;
use axum::body::Body;
use axum::extract::ws::WebSocket;
use axum::extract::{Multipart, Query, State, WebSocketUpgrade};
use axum::http::{Response, StatusCode};
use axum::response::{Html, IntoResponse, Redirect};
use futures_util::StreamExt;
use models::paper::PaperSize;
use reqwest::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use serde_json::{Value, json};
use tokio::fs;
use tokio::net::lookup_host;
use tokio_util::bytes;
use tokio_util::io::ReaderStream;
use tracing::{error, warn};
use url::Url;
use uuid::Uuid;

use app::state::AppState;
use models::Recipe;
use models::download::Download;
use models::params::{DownloadParams, FetchParams, SearchParams};
use models::user::User;

use crate::handlers::message::broadcast_error;
use crate::middleware::mw_auth::{OptionalAuth, RequireAuth};
use crate::{Error, Result};

const MAX_IMAGE_SIZE: usize = 10 * 1024 * 1024;

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
    let token = params.token;

    let file_path = match Download::find_by_token(&state.mm, token).await {
        Ok(Some(dl)) => dl.file_path,
        Ok(_) => {
            warn!("Token '{token}' not found for user '{}'", user.id);
            return StatusCode::NOT_FOUND.into_response();
        }
        Err(err) => {
            error!(user = ?user.id, ?token, ?err, "Failed to find download token");
            broadcast_error(&state, user.id, "Failed to find download token.").await;
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
            broadcast_error(&state, user.id, "Failed to open export file.").await;
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
            broadcast_error(&state, user.id, "Failed to create export data response.").await;
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
    let parsed = match Url::parse(&params.url) {
        Ok(u) if matches!(u.scheme(), "http" | "https") => u,
        _ => {
            broadcast_error(&state, user.id, "Invalid URL").await;
            return StatusCode::BAD_REQUEST.into_response();
        }
    };

    if let Err(reason) = resolve_and_validate(&parsed).await {
        warn!("Fetch handler blocked '{parsed}' by SSRF guard: {reason}");
        broadcast_error(&state, user.id, "Invalid URL").await;
        return StatusCode::BAD_REQUEST.into_response();
    }

    let client = reqwest::Client::new();
    let mut res = match client.get(parsed).send().await {
        Ok(r) => r,
        Err(err) => {
            error!(?err, "Failed to fetch URL");
            broadcast_error(&state, user.id, "Could not fetch URL").await;
            return StatusCode::BAD_REQUEST.into_response();
        }
    };

    let content_type = res.headers_mut().remove(CONTENT_TYPE);

    let body = match res.bytes().await {
        Ok(b) => b,
        Err(err) => {
            error!(?err, "Failed to read response");
            broadcast_error(&state, user.id, "Could not read response").await;
            return StatusCode::BAD_GATEWAY.into_response();
        }
    };

    let mut res = body.into_response();
    if let Some(ct) = content_type {
        res.headers_mut().insert(CONTENT_TYPE, ct);
    }

    res
}

async fn resolve_and_validate(url: &Url) -> Result<()> {
    let host = url.host_str().ok_or(Error::MissingHost)?;
    if let Ok(ip) = host.parse::<IpAddr>() {
        if is_forbidden_ip(ip) {
            return Err(Error::ForbiddenIP);
        }
        return Ok(());
    }

    let port = url.port_or_known_default().unwrap_or(80);
    let addrs: Vec<IpAddr> = lookup_host((host, port))
        .await
        .map_err(|_| Error::DNSResolution)?
        .map(|s| s.ip())
        .collect();

    if addrs.is_empty() {
        return Err(Error::DNSResolution);
    }

    if addrs.iter().any(|ip| is_forbidden_ip(*ip)) {
        return Err(Error::ForbiddenIP);
    }

    Ok(())
}

fn is_forbidden_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => is_forbidden_v4(v4),
        IpAddr::V6(v6) => is_forbidden_v6(v6),
    }
}

const fn is_forbidden_v4(ip: Ipv4Addr) -> bool {
    matches!(
        ip.octets(),
        [127 | 10 | 0 | 224..=239 | 240..=255, ..]  // Loopback, private, unspecified, multicast, reserved
        | [172, 16..=31, ..]                        // Private 172.16.0.0/12
        | [192, 168, ..]                            // Private 192.168.0.0/16
        | [169, 254, ..]                            // Link-local + metadata (169.254.169.254)
        | [100, 64..=127, ..]                       // CGN 100.64.0.0/10
        | [198, 18..=19, ..]                        // Benchmarking
        | [192, 0, 2, _]                            // TEST-NET-1
        | [198, 51, 100, _]                         // TEST-NET-2
        | [203, 0, 113, _]                          // TEST-NET-3
    )
}

fn is_forbidden_v6(ip: Ipv6Addr) -> bool {
    if ip == Ipv6Addr::LOCALHOST || ip == Ipv6Addr::UNSPECIFIED {
        return true;
    }

    let segs = ip.segments();
    matches!(segs[0],
        // Link-local fe80::/10
        0xfe80..=0xfebf |
        // Unique-local fc00::/7 (includes fd00::/8)
        0xfc00..=0xfdff |
        // Multicast ff00::/8
        0xff00..=0xffff
    ) ||
    // IPv4-mapped ::ffff:0:0/96
    ip.to_ipv4_mapped()
        .is_some_and(is_forbidden_v4)
}

pub async fn paper_sizes_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let papers = match PaperSize::get_all(&state.mm).await {
        Ok(papers) => papers,
        Err(err) => {
            error!(?err, "Failed to get paper sizes");
            broadcast_error(&state, user.id, "Failed to fetch paper sizes").await;
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
                broadcast_error(&state, user.id, "Error fetching components.").await;
                return Err(Error::Database);
            }
        };

        Ok(Html(options))
    } else {
        Ok(Html(String::new()))
    }
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

/// WebSocket connection handler.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(user.id, socket, state))
}

async fn handle_socket(user_id: Uuid, socket: WebSocket, state: AppState) {
    state
        .subscribers
        .lock()
        .await
        .entry(user_id)
        .or_insert_with(Vec::new)
        .push(socket);
}
