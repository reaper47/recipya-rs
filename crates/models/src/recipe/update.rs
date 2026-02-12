use diesel::dsl::not;
use diesel::prelude::*;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};
use uuid::Uuid;

use repository::{ModelManager, schema};

use crate::Recipe;
use crate::recipe::helpers::{
    get_cuisine_id, insert_additional_images, insert_ingredients, insert_instructions,
    insert_keywords, insert_nutrition, insert_sections, insert_tools, insert_videos,
    update_category,
};
use crate::recipe::structs::recipe::RecipeForCreate;
use crate::recipe::structs::time::TimesForInsert;
use crate::settings::UserSettingDetails;
use crate::{Error, Result};

impl Recipe {
    /// Updates a recipe from the updated fields.
    #[allow(clippy::too_many_lines)]
    pub async fn update(
        mm: &ModelManager,
        user_id: Uuid,
        recipe_id: i64,
        new_recipe: &mut RecipeForCreate,
    ) -> Result<()> {
        let mut old_recipe = Self::get(mm, user_id, recipe_id).await?;

        let (main_image, additional_images) = new_recipe.first_and_rest_images();

        let mut recipe = old_recipe.recipe;
        if recipe.name != new_recipe.name {
            recipe.name = new_recipe.name.clone();
        }
        if recipe.description != new_recipe.description {
            recipe.description = new_recipe.description.clone();
        }
        if recipe.image != main_image {
            recipe.image = main_image;
        }
        if recipe.notes != new_recipe.notes {
            recipe.notes = new_recipe.notes.clone();
        }
        if let Some(n) = new_recipe.r#yield
            && recipe.r#yield != n
        {
            recipe.r#yield = n;
        }
        if recipe.source != new_recipe.source {
            recipe.source = new_recipe.source.clone();
        }
        if recipe.rating != new_recipe.rating {
            recipe.rating = new_recipe.rating;
        }

        let user_settings = UserSettingDetails::get(mm, user_id).await?;

        mm.pool
            .get()
            .await?
            .transaction::<_, Error, _>(|conn| {
                async move {
                    diesel::update(
                        schema::recipes::table.filter(schema::recipes::id.eq(recipe_id)),
                    )
                    .set(&recipe)
                    .execute(conn)
                    .await?;

                    // Additional images
                    diesel::delete(
                        schema::additional_images_recipe::table
                            .filter(schema::additional_images_recipe::recipe_id.eq(recipe_id)),
                    )
                    .execute(conn)
                    .await?;

                    insert_additional_images(conn, recipe_id, additional_images).await?;

                    // Category
                    match &new_recipe.category {
                        None if old_recipe.category != *"uncategorized" => {
                            update_category(conn, user_id, recipe_id, new_recipe.category.as_ref())
                                .await?;
                        }
                        Some(category) if old_recipe.category != *category => {
                            update_category(conn, user_id, recipe_id, new_recipe.category.as_ref())
                                .await?;
                        }
                        _ => {}
                    }

                    // Cuisine
                    match &new_recipe.cuisine {
                        Some(cuisine) if old_recipe.cuisine != new_recipe.cuisine => {
                            let cuisine_id = get_cuisine_id(conn, cuisine.into()).await?;

                            diesel::update(
                                schema::cuisines_recipes::table
                                    .filter(schema::cuisines_recipes::recipe_id.eq(recipe_id)),
                            )
                            .set(schema::cuisines_recipes::cuisine_id.eq(cuisine_id))
                            .execute(conn)
                            .await?;
                        }
                        None => {
                            diesel::delete(
                                schema::cuisines_recipes::table
                                    .filter(schema::cuisines_recipes::recipe_id.eq(recipe_id)),
                            )
                            .execute(conn)
                            .await?;
                        }
                        _ => {}
                    }

                    // Sections
                    let sections_map = insert_sections(conn, new_recipe).await?;

                    // Ingredients
                    let old_ingredients = old_recipe.ingredients.items_as_text();
                    let new_ingredients = new_recipe.ingredients.items_as_text();
                    let is_ingredients_changed = old_ingredients != new_ingredients;
                    if is_ingredients_changed {
                        diesel::delete(
                            schema::ingredients_recipes::table
                                .filter(schema::ingredients_recipes::recipe_id.eq(recipe_id)),
                        )
                        .execute(conn)
                        .await?;

                        insert_ingredients(conn, &sections_map, &new_recipe.ingredients, recipe_id)
                            .await?;
                    }

                    // Instructions
                    let old_instructions = old_recipe.instructions.items_as_text();
                    let new_instructions = new_recipe.instructions.items_as_text();
                    if old_instructions != new_instructions {
                        diesel::delete(
                            schema::instructions_recipes::table
                                .filter(schema::instructions_recipes::recipe_id.eq(recipe_id)),
                        )
                        .execute(conn)
                        .await?;

                        insert_instructions(
                            conn,
                            &sections_map,
                            &new_recipe.instructions,
                            recipe_id,
                        )
                        .await?;
                    }

                    // Keywords
                    old_recipe.keywords.sort();
                    new_recipe.keywords.sort();
                    new_recipe.keywords.dedup();

                    if old_recipe.keywords != new_recipe.keywords {
                        diesel::delete(
                            schema::keywords_recipes::table
                                .filter(schema::keywords_recipes::recipe_id.eq(recipe_id)),
                        )
                        .execute(conn)
                        .await?;

                        insert_keywords(conn, &new_recipe.keywords, user_id, recipe_id).await?;
                    }

                    // Nutrition
                    let new_ingredients_slice = new_ingredients.as_slice();

                    if is_ingredients_changed {
                        insert_nutrition(
                            conn,
                            recipe_id,
                            &new_recipe.nutrition,
                            new_ingredients_slice,
                            user_settings.nutrition_source,
                            new_recipe.r#yield.unwrap_or(1),
                        )
                        .await?;
                    }

                    // Times
                    let times = new_recipe.times.clone().unwrap_or_default();
                    let old_times_for_insert = TimesForInsert {
                        recipe_id,
                        prep_seconds: old_recipe.times.prep_seconds,
                        cook_seconds: old_recipe.times.cook_seconds,
                    };
                    let new_times_for_insert = TimesForInsert {
                        recipe_id,
                        prep_seconds: new_recipe
                            .times
                            .clone()
                            .unwrap_or_else(|| times.clone())
                            .prep_seconds,
                        cook_seconds: new_recipe.times.clone().unwrap_or(times).cook_seconds,
                    };
                    if old_times_for_insert != new_times_for_insert {
                        diesel::update(
                            schema::times::table.filter(schema::times::recipe_id.eq(recipe_id)),
                        )
                        .set(&new_times_for_insert)
                        .execute(conn)
                        .await?;
                    }

                    // Tools
                    diesel::delete(
                        schema::tools_recipes::table
                            .filter(schema::tools_recipes::recipe_id.eq(recipe_id)),
                    )
                    .execute(conn)
                    .await?;

                    insert_tools(conn, &new_recipe.tools, recipe_id).await?;

                    // Videos
                    diesel::delete(
                        schema::videos_recipes::table
                            .filter(schema::videos_recipes::recipe_id.eq(recipe_id)),
                    )
                    .execute(conn)
                    .await?;

                    insert_videos(conn, &new_recipe.videos, recipe_id).await?;

                    Ok(())
                }
                .scope_boxed()
            })
            .await?;

        Ok(())
    }

