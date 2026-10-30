use axum::{
    Form,
    extract::{Path, State},
    response::IntoResponse,
};
use fluent_static::support::axum::RequestLanguage;
use time::{PrimitiveDateTime, macros::format_description};
use tracing::error;

use app::{message::ToastCtx, state::AppState};
use l10n::Messages;
use models::share::ShareRecipe;
use support::states::AuthenticatedState;

use crate::{Error, middleware::mw_auth::RequireAuth, schemas::recipes::ShareRecipeForm};

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
            let toast = ToastCtx::<AuthenticatedState>::new(&state, user.id, &messages);
            error!(?recipe_id, user = ?user.id, ?err, "Error generating shared recipe link");
            toast
                .error(&messages.toast_recipes_share_link_failed())
                .await;
            Error::BadTimeFormat.into_response()
        }
    }
}
