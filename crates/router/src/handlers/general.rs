use axum::Json;
use axum::extract::ws::WebSocket;
use axum::extract::{Multipart, Query, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use serde_json::{Value, json};
use tokio::fs;
use tracing::error;
use uuid::Uuid;

use app::state::AppState;
use models::Recipe;
use models::params::SearchParams;
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
                .map(|item| {
                    let full_value = format!("{q}{item}");
                    format!(
                        r#"<li><a tabindex="0" _="on click
                            set #search-recipes.value to '{}'
                            then add .hidden to #search-suggestions-menu
                            then call #search-recipes.focus()
                            then set #search-recipes.selectionStart to #search-recipes.value.length
                            then set #search-recipes.selectionEnd to #search-recipes.value.length">{}</a></li>"#,
                        full_value.replace("'", "\\'"),
                        item
                    )
                })
                .collect::<Vec<String>>()
                .join(""),
            Err(err) => {
                error!(
                    "(search_suggestions_handler) Error fetching items '{q}' for user '{}': {err}",
                    user.id,
                );
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
                error!("Error reading file: {err}");
                return err_json(StatusCode::BAD_REQUEST, "importError");
            }
        };

        let kind = infer::get(&bytes);
        let _ = match kind {
            Some(k) if is_image_allowed(k.mime_type()) => (k.extension(), k.mime_type()),
            _ => return err_json(StatusCode::UNSUPPORTED_MEDIA_TYPE, "typeNotAllowed"),
        };

        if let Ok(tmp_path) = state.fs_support.upload_to_temp(bytes.clone()).await {
            let filename = Uuid::new_v4();

            state
                .fs_support
                .upload_image(&tmp_path, filename, &state.data_dir.images.notes);

            if let Err(err) = fs::remove_file(tmp_path).await {
                error!("Error removing temporary file '{filename}': {err}");
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
        Ok(Some(user)) => {
            if let Some(first) = user.email.to_uppercase().chars().next() {
                first.to_string()
            } else {
                "A".into()
            }
        }
        Ok(None) => {
            error!("User {} does not exist", user.id);
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