    /// Toggles whether the recipe is a favourite.
    pub async fn toggle_favourite(
        mm: &ModelManager,
        user_id: Uuid,
        recipe_id: i64,
    ) -> Result<bool> {
        use schema::recipes;

        let mut conn = mm.pool.get().await?;

        let new_value = diesel::update(
            recipes::table
                .filter(recipes::id.eq(recipe_id))
                .filter(recipes::user_id.eq(user_id)),
        )
        .set(recipes::is_favourite.eq(not(recipes::is_favourite)))
        .returning(recipes::is_favourite)
        .get_result::<bool>(&mut conn)
        .await?;

        Ok(new_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recipe::structs::test_utils::a_complete_recipe_for_create;
    use testing::utils::{TestDb, create_app_state, insert_user};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_mark_favourite {
        use super::*;

        #[tokio::test]
        async fn test_user_does_not_exist_err() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let user = insert_user(config.clone()).await?;
            let recipe = a_complete_recipe_for_create();
            let id = Recipe::create(&state.mm, user.id, &recipe).await?;

            if Recipe::toggle_favourite(&state.mm, Uuid::new_v4(), id)
                .await
                .is_ok()
            {
                panic!("Expected error");
            }
            Ok(())
        }

        #[tokio::test]
        async fn test_valid_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let user = insert_user(config.clone()).await?;
            let recipe = a_complete_recipe_for_create();
            let initial_state = recipe.is_favourite;
            let id = Recipe::create(&state.mm, user.id, &recipe).await?;

            let current_state = Recipe::toggle_favourite(&state.mm, user.id, id).await?;

            pretty_assertions::assert_eq!(current_state, !initial_state);
            Ok(())
        }
    }
}
