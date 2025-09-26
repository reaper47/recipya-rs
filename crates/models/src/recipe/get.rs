use std::collections::BTreeMap;

use diesel::internal::derives::multiconnection::chrono;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use repository::{ModelManager, PgPooledConn, schema};
use uuid::Uuid;

use crate::params::SearchParams;
use crate::recipe::{
    Nutrition, RecipeDetails, RecipeSearch, Times, ToolRecipe, Video, VideoRecipe,
};
use crate::{Error, Recipe, Result};

impl Recipe {
    /// Retrieves the total number of recipes that belong to a given user.
    pub async fn count(mm: &ModelManager, user_id: i64) -> Result<i64> {
        let mut conn = mm.pool.get().await?;

        let count = schema::recipes::table
            .filter(schema::recipes::user_id.eq(user_id))
            .select(diesel::dsl::count(schema::recipes::id))
            .first(&mut conn)
            .await?;

        Ok(count)
    }

    /// Retrieves the details of a specific recipe for a given user.
    pub async fn get(mm: &ModelManager, user_id: i64, recipe_id: i64) -> Result<RecipeDetails> {
        let mut conn = mm.pool.get().await?;

        let (recipe, category, cuisine, keywords, nutrition, times) = schema::recipes::table
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
            .left_join(
                schema::nutrition::table.on(schema::nutrition::recipe_id.eq(schema::recipes::id)),
            )
            .inner_join(schema::times::table.on(schema::times::recipe_id.eq(schema::recipes::id)))
            .select((
                (
                    schema::recipes::id,
                    schema::recipes::name,
                    schema::recipes::description,
                    schema::recipes::image,
                    schema::recipes::yield_,
                    schema::recipes::language,
                    schema::recipes::measurement_system_id,
                    schema::recipes::notes,
                    schema::recipes::source,
                    schema::recipes::is_favourite,
                    schema::recipes::rating,
                    schema::recipes::created_at,
                    schema::recipes::updated_at,
                    schema::recipes::user_id,
                ),
                schema::categories::name,
                schema::cuisines::name.nullable(),
                schema::keywords::name.nullable(),
                schema::nutrition::all_columns.nullable(),
                schema::times::all_columns,
            ))
            .first::<(
                Recipe,
                String,
                Option<String>,
                Option<String>,
                Option<Nutrition>,
                Times,
            )>(&mut conn)
            .await
            .optional()?
            .ok_or_else(|| Error::EntityNotFound {
                entity: "recipe",
                id: recipe_id,
            })?;

        fetch_recipe_details(
            &mut conn, recipe, category, cuisine, keywords, nutrition, times,
        )
        .await
    }

    /// Gets the recipe only.
    pub async fn get_recipe_only(
        mm: &ModelManager,
        user_id: i64,
        recipe_id: i64,
    ) -> Result<Recipe> {
        let mut conn = mm.pool.get().await?;

        let recipe = schema::recipes::table
            .filter(schema::recipes::user_id.eq(user_id))
            .filter(schema::recipes::id.eq(recipe_id))
            .select(Recipe::as_select())
            .first::<Recipe>(&mut conn)
            .await?;

        Ok(recipe)
    }

    /// Gets a page of recipes belonging to the user.
    pub async fn get_page(
        mm: &ModelManager,
        user_id: i64,
        search_params: &SearchParams,
    ) -> Result<Vec<RecipeDetails>> {
        let query = search_params.q.as_deref().unwrap_or_default();
        let page = search_params.page.unwrap_or(1) as i64;
        let is_favourites = search_params.is_favourites.unwrap_or(false);

        let recipe_search = RecipeSearch::new(query, page, is_favourites, user_id)?;
        let recipes = recipe_search.search(mm).await?;

        Ok(recipes)
    }
}

