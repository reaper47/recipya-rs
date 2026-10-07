use axum::{
    Form,
    extract::{Path, State},
    response::IntoResponse,
};
use fluent_static::support::axum::RequestLanguage;
use time::{PrimitiveDateTime, macros::format_description};
use tracing::error;

use app::{
    message::{Broadcaster, Toast},
    state::AppState,
};
use l10n::Messages;
use models::share::ShareRecipe;

use crate::{Error, middleware::mw_auth::RequireAuth, params::ShareRecipeForm};

/// Handles generating a link for the recipe to share.
pub async fn share_recipe_post_handler(
    RequireAuth(user): RequireAuth,
    RequestLanguage(messages): RequestLanguage<Messages>,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
    Form(form): Form<ShareRecipeForm>,
) -> impl IntoResponse {
    let expires_at: Option<PrimitiveDateTime> = form.datetime.and_then(|dt| {
        PrimitiveDateTime::parse(
            dt.as_str(),
            format_description!("[year]-[month]-[day]T[hour]:[minute]"),
        )
        .or_else(|_| {
            PrimitiveDateTime::parse(
                dt.as_str(),
                format_description!(
                    "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:1+]"
                ),
            )
        })
        .or_else(|_| {
            PrimitiveDateTime::parse(
                dt.as_str(),
                format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
            )
        })
        .inspect_err(|err| {
            error!(?dt, ?err, "Invalid datetime");
        })
        .ok()
    });

    match ShareRecipe::new(&state.mm, recipe_id, user.id, expires_at).await {
        Ok(share) => {
            let url = format!(
                "{}/shared/r/{}",
                state.config.read().await.base_url,
                share.link
            );
            templates::general::share_link(&url, &messages).into_response()
        }
        Err(err) => {
            error!(?recipe_id, user = ?user.id, ?err, "Error generating shared recipe link");
            Toast::broadcast_error(
                &state,
                user.id,
                &messages.toast_recipes_share_link_failed(),
                &messages,
            )
            .await;
            Error::BadTimeFormat.into_response()
        }
    }
}
