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

        let futures = rows
            .into_iter()
            .map(|(recipe, category, cuisine, keywords, times)| {
                let mm = mm.clone();
                async move {
                    let mut conn = mm.pool.get().await?;
                    fetch_recipe_details(&mut conn, recipe, category, cuisine, keywords, times)
                        .await
                }
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
        error!("Failed to load ingredients: {err}");
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
        error!("Failed to load instructions: {err}");
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

#[cfg(test)]
mod tests {
    use test_db::default_config;
    use test_utils::{build_server_anonymous, create_app_state};

    use super::*;
    use crate::recipe::structs::test_utils::a_complete_recipe_for_create;
    use crate::recipe::structs::types::Source;
    use crate::settings::UserSettingDetails;
    use crate::user::User;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_all {

        use super::*;

        #[tokio::test]
        async fn test_all_ok() -> Result<()> {
            let (_, state) = build_server_anonymous(default_config()).await?;
            let all_users = User::all(&state.mm).await?;
            let user1_id = all_users[0].id;
            let user2_id = all_users[1].id;
            let settings1 = UserSettingDetails::get(&state.mm, user1_id).await?;
            let settings2 = UserSettingDetails::get(&state.mm, user2_id).await?;
            for i in 0..5 {
                let (mut recipe, _) = a_complete_recipe_for_create();
                recipe.name.push_str(i.to_string().as_str());
                let _ = Recipe::create(
                    &state.mm,
                    if i % 2 == 0 { user1_id } else { user2_id },
                    &recipe,
                    if i % 2 == 0 { &settings1 } else { &settings2 },
                )
                .await?;
            }

            let recipes = Recipe::all(&state.mm, user1_id).await?;
            assert_eq!(recipes.len(), 3);
            Ok(())
        }
    }

    mod tests_count {
        use super::*;

        #[tokio::test]
        async fn test_count_ok() -> Result<()> {
            let (_, state) = build_server_anonymous(default_config()).await?;
            let all_users = User::all(&state.mm).await?;
            let user = all_users[0].clone();
            let user2 = all_users[1].clone();
            let settings1 = UserSettingDetails::get(&state.mm, user.id).await?;
            let settings2 = UserSettingDetails::get(&state.mm, user2.id).await?;
            for i in 0..5 {
                let (mut recipe, _) = a_complete_recipe_for_create();
                recipe.name.push_str(i.to_string().as_str());
                let _ = Recipe::create(&state.mm, user.id, &recipe, &settings1).await?;
            }
            for i in 0..10 {
                let (mut recipe, _) = a_complete_recipe_for_create();
                recipe.name.push_str((i + 1002).to_string().as_str());
                let _ = Recipe::create(&state.mm, user2.id, &recipe, &settings2).await?;
            }

            let count_user1 = Recipe::count(&state.mm, user.id).await?;
            let count_user2 = Recipe::count(&state.mm, user2.id).await?;

            assert_eq!(count_user1, 5);
            assert_eq!(count_user2, 10);
            Ok(())
        }
    }

    mod tests_get_page {
        use super::*;

        #[tokio::test]
        async fn test_get_page_no_recipes_ok() -> Result<()> {
            let state = create_app_state(default_config()).await;

            let recipes = Recipe::get_page(
                &state.mm,
                Uuid::new_v4(),
                &SearchParams {
                    page: Some(1),
                    ..Default::default()
                },
            )
            .await?;

            pretty_assertions::assert_eq!(recipes, Vec::new());
            Ok(())
        }

        #[tokio::test]
        async fn test_get_page_first_page_ok() -> Result<()> {
            let (_, state) = build_server_anonymous(default_config()).await?;
            let user = User::all(&state.mm).await?[0].clone();
            let settings = UserSettingDetails::get(&state.mm, user.id).await?;
            let mut expected = Vec::with_capacity(15);
            for i in 0..15 {
                let (mut recipe, _) = a_complete_recipe_for_create();
                recipe.name.push_str(i.to_string().as_str());
                let _ = Recipe::create(&state.mm, user.id, &recipe, &settings).await?;
                expected.push(recipe.name);
            }

            let recipes = Recipe::get_page(
                &state.mm,
                user.id,
                &SearchParams {
                    page: Some(1),
                    ..Default::default()
                },
            )
            .await?;

            let got = recipes
                .into_iter()
                .map(|r| r.recipe.name)
                .collect::<Vec<_>>();
            pretty_assertions::assert_eq!(got, expected);
            Ok(())
        }
    }

    #[tokio::test]
    #[allow(clippy::too_many_lines)]
    async fn test_get_many_ok() -> Result<()> {
        let (_, state) = build_server_anonymous(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (recipe1, images1) = a_complete_recipe_for_create();
        let (mut recipe2, images2) = a_complete_recipe_for_create();
        recipe2.name = "Hello".to_string();
        let id1 = Recipe::create(&state.mm, user_id, &recipe1, &settings).await?;
        let id2 = Recipe::create(&state.mm, user_id, &recipe2, &settings).await?;

        let mut got = Recipe::get_many(&state.mm, user_id, &[id1, id2]).await?;

        for r in &mut got {
            match &mut r.instructions {
                SectionComponents::Grouped(section_items) => section_items
                    .iter_mut()
                    .for_each(|i| i.items.iter_mut().for_each(|i| i.id = None)),
                SectionComponents::Flat(items) => items.iter_mut().for_each(|i| i.id = None),
            }
        }
        let want_recipe1 = RecipeDetails {
            recipe: Recipe {
                id: id1,
                name: "Best Chinese Kale".into(),
                description: Some("This is the most delicious recipe!".into()),
                r#yield: 4,
                language: "eng".into(),
                measurement_system_id: 2,
                notes: Some("# My Recipe Notes\n\nThis dish should be served medium-cold".into()),
                source: Source::Url(
                    "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/".into(),
                ),
                rating: Some(4),
                created_at: got[0].recipe.created_at,
                updated_at: got[0].recipe.updated_at,
                user_id,
                image: Some(images1.main),
                ..Default::default()
            },
            additional_images: vec![images1.additional],
            category: "dinner".into(),
            cuisine: Some("thai".into()),
            ingredients: SectionComponents::Grouped(vec![
                SectionItem {
                    title: "Sauce".into(),
                    items: vec![
                        Item::new("1 cup blue spinach"),
                        Item::new("1/2 tbsp cinnamon"),
                    ],
                },
                SectionItem {
                    title: "Main".into(),
                    items: vec![
                        Item::new("4 pounds top quality chicken filet"),
                        Item::new("1/8 cup lemon juice"),
                    ],
                },
            ]),
            instructions: SectionComponents::Grouped(vec![
                SectionItem {
                    title: "Sauce".into(),
                    items: vec![Item::new("Mix all these ingredients")],
                },
                SectionItem {
                    title: "Chicken".into(),
                    items: vec![
                        Item::new("Turn the oven at 300 F"),
                        Item::new("Soak the chicken in the lemon juice"),
                        Item::new("Bake for 35 minutes").with_duration(2100),
                    ],
                },
            ]),
            keywords: vec!["tofu".into(), "vegetarian".into()],
            nutrition: NutritionDetails {
                per_100g: Some(Nutrition {
                    id: got[0].nutrition.clone().per_100g.map_or(1, |n| n.id),
                    is_precalculated_by_source: true,
                    calories_kcal: Some(300),
                    total_carbohydrates: Some(55.0),
                    sugars_g: Some(43.0),
                    protein_g: Some(7.0),
                    total_fat_g: Some(6.0),
                    saturated_fat_g: Some(1.0),
                    unsaturated_fat_g: Some(2.0),
                    cholesterol_mg: Some(5.0),
                    sodium_mg: Some(12.0),
                    fiber_g: Some(10.0),
                    trans_fat_g: Some(3.0),
                }),
                per_serving: Some(NutritionPerServingDetails {
                    nutrition: Nutrition {
                        id: got[0]
                            .nutrition
                            .clone()
                            .per_serving
                            .map_or(1, |n| n.nutrition.id),
                        is_precalculated_by_source: true,
                        calories_kcal: Some(240),
                        total_carbohydrates: Some(30.0),
                        sugars_g: Some(24.0),
                        protein_g: Some(4.0),
                        total_fat_g: Some(6.0),
                        saturated_fat_g: Some(2.0),
                        unsaturated_fat_g: Some(7.0),
                        cholesterol_mg: Some(12.0),
                        sodium_mg: Some(100.0),
                        fiber_g: Some(18.0),
                        trans_fat_g: Some(2.0),
                    },
                    serving_size: "2 buns".into(),
                }),
            },
            times: Times {
                id: got[0].times.id,
                recipe_id: id1,
                prep_seconds: 120,
                cook_seconds: 3600,
                total_seconds: 3720,
            },
            tools: vec![
                ToolRecipe {
                    name: "wok".into(),
                    quantity: 1,
                    tool_order: 1,
                },
                ToolRecipe {
                    name: "frying pan".into(),
                    quantity: 1,
                    tool_order: 2,
                },
            ],
            videos: vec![Video {
                video: images1.video,
                duration: Some(Duration::seconds(420)),
                content_url: Some("https://example.com/best-food.mp4".into()),
                embed_url: Some("https://example.com/embed/j43yfe3.mp4".into()),
                created_at: got[0].videos[0].created_at,
            }],
        };
        let want_recipe2 = RecipeDetails {
            recipe: Recipe {
                id: id2,
                name: "Hello".into(),
                description: want_recipe1.recipe.description.clone(),
                r#yield: want_recipe1.recipe.r#yield,
                language: want_recipe1.recipe.language.clone(),
                measurement_system_id: 2,
                notes: want_recipe1.recipe.notes.clone(),
                source: want_recipe1.recipe.source.clone(),
                rating: want_recipe1.recipe.rating,
                created_at: got[1].recipe.created_at,
                updated_at: got[1].recipe.updated_at,
                user_id,
                image: Some(images2.main),
                ..Default::default()
            },
            additional_images: vec![images2.additional],
            category: want_recipe1.category.clone(),
            cuisine: want_recipe1.cuisine.clone(),
            ingredients: want_recipe1.ingredients.clone(),
            instructions: want_recipe1.instructions.clone(),
            keywords: want_recipe1.keywords.clone(),
            nutrition: NutritionDetails {
                per_100g: Some(Nutrition {
                    id: got[1].nutrition.clone().per_100g.map_or(1, |n| n.id),
                    is_precalculated_by_source: true,
                    calories_kcal: Some(300),
                    total_carbohydrates: Some(55.0),
                    sugars_g: Some(43.0),
                    protein_g: Some(7.0),
                    total_fat_g: Some(6.0),
                    saturated_fat_g: Some(1.0),
                    unsaturated_fat_g: Some(2.0),
                    cholesterol_mg: Some(5.0),
                    sodium_mg: Some(12.0),
                    fiber_g: Some(10.0),
                    trans_fat_g: Some(3.0),
                }),
                per_serving: Some(NutritionPerServingDetails {
                    nutrition: Nutrition {
                        id: got[1]
                            .nutrition
                            .clone()
                            .per_serving
                            .map_or(1, |n| n.nutrition.id),
                        is_precalculated_by_source: true,
                        calories_kcal: Some(240),
                        total_carbohydrates: Some(30.0),
                        sugars_g: Some(24.0),
                        protein_g: Some(4.0),
                        total_fat_g: Some(6.0),
                        saturated_fat_g: Some(2.0),
                        unsaturated_fat_g: Some(7.0),
                        cholesterol_mg: Some(12.0),
                        sodium_mg: Some(100.0),
                        fiber_g: Some(18.0),
                        trans_fat_g: Some(2.0),
                    },
                    serving_size: "2 buns".into(),
                }),
            },
            times: Times {
                id: got[1].times.id,
                recipe_id: id2,
                prep_seconds: 120,
                cook_seconds: 3600,
                total_seconds: 3720,
            },
            tools: want_recipe1.tools.clone(),
            videos: vec![Video {
                video: images2.video,
                duration: Some(Duration::seconds(420)),
                content_url: Some("https://example.com/best-food.mp4".into()),
                embed_url: Some("https://example.com/embed/j43yfe3.mp4".into()),
                created_at: got[1].videos[0].created_at,
            }],
        };
        pretty_assertions::assert_eq!(got, vec![want_recipe1, want_recipe2]);
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_categories_ok() -> Result<()> {
        let (_, state) = build_server_anonymous(default_config()).await?;
        let user = User::all(&state.mm).await?[0].clone();
        let settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let category1 = String::from("late snack");
        let category2 = String::from("dinner");
        let (mut recipe, _) = a_complete_recipe_for_create();
        recipe.category = Some(category1.clone());
        let _ = Recipe::create(&state.mm, user.id, &recipe, &settings).await?;
        let (mut recipe, _) = a_complete_recipe_for_create();
        recipe.name = "Hello".to_string();
        recipe.category = Some(category2.clone());
        let _ = Recipe::create(&state.mm, user.id, &recipe, &settings).await?;

        let categories = Recipe::fetch_categories(&state.mm, user.id).await?;

        pretty_assertions::assert_eq!(categories, vec![category2, category1]);
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_cuisines_ok() -> Result<()> {
        let (_, state) = build_server_anonymous(default_config()).await?;
        let user = User::all(&state.mm).await?[0].clone();
        let settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let cuisine1 = String::from("italian");
        let cuisine2 = String::from("mexican");
        let (mut recipe, _) = a_complete_recipe_for_create();
        recipe.cuisine = Some(cuisine1.clone());
        let _ = Recipe::create(&state.mm, user.id, &recipe, &settings).await?;
        let (mut recipe, _) = a_complete_recipe_for_create();
        recipe.name = "Hello".to_string();
        recipe.cuisine = Some(cuisine2.clone());
        let _ = Recipe::create(&state.mm, user.id, &recipe, &settings).await?;

        let got = Recipe::fetch_cuisines(&state.mm, user.id).await?;

        pretty_assertions::assert_eq!(got, vec![cuisine1, cuisine2]);
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_ingredients_ok() -> Result<()> {
        let (_, state) = build_server_anonymous(default_config()).await?;
        let user = User::all(&state.mm).await?[0].clone();
        let settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let _ = Recipe::create(&state.mm, user.id, &recipe, &settings).await?;

        let ingredients = Recipe::fetch_ingredients(&state.mm, user.id).await?;

        pretty_assertions::assert_eq!(
            ingredients,
            vec![
                "blue spinach".to_string(),
                "cinnamon".to_string(),
                "lemon juice".to_string(),
                "top quality chicken filet".to_string(),
            ]
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_keywords_ok() -> Result<()> {
        let (_, state) = build_server_anonymous(default_config()).await?;
        let user = User::all(&state.mm).await?[0].clone();
        let settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let _ = Recipe::create(&state.mm, user.id, &recipe, &settings).await?;

        let keywords = Recipe::fetch_keywords(&state.mm, user.id).await?;

        pretty_assertions::assert_eq!(
            keywords,
            vec!["tofu".to_string(), "vegetarian".to_string(),]
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_tools_ok() -> Result<()> {
        let (_, state) = build_server_anonymous(default_config()).await?;
        let user = User::all(&state.mm).await?[0].clone();
        let settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let _ = Recipe::create(&state.mm, user.id, &recipe, &settings).await?;

        let tools = Recipe::fetch_tools(&state.mm, user.id).await?;

        pretty_assertions::assert_eq!(tools, vec!["frying pan".to_string(), "wok".to_string(),]);
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_sources_ok() -> Result<()> {
        let (_, state) = build_server_anonymous(default_config()).await?;
        let user = User::all(&state.mm).await?[0].clone();
        let settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let _ = Recipe::create(&state.mm, user.id, &recipe, &settings).await?;

        let sources = Recipe::fetch_sources(&state.mm, user.id).await?;

        pretty_assertions::assert_eq!(sources, vec!["www.allrecipes.com".to_string(),]);
        Ok(())
    }

    #[tokio::test]
    async fn test_ingredients_ok() -> Result<()> {
        let (_, state) = build_server_anonymous(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let (recipe, _) = a_complete_recipe_for_create();
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

        let got = Recipe::ingredients(&state.mm, recipe_id).await?;

        pretty_assertions::assert_eq!(
            got,
            vec![
                IngredientParser::new(false).from_str("1 cup blue spinach"),
                IngredientParser::new(false).from_str("1/2 tbsp cinnamon"),
                IngredientParser::new(false).from_str("4 pounds top quality chicken filet"),
                IngredientParser::new(false).from_str("1/8 cup lemon juice"),
            ]
        );
        Ok(())
    }
}
