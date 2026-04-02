use axum::response::IntoResponse;

use crate::{Result, middleware::mw_auth::RequireAuth};

/// Handles the recipe schema endpoint.
pub async fn recipe_schema_handler(RequireAuth(_): RequireAuth) -> Result<impl IntoResponse> {
    Ok(schema_org::Recipe::schema().into_response())
}
