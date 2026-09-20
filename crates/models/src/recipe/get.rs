use std::collections::BTreeSet;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use ingredient::{Ingredient, IngredientParser};
use itertools::Itertools;
use time::Duration;
use tracing::error;
use url::Url;
use uuid::Uuid;

use repository::{ModelManager, PgPooledConn, schema};

use crate::params::SearchParams;
use crate::recipe::RecipeSearch;
use crate::recipe::structs::media::{Video, VideoRecipe};
use crate::recipe::structs::nutrition::{Nutrition, NutritionDetails, NutritionPerServingDetails};
use crate::recipe::structs::recipe::RecipeDetails;
use crate::recipe::structs::section::{Item, SectionComponents, SectionItem};
use crate::recipe::structs::time::Times;
use crate::recipe::structs::tool::ToolRecipe;
use crate::{Error, Recipe, Result};

impl Recipe {
    /// Retrieves all recipes belonging to the user.
    pub async fn all(mm: &ModelManager, user_id: Uuid) -> Result<Vec<Self>> {
        let recipes = schema::recipes::table
            .filter(schema::recipes::user_id.eq(user_id))
            .select(Self::as_select())
            .order_by(schema::recipes::name.asc())
            .load(&mut mm.pool.get().await?)
            .await?;

        Ok(recipes)
    }

    /// Retrieves the total number of recipes that belong to a given user.
    pub async fn count(mm: &ModelManager, user_id: Uuid) -> Result<i64> {
        let count = schema::recipes::table
            .filter(schema::recipes::user_id.eq(user_id))
            .select(diesel::dsl::count(schema::recipes::id))
            .first(&mut mm.pool.get().await?)
            .await?;

        Ok(count)
    }

    /// Retrieves the details of a specific recipe for a given user.
    pub async fn get(mm: &ModelManager, user_id: Uuid, recipe_id: i64) -> Result<RecipeDetails> {
        let mut conn = mm.pool.get().await?;

        let (recipe, category, cuisine, keywords, times) = schema::recipes::table
            .inner_join(
                schema::users_recipes::table
                    .on(schema::users_recipes::recipe_id.eq(schema::recipes::id)),
            )
            .filter(
                schema::users_recipes::user_id
                    .eq(user_id)
                    .and(schema::users_recipes::recipe_id.eq(recipe_id)),
            )
            .inner_join(schema::categories_recipes::table.inner_join(schema::categories::table))
            .left_join(schema::cuisines_recipes::table.left_join(schema::cuisines::table))
            .left_join(schema::keywords_recipes::table.left_join(schema::keywords::table))
            .inner_join(schema::times::table.on(schema::times::recipe_id.eq(schema::recipes::id)))
            .select((
                Self::as_select(),
                schema::categories::name,
                schema::cuisines::name.nullable(),
                schema::keywords::name.nullable(),
                schema::times::all_columns,
            ))
            .first::<(Self, String, Option<String>, Option<String>, Times)>(&mut conn)
            .await
            .optional()?
            .ok_or_else(|| Error::EntityNotFound {
                entity: "recipe",
                id: recipe_id.to_string(),
            })?;

        fetch_recipe_details(&mut conn, recipe, category, cuisine, keywords, times).await
    }

    pub async fn get_many(
        mm: &ModelManager,
        user_id: Uuid,
        recipe_ids: &[i64],
    ) -> Result<Vec<RecipeDetails>> {
        let rows = schema::recipes::table
            .inner_join(
                schema::users_recipes::table
                    .on(schema::users_recipes::recipe_id.eq(schema::recipes::id)),
            )
            .filter(
                schema::users_recipes::user_id
                    .eq(user_id)
                    .and(schema::users_recipes::recipe_id.eq_any(recipe_ids)),
            )
            .inner_join(schema::categories_recipes::table.inner_join(schema::categories::table))
            .left_join(schema::cuisines_recipes::table.left_join(schema::cuisines::table))
            .left_join(schema::keywords_recipes::table.left_join(schema::keywords::table))
            .inner_join(schema::times::table.on(schema::times::recipe_id.eq(schema::recipes::id)))
            .select((
                Self::as_select(),
                schema::categories::name,
                schema::cuisines::name.nullable(),
                schema::keywords::name.nullable(),
                schema::times::all_columns,
            ))
            .distinct_on(schema::recipes::id)
            .load::<(Self, String, Option<String>, Option<String>, Times)>(
                &mut mm.pool.get().await?,
            )
            .await?;

        let futures =
            rows.into_iter()
                .map(|(recipe, category, cuisine, keywords, times)| async move {
                    let mut conn = mm.pool.get().await?;
                    fetch_recipe_details(&mut conn, recipe, category, cuisine, keywords, times)
                        .await
                });

        futures::future::try_join_all(futures).await
    }

