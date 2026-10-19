use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use fluent_static::support::axum::RequestLanguage;
use tracing::error;
use uuid::Uuid;

use app::message::ToastCtx;
use app::state::AppState;
use l10n::Messages;
use models::user::{User, UserForCreate};
use support::states::AuthenticatedState;
use validator::Validate;

use crate::Error;
use crate::extractors::{ServerError, ValidatedForm};
use crate::middleware::mw_auth::RequireAuth;
use crate::schemas::admin::{UpdatePasswordForm, UserRowParams};
use crate::schemas::auth::RegisterForm;

/// Handles adding a user in the application.
pub async fn add_user_handler(
    RequireAuth(user): RequireAuth,
    RequestLanguage(messages): RequestLanguage<Messages>,
    State(state): State<AppState>,
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {
    let user_id = user.id;
    let toast = ToastCtx::<AuthenticatedState>::new(&state, user_id, &messages);

    if form.validate().is_err() {
        toast
            .error(&messages.toast_auth_email_or_password_invalid())
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
            error!(?user_id, ?err, "Error inserting user as admin");
            toast
                .error(&messages.toast_auth_email_or_password_invalid())
                .await;
            return Error::EntityExists { entity: "user" }.into_response();
        }
    };

    let num_users = match User::num_users(&state.mm).await {
        Ok(count) => count,
        Err(err) => {
            error!(?err, "Error fetching user count");
            toast
                .error(&messages.toast_users_fetch_users_failed())
                .await;
            return Error::Database.into_response();
        }
    };

    templates::settings::new_user_with_new_row(
        usize::try_from(num_users)
            .inspect_err(|err| error!(?num_users, ?err, "Failed to cast num users to usize"))
            .unwrap_or_default(),
        &user,
        &messages,
    )
    .into_response()
}

/// Handles deleting a user.
pub async fn delete_user_handler(
    RequireAuth(user): RequireAuth,
    RequestLanguage(messages): RequestLanguage<Messages>,
    Path(user_id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let toast = ToastCtx::<AuthenticatedState>::new(&state, user.id, &messages);

    let user = match User::get_user_by_id(&state.mm, user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            toast.error(&messages.toast_users_not_found()).await;
            return Error::EntityNotFound { entity: "user" }.into_response();
        }
        Err(err) => {
            error!(?user_id, ?err, "Error fetching user with id");
            toast.error(&messages.toast_users_fetch_user_failed()).await;
            return Error::Database.into_response();
        }
    };

    if user.is_admin {
        toast
            .error(&messages.toast_users_cannot_delete_admin())
            .await;
        return Error::DeleteForbidden.into_response();
    }

    if let Err(err) = User::delete(&state.mm, user_id).await {
        error!(?user_id, ?err, "Could not delete user with id");
        toast.error(&messages.toast_users_delete_failed()).await;
        return Error::DeleteUser.into_response();
    }

    let users = match User::all(&state.mm).await {
        Ok(users) => users,
        Err(err) => {
            error!(?err, "Error fetching users");
            toast
                .error(&messages.toast_users_fetch_users_failed())
                .await;
            return Error::Database.into_response();
        }
    };

    toast.success(&messages.toast_users_delete_success()).await;
    templates::settings::render_users_table(&users, true, &messages).into_response()
}

/// Renders the form to update the user form from the admin table.
pub async fn update_user_form_handler(
    RequireAuth(user): RequireAuth,
    RequestLanguage(messages): RequestLanguage<Messages>,
    Path(user_id): Path<Uuid>,
    Query(params): Query<UserRowParams>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let caller_user_id = user.id;
    let toast = ToastCtx::<AuthenticatedState>::new(&state, caller_user_id, &messages);

    let user = match User::get_user_by_id(&state.mm, user_id).await {
        Ok(user) => match user {
            None => {
                error!(?user_id, "User not found in database.");
                toast.error(&messages.toast_users_not_found()).await;
                return Error::NoUser.into_response();
            }
            Some(user) => user,
        },
        Err(err) => {
            error!(?user_id, ?err, "Error fetching user as admin");
            toast.error(&messages.toast_users_fetch_user_failed()).await;
            return Error::Database.into_response();
        }
    };

    templates::settings::edit_user_row(params.row_index, &user, &messages).into_response()
}

/// Handles updating a user.
pub async fn update_user_handler(
    RequireAuth(user): RequireAuth,
    RequestLanguage(messages): RequestLanguage<Messages>,
    state: State<AppState>,
    Path(user_id): Path<Uuid>,
    form: Result<ValidatedForm<UpdatePasswordForm>, ServerError>,
) -> impl IntoResponse {
    let toast = ToastCtx::<AuthenticatedState>::new(&state, user.id, &messages);

    let ValidatedForm(form) = match form {
        Ok(form) => form,
        Err(err) => {
            error!(?err, "Error validating update user form");
            toast.error(&messages.toast_payload_empty()).await;
            return Error::InvalidPayload.into_response();
        }
    };

    match user.update_password(&state.mm, form.new_password).await {
        Ok(()) => {}
        Err(err) => {
            error!(?user_id, ?err, "Error updating user password for user");
            toast
                .error(&messages.toast_auth_password_update_failed())
                .await;
            return Error::Database.into_response();
        }
    }

    let user = match User::get_user_by_id(&state.mm, user_id).await {
        Ok(user) => match user {
            None => {
                error!(?user_id, "User not found in database.");
                toast.error(&messages.toast_users_not_found()).await;
                return Error::NoUser.into_response();
            }
            Some(user) => user,
        },
        Err(err) => {
            error!(?user_id, ?err, "Error fetching user as admin");
            toast.error(&messages.toast_users_fetch_user_failed()).await;
            return Error::Database.into_response();
        }
    };

    toast
        .success(&messages.toast_auth_user_password_updated())
        .await;

    templates::settings::user_row(form.row_index, &user, &messages).into_response()
}

/// Renders a user row in the administrator's users panel.
pub async fn user_row_handler(
    RequireAuth(user): RequireAuth,
    RequestLanguage(messages): RequestLanguage<Messages>,
    Path(user_id): Path<Uuid>,
    Query(params): Query<UserRowParams>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let toast = ToastCtx::<AuthenticatedState>::new(&state, user.id, &messages);

    let user = match User::get_user_by_id(&state.mm, user_id).await {
        Ok(user) => match user {
            None => {
                error!(?user_id, "User not found in database.");
                toast.error(&messages.toast_users_not_found()).await;
                return Error::NoUser.into_response();
            }
            Some(user) => user,
        },
        Err(err) => {
            error!(?user_id, ?err, "Error fetching user as admin");
            toast.error(&messages.toast_users_fetch_user_failed()).await;
            return Error::Database.into_response();
        }
    };

    templates::settings::user_row(params.row_index, &user, &messages).into_response()
}
