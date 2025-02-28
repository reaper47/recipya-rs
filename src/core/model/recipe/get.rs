use std::collections::BTreeMap;

use diesel::internal::derives::multiconnection::chrono;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::core::model::recipe::{Nutrition, RecipeDetails, Times, ToolRecipe, Video, VideoRecipe};
use crate::core::model::{Error, Recipe, Result};
use crate::core::repository::ModelManager;

impl Recipe {
    /// Retrieves the details of a specific recipe for a given user.
    pub async fn get(mm: &ModelManager, user_id: i64, recipe_id: i64) -> Result<RecipeDetails> {
        use crate::core::repository::schema;

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
                schema::recipes::all_columns,
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

        let additional_images = schema::additional_images_recipe::table
            .select(schema::additional_images_recipe::image)
            .filter(schema::additional_images_recipe::recipe_id.eq(recipe_id))
            .load::<Uuid>(&mut conn)
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
            .load::<(String, String, i64)>(&mut conn)
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
            .load::<(String, String, i64)>(&mut conn)
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
                .load::<String>(&mut conn)
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
            .load::<(String, i16, i16)>(&mut conn)
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
            .load::<VideoRecipe>(&mut conn)
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
}
