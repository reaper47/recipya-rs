use std::{collections::HashMap, ops::Not, path::PathBuf, sync::Arc};

use axum::{
    Form,
    extract::{Path, State},
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
};
use axum_htmx::HX_REDIRECT;
use config::States;
use indexmap::IndexMap;
use reqwest::StatusCode;
use tracing::error;
use uuid::Uuid;

use app::state::AppState;
use models::{
    Error::EntityNotFound,
    data::Data,
    recipe::structs::{
        media::VideoForCreate,
        nutrition::{
            NutritionDetailsForCreate, NutritionForCreate, NutritionPerServingDetailsForCreate,
        },
        section::{Item, SectionComponents, SectionItem},
        tool::ToolForCreate,
    },
};
use models::{Recipe, recipe::structs::recipe::RecipeForCreate};
use support::strings::calc_seconds_from_parts;
use templates::recipes::RecipeDiff;

use crate::{
    Error, Result,
    handlers::{
        get_settings,
        helpers::is_hx_request,
        message::{broadcast_error, broadcast_warning},
        recipes::common::schema_to_recipe_for_create,
    },
    middleware::mw_auth::RequireAuth,
};

pub async fn recrape_recipe_handler(
    header_map: HeaderMap,
    RequireAuth(user): RequireAuth,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let user_id = user.id;

    let (old_recipe_c, is_nutrition_calculated_by_source) =
        match Recipe::get(&state.mm, user_id, recipe_id).await {
            Ok(r) => {
                let is_precalculated = r.nutrition.is_precalculated();
                (RecipeForCreate::from(r), is_precalculated)
            }
            Err(err) => {
                error!(?recipe_id, user = ?user.id, ?err, "Error fetching recipe");
                broadcast_error(&state, user_id, "Recipe not found.").await;
                return Err(Error::Model(EntityNotFound {
                    id: recipe_id.to_string(),
                    entity: "recipe",
                }));
            }
        };

    let new_recipe_c = match state.scraper.scrape(old_recipe_c.source.as_str()).await {
        Ok(r) => schema_to_recipe_for_create(&state, r).await,
        Err(err) => {
            error!(?recipe_id, user = ?user.id, ?err, "Error scraping recipe");
            broadcast_error(
                &state,
                user.id,
                "Error scraping recipe or recipe source is not a URL.",
            )
            .await;
            return Err(Error::FailFetch);
        }
    };

    let changes = new_recipe_c.diff(&old_recipe_c, is_nutrition_calculated_by_source);

    if changes.is_empty() {
        broadcast_warning(&state, user.id, "Recipe has not changed.").await;
        return Ok(().into_response());
    }

    let is_hx_request = is_hx_request(&header_map);

    let mut res = templates::recipes::rescrape_recipe_diff(
        &Data {
            is_admin: user.is_admin,
            is_authenticated: true,
            states: States {
                autologin: state.config.read().await.states.autologin,
                ..Default::default()
            },
            is_hx_request,
            recipes: vec![],
            ..Default::default()
        },
        &state.data_dir,
        &state.fs_support,
        &get_settings(&state, user.id).await?,
        recipe_id,
        RecipeDiff {
            old: old_recipe_c,
            new: new_recipe_c,
            changes,
        },
    )
    .into_response();

    if is_hx_request {
        res.headers_mut().insert(
            axum_htmx::HX_PUSH_URL,
            format!("/recipes/{recipe_id}/rescrape").parse().unwrap(),
        );
        res.headers_mut().insert(
            axum_htmx::HX_RESWAP,
            "innerHTML transition:true".parse().unwrap(),
        );
    }

    Ok(res)
}

