use std::collections::HashMap;
use std::str::FromStr;

use axum::Form;
use axum::body::Body;
use axum::extract::{OriginalUri, Path, Query};
use axum::http::HeaderMap;
use axum::response::Response;
use axum::{extract::State, response::IntoResponse};
use axum_htmx::{HX_PROMPT, HX_TRIGGER};
use itertools::izip;
use mime_guess::mime::TEXT_PLAIN_UTF_8;
use reqwest::StatusCode;
use reqwest::header::CONTENT_TYPE;
use serde_json::json;
use time::PrimitiveDateTime;
use time::macros::format_description;
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies};
use tracing::error;
use uuid::Uuid;

use app::state::AppState;
use models::Recipe;
use models::data::{Data, ShoppingData};
use models::download::{Download, DownloadForCreate};
use models::export::{ExportOptions, ExportType};
use models::paper::PaperSize;
use models::params::{ShoppingListExportParams, ViewParams};
use models::settings::UserSettingDetails;
use models::shopping::{
    ShareShoppingList, ShoppingList, ShoppingListDetails, ShoppingListItemForCreate,
    ShoppingListItemForUpdate,
};
use models::view::ViewMode;
use templates::shopping::AddShoppingIngredient;

use crate::handlers::get_settings;
use crate::handlers::helpers::is_hx_request;
use crate::handlers::message::{broadcast_error, broadcast_success, broadcast_warning};
use crate::middleware::mw_auth::{OptionalAuth, RequireAuth};
use crate::recipes_router::params::ShareRecipeForm;
use crate::schemas::shopping::{ListItemPayload, ListPayload, RecipeIngredientsPayload};
use crate::{Error, Result};

/// Name of the cookie that stores the selected view mode.
pub const SHOPPING_VIEW_COOKIE_NAME: &str = "view:shopping-list";

/// Handles fetching the user's shopping lists.
pub async fn shopping_lists_handler(
    header_map: HeaderMap,
    RequireAuth(user): RequireAuth,
    OriginalUri(uri): OriginalUri,
    State(state): State<AppState>,
    cookies: Cookies,
) -> Result<impl IntoResponse> {
    let settings = get_settings(&state, user.id).await?;

    let shopping_lists = match ShoppingList::get_all(&state.mm, user.id).await {
        Ok(lists) => lists,
        Err(err) => {
            error!("Failed to get shopping lists for user '{}': {err}", user.id);
            broadcast_error(&state, user.id, "Failed to get shopping lists.").await;
            return Err(Error::Database);
        }
    };

    let selected_shopping_list = if let Some(list) = shopping_lists.first() {
        match ShoppingListDetails::get(&state.mm, list.id, user.id).await {
            Ok(details) => Some(details),
            Err(err) => {
                error!(
                    "Failed to get shopping list details of list '{}' for user '{}': {err}",
                    list.id, user.id
                );
                broadcast_error(&state, user.id, "Failed to get shopping list details.").await;
                None
            }
        }
    } else {
        None
    };

    let labels = match ShoppingList::labels(&state.mm, user.id).await {
        Ok(labels) => Some(labels),
        Err(err) => {
            error!("Failed to fetch the labels for user '{}': {err}", user.id);
            None
        }
    };

    Ok(templates::shopping::lists_index(
        uri.path(),
        &Data {
            is_admin: user.is_admin,
            is_authenticated: true,
            is_autologin: state.config.read().await.is_autologin,
            is_hx_request: is_hx_request(&header_map),
            shopping: Some(ShoppingData {
                labels,
                shopping_lists,
                selected_shopping_list,
                selected_view_mode: get_view_mode_from_cookie(&cookies),
            }),
            ..Default::default()
        },
        &settings,
    )
    .into_response())
}

/// Handles GET requests to retrieve a shopping list.
pub async fn shopping_list_handler(
    header_map: HeaderMap,
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    cookies: Cookies,
    Path(list_id): Path<Uuid>,
) -> impl IntoResponse {
    let list = match ShoppingListDetails::get(&state.mm, list_id, user.id).await {
        Ok(list) => list,
        Err(err) => {
            error!("Failed to get shopping list: {err}");
            return Error::Database.into_response();
        }
    };

    let is_hx_request = is_hx_request(&header_map);

    match get_view_mode_from_cookie(&cookies) {
        ViewMode::Edit | ViewMode::Print => {
            templates::shopping::render_shopping_list_view_edit(&list, is_hx_request)
                .into_response()
        }
        ViewMode::View => templates::shopping::render_shopping_list_view_view(&list, is_hx_request)
            .into_response(),
    }
}

