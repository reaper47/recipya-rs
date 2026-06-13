use axum::{
    Form,
    extract::{Path, State},
    response::IntoResponse,
};
use time::{PrimitiveDateTime, macros::format_description};
use tracing::error;

use app::state::AppState;
use models::share::ShareRecipe;

use crate::{
    Error, handlers::message::broadcast_error, middleware::mw_auth::RequireAuth,
    recipes_router::params::ShareRecipeForm,
};

/// Handles generating a link for the recipe to share.
pub async fn share_recipe_post_handler(
    RequireAuth(user): RequireAuth,
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
            error!("Invalid datetime '{}': {}", dt, err);
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
            templates::general::share_link(&url).into_response()
        }
        Err(err) => {
            error!(
                "Error generating shared recipe link for recipe '{recipe_id}' and user '{}': {err}",
                user.id
            );
            broadcast_error(&state, user.id, "Error creating shared recipe link.").await;
            Error::BadTimeFormat.into_response()
        }
    }
}
