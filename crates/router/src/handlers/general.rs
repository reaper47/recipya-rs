use axum::Json;
use axum::extract::ws::WebSocket;
use axum::extract::{Multipart, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use serde_json::{Value, json};
use tokio::fs;
use tracing::error;
use uuid::Uuid;

use app::state::AppState;
use models::user::User;

use crate::middleware::mw_auth::CtxW;

const MAX_IMAGE_SIZE: usize = 10 * 1024 * 1024;

/// Handles the index page.
pub async fn index_handler() -> Redirect {
    Redirect::to("/auth/login")
}

/// Handles uploading a note's image to the appropriate data directory.
pub async fn upload_note_image(
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
pub async fn user_initials_handler(ctx: CtxW, State(state): State<AppState>) -> impl IntoResponse {
    let user_id = ctx.0.user_id();
    match User::get_user_by_id(&state.mm, user_id).await {
        Ok(Some(user)) => {
            if let Some(first) = user.email.to_uppercase().chars().next() {
                first.to_string()
            } else {
                "A".into()
            }
        }
        Ok(None) => {
            error!("User {user_id} does not exist");
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
    ctx: CtxW,
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(ctx.0.user_id(), socket, state))
}

async fn handle_socket(user_id: i64, socket: WebSocket, state: AppState) {
    state
        .subscribers
        .lock()
        .await
        .entry(user_id)
        .or_insert_with(Vec::new)
        .push(socket);
}