/// Handles DELETE requests to delete a shopping list.
pub async fn shopping_list_delete_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(list_id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(err) = ShoppingList::delete(&state.mm, list_id, user.id).await {
        error!("Failed to delete shopping list: {err}");
        broadcast_error(&state, user.id, "Failed to delete shopping list.").await;
        return Error::Database.into_response();
    }

    (StatusCode::SEE_OTHER, [("HX-Redirect", "/shopping/lists")]).into_response()
}

/// Handles PUT requests to update the title of a shopping list.
pub async fn shopping_list_put_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(list_id): Path<Uuid>,
    Form(payload): Form<ListPayload>,
) -> impl IntoResponse {
    if payload.name.is_empty() {
        broadcast_error(&state, user.id, "Title cannot be empty.").await;
        return Error::InvalidPayload.into_response();
    }

    match ShoppingList::update_title(&state.mm, list_id, &payload.name, user.id).await {
        Ok(()) => {
            let num_items = ShoppingList::get(&state.mm, list_id, user.id)
                .await
                .map_or(0, |l| l.num_items);

            templates::shopping::render_shopping_list_title(list_id, &payload.name, num_items)
                .into_response()
        }
        Err(err) if err.to_string().contains("duplicate key") => {
            broadcast_warning(&state, user.id, "Title already exists.").await;
            Error::InvalidPayload.into_response()
        }
        Err(err) => {
            error!("Failed to update shopping list title: {err}");
            broadcast_error(&state, user.id, "Failed to update shopping list title.").await;
            Error::Database.into_response()
        }
    }
}

/// Handles POST requests to copy a shopping list.
pub async fn shopping_list_copy_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(list_id): Path<Uuid>,
    Query(params): Query<ShoppingListExportParams>,
) -> Result<impl IntoResponse> {
    let list = ShoppingListDetails::get(&state.mm, list_id, user.id).await?;

    let mut writer = Vec::new();

    let res = match params.format {
        models::export::ExportType::Markdown => list.write_markdown(&mut writer),
        models::export::ExportType::Text => list.write_text(&mut writer),
        models::export::ExportType::Json | models::export::ExportType::Pdf => {
            unimplemented!()
        }
    };

    let body = match res {
        Ok(()) => Body::from(String::from_utf8(writer).unwrap_or_default()),
        Err(models::Error::EmptyInput) => {
            broadcast_warning(&state, user.id, "Shopping list is empty.").await;
            return Err(Error::Write);
        }
        Err(err) => {
            error!(
                "Failed to write shopping list to '{:?}': {err}",
                params.format
            );
            broadcast_error(&state, user.id, "Failed to write shopping list.").await;
            return Err(Error::Write);
        }
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, TEXT_PLAIN_UTF_8.to_string())
        .body(body)?)
}