    /// Gets the recipe only.
    pub async fn get_recipe_only(mm: &ModelManager, user_id: Uuid, recipe_id: i64) -> Result<Self> {
        let recipe = schema::recipes::table
            .filter(schema::recipes::user_id.eq(user_id))
            .filter(schema::recipes::id.eq(recipe_id))
            .select(Self::as_select())
            .first::<Self>(&mut mm.pool.get().await?)
            .await?;

        Ok(recipe)
    }

    /// Gets a page of recipes belonging to the user.
    pub async fn get_page(
        mm: &ModelManager,
        user_id: Uuid,
        search_params: &SearchParams,
    ) -> Result<Vec<RecipeDetails>> {
        let query = search_params.q.as_deref().unwrap_or_default();
        let page = search_params.page.unwrap_or(1).cast_signed();
        let is_favourites = search_params.is_favourites.unwrap_or(false);

        let recipe_search = RecipeSearch::new(query, page, is_favourites, user_id)?;
        let recipes = recipe_search.search(mm).await?;

        Ok(recipes)
    }

    /// Fetches all category names belonging to a user.
    pub async fn fetch_categories(mm: &ModelManager, user_id: Uuid) -> Result<Vec<String>> {
        let categories = schema::recipes::table
            .filter(schema::recipes::user_id.eq(user_id))
            .inner_join(
                schema::categories_recipes::table
                    .on(schema::categories_recipes::recipe_id.eq(schema::recipes::id)),
            )
            .inner_join(
                schema::categories::table
                    .on(schema::categories::id.eq(schema::categories_recipes::category_id)),
            )
            .select(schema::categories::name)
            .order(schema::categories::name.asc())
            .distinct()
            .load::<String>(&mut mm.pool.get().await?)
            .await?;

        Ok(categories)
    }

    /// Fetches all cuisine names belonging to a user.
    pub async fn fetch_cuisines(mm: &ModelManager, user_id: Uuid) -> Result<Vec<String>> {
        let cuisines = schema::recipes::table
            .filter(schema::recipes::user_id.eq(user_id))
            .inner_join(
                schema::cuisines_recipes::table
                    .on(schema::cuisines_recipes::recipe_id.eq(schema::recipes::id)),
            )
            .inner_join(
                schema::cuisines::table
                    .on(schema::cuisines::id.eq(schema::cuisines_recipes::cuisine_id)),
            )
            .select(schema::cuisines::name)
            .order(schema::cuisines::name.asc())
            .distinct()
            .load::<String>(&mut mm.pool.get().await?)
            .await?;

        Ok(cuisines)
    }

