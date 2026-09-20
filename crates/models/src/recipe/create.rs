use diesel::prelude::*;
use diesel_async::{AsyncConnection, RunQueryDsl};
use uuid::Uuid;

use repository::{ModelManager, schema};

use super::helpers::{
    get_category_id, get_cuisine_id, insert_additional_images, insert_ingredients,
    insert_instructions, insert_keywords, insert_nutrition, insert_sections, insert_tools,
    insert_videos,
};

use crate::recipe::structs::recipe::{
    CategoryForInsert, CategoryRecipe, CuisineRecipe, Recipe, RecipeForCreate, RecipeForInsert,
};
use crate::recipe::structs::time::TimesForInsert;
use crate::settings::UserSettingDetails;
use crate::user::UserCategory;
use crate::{Error, Result};

impl Recipe {
    /// Adds a recipe category into the database.
    pub async fn add_category(mm: &ModelManager, category: &str, user_id: Uuid) -> Result<()> {
        let mut conn = mm.pool.get().await?;

        let category_id = diesel::insert_into(schema::categories::table)
            .values(&CategoryForInsert {
                name: Some(category.to_string()),
            })
            .on_conflict(schema::categories::name)
            .do_update()
            .set(schema::categories::name.eq(category.to_string()))
            .returning(schema::categories::id)
            .get_result(&mut conn)
            .await?;

        diesel::insert_into(schema::users_categories::table)
            .values(&UserCategory {
                user_id,
                category_id,
            })
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Creates a new recipe in the database for a given user.
    ///
    /// # Notes
    ///
    /// - If images are provided, the first image is treated as the main image, and the rest are
    ///   stored as additional images.
    /// - The detected language of the recipe is used for further processing and localization.
    #[allow(clippy::too_many_lines)]
    pub async fn create(
        mm: &ModelManager,
        user_id: Uuid,
        recipe_c: &RecipeForCreate,
        user_settings: &UserSettingDetails,
    ) -> Result<i64> {
        let mut conn = mm.pool.get().await?;

        let recipe_id = conn
            .transaction::<i64, Error, _>(async move |mut conn| {
                // Images
                let (main_image, additional_images) = recipe_c.first_and_rest_images();

                let insert_result = diesel::insert_into(schema::recipes::table)
                    .values(&RecipeForInsert {
                        name: recipe_c.name.clone(),
                        description: recipe_c.description.clone(),
                        image: main_image,
                        r#yield: recipe_c.r#yield,
                        language: recipe_c.detect_language().code().to_string(),
                        notes: recipe_c.notes.clone(),
                        source: recipe_c.source.clone(),
                        is_favourite: recipe_c.is_favourite,
                        rating: recipe_c.rating,
                        user_id,
                    })
                    .on_conflict((
                        schema::recipes::name,
                        schema::recipes::source,
                        schema::recipes::yield_,
                        schema::recipes::user_id,
                    ))
                    .do_nothing()
                    .returning(schema::recipes::id)
                    .get_result::<i64>(&mut conn)
                    .await
                    .optional()?;

                let Some(recipe_id) = insert_result else {
                    let recipe_id = schema::recipes::table
                        .filter(schema::recipes::name.eq(&recipe_c.name))
                        .select(schema::recipes::id)
                        .get_result::<i64>(&mut conn)
                        .await?;
                    return Err(Error::DuplicateEntityWithID(recipe_id));
                };

                // Additional Images
                insert_additional_images(&mut conn, recipe_id, additional_images).await?;

                // Category
                let category_id = get_category_id(conn, recipe_c.category.as_ref()).await?;

                diesel::insert_into(schema::categories_recipes::table)
                    .values(&CategoryRecipe {
                        category_id,
                        recipe_id,
                    })
                    .execute(&mut conn)
                    .await?;

                diesel::insert_into(schema::users_categories::table)
                    .values(&UserCategory {
                        user_id,
                        category_id,
                    })
                    .on_conflict_do_nothing()
                    .execute(&mut conn)
                    .await?;

                // Cuisine
                if let Some(cuisine) = recipe_c.cuisine.as_ref() {
                    let cuisine_id = get_cuisine_id(conn, cuisine.into()).await?;

                    diesel::insert_into(schema::cuisines_recipes::table)
                        .values(&CuisineRecipe {
                            cuisine_id,
                            recipe_id,
                        })
                        .execute(&mut conn)
                        .await?;
                }

                // Sections for ingredients and instructions
                let sections_map = insert_sections(conn, recipe_c).await?;

                // Ingredients
                let ingredient_names =
                    insert_ingredients(conn, &sections_map, &recipe_c.ingredients, recipe_id)
                        .await?;

                // Instructions
                insert_instructions(
                    conn,
                    &sections_map,
                    &recipe_c.instructions,
                    recipe_id,
                    ingredient_names.as_slice(),
                )
                .await?;

                // Keywords
                insert_keywords(&mut conn, &recipe_c.keywords, user_id, recipe_id).await?;

                // Nutrition
                insert_nutrition(
                    conn,
                    recipe_id,
                    &recipe_c.nutrition,
                    recipe_c.ingredients.items_as_text().as_slice(),
                    &user_settings.nutrition_source,
                    recipe_c.r#yield.unwrap_or(1),
                )
                .await?;

                // Times
                let times = recipe_c.times.clone().unwrap_or_default();

                diesel::insert_into(schema::times::table)
                    .values(&TimesForInsert {
                        recipe_id,
                        prep_seconds: times.prep_seconds,
                        cook_seconds: times.cook_seconds,
                    })
                    .execute(&mut conn)
                    .await?;

                // Tools
                insert_tools(conn, &recipe_c.tools, recipe_id).await?;

                // Videos
                insert_videos(conn, &recipe_c.videos, recipe_id).await?;

                diesel::insert_into(schema::users_recipes::table)
                    .values((
                        schema::users_recipes::user_id.eq(user_id),
                        schema::users_recipes::recipe_id.eq(recipe_id),
                    ))
                    .execute(&mut conn)
                    .await?;

                Ok(recipe_id)
            })
            .await?;

        Ok(recipe_id)
    }
}