/// Handles GET requests to export a shopping list.
pub async fn shopping_list_export_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(list_id): Path<Uuid>,
    Query(params): Query<ShoppingListExportParams>,
) -> Result<impl IntoResponse> {
    let list = ShoppingListDetails::get(&state.mm, list_id, user.id).await?;

    let options = if params.format == ExportType::Pdf {
        pdf_export_options(&state, user.id).await?
    } else {
        None
    };

    let path = match list.export(&params.format, options) {
        Ok(f) => f,
        Err(models::Error::EmptyInput) => {
            broadcast_warning(&state, user.id, "Shopping list is empty.").await;
            return Err(Error::Write);
        }
        Err(err) => {
            error!("Failed to export shopping list: {err}");
            broadcast_error(&state, user.id, "Failed to export shopping list.").await;
            return Err(Error::Database);
        }
    };

    let token = Uuid::new_v4();
    let dl_c = DownloadForCreate::new(user.id, token, path);

    if let Err(err) = Download::create(&state.mm, dl_c).await {
        error!("Failed to create download for user {}: {err}", user.id);
        broadcast_error(&state, user.id, "Failed to create export data response.").await;
        return Err(Error::Database);
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
        Ok(res) => Ok(res),
        Err(err) => {
            error!("Failed to create response for user {}: {err}", user.id);
            broadcast_error(&state, user.id, "Failed to create export data response.").await;
            Err(Error::Fs)
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
async fn pdf_export_options(state: &AppState, user_id: Uuid) -> Result<Option<ExportOptions>> {
    let settings = UserSettingDetails::get(&state.mm, user_id)
        .await
        .inspect_err(|err| error!("Failed to get user settings: {err}"))
        .map_err(|_| {
            let state = state.clone();
            tokio::spawn(async move {
                broadcast_error(&state, user_id, "Failed to get user settings.").await;
            });
            Error::Database
        })?;

    let paper_size = PaperSize::get(&state.mm, settings.paper_size_id)
        .await
        .inspect_err(|err| error!("Failed to get paper size for user '{}': {err}", user_id))
        .map_err(|_| {
            let state = state.clone();
            tokio::spawn(async move {
                broadcast_error(&state, user_id, "Failed to get paper size.").await;
            });
            Error::Database
        })?;

    Ok(Some(ExportOptions {
        paper_size: (
            paper_size.width_in as f32 * 72.0,
            paper_size.height_in as f32 * 72.0,
        ),
    }))
}

/// Handles GET requests to print a shopping list.
pub async fn shopping_list_print_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(list_id): Path<Uuid>,
) -> impl IntoResponse {
    let list = match ShoppingListDetails::get(&state.mm, list_id, user.id).await {
        Ok(list) => list,
        Err(err) => {
            error!("Failed to get shopping list: {err}");
            return Error::Database.into_response();
        }
    };

    let content = templates::shopping::render_shopping_list_print_mode(&list);

    templates::general::render_print_view(&content).into_response()
}

/// Handles GET requests to view a shopping list.
pub async fn shopping_list_view_handler(
    header_map: HeaderMap,
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    cookies: Cookies,
    Path(list_id): Path<Uuid>,
    Query(params): Query<ViewParams>,
) -> impl IntoResponse {
    let list = match ShoppingListDetails::get(&state.mm, list_id, user.id).await {
        Ok(list) => list,
        Err(err) => {
            error!(
                "Failed to get shopping list '{list_id}' for user '{}': {err}",
                user.id
            );
            broadcast_error(&state, user.id, "Failed to fetch shopping list.").await;
            return Error::Database.into_response();
        }
    };

    set_shopping_view_cookie(
        &cookies,
        params.mode.to_string(),
        state.config.read().await.is_production,
    );

    let is_hx_request = is_hx_request(&header_map);

    match params.mode {
        ViewMode::Edit => templates::shopping::render_shopping_list_view_edit(&list, is_hx_request),
        ViewMode::Print | ViewMode::View => {
            templates::shopping::render_shopping_list_view_view(&list, is_hx_request)
        }
    }
    .into_response()
}

fn set_shopping_view_cookie(cookies: &Cookies, token_value: String, is_production: bool) {
    let mut cookie = Cookie::new(SHOPPING_VIEW_COOKIE_NAME, token_value);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Strict);
    cookie.set_path("/");
    // cookie.set_max_age(cookie::time::Duration::hours(7 * 24));

    if is_production {
        cookie.set_secure(true);
    }

    cookies.add(cookie);
}

/// Handles POST requests to share a shopping list.
pub async fn shopping_list_share_post_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(list_id): Path<Uuid>,
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
                format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"),
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

    match ShareShoppingList::new(&state.mm, list_id, user.id, expires_at).await {
        Ok(share) => {
            let url = format!(
                "{}/shared/sl/{}",
                state.config.read().await.base_url,
                share.link
            );
            templates::general::share_link(&url).into_response()
        }
        Err(err) => {
            error!(
                "Error generating shared recipe link for recipe '{list_id}' and user '{}': {err}",
                user.id
            );
            broadcast_error(&state, user.id, "Error creating shared shopping list link.").await;
            Error::BadTimeFormat.into_response()
        }
    }
}