    /// Fetches all ingredient names belonging to a user.
    pub async fn fetch_ingredients(mm: &ModelManager, user_id: Uuid) -> Result<Vec<String>> {
        let ingredients = schema::recipes::table
            .filter(schema::recipes::user_id.eq(user_id))
            .inner_join(
                schema::ingredients_recipes::table
                    .on(schema::ingredients_recipes::recipe_id.eq(schema::recipes::id)),
            )
            .inner_join(
                schema::ingredients::table
                    .on(schema::ingredients::id.eq(schema::ingredients_recipes::ingredient_id)),
            )
            .select(schema::ingredients::name)
            .distinct()
            .load::<String>(&mut mm.pool.get().await?)
            .await?;

        Ok(ingredients
            .iter()
            .filter_map(|ing| {
                let ing = ing.trim();
                if ing.is_empty() {
                    None
                } else {
                    Some(ingredient::from_str(ing).name.to_lowercase())
                }
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect())
    }

    /// Fetches all keywords belonging to a user.
    pub async fn fetch_keywords(mm: &ModelManager, user_id: Uuid) -> Result<Vec<String>> {
        let keywords = schema::recipes::table
            .filter(schema::recipes::user_id.eq(user_id))
            .inner_join(
                schema::keywords_recipes::table
                    .on(schema::keywords_recipes::recipe_id.eq(schema::recipes::id)),
            )
            .inner_join(
                schema::keywords::table
                    .on(schema::keywords::id.eq(schema::keywords_recipes::keyword_id)),
            )
            .select(schema::keywords::name)
            .order(schema::keywords::name.asc())
            .distinct()
            .load::<String>(&mut mm.pool.get().await?)
            .await?;

        Ok(keywords)
    }

    /// Fetches all the tools belonging to a user.
    pub async fn fetch_tools(mm: &ModelManager, user_id: Uuid) -> Result<Vec<String>> {
        let tools = schema::recipes::table
            .filter(schema::recipes::user_id.eq(user_id))
            .inner_join(
                schema::tools_recipes::table
                    .on(schema::tools_recipes::recipe_id.eq(schema::recipes::id)),
            )
            .inner_join(
                schema::tools::table.on(schema::tools::id.eq(schema::tools_recipes::tool_id)),
            )
            .select(schema::tools::name)
            .order(schema::tools::name.asc())
            .distinct()
            .load::<String>(&mut mm.pool.get().await?)
            .await?;

        Ok(tools)
    }

    /// Fetches all the sources belonging to a user.
    pub async fn fetch_sources(mm: &ModelManager, user_id: Uuid) -> Result<Vec<String>> {
        let sources = schema::recipes::table
            .filter(schema::recipes::user_id.eq(user_id))
            .select(schema::recipes::source)
            .order(schema::recipes::source.asc())
            .distinct()
            .load::<String>(&mut mm.pool.get().await?)
            .await?;

        Ok(sources
            .into_iter()
            .map(|s| {
                Url::parse(&s).map_or_else(
                    |_| s.clone(),
                    |url| {
                        url.host_str()
                            .map_or_else(|| s.clone(), ToString::to_string)
                    },
                )
            })
            .collect())
    }

    /// Fetches the ingredients of a recipe.
    pub async fn ingredients(mm: &ModelManager, recipe_id: i64) -> Result<Vec<Ingredient>> {
        Ok(schema::ingredients_recipes::table
            .inner_join(
                schema::ingredients::table
                    .on(schema::ingredients::id.eq(schema::ingredients_recipes::ingredient_id)),
            )
            .filter(schema::ingredients_recipes::recipe_id.eq(recipe_id))
            .select(schema::ingredients::name)
            .load::<String>(&mut mm.pool.get().await?)
            .await?
            .into_iter()
            .map(|s| IngredientParser::new(false).from_str(&s))
            .collect())
    }
}

/// Fetches all the details of a recipe.
#[allow(clippy::too_many_lines)]
pub async fn fetch_recipe_details(
    conn: &mut PgPooledConn<'_>,
    mut recipe: Recipe,
    category: String,
    cuisine: Option<String>,
    keywords: Option<String>,
    times: Times,
) -> Result<RecipeDetails> {
    let recipe_id = recipe.id;

    if recipe.image == Some(Uuid::nil()) {
        recipe.image = None;
    }

    let additional_images = schema::additional_images_recipe::table
        .select(schema::additional_images_recipe::image)
        .filter(schema::additional_images_recipe::recipe_id.eq(recipe_id))
        .load::<Uuid>(conn)
        .await?
        .into_iter()
        .filter(|image| *image != Uuid::nil())
        .collect::<Vec<_>>();

    let ingredients = SectionComponents::try_from(
        schema::ingredients_recipes::table
            .filter(schema::ingredients_recipes::recipe_id.eq(recipe_id))
            .inner_join(schema::ingredients::table)
            .inner_join(schema::sections::table)
            .order((
                schema::ingredients_recipes::section_order,
                schema::ingredients_recipes::item_order,
            ))
            .select((
                schema::ingredients::name,
                schema::sections::name,
                schema::ingredients_recipes::section_order,
                schema::ingredients_recipes::item_order,
            ))
            .load::<(String, String, i16, i16)>(conn)
            .await?
            .into_iter()
            .sorted_by_key(|(_, _, section_order, item_order)| (*section_order, *item_order))
            .fold(
                Vec::<SectionItem>::new(),
                |mut acc, (ingredient, section, _, _)| {
                    let item = Item::new(ingredient);
                    if let Some(existing) = acc.iter_mut().find(|s| s.title == section) {
                        existing.items.push(item);
                    } else {
                        acc.push(SectionItem {
                            title: section,
                            items: vec![item],
                        });
                    }
                    acc
                },
            ),
    )
    .inspect_err(|err| {
        error!(?err, "Failed to load ingredients");
    })
    .unwrap_or_default();

    let instructions = SectionComponents::try_from(
        schema::instructions_recipes::table
            .filter(schema::instructions_recipes::recipe_id.eq(recipe_id))
            .inner_join(schema::instructions::table)
            .inner_join(schema::sections::table)
            .order((
                schema::instructions_recipes::section_order,
                schema::instructions_recipes::item_order,
            ))
            .select((
                schema::instructions::id,
                schema::instructions::name,
                schema::sections::name,
                schema::instructions_recipes::section_order,
                schema::instructions_recipes::item_order,
                schema::instructions::duration_seconds,
            ))
            .load::<(i64, String, String, i16, i16, Option<i32>)>(conn)
            .await?
            .into_iter()
            .sorted_by_key(|(_, _, _, section_order, item_order, _)| (*section_order, *item_order))
            .fold(
                Vec::<SectionItem>::new(),
                |mut acc, (id, instruction, section, _, _, duration_seconds)| {
                    let item = Item::new(instruction)
                        .with_id(id)
                        .with_duration(duration_seconds.unwrap_or_default());

                    if let Some(existing) = acc.iter_mut().find(|s| s.title == section) {
                        existing.items.push(item);
                    } else {
                        acc.push(SectionItem {
                            title: section,
                            items: vec![item],
                        });
                    }
                    acc
                },
            ),
    )
    .inspect_err(|err| {
        error!(?err, "Failed to load instructions");
    })
    .unwrap_or_default();

    let keywords = if keywords.is_some() {
        schema::keywords_recipes::table
            .filter(schema::keywords_recipes::recipe_id.eq(recipe_id))
            .inner_join(schema::keywords::table)
            .select(schema::keywords::name)
            .load::<String>(conn)
            .await?
    } else {
        Vec::new()
    };

    let tools = schema::tools_recipes::table
        .inner_join(schema::tools::table)
        .filter(schema::tools_recipes::recipe_id.eq(recipe_id))
        .select((
            schema::tools::name,
            schema::tools_recipes::quantity,
            schema::tools_recipes::tool_order,
        ))
        .load::<(String, i16, i16)>(conn)
        .await?
        .into_iter()
        .map(|(name, quantity, tool_order)| ToolRecipe {
            name,
            quantity,
            tool_order: tool_order + 1,
        })
        .collect::<Vec<_>>();

    let videos = schema::videos_recipes::table
        .filter(schema::videos_recipes::recipe_id.eq(recipe_id))
        .select(VideoRecipe::as_select())
        .load::<VideoRecipe>(conn)
        .await?
        .into_iter()
        .map(|v| Video {
            video: v.video,
            duration: v.duration.map(|d| {
                let ms = Duration::milliseconds(d.microseconds / 1000);
                let days = Duration::days(i64::from(d.days));
                ms + days
            }),
            content_url: v.content_url,
            embed_url: v.embed_url,
            created_at: v.created_at,
        })
        .collect::<Vec<_>>();

    let nutrition_per_100g: Option<Nutrition> = schema::nutrition_per_100g::table
        .inner_join(
            schema::nutrition::table
                .on(schema::nutrition_per_100g::nutrition_id.eq(schema::nutrition::id)),
        )
        .filter(schema::nutrition_per_100g::recipe_id.eq(recipe_id))
        .select(schema::nutrition::all_columns)
        .first::<Nutrition>(conn)
        .await
        .optional()?
        .and_then(Nutrition::sanitize);

    let nutrition_per_serving: Option<(Nutrition, String)> = schema::nutrition_per_serving::table
        .inner_join(
            schema::nutrition::table
                .on(schema::nutrition_per_serving::nutrition_id.eq(schema::nutrition::id)),
        )
        .filter(schema::nutrition_per_serving::recipe_id.eq(recipe_id))
        .select((
            schema::nutrition::all_columns,
            schema::nutrition_per_serving::serving_size,
        ))
        .first::<(Nutrition, String)>(conn)
        .await
        .optional()?
        .and_then(|(n, serving_size)| n.sanitize().map(|nutrition| (nutrition, serving_size)));

    Ok(RecipeDetails {
        recipe,
        additional_images,
        category,
        cuisine,
        ingredients,
        instructions,
        keywords,
        nutrition: NutritionDetails {
            per_100g: nutrition_per_100g,
            per_serving: nutrition_per_serving.map(|(nutrition, serving_size)| {
                NutritionPerServingDetails {
                    nutrition,
                    serving_size,
                }
            }),
        },
        times,
        tools,
        videos,
    })
}
