use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Form;
use tracing::error;

use app::state::AppState;
use models::data::{AboutData, Data};
use models::settings::{Theme, UserSettingDetails};
use repository::ModelManager;
use templates::settings::SettingsForView;

use crate::Error;
use crate::handlers::helpers::is_hx_request;
use crate::handlers::message::broadcast_error;
use crate::handlers::recipes::fetch_categories_keywords;
use crate::middleware::mw_auth::CtxW;
use crate::settings_router::ThemePayload;

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

/// Handles setting the default theme for the target user.
pub async fn set_default_theme_handler(
    ctx: CtxW,
    State(state): State<AppState>,
    Form(payload): Form<ThemePayload>,
) -> impl IntoResponse {
    handle_theme_request(ctx, state, payload, |theme, mm, user_id| async move {
        theme.save_default(&mm, user_id).await
    })
    .await
}

/// Handles setting the selected theme for the target user.
pub async fn set_selected_theme_handler(
    ctx: CtxW,
    State(state): State<AppState>,
    Form(payload): Form<ThemePayload>,
) -> impl IntoResponse {
    handle_theme_request(ctx, state, payload, |theme, mm, user_id| async move {
        theme.save_selected(&mm, user_id).await
    })
    .await
}

async fn handle_theme_request<F, Fut>(
    ctx: CtxW,
    state: AppState,
    payload: ThemePayload,
    save_operation: F,
) -> impl IntoResponse
where
    F: FnOnce(Theme, ModelManager, i64) -> Fut,
    Fut: Future<Output = Result<(), models::Error>> + Send,
{
    let user_id = ctx.0.user_id();

    let theme = match payload.theme.parse::<Theme>() {
        Ok(theme) => theme,
        Err(_) => {
            error!("Invalid theme: {}", payload.theme);
            broadcast_error(
                &state,
                user_id,
                &format!("Theme '{}' is invalid.", payload.theme),
            )
            .await;
            return Error::InvalidPayload.into_response();
        }
    };

    if let Err(err) = save_operation(theme, state.mm.clone(), user_id).await {
        error!("Error saving selected theme for user {user_id}: {err}");
        broadcast_error(&state, user_id, "Error saving selected theme.").await;
        return Error::Database.into_response();
    }

    (StatusCode::NO_CONTENT, "").into_response()
}
