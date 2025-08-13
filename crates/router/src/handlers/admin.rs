use axum::Form;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::error;
use validator::Validate;

use app::state::AppState;
use models::user::{User, UserForCreate};

use crate::Error;
use crate::auth_router::RegisterForm;
use crate::handlers::message::broadcast_error;
use crate::middleware::mw_auth::CtxW;

/// Handles adding a user in the application.
pub async fn add_user_handler(
    ctx: CtxW,
    State(state): State<AppState>,
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    if form.validate().is_err() {
        broadcast_error(
            &state,
            user_id,
            "Email is invalid or passwords do not match.",
        )
        .await;
        return Error::InvalidPayload.into_response();
    }

    let user = match User::new(
        &state.mm,
        UserForCreate {
            email: form.email,
            password_clear: form.password,
        },
    )
    .await
    {
        Ok(user) => user,
        Err(err) => {
            error!("Error inserting user '{user_id}' as admin: {err}");
            broadcast_error(&state, user_id, "A user with this email exists.").await;
            return Error::EntityExists { entity: "user" }.into_response();
        }
    };

    let num_users = match User::num_users(&state.mm).await {
        Ok(count) => count,
        Err(err) => {
            error!("Error fetching user count: {err}");
            broadcast_error(&state, user_id, "Failed to fetch number of users.").await;
            return Error::Database.into_response();
        }
    };

    templates::settings::new_user_with_new_row(num_users as usize, &user).into_response()
}

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