/// Handles POST requests to add a label to a shopping list.
pub async fn shopping_list_labels_post_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(list_id): Path<Uuid>,
    Form(form): Form<ListPayload>,
) -> impl IntoResponse {
    if form.name.is_empty() {
        broadcast_warning(&state, user.id, "Label name cannot be empty.").await;
        return Error::InvalidPayload.into_response();
    }

    match ShoppingList::new_label(&state.mm, &form.name, list_id, user.id).await {
        Err(models::Error::DuplicateEntity) => {
            broadcast_warning(
                &state,
                user.id,
                "Label already exists in the shopping list.",
            )
            .await;
            return Error::EntityExists {
                entity: "shopping_list_label",
            }
            .into_response();
        }
        Err(err) => {
            broadcast_error(&state, user.id, "Error creating shopping list label.").await;
            error!("Failed to create shopping list label: {err}");
            return Error::Database.into_response();
        }
        _ => {}
    }

    templates::shopping::render_shopping_list_items(form.name, list_id, &[], true).into_response()
}

/// Handles GET requests to add a new label to a shopping list.
pub async fn shopping_list_labels_new_handler(
    RequireAuth(_): RequireAuth,
    Path(list_id): Path<Uuid>,
) -> impl IntoResponse {
    templates::shopping::render_label_new(list_id).into_response()
}

/// Handles PUT requests to update a label on a shopping list.
pub async fn shopping_list_label_put_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path((list_id, label_id)): Path<(Uuid, i64)>,
    Form(payload): Form<ListPayload>,
) -> impl IntoResponse {
    let new_label_id =
        match ShoppingList::get_or_insert_label(&state.mm, &payload.name, user.id).await {
            Ok(id) => id,
            Err(err) => {
                error!("Failed to update shopping list label: {err}");
                broadcast_error(&state, user.id, "Failed to update shopping list label.").await;
                return Error::Database.into_response();
            }
        };

    if let Err(err) =
        ShoppingList::update_item_labels(&state.mm, list_id, label_id, new_label_id, user.id).await
    {
        error!("Failed to update shopping list label: {err}");
        broadcast_error(&state, user.id, "Failed to update shopping list label.").await;
        return Error::Database.into_response();
    }

    templates::shopping::render_label(&payload.name, list_id, new_label_id).into_response()
}

/// Handles POST requests to create a new shopping list.
pub async fn shopping_lists_post_handler(
    header_map: HeaderMap,
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    cookies: Cookies,
) -> impl IntoResponse {
    let Some(title) = header_map
        .get(HX_PROMPT)
        .map(|h| h.to_str().ok().unwrap_or_default().trim())
        .filter(|s| !s.is_empty())
    else {
        broadcast_warning(&state, user.id, "List title must not be empty.").await;
        return Error::InvalidPayload.into_response();
    };

    let list_id = match ShoppingList::create(&state.mm, title, user.id).await {
        Ok(id) => id,
        Err(err) => {
            error!("Failed to create shopping list: {err}");
            broadcast_error(&state, user.id, "Failed to create shopping list.").await;
            return Error::Database.into_response();
        }
    };

    templates::shopping::render_new_shopping_list(
        &get_view_mode_from_cookie(&cookies),
        list_id,
        title,
    )
    .into_response()
}

fn get_view_mode_from_cookie(cookies: &Cookies) -> ViewMode {
    cookies
        .get(SHOPPING_VIEW_COOKIE_NAME)
        .map(|s| ViewMode::from_str(s.value()).unwrap_or_default())
        .unwrap_or_default()
}

/// Handles POST requests to add an item to a shopping list.
pub async fn shopping_list_item_post_handler(
    RequireAuth(user): RequireAuth,
    Path(list_id): Path<Uuid>,
    State(state): State<AppState>,
    Form(payload): Form<ListItemPayload>,
) -> impl IntoResponse {
    let item = payload.item.trim();
    if item.is_empty() {
        broadcast_warning(&state, user.id, "Item name must not be empty.").await;
        return Error::InvalidPayload.into_response();
    }

    let item = match ShoppingList::add_item(
        &state.mm,
        list_id,
        ShoppingListItemForCreate::new(
            payload.quantity.filter(|q| !q.is_empty()),
            item,
            payload.label,
            payload.notes,
            None,
        ),
        user.id,
    )
    .await
    {
        Ok(id) => id,
        Err(err) if err.to_string().contains("duplicate") => {
            broadcast_warning(&state, user.id, "Item already exists in the label.").await;
            return Error::EntityExists {
                entity: "shopping list item",
            }
            .into_response();
        }
        Err(err) => {
            error!("Failed to add shopping list item: {err}");
            broadcast_error(&state, user.id, "Failed to add shopping list item.").await;
            return Error::Database.into_response();
        }
    };

    let num_items = ShoppingList::items_count(&state.mm, list_id)
        .await
        .unwrap_or_default();

    templates::shopping::render_shopping_list_item_with_count(list_id, &item, num_items, false)
        .into_response()
}

