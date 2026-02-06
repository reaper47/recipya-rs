use axum::Form;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use iso8601::DateTime;
use models::nutrition::NutritionDataSource;
use tracing::error;

use app::state::AppState;
use models::data::{AboutData, Data};
use models::settings::{Theme, UserSettingDetails};
use models::user::User;
use repository::ModelManager;
use templates::settings::{EmailSettingsForView, SettingsForView};
use uuid::Uuid;

use crate::Error;
use crate::handlers::helpers::is_hx_request;
use crate::handlers::message::broadcast_error;
use crate::handlers::recipes::fetch_categories_keywords;
use crate::middleware::mw_auth::RequireAuth;
use crate::schemas::settings::{NutritionSourcePayload, ThemePayload};

/// Handles rendering the settings page.
pub async fn settings_handler(
    header_map: HeaderMap,
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let caller_user = user;
    let caller_user_id = caller_user.id;

    let settings = match UserSettingDetails::get(&state.mm, caller_user_id).await {
        Ok(settings) => settings,
        Err(err) => {
            error!("Error fetching user settings for user {caller_user_id}: {err}");
            broadcast_error(&state, caller_user_id, "Error fetching user settings.").await;
            return Error::Database.into_response();
        }
    };

    let categories = match fetch_categories_keywords(&state, caller_user_id).await {
        Ok((categories, _)) => categories
            .into_iter()
            .filter(|c| c.name != "uncategorized")
            .collect::<Vec<_>>(),
        Err(err) => {
            error!("Error fetching categories for user {caller_user_id}: {err}");
            broadcast_error(&state, caller_user_id, "Error fetching categories.").await;
            return Error::Database.into_response();
        }
    };

    let config = state.config.read().await;
    let is_admin = caller_user.is_admin;

    let users = if is_admin {
        User::all(&state.mm).await.ok()
    } else {
        None
    };

    let email_config = if config.is_demo {
        &email::Config {
            smtp_host: "smtp.gmail.com".into(),
            smtp_port: 587,
            smtp_username: "demo@demo.com".into(),
            smtp_password: "demo-password".into(),
            smtp_from_email: "demo@demo.com".into(),
        }
    } else {
        email::email_config()
    };

    templates::settings::settings(
        &Data {
            is_admin,
            is_authenticated: true,
            is_autologin: config.is_autologin,
            is_hx_request: is_hx_request(&header_map),
            // TODO: Populate AboutData with good values.
            about: AboutData {
                is_update_available: false,
                is_check_update: false,
                last_checked_update_at: DateTime::default(),
                last_updated_at: DateTime::default(),
                version: String::new(),
            },
            ..Default::default()
        },
        users,
        &settings,
        &categories,
        &SettingsForView {
            is_autologin: config.is_autologin,
            is_allow_signups: config.is_no_signups,
            is_demo: config.is_demo,
            email: EmailSettingsForView {
                email_admin: email_config.smtp_from_email.clone(),
                host: email_config.smtp_host.clone(),
                username: email_config.smtp_username.clone(),
                is_connected: state.email_service.is_some_and(|s| s.is_connected),
            },
            azure_di_key: String::new(),
            azure_di_endpoint: String::new(),
        },
    )
    .into_response()
}

/// Handles setting the nutrition source for the target user.
pub async fn set_nutrition_source_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Form(payload): Form<NutritionSourcePayload>,
) -> impl IntoResponse {
    let Ok(source) = payload.nutrition_source.parse::<NutritionDataSource>() else {
        error!("Invalid nutrition source: {}", payload.nutrition_source);
        broadcast_error(
            &state,
            user.id,
            &format!(
                "Nutrition source '{}' is invalid.",
                payload.nutrition_source
            ),
        )
        .await;
        return Error::InvalidPayload.into_response();
    };

    if let Err(err) = source.save(&state.mm, user.id).await {
        error!(
            "Error saving selected nutrition source for user {}: {err}",
            user.id
        );
        broadcast_error(&state, user.id, "Error saving selected nutrition source.").await;
        return Error::Database.into_response();
    }

    (StatusCode::NO_CONTENT, "").into_response()
}

/// Handles setting the default theme for the target user.
pub async fn set_default_theme_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Form(payload): Form<ThemePayload>,
) -> impl IntoResponse {
    handle_theme_request(user, state, payload, |theme, mm, user_id| async move {
        theme.save_default(&mm, user_id).await
    })
    .await
    .into_response()
}

/// Handles setting the selected theme for the target user.
pub async fn set_selected_theme_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Form(payload): Form<ThemePayload>,
) -> impl IntoResponse {
    handle_theme_request(user, state, payload, |theme, mm, user_id| async move {
        theme.save_selected(&mm, user_id).await
    })
    .await
}

async fn handle_theme_request<F, Fut>(
    user: User,
    state: AppState,
    payload: ThemePayload,
    save_operation: F,
) -> impl IntoResponse
where
    F: FnOnce(Theme, ModelManager, Uuid) -> Fut,
    Fut: Future<Output = Result<(), models::Error>> + Send,
{
    let Ok(theme) = payload.theme.parse::<Theme>() else {
        error!("Invalid theme: {}", payload.theme);
        broadcast_error(
            &state,
            user.id,
            &format!("Theme '{}' is invalid.", payload.theme),
        )
        .await;
        return Error::InvalidPayload.into_response();
    };

    if let Err(err) = save_operation(theme, state.mm.clone(), user.id).await {
        error!("Error saving selected theme for user {}: {err}", user.id);
        broadcast_error(&state, user.id, "Error saving selected theme.").await;
        return Error::Database.into_response();
    }

    (StatusCode::NO_CONTENT, "").into_response()
}
