use axum::Form;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use models::nutrition::NutritionDataSource;
use tracing::error;

use app::state::AppState;
use models::data::{AboutData, Data};
use models::settings::{Theme, UserSettingDetails};
use models::user::User;
use repository::ModelManager;
use templates::settings::{EmailSettingsForView, SettingsForView};

use crate::Error;
use crate::handlers::helpers::is_hx_request;
use crate::handlers::message::broadcast_error;
use crate::handlers::recipes::fetch_categories_keywords;
use crate::middleware::mw_auth::CtxW;
use crate::settings_router::{NutritionSourcePayload, ThemePayload};

/// Handles rendering the settings page.
pub async fn settings_handler(
    ctx: CtxW,
    header_map: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    let settings = match UserSettingDetails::get(&state.mm, user_id).await {
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

    let users = if user_id == 1 {
        User::all(&state.mm).await.ok()
    } else {
        None
    };

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
        users,
        settings,
        categories,
        &SettingsForView {
            is_autologin: config.is_autologin,
            is_allow_signups: config.is_no_signups,
            is_demo: config.is_demo,
            email: EmailSettingsForView {
                email_admin: email_config.email_admin.clone(),
                host: email_config.smtp_host.clone(),
                username: email_config.smtp_username.clone(),
                is_connected: state
                    .email_service
                    .map(|s| s.is_connected)
                    .unwrap_or_default(),
            },
            azure_di_key: "".to_string(),
            azure_di_endpoint: "".to_string(),
        },
    )
    .into_response()
}

/// Handles setting the nutrition source for the target user.
pub async fn set_nutrition_source_handler(
    ctx: CtxW,
    State(state): State<AppState>,
    Form(payload): Form<NutritionSourcePayload>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    let source = match payload.nutrition_source.parse::<NutritionDataSource>() {
        Ok(s) => s,
        Err(_) => {
            error!("Invalid nutrition source: {}", payload.nutrition_source);
            broadcast_error(
                &state,
                user_id,
                &format!(
                    "Nutrition source '{}' is invalid.",
                    payload.nutrition_source
                ),
            )
            .await;
            return Error::InvalidPayload.into_response();
        }
    };

    if let Err(err) = source.save(&state.mm, user_id).await {
        error!("Error saving selected nutrition source for user {user_id}: {err}");
        broadcast_error(&state, user_id, "Error saving selected nutrition source.").await;
        return Error::Database.into_response();
    }

    (StatusCode::NO_CONTENT, "").into_response()
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