/// Handles PUT requests to update the positions of shopping list items.
pub async fn shopping_list_items_positions_put_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(list_id): Path<Uuid>,
    Form(payload): Form<HashMap<i64, i32>>,
) -> impl IntoResponse {
    if let Err(err) =
        ShoppingList::update_item_positions(&state.mm, list_id, payload, user.id).await
        && !err
            .to_string()
            .contains("shopping_list_items_quantity_check")
    {
        error!("Failed to update shopping list item: {err}");
        broadcast_error(&state, user.id, "Failed to update shopping list item.").await;
        return Error::Database.into_response();
    }

    ().into_response()
}

/// Handles DELETE requests to delete a shopping list item.
pub async fn shopping_list_item_delete_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path((list_id, item_id)): Path<(Uuid, i64)>,
) -> impl IntoResponse {
    if let Err(err) = ShoppingList::delete_item(&state.mm, list_id, item_id, user.id).await {
        error!("Failed to delete shopping list item: {err}");
        broadcast_error(&state, user.id, "Failed to delete shopping list item.").await;
        return Error::Database.into_response();
    }

    let num_items = ShoppingList::items_count(&state.mm, list_id)
        .await
        .unwrap_or_default();

    templates::shopping::render_shopping_list_item_count(list_id, num_items, true).into_response()
}

/// Handles PUT requests to update a shopping list item.
pub async fn shopping_list_item_put_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path((list_id, item_id)): Path<(Uuid, i64)>,
    Form(payload): Form<ListItemPayload>,
) -> impl IntoResponse {
    let item_u = ShoppingListItemForUpdate {
        ingredient: Some(payload.item),
        quantity: payload.quantity,
        label: None,
        position: payload.position,
        is_checked: None,
        notes: payload.notes,
    };

    if let Err(err) = ShoppingList::update_item(&state.mm, list_id, item_id, item_u, user.id).await
        && !err
            .to_string()
            .contains("shopping_list_items_quantity_check")
    {
        error!("Failed to update shopping list item: {err}");
        broadcast_error(&state, user.id, "Failed to update shopping list item.").await;
        return Error::Database.into_response();
    }

    match ShoppingList::get_item(&state.mm, list_id, item_id, user.id).await {
        Ok(item) => {
            let num_items = ShoppingList::items_count(&state.mm, list_id)
                .await
                .unwrap_or_default();

            templates::shopping::render_shopping_list_item_with_count(
                list_id, &item, num_items, false,
            )
            .into_response()
        }
        Err(err) => {
            error!("Failed to get shopping list item: {err}");
            broadcast_error(&state, user.id, "Failed to get shopping list item.").await;
            Error::Database.into_response()
        }
    }
}

/// Handles GET requests to edit a shopping list item.
pub async fn shopping_list_item_edit_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path((list_id, item_id)): Path<(Uuid, i64)>,
) -> impl IntoResponse {
    match ShoppingList::get_item(&state.mm, list_id, item_id, user.id).await {
        Ok(item) => {
            templates::shopping::shopping_list_item(list_id, Some(&item.label), Some(&item))
                .into_response()
        }
        Err(err) => {
            error!("Failed to get shopping list item: {err}");
            broadcast_error(&state, user.id, "Failed to get shopping list item.").await;
            Error::EntityNotFound {
                entity: "shopping_list_item",
            }
            .into_response()
        }
    }
}

