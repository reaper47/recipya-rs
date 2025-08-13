use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::error;

use app::state::AppState;
use models::user::User;

use crate::Error;
use crate::handlers::message::broadcast_error;
use crate::middleware::mw_auth::CtxW;

/// Handles deleting a user.
pub async fn delete_user_handler(
    ctx: CtxW,
    Path(user_id): Path<i64>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if user_id == 1 {
        broadcast_error(&state, ctx.0.user_id(), "Cannot delete an admin.").await;
        return Error::DeleteForbidden.into_response();
    }

    match User::delete(&state.mm, user_id).await {
        Ok(_) => (StatusCode::NO_CONTENT, "").into_response(),
        Err(err) => {
            error!("Could not delete user with id {user_id}: {err}");
            Error::DeleteUser.into_response()
        }
    }
}
