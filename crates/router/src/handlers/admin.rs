use axum::Form;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use tracing::error;
use uuid::Uuid;
use validator::Validate;

use app::state::AppState;
use models::user::{User, UserForCreate};

use crate::Error;
use crate::handlers::message::{broadcast_error, broadcast_success};
use crate::middleware::mw_auth::RequireAuth;
use crate::schemas::admin::{UpdatePasswordForm, UserRowParams};
use crate::schemas::auth::RegisterForm;

/// Handles adding a user in the application.
pub async fn add_user_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {
    let user_id = user.id;

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

    templates::settings::new_user_with_new_row(
        usize::try_from(num_users)
            .inspect_err(|err| error!("Failed to cast num users to usize '{num_users}': {err}"))
            .unwrap_or_default(),
        &user,
    )
    .into_response()
}

/// Handles deleting a user.
pub async fn delete_user_handler(
    RequireAuth(user): RequireAuth,
    Path(user_id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let caller_user_id = user.id;

    let user = match User::get_user_by_id(&state.mm, user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            broadcast_error(&state, caller_user_id, "User not found.").await;
            return Error::EntityNotFound { entity: "user" }.into_response();
        }
        Err(err) => {
            error!("Error fetching user with id {user_id}: {err}");
            broadcast_error(&state, caller_user_id, "Failed to fetch user.").await;
            return Error::Database.into_response();
        }
    };

    if user.is_admin {
        broadcast_error(&state, caller_user_id, "Cannot delete an admin.").await;
        return Error::DeleteForbidden.into_response();
    }

    if let Err(err) = User::delete(&state.mm, user_id).await {
        error!("Could not delete user with id {user_id}: {err}");
        broadcast_error(&state, caller_user_id, "Failed to delete user.").await;
        return Error::DeleteUser.into_response();
    }

    let users = match User::all(&state.mm).await {
        Ok(users) => users,
        Err(err) => {
            error!("Error fetching users: {err}");
            broadcast_error(&state, caller_user_id, "Failed to fetch users.").await;
            return Error::Database.into_response();
        }
    };

    broadcast_success(&state, caller_user_id, "User deleted.").await;
    templates::settings::render_users_table(&users, true).into_response()
}

/// Renders the form to update the user form from the admin table.
pub async fn update_user_form_handler(
    RequireAuth(user): RequireAuth,
    Path(user_id): Path<Uuid>,
    Query(params): Query<UserRowParams>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let caller_user_id = user.id;

    let user = match User::get_user_by_id(&state.mm, user_id).await {
        Ok(user) => match user {
            None => {
                error!("User '{user_id}' not found in database.");
                broadcast_error(&state, caller_user_id, "User not found.").await;
                return Error::NoUser.into_response();
            }
            Some(user) => user,
        },
        Err(err) => {
            error!("Error fetching user '{user_id}' as admin: {err}");
            broadcast_error(&state, caller_user_id, "Error fetching user.").await;
            return Error::Database.into_response();
        }
    };

    templates::settings::edit_user_row(params.row_index, &user).into_response()
}

/// Handles updating a user.
pub async fn update_user_handler(
    RequireAuth(user): RequireAuth,
    state: State<AppState>,
    Path(user_id): Path<Uuid>,
    Form(form): Form<UpdatePasswordForm>,
) -> impl IntoResponse {
    let caller_user_id = user.id;

    if let Err(err) = form.validate() {
        error!("Error validating update user form: {err}");
        broadcast_error(&state, caller_user_id, "Payload cannot be empty.").await;
        return Error::InvalidPayload.into_response();
    }

    match User::update_password_by_user_id(&state.mm, user_id, &form.new_password).await {
        Ok(()) => {}
        Err(err) => {
            error!("Error updating user password for user #'{user_id}': {err}");
            broadcast_error(&state, caller_user_id, "Failed to update user password.").await;
            return Error::Database.into_response();
        }
    }

    let user = match User::get_user_by_id(&state.mm, user_id).await {
        Ok(user) => match user {
            None => {
                error!("User '{user_id}' not found in database.");
                broadcast_error(&state, caller_user_id, "User not found.").await;
                return Error::NoUser.into_response();
            }
            Some(user) => user,
        },
        Err(err) => {
            error!("Error fetching user '{user_id}' as admin: {err}");
            broadcast_error(&state, caller_user_id, "Error fetching user.").await;
            return Error::Database.into_response();
        }
    };

    broadcast_success(&state, caller_user_id, "User password updated.").await;
    templates::settings::user_row(form.row_index, &user).into_response()
}

/// Renders a user row in the administrator's users panel.
pub async fn user_row_handler(
    RequireAuth(user): RequireAuth,
    Path(user_id): Path<Uuid>,
    Query(params): Query<UserRowParams>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let caller_user_id = user.id;

    let user = match User::get_user_by_id(&state.mm, user_id).await {
        Ok(user) => match user {
            None => {
                error!("User '{user_id}' not found in database.");
                broadcast_error(&state, caller_user_id, "User not found.").await;
                return Error::NoUser.into_response();
            }
            Some(user) => user,
        },
        Err(err) => {
            error!("Error fetching user '{user_id}' as admin: {err}");
            broadcast_error(&state, caller_user_id, "Error fetching user.").await;
            return Error::Database.into_response();
        }
    };

    templates::settings::user_row(params.row_index, &user).into_response()
}
