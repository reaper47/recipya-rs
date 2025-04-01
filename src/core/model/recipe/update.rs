use diesel::prelude::*;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};

use crate::core::model::Recipe;
use crate::core::model::recipe::helpers::{
    get_cuisine_id, insert_additional_images, insert_ingredients, insert_instructions,
    insert_keywords, insert_nutrition, insert_sections, insert_tools, insert_videos,
    update_category,
};
use crate::core::model::recipe::{Nutrition, RecipeForCreate, TimesForInsert};
use crate::core::model::{Error, Result};
use crate::core::repository::ModelManager;
use crate::core::repository::schema;

impl Recipe {
    /// Updates a recipe from the updated fields.
    pub async fn update(
        mm: &ModelManager,
        user_id: i64,
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
        if let Some(n) = new_recipe.yield_ {
            if recipe.yield_ != n {
                recipe.yield_ = n;
            }
        }
        if recipe.source != new_recipe.source {
            recipe.source = new_recipe.source.clone();
        }

        mm.pool
            .get()
            .await?.transaction::<_, Error, _>(|conn| {
                async move {
                    diesel::update(schema::recipes::table)
                        .filter(schema::recipes::id.eq(recipe_id))
                        .set(&recipe)
                        .execute(conn)
                        .await?;

                    // Additional images
                    diesel::delete(schema::additional_images_recipe::table)
                        .filter(schema::additional_images_recipe::recipe_id.eq(recipe_id))
                        .execute(conn)
                        .await?;

                    insert_additional_images(conn, recipe_id, additional_images).await?;

                    // Category
                    match &new_recipe.category {
                        None if old_recipe.category != *"uncategorized" => {
                            update_category(conn, user_id, recipe_id, &new_recipe.category).await?
                        }
                        Some(category) if old_recipe.category != *category => {
                            update_category(conn, user_id, recipe_id, &new_recipe.category).await?
                        }
                        _ => {}
                    }

                    // Cuisine
                    match &new_recipe.cuisine {
                        Some(cuisine) if old_recipe.cuisine != new_recipe.cuisine => {
                            let cuisine_id = get_cuisine_id(conn, cuisine.into()).await?;

                            diesel::update(schema::cuisines_recipes::table)
                                .filter(schema::cuisines_recipes::recipe_id.eq(recipe_id))
                                .set(schema::cuisines_recipes::cuisine_id.eq(cuisine_id))
                                .execute(conn)
                                .await?;
                        }
                        None => {
                            diesel::delete(schema::cuisines_recipes::table)
                                .filter(schema::cuisines_recipes::recipe_id.eq(recipe_id))
                                .execute(conn)
                                .await?;
                        }
                        _ => {}
                    }

                    // Sections
                    let sections_map = insert_sections(conn, new_recipe).await?;

                    // Ingredients
                    let old_ingredients = old_recipe
                        .ingredients
                        .iter()
                        .flat_map(|(_, ing)| ing)
                        .collect::<Vec<_>>();
                    let new_ingredients = new_recipe
                        .ingredients
                        .iter()
                        .flat_map(|(_, ing)| ing)
                        .collect::<Vec<_>>();
                    if old_ingredients != new_ingredients {
                        diesel::delete(schema::ingredients_recipes::table)
                            .filter(schema::ingredients_recipes::recipe_id.eq(recipe_id))
                            .execute(conn)
                            .await?;

                        insert_ingredients(conn, &sections_map, &new_recipe.ingredients, recipe_id)
                            .await?;
                    }

                    // Instructions
                    let old_instructions = old_recipe
                        .instructions
                        .iter()
                        .flat_map(|(_, ing)| ing)
                        .collect::<Vec<_>>();
                    let new_instructions = new_recipe
                        .instructions
                        .iter()
                        .flat_map(|(_, ing)| ing)
                        .collect::<Vec<_>>();
                    if old_instructions != new_instructions {
                        diesel::delete(schema::instructions_recipes::table)
                            .filter(schema::instructions_recipes::recipe_id.eq(recipe_id))
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
                        diesel::delete(schema::keywords_recipes::table)
                            .filter(schema::keywords_recipes::recipe_id.eq(recipe_id))
                            .execute(conn)
                            .await?;

                        insert_keywords(conn, &new_recipe.keywords, user_id, recipe_id).await?;
                    }

                    // Nutrition
                    match &new_recipe.nutrition {
                        None => {
                            diesel::delete(schema::nutrition::table)
                                .filter(schema::nutrition::recipe_id.eq(recipe_id))
                                .execute(conn)
                                .await?;
                        }
                        Some(new_nutrition_c) => match old_recipe.nutrition {
                            None => insert_nutrition(conn, new_nutrition_c, recipe_id).await?,
                            Some(old_nutrition) => {
                                let new_nutrition = Nutrition {
                                    id: old_nutrition.id,
                                    recipe_id,
                                    calories_kcal: new_nutrition_c.calories_kcal,
                                    total_carbohydrates: new_nutrition_c.total_carbohydrates,
                                    sugars_g: new_nutrition_c.sugars_g,
                                    protein_g: new_nutrition_c.protein_g,
                                    total_fat_g: new_nutrition_c.total_fat_g,
                                    saturated_fat_g: new_nutrition_c.saturated_fat_g,
                                    unsaturated_fat_g: new_nutrition_c.unsaturated_fat_g,
                                    cholesterol_mg: new_nutrition_c.cholesterol_mg,
                                    sodium_mg: new_nutrition_c.sodium_mg,
                                    fiber_g: new_nutrition_c.fiber_g,
                                    trans_fat_g: new_nutrition_c.trans_fat_g,
                                    serving_size: new_nutrition_c.serving_size.clone(),
                                };
                                if new_nutrition != old_nutrition {
                                    diesel::update(schema::nutrition::table)
                                        .filter(schema::nutrition::recipe_id.eq(recipe_id))
                                        .set(&new_nutrition)
                                        .execute(conn)
                                        .await?;
                                }
                            }
                        },
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
                        diesel::update(schema::times::table)
                            .filter(schema::times::recipe_id.eq(recipe_id))
                            .set(&new_times_for_insert)
                            .execute(conn)
                            .await?;
                    }

                    // Tools
                    diesel::delete(schema::tools_recipes::table)
                        .filter(schema::tools_recipes::recipe_id.eq(recipe_id))
                        .execute(conn)
                        .await?;

                    insert_tools(conn, &new_recipe.tools, recipe_id).await?;

                    // Videos
                    diesel::delete(schema::videos_recipes::table)
                        .filter(schema::videos_recipes::recipe_id.eq(recipe_id))
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
}
