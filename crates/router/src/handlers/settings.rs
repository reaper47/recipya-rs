use axum::Form;
use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderMap, Response, StatusCode};
use axum::response::IntoResponse;
use axum_htmx::{HX_CURRENT_URL, HX_TRIGGER};
use config::{DemoState, States};
use iso8601::DateTime;
use serde_json::json;
use tracing::error;
use url::Url;
use uuid::Uuid;

use app::state::AppState;
use models::Recipe;
use models::data::{AboutData, Data};
use models::download::{Download, DownloadForCreate};
use models::export::ExportData;
use models::nutrition::NutritionDataSource;
use models::settings::{Theme, UserSettingDetails};
use models::user::User;
use repository::ModelManager;
use templates::settings::{EmailSettingsForView, SettingsForView};

use crate::Error;
use crate::handlers::helpers::is_hx_request;
use crate::handlers::message::{broadcast_error, broadcast_warning};
use crate::handlers::recipes::common::fetch_categories_keywords;
use crate::middleware::mw_auth::RequireAuth;
use crate::schemas::settings::{
    BoldIngredientsPayload, ExportDataPayload, NutritionSourcePayload, PaperSizeForm, ThemePayload,
    TzPayload,
};

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
            error!(?caller_user_id, ?err, "Error fetching user settings");
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
            error!(?caller_user_id, ?err, "Error fetching categories");
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

    let email_config = if config.states.demo == DemoState::On {
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
            states: States {
                autologin: config.states.autologin,
                ..Default::default()
            },
            is_hx_request: is_hx_request(&header_map),
            // TODO: Populate AboutData with good values.
            about: AboutData::new(false, false, DateTime::default(), DateTime::default()),
            ..Default::default()
        },
        users,
        &settings,
        &categories,
        &SettingsForView {
            states: config.states.clone(),
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

/// Handles exporting data for the target user.
pub async fn export_data_handler(
    RequireAuth(user): RequireAuth,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let recipes = match Recipe::all(&state.mm, user.id).await {
        Ok(recipes) => recipes,
        Err(err) => {
            error!(user = ?user.id, ?err, "Failed to retrieve recipes");
            broadcast_error(&state, user.id, "Failed to retrieve recipes.").await;
            return Error::Database.into_response();
        }
    };

    if recipes.is_empty() {
        broadcast_warning(&state, user.id, "No recipes found for export.").await;
        return StatusCode::NOT_FOUND.into_response();
    }

    let current_url = headers
        .get(HX_CURRENT_URL)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Url::parse(s).ok())
        .map_or_else(
            || "/".into(),
            |u| {
                format!(
                    "{}://{}{}",
                    u.scheme(),
                    u.host_str().unwrap_or(""),
                    u.port().map(|p| format!(":{p}")).unwrap_or_default()
                )
            },
        );

    templates::settings::render_export_data_dialog_recipes(&current_url, recipes).into_response()
}

