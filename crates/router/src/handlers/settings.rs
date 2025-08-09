use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use tracing::error;

use app::state::AppState;
use models::data::{AboutData, Data};
use models::settings::UserSettingDetails;
use templates::settings::SettingsForView;

use crate::Error;
use crate::handlers::helpers::is_hx_request;
use crate::handlers::message::broadcast_error;
use crate::handlers::recipes::fetch_categories_keywords;
use crate::middleware::mw_auth::CtxW;

/// Handles rendering the settings page.
pub async fn settings_handler(
    ctx: CtxW,
    header_map: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    let settings = match UserSettingDetails::get_settings(&state.mm, user_id).await {
        Ok(settings) => settings,
        Err(err) => {
            error!("Error fetching user settings for user {user_id}: {err}");
            broadcast_error(&state, user_id, "Error fetching user settings.").await;
            return Error::Database.into_response();
        }
    };

    let categories = match fetch_categories_keywords(&state, user_id).await {
        Ok((categories, _)) => categories
            .into_iter()
            .filter(|c| c.name != "uncategorized")
            .collect::<Vec<_>>(),
        Err(err) => {
            error!("Error fetching categories for user {user_id}: {err}");
            broadcast_error(&state, user_id, "Error fetching categories.").await;
            return Error::Database.into_response();
        }
    };

    let config = state.config.read().await;

    let email_config = if config.is_demo {
        &email::Config {
            smtp_host: "smtp.gmail.com".into(),
            smtp_username: "demo@demo.com".into(),
            smtp_password: "demo-password".into(),
            email_admin: "demo@demo.com".into(),
        }
    } else {
        email::email_config()
    };

    templates::settings::settings(
        Data {
            is_admin: user_id == 1,
            is_authenticated: true,
            is_autologin: config.is_autologin,
            is_hx_request: is_hx_request(&header_map),
            // TODO: Populate AboutData with good values.
            about: AboutData {
                is_update_available: false,
                is_check_update: false,
                last_checked_update_at: Default::default(),
                last_updated_at: Default::default(),
                version: "".to_string(),
            },
            ..Default::default()
        },
        settings,
        categories,
        &SettingsForView {
            is_autologin: config.is_autologin,
            is_no_signups: config.is_no_signups,
            is_production: false,
            email_admin: email_config.email_admin.clone(),
            smtp_host: email_config.smtp_host.clone(),
            smtp_username: email_config.smtp_username.clone(),
            smtp_password: email_config.smtp_password.clone(),
            azure_di_key: "".to_string(),
            azure_di_endpoint: "".to_string(),
        },
    )
    .into_response()
}