/// Handles PUT requests to update a shopping list item.
pub async fn shopping_list_item_toggle_handler(
    OptionalAuth(user): OptionalAuth,
    State(state): State<AppState>,
    Path(item_id): Path<i64>,
) -> impl IntoResponse {
    if let Err(err) = ShoppingList::toggle_item_check(&state.mm, item_id).await {
        error!("Failed to toggle item check: {err}");
        if let Some(user) = user {
            broadcast_error(&state, user.id, "Failed to toggle item check.").await;
        }
        return Error::Database.into_response();
    }

    ().into_response()
}

/// Handles the GET request to render a recipe's ingredients for a shopping list.
pub async fn shopping_recipe_ingredients_handler(
    RequireAuth(user): RequireAuth,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let ingredients = match Recipe::ingredients(&state.mm, recipe_id).await {
        Ok(v) => v
            .into_iter()
            .map(|ing| AddShoppingIngredient {
                name: ing.name,
                quantity: ing.amounts.first().map(|s| {
                    let value = s.values().0;
                    let unit = s.unit().to_str();

                    if &unit == "whole" {
                        format!("{value}")
                    } else {
                        format!("{value} {unit}")
                    }
                }),
                notes: ing.modifier,
            })
            .collect::<Vec<_>>(),
        Err(err) => {
            error!("Failed to fetch ingredients for recipe '{recipe_id}': {err}");
            broadcast_error(&state, user.id, "Failed to fetch ingredients.").await;
            return Error::Database.into_response();
        }
    };

    let mut shopping_lists = match ShoppingList::get_all(&state.mm, user.id).await {
        Ok(lists) => lists,
        Err(err) => {
            error!(
                "Failed to fetch shopping lists for user '{}': {err}",
                user.id
            );
            broadcast_error(&state, user.id, "Failed to fetch shopping lists.").await;
            return Error::Database.into_response();
        }
    };
    shopping_lists.sort_by(|a, b| a.name.cmp(&b.name));

    templates::shopping::render_recipe_add_shopping_dialog_content(
        recipe_id,
        shopping_lists,
        ingredients,
    )
    .into_response()
}

/// Handles the POST request for adding ingredients to a shopping list for a recipe.
pub async fn shopping_recipe_ingredients_post_handler(
    RequireAuth(user): RequireAuth,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
    axum_extra::extract::Form(mut payload): axum_extra::extract::Form<RecipeIngredientsPayload>,
) -> impl IntoResponse {
    payload.clean();
    if payload.is_empty() {
        broadcast_warning(&state, user.id, "Payload is invalid.").await;
        return Error::InvalidPayload.into_response();
    }

    let list_id = match Uuid::parse_str(&payload.list) {
        Ok(id) => id,
        Err(_) => match ShoppingList::create(&state.mm, payload.list, user.id).await {
            Ok(list) => list,
            Err(err) => {
                error!("Failed to create new shopping list: {err}",);
                broadcast_error(&state, user.id, "Failed to create new shopping list.").await;
                return Error::Database.into_response();
            }
        },
    };

    let list = match ShoppingList::get(&state.mm, list_id, user.id).await {
        Ok(list) => list,
        Err(err) => {
            error!("Failed to fetch shopping list '{list_id}': {err}");
            broadcast_error(&state, user.id, "Failed to fetch shopping list.").await;
            return Error::Database.into_response();
        }
    };

    let items = izip!(
        &payload.ingredients,
        payload.quantities,
        payload.with_quantities,
        payload.notes
    )
    .map(
        |(ingredient, quantity, with_quantity, notes)| ShoppingListItemForCreate {
            ingredient: ingredient.clone(),
            quantity: if &with_quantity == "true" && &quantity != "-" {
                Some(quantity)
            } else {
                None
            },
            label: None,
            recipe_id: Some(recipe_id),
            notes: if &notes == "-" { None } else { Some(notes) },
        },
    )
    .collect::<Vec<_>>();

    if let Err(err) = list
        .add_items_for_recipe(&state.mm, items.as_slice(), recipe_id, user.id)
        .await
    {
        error!("Failed to add items for recipe '{}': {err}", recipe_id);
        broadcast_error(&state, user.id, "Failed to add items for recipe.").await;
        return Error::Database.into_response();
    }

    broadcast_success(&state, user.id, "Items added to shopping list.").await;
    ().into_response()
}