/// Handles saving the rescraped recipe.
#[allow(clippy::too_many_lines)]
#[allow(clippy::cast_possible_truncation)]
pub async fn recrape_recipe_put_handler(
    RequireAuth(user): RequireAuth,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
    Form(form): Form<Vec<(String, String)>>,
) -> Result<impl IntoResponse> {
    let mut recipe = match Recipe::get(&state.mm, user.id, recipe_id).await {
        Ok(r) => RecipeForCreate::from(r),
        Err(err) => {
            error!(?recipe_id, user = ?user.id, ?err, "Error fetching recipe");
            broadcast_error(&state, user.id, "Recipe not found.").await;
            return Err(Error::Model(EntityNotFound {
                id: recipe_id.to_string(),
                entity: "recipe",
            }));
        }
    };

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut ingredients: IndexMap<String, Vec<String>> = IndexMap::new();
    let mut instructions: IndexMap<String, Vec<String>> = IndexMap::new();

    for (name, value) in form {
        match name.as_ref() {
            "category-new" => {
                recipe.category = value.is_empty().not().then_some(value);
            }
            "cook-new" => {
                if let Some(t) = recipe.times.as_mut() {
                    t.cook_seconds = calc_seconds_from_parts(&value);
                }
            }
            "cuisine-new" => {
                recipe.cuisine = value.is_empty().not().then_some(value);
            }
            "description-new" => {
                recipe.description = value.is_empty().not().then_some(value);
            }
            "keywords-old" => {
                map.entry("keywords-source".into())
                    .or_insert_with(|| vec!["old".into()]);
            }
            "keywords-new" => {
                map.entry("keywords-source".into())
                    .or_insert_with(|| vec!["new".into()]);
                map.entry("keywords".into())
                    .and_modify(|v| v.push(value.clone()))
                    .or_insert_with(|| vec![value]);
            }
            "media-source" if value == "new" => {
                recipe.images.clear();
                recipe.videos.clear();
            }
            "media-new-image" => {
                if let Ok(file_name) = Uuid::parse_str(
                    &PathBuf::from(value)
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy(),
                ) {
                    let fs_support = Arc::clone(&state.fs_support);
                    let thumbnails_dir = state.data_dir.images.thumbnails.clone();
                    let path = state.data_dir.images.root.join(format!("{file_name}.webp"));
                    tokio::spawn(async move {
                        fs_support.generate_thumbnail(&path, file_name, &thumbnails_dir);
                    });
                    recipe.images.push(file_name);
                }
            }
            "media-new-video" => {
                if let Ok(video) = Uuid::parse_str(
                    &PathBuf::from(value.clone())
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy(),
                ) {
                    recipe.videos.push(VideoForCreate {
                        video,
                        duration: None,
                        content_url: Some(value.clone()),
                        embed_url: Some(value),
                    });
                }
            }
            "ingredients-source" => {
                map.entry("ingredients-source".into())
                    .or_insert_with(|| vec![value]);
            }
            name if name.starts_with("ingredients-new") => {
                if map
                    .get("ingredients-source")
                    .and_then(|v| v.first())
                    .map(String::as_str)
                    == Some("old")
                {
                    continue;
                }

                map.entry("ingredients-source".into())
                    .or_insert_with(|| vec!["new".into()]);

                let section = name.split_once("<>").map_or("", |(_, s)| s);
                ingredients
                    .entry(section.to_string())
                    .or_default()
                    .push(value);
            }
            "instructions-source" => {
                map.entry("instructions-source".into())
                    .or_insert_with(|| vec![value]);
            }
            name if name.starts_with("instructions-new") => {
                if map
                    .get("instructions-source")
                    .and_then(|v| v.first())
                    .map(String::as_str)
                    == Some("old")
                {
                    continue;
                }

                let section = name.split_once("<>").map_or("", |(_, s)| s);
                instructions
                    .entry(section.to_string())
                    .or_default()
                    .push(value);
            }
            "notes-new" => {
                recipe.notes = value.is_empty().not().then_some(value);
            }
            "nutrition-source" => {
                map.entry("nutrition-source".into())
                    .or_insert_with(|| vec![value]);
            }
            name if name.starts_with("nutrition-new-") => {
                map.entry(name.to_string()).or_insert_with(|| vec![value]);
            }
            "prep-new" => {
                if let Some(t) = recipe.times.as_mut() {
                    t.prep_seconds = calc_seconds_from_parts(&value);
                }
            }
            "rating-source" => {
                map.entry("rating-source".into())
                    .or_insert_with(|| vec![value]);
            }
            "rating-new" => {
                map.entry("rating".into()).or_insert_with(|| vec![value]);
            }
            "title-new" => {
                recipe.name = value;
            }
            "tools-old" => {
                map.entry("tools-source".into())
                    .or_insert_with(|| vec!["old".into()]);
            }
            "tools-new" => {
                map.entry("tools-source".into())
                    .or_insert_with(|| vec!["new".into()]);
                map.entry("tools".into())
                    .and_modify(|v| v.push(value.clone()))
                    .or_insert_with(|| vec![value]);
            }
            "yield-new" => {
                recipe.r#yield = value.parse().ok();
            }
            _ => {}
        }
    }

    if let Some(sources) = map.get("keywords-source")
        && let Some(source) = sources.first()
        && source == "new"
    {
        recipe.keywords = map.remove("keywords").unwrap_or_default();
    }

    if map
        .get("ingredients-source")
        .and_then(|v| v.first())
        .map(String::as_str)
        == Some("new")
    {
        recipe.ingredients = if ingredients.len() == 1 && ingredients.contains_key("") {
            SectionComponents::Flat(
                ingredients[""]
                    .iter()
                    .filter(|s| !s.trim().is_empty())
                    .map(Item::new)
                    .collect(),
            )
        } else {
            SectionComponents::Grouped(
                ingredients
                    .iter()
                    .filter(|(title, _)| !title.is_empty())
                    .map(|(title, items)| SectionItem {
                        title: title.clone(),
                        items: items
                            .iter()
                            .filter(|s| !s.trim().is_empty())
                            .map(Item::new)
                            .collect(),
                    })
                    .collect(),
            )
        };
    }

    if map
        .get("instructions-source")
        .and_then(|v| v.first())
        .map(String::as_str)
        == Some("new")
    {
        recipe.instructions = if instructions.len() == 1 && instructions.contains_key("") {
            SectionComponents::Flat(
                instructions[""]
                    .iter()
                    .filter(|s| !s.trim().is_empty())
                    .map(Item::new)
                    .collect(),
            )
        } else {
            SectionComponents::Grouped(
                instructions
                    .iter()
                    .filter(|(title, _)| !title.is_empty())
                    .map(|(title, items)| SectionItem {
                        title: title.clone(),
                        items: items
                            .iter()
                            .filter(|s| !s.trim().is_empty())
                            .map(Item::new)
                            .collect(),
                    })
                    .collect(),
            )
        };
    }

    if let Some(sources) = map.get("tools-source")
        && let Some(source) = sources.first()
        && source == "new"
    {
        recipe.tools = map
            .remove("tools")
            .unwrap_or_default()
            .into_iter()
            .map(|t| ToolForCreate {
                name: t,
                quantity: 0,
            })
            .collect();
    }

    if map
        .get("rating-source")
        .and_then(|v| v.first())
        .map(String::as_str)
        == Some("new")
    {
        recipe.rating = map
            .get("rating")
            .and_then(|v| v.first())
            .and_then(|s| s.parse::<i16>().ok())
            .filter(|&r| (1..=5).contains(&r));
    }

    if map
        .get("nutrition-source")
        .and_then(|v| v.first())
        .map(String::as_str)
        == Some("new")
    {
        let get = |key: &str| -> Option<f64> {
            map.get(key).and_then(|v| v.first()).and_then(|s| {
                s.chars()
                    .take_while(|c| c.is_ascii_digit() || *c == '.')
                    .collect::<String>()
                    .parse::<f64>()
                    .ok()
            })
        };

        let per_100g = NutritionForCreate {
            calories_kcal: get("nutrition-new-calories-per-100g").map(|v| v as i16),
            total_carbohydrates: get("nutrition-new-total-carbohydrates-per-100g"),
            sugars_g: get("nutrition-new-sugars-per-100g"),
            protein_g: get("nutrition-new-protein-per-100g"),
            total_fat_g: get("nutrition-new-total-fat-per-100g"),
            saturated_fat_g: get("nutrition-new-saturated-fat-per-100g"),
            unsaturated_fat_g: get("nutrition-new-unsaturated-fat-per-100g"),
            trans_fat_g: get("nutrition-new-trans-fat-per-100g"),
            cholesterol_mg: get("nutrition-new-cholesterol-per-100g"),
            sodium_mg: get("nutrition-new-sodium-per-100g"),
            fiber_g: get("nutrition-new-fiber-per-100g"),
        };

        let per_serving = NutritionForCreate {
            calories_kcal: get("nutrition-new-calories-per-serving").map(|v| v as i16),
            total_carbohydrates: get("nutrition-new-total-carbohydrates-per-serving"),
            sugars_g: get("nutrition-new-sugars-per-serving"),
            protein_g: get("nutrition-new-protein-per-serving"),
            total_fat_g: get("nutrition-new-total-fat-per-serving"),
            saturated_fat_g: get("nutrition-new-saturated-fat-per-serving"),
            unsaturated_fat_g: get("nutrition-new-unsaturated-fat-per-serving"),
            trans_fat_g: get("nutrition-new-trans-fat-per-serving"),
            cholesterol_mg: get("nutrition-new-cholesterol-per-serving"),
            sodium_mg: get("nutrition-new-sodium-per-serving"),
            fiber_g: get("nutrition-new-fiber-per-serving"),
        };

        let serving_size = map
            .remove("serving-size")
            .and_then(|v| v.into_iter().next())
            .unwrap_or_default();

        recipe.nutrition = NutritionDetailsForCreate {
            per_100g: Some(per_100g),
            per_serving: Some(NutritionPerServingDetailsForCreate {
                nutrition: per_serving,
                serving_size,
            }),
        };
    }

    match Recipe::update(&state.mm, user.id, recipe_id, &mut recipe).await {
        Ok(()) => {
            state.remove_cached_recipe((user.id, recipe_id)).await;
        }
        Err(err) => {
            error!(?recipe_id, user = ?user.id, ?err, "Failed to update recipe user");
            broadcast_error(&state, user.id, "Failed to add recipe to collection.").await;
            return Err(Error::Database);
        }
    }

    let mut res = (StatusCode::SEE_OTHER, "").into_response();
    if let Ok(value) = HeaderValue::from_str(&format!("/recipes/{recipe_id}")) {
        res.headers_mut().insert(HX_REDIRECT, value);
    }
    Ok(res)
}