/// Fetches all the details of a recipe.
pub async fn fetch_recipe_details(
    conn: &mut PgPooledConn<'_>,
    recipe: Recipe,
    category: String,
    cuisine: Option<String>,
    keywords: Option<String>,
    nutrition: Option<Nutrition>,
    times: Times,
) -> Result<RecipeDetails> {
    let recipe_id = recipe.id;

    let additional_images = schema::additional_images_recipe::table
        .select(schema::additional_images_recipe::image)
        .filter(schema::additional_images_recipe::recipe_id.eq(recipe_id))
        .load::<Uuid>(conn)
        .await?;

    let ingredients = schema::ingredients_recipes::table
        .filter(schema::ingredients_recipes::recipe_id.eq(recipe_id))
        .inner_join(schema::ingredients::table)
        .inner_join(schema::sections::table)
        .order(schema::ingredients_recipes::item_order)
        .select((
            schema::ingredients::name,
            schema::sections::name,
            schema::ingredients_recipes::section_id,
        ))
        .load::<(String, String, i64)>(conn)
        .await?
        .into_iter()
        .fold(
            BTreeMap::new(),
            |mut acc, (ingredient, section, section_id)| {
                acc.entry(section_id)
                    .and_modify(|entry: &mut (String, Vec<String>)| {
                        entry.1.push(ingredient.clone())
                    })
                    .or_insert_with(|| (section, vec![ingredient]));
                acc
            },
        )
        .into_values()
        .collect();

    let instructions = schema::instructions_recipes::table
        .filter(schema::instructions_recipes::recipe_id.eq(recipe_id))
        .inner_join(schema::instructions::table)
        .inner_join(schema::sections::table)
        .order(schema::instructions_recipes::item_order)
        .select((
            schema::instructions::name,
            schema::sections::name,
            schema::instructions_recipes::section_id,
        ))
        .load::<(String, String, i64)>(conn)
        .await?
        .into_iter()
        .fold(
            BTreeMap::new(),
            |mut acc, (instruction, section, section_id)| {
                acc.entry(section_id)
                    .and_modify(|entry: &mut (String, Vec<String>)| {
                        entry.1.push(instruction.clone())
                    })
                    .or_insert_with(|| (section, vec![instruction]));
                acc
            },
        )
        .into_values()
        .collect();

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
            duration: match v.duration {
                None => None,
                Some(d) => {
                    let ms = chrono::Duration::milliseconds(d.microseconds / 1000);
                    let days = chrono::Duration::days(d.days as i64);
                    Some(ms + days)
                }
            },
            content_url: v.content_url,
            embed_url: v.embed_url,
            created_at: v.created_at,
        })
        .collect::<Vec<_>>();

    Ok(RecipeDetails {
        recipe,
        additional_images,
        category,
        cuisine,
        ingredients,
        instructions,
        keywords,
        nutrition,
        times,
        tools,
        videos,
    })
}

#[cfg(test)]
mod tests {
    use testing::utils::{TestDb, build_server_anonymous, create_app_state};

    use super::*;
    use crate::recipe::test_utils::a_complete_recipe_for_create;
    use crate::user::User;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_count {
        use super::*;

        #[tokio::test]
        async fn test_count_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let _ = build_server_anonymous(config.clone()).await?;
            let user = User::get_user_by_id(&state.mm, 1)
                .await?
                .expect("no such user");
            let user2 = User::get_user_by_id(&state.mm, 2)
                .await?
                .expect("no such user");
            for i in 0..5 {
                let mut recipe = a_complete_recipe_for_create();
                recipe.name.push_str(i.to_string().as_str());
                let _ = Recipe::create(&state.mm, user.id, &recipe).await?;
            }
            for i in 0..10 {
                let mut recipe = a_complete_recipe_for_create();
                recipe.name.push_str((i + 1002).to_string().as_str());
                let _ = Recipe::create(&state.mm, user2.id, &recipe).await?;
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
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;

            let recipes = Recipe::get_page(
                &state.mm,
                1,
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
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let _ = build_server_anonymous(config.clone()).await?;
            let mut expected = Vec::with_capacity(15);
            for i in 0..15 {
                let mut recipe = a_complete_recipe_for_create();
                recipe.name.push_str(i.to_string().as_str());
                let _ = Recipe::create(&state.mm, 1, &recipe).await?;
                expected.push(recipe.name);
            }

            let recipes = Recipe::get_page(
                &state.mm,
                1,
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
}