/// Handles exporting data for the target user.
pub async fn export_data_post_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    axum_extra::extract::Form(payload): axum_extra::extract::Form<ExportDataPayload>,
) -> impl IntoResponse {
    if payload.recipe_ids.is_empty() {
        broadcast_warning(&state, user.id, "No recipes selected for export.").await;
        return StatusCode::BAD_REQUEST.into_response();
    }

    let recipes = match Recipe::get_many(&state.mm, user.id, &payload.recipe_ids).await {
        Ok(r) => r,
        Err(err) => {
            error!(user = ?user.id, ?payload, ?err, "Failed to fetch recipes");
            broadcast_error(&state, user.id, "Failed to fetch recipes.").await;
            return Error::Database.into_response();
        }
    };

    if recipes.is_empty() {
        broadcast_error(&state, user.id, "Failed to fetch recipes.").await;
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let file_path = match ExportData::new(payload.r#type, recipes, &state.data_dir.images.root)
        .export()
        .await
    {
        Ok(file) => file,
        Err(err) => {
            error!(user = ?user.id, ?err, "Failed to export recipes");
            broadcast_error(&state, user.id, "Failed to export recipes.").await;
            return Error::Fs.into_response();
        }
    };

    let token = Uuid::new_v4();
    let dl_c = DownloadForCreate::new(user.id, token, file_path);

    if let Err(err) = Download::create(&state.mm, dl_c).await {
        error!(user = ?user.id, ?err, "Failed to create download");
        broadcast_error(&state, user.id, "Failed to create export data response.").await;
        return Error::Database.into_response();
    }

    match Response::builder()
        .header(
            HX_TRIGGER,
            json!({
                "downloadReady": {
                    "url": format!("/download?token={}", token)
                }
            })
            .to_string(),
        )
        .body(Body::empty())
    {
        Ok(res) => res,
        Err(err) => {
            error!(user = ?user.id, ?err, "Failed to create response");
            broadcast_error(&state, user.id, "Failed to create export data response.").await;
            Error::Fs.into_response()
        }
    }
}

/// Handles setting the bold ingredients preference for the target user.
pub async fn set_bold_ingredients_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Form(form): Form<BoldIngredientsPayload>,
) -> impl IntoResponse {
    if let Err(err) = user
        .update_bold_ingredients(
            &state.mm,
            form.is_bold_ingredients.is_some_and(|s| &s == "on"),
        )
        .await
    {
        error!(user = ?user.id, ?err, "Error updating paper size");
        broadcast_error(&state, user.id, "Error updating paper size.").await;
        return Error::Database.into_response();
    }

    ().into_response()
}

/// Handles setting the nutrition source for the target user.
pub async fn set_nutrition_source_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Form(payload): Form<NutritionSourcePayload>,
) -> impl IntoResponse {
    let Ok(source) = payload.nutrition_source.parse::<NutritionDataSource>() else {
        error!(
            payload = payload.nutrition_source,
            "Invalid nutrition source"
        );
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
        error!(user = ?user.id, ?err, "Error saving selected nutrition source");
        broadcast_error(&state, user.id, "Error saving selected nutrition source.").await;
        return Error::Database.into_response();
    }

    (StatusCode::NO_CONTENT, "").into_response()
}

/// Handles setting the paper size for the target user.
pub async fn set_paper_size_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Form(form): Form<PaperSizeForm>,
) -> impl IntoResponse {
    if let Err(err) = user.update_paper_size(&state.mm, form.paper_size).await {
        error!(user = ?user.id, ?err, "Error updating paper size");
        broadcast_error(&state, user.id, "Error updating paper size.").await;
        return Error::Database.into_response();
    }

    ().into_response()
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

/// Handles setting the selected timezone for the target user.
pub async fn set_selected_timezone_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Form(payload): Form<TzPayload>,
) -> impl IntoResponse {
    match user.update_timezone(&state.mm, &payload.tz).await {
        Ok(()) => ().into_response(),
        Err(models::Error::Time) => {
            error!(tz = ?payload.tz, "Selected tz is invalid");
            broadcast_error(&state, user.id, "Invalid timezone.").await;
            Error::InvalidPayload.into_response()
        }
        Err(err) => {
            error!(user = ?user.id, ?err, "Error updating timezone");
            broadcast_error(&state, user.id, "Error updating timezone.").await;
            Error::Database.into_response()
        }
    }
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
        error!(theme = payload.theme, "Invalid theme");
        broadcast_error(
            &state,
            user.id,
            &format!("Theme '{}' is invalid.", payload.theme),
        )
        .await;
        return Error::InvalidPayload.into_response();
    };

    if let Err(err) = save_operation(theme, state.mm.clone(), user.id).await {
        error!(user = ?user.id, ?err, "Error saving selected theme");
        broadcast_error(&state, user.id, "Error saving selected theme.").await;
        return Error::Database.into_response();
    }

    (StatusCode::NO_CONTENT, "").into_response()
}
