use std::collections::{BTreeSet, HashMap, HashSet};

use diesel::data_types::PgInterval;
use diesel::prelude::*;
use diesel::upsert::excluded;
use diesel_async::{AsyncConnection, RunQueryDsl};
use uuid::Uuid;

use recipe_schema::Sections;
use repository::schema;

use crate::Result;
use crate::recipe::structs::{
    AdditionalImageForInsert, CategoryForInsert, CuisineForInsert, IngredientForInsert,
    IngredientRecipeForInsert, InstructionForInsert, InstructionRecipeForInsert, KeywordForInsert,
    NutritionForInsert, SectionForInsert, ToolForInsert, ToolRecipeForInsert, VideoForInsert,
};
use crate::recipe::{
    KeywordRecipe, NutritionForCreate, RecipeForCreate, ToolForCreate, VideoForCreate,
};
use crate::user::UserKeyword;

pub(super) async fn get_category_id<C>(mut conn: &mut C, category: &Option<String>) -> Result<i64>
where
    C: AsyncConnection<Backend = diesel::pg::Pg>,
{
    let default = String::from("uncategorized");

    let category = Option::from(
        category
            .clone()
            .map(|c| {
                c.split_once([',', ';'])
                    .map(|(first, _)| first.to_lowercase())
                    .unwrap_or_else(|| c.to_lowercase())
            })
            .unwrap_or(default.clone()),
    );

    Ok(diesel::insert_into(schema::categories::table)
        .values(&CategoryForInsert {
            name: category.clone(),
        })
        .on_conflict(schema::categories::name)
        .do_update()
        .set(schema::categories::name.eq(category.unwrap_or(default)))
        .returning(schema::categories::id)
        .get_result::<i64>(&mut conn)
        .await?)
}

pub(super) async fn get_cuisine_id<C>(mut conn: &mut C, cuisine: String) -> Result<i64>
where
    C: AsyncConnection<Backend = diesel::pg::Pg>,
{
    Ok(diesel::insert_into(schema::cuisines::table)
        .values(&CuisineForInsert {
            name: Some(cuisine.to_lowercase()),
        })
        .on_conflict(schema::cuisines::name)
        .do_update()
        .set(schema::cuisines::name.eq(excluded(schema::cuisines::name)))
        .returning(schema::cuisines::id)
        .get_result::<i64>(&mut conn)
        .await?)
}

pub(super) async fn insert_additional_images<C>(
    mut conn: &mut C,
    recipe_id: i64,
    additional_images: Vec<Uuid>,
) -> Result<()>
where
    C: AsyncConnection<Backend = diesel::pg::Pg>,
{
    diesel::insert_into(schema::additional_images_recipe::table)
        .values(
            &additional_images
                .into_iter()
                .map(|image| AdditionalImageForInsert { recipe_id, image })
                .collect::<Vec<_>>(),
        )
        .execute(&mut conn)
        .await?;

    Ok(())
}

pub(super) async fn insert_ingredients<C>(
    mut conn: &mut C,
    sections_map: &HashMap<String, i64>,
    ingredients: &Sections,
    recipe_id: i64,
) -> Result<()>
where
    C: AsyncConnection<Backend = diesel::pg::Pg>,
{
    let mut uniques = HashSet::new();

    for (section, ingredients) in ingredients.iter() {
        let ingredient_recipes: Vec<_> = diesel::insert_into(schema::ingredients::table)
            .values(
                ingredients
                    .iter()
                    .map(|name| IngredientForInsert {
                        name: name.to_string(),
                    })
                    .filter(|s| uniques.insert(s.name.clone()))
                    .collect::<Vec<_>>(),
            )
            .on_conflict(schema::ingredients::name)
            .do_update()
            .set(schema::ingredients::name.eq(excluded(schema::ingredients::name)))
            .returning((schema::ingredients::id, schema::ingredients::name))
            .get_results::<(i64, String)>(&mut conn)
            .await?
            .into_iter()
            .enumerate()
            .map(|(idx, (id, _))| IngredientRecipeForInsert {
                ingredient_id: id,
                recipe_id,
                section_id: *sections_map.get(section).unwrap_or(&1),
                item_order: idx as i16,
            })
            .collect::<Vec<_>>();

        diesel::insert_into(schema::ingredients_recipes::table)
            .values(&ingredient_recipes)
            .execute(&mut conn)
            .await?;
    }

    Ok(())
}

pub(super) async fn insert_instructions<C>(
    mut conn: &mut C,
    sections_map: &HashMap<String, i64>,
    instructions: &Sections,
    recipe_id: i64,
) -> Result<()>
where
    C: AsyncConnection<Backend = diesel::pg::Pg>,
{
    let mut uniques = HashSet::new();

    for (section, instructions) in instructions.iter() {
        let instruction_recipes: Vec<InstructionRecipeForInsert> =
            diesel::insert_into(schema::instructions::table)
                .values(
                    instructions
                        .iter()
                        .map(|name| InstructionForInsert { name: name.into() })
                        .filter(|s| uniques.insert(s.name.clone()))
                        .collect::<Vec<_>>(),
                )
                .on_conflict(schema::instructions::name)
                .do_update()
                .set(schema::instructions::name.eq(excluded(schema::instructions::name)))
                .returning((schema::instructions::id, schema::instructions::name))
                .get_results::<(i64, String)>(&mut conn)
                .await?
                .into_iter()
                .enumerate()
                .map(|(idx, (id, _))| InstructionRecipeForInsert {
                    instruction_id: id,
                    recipe_id,
                    section_id: *sections_map.get(section).unwrap_or(&1),
                    item_order: idx as i16,
                })
                .collect::<Vec<_>>();

        diesel::insert_into(schema::instructions_recipes::table)
            .values(&instruction_recipes)
            .execute(&mut conn)
            .await?;
    }

    Ok(())
}

pub(super) async fn insert_keywords<C>(
    mut conn: &mut C,
    keywords: &[String],
    user_id: i64,
    recipe_id: i64,
) -> Result<()>
where
    C: AsyncConnection<Backend = diesel::pg::Pg>,
{
    let mut keywords = Vec::from(keywords);
    keywords.sort();
    keywords.dedup();

    for keyword in keywords.iter() {
        let kw = keyword.to_lowercase();

        let keyword_id = diesel::insert_into(schema::keywords::table)
            .values(&KeywordForInsert {
                name: Some(kw.to_owned()),
            })
            .on_conflict(schema::keywords::name)
            .do_update()
            .set(schema::keywords::name.eq(kw.to_owned()))
            .returning(schema::keywords::id)
            .get_result(&mut conn)
            .await?;

        diesel::insert_into(schema::users_keywords::table)
            .values(&UserKeyword {
                user_id,
                keyword_id,
            })
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .await?;

        diesel::insert_into(schema::keywords_recipes::table)
            .values(&KeywordRecipe {
                keyword_id,
                recipe_id,
            })
            .execute(&mut conn)
            .await?;
    }

    Ok(())
}

pub(super) async fn insert_nutrition<C>(
    mut conn: &mut C,
    nutrition: &NutritionForCreate,
    recipe_id: i64,
) -> Result<()>
where
    C: AsyncConnection<Backend = diesel::pg::Pg>,
{
    diesel::insert_into(schema::nutrition::table)
        .values(&NutritionForInsert {
            recipe_id,
            calories_kcal: nutrition.calories_kcal,
            total_carbohydrates: nutrition.total_carbohydrates,
            sugars_g: nutrition.sugars_g,
            protein_g: nutrition.protein_g,
            total_fat_g: nutrition.total_fat_g,
            saturated_fat_g: nutrition.saturated_fat_g,
            unsaturated_fat_g: nutrition.unsaturated_fat_g,
            cholesterol_mg: nutrition.cholesterol_mg,
            sodium_mg: nutrition.sodium_mg,
            fiber_g: nutrition.fiber_g,
            trans_fat_g: nutrition.trans_fat_g,
            serving_size: nutrition.serving_size.clone(),
        })
        .execute(&mut conn)
        .await?;

    Ok(())
}

pub(super) async fn insert_sections<C>(
    mut conn: &mut C,
    recipe_c: &RecipeForCreate,
) -> Result<HashMap<String, i64>>
where
    C: AsyncConnection<Backend = diesel::pg::Pg>,
{
    let sections_for_insert = recipe_c
        .ingredients
        .iter()
        .chain(recipe_c.instructions.iter())
        .map(|(name, _)| name.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .rev()
        .map(|name| SectionForInsert { name })
        .filter(|s| !s.name.is_empty())
        .collect::<Vec<_>>();

    match sections_for_insert.is_empty() {
        true => Ok(HashMap::new()),
        false => Ok(diesel::insert_into(schema::sections::table)
            .values(&sections_for_insert)
            .on_conflict(schema::sections::name)
            .do_update()
            .set(schema::sections::name.eq(excluded(schema::sections::name)))
            .returning((schema::sections::id, schema::sections::name))
            .get_results::<(i64, String)>(&mut conn)
            .await?
            .into_iter()
            .map(|(id, name)| (name, id))
            .collect()),
    }
}

pub(super) async fn insert_tools<C>(
    mut conn: &mut C,
    tools: &[ToolForCreate],
    recipe_id: i64,
) -> Result<()>
where
    C: AsyncConnection<Backend = diesel::pg::Pg>,
{
    let mut uniques = HashSet::new();

    if !tools.is_empty() {
        let tool_recipes: Vec<_> = diesel::insert_into(schema::tools::table)
            .values(
                tools
                    .iter()
                    .map(|tool| ToolForInsert {
                        name: tool.name.to_lowercase(),
                    })
                    .filter(|s| uniques.insert(s.name.clone()))
                    .collect::<Vec<_>>(),
            )
            .on_conflict(schema::tools::name)
            .do_update()
            .set(schema::tools::name.eq(excluded(schema::tools::name)))
            .returning((schema::tools::id, schema::tools::name))
            .get_results::<(i64, String)>(&mut conn)
            .await?
            .into_iter()
            .enumerate()
            .map(|(idx, (id, _))| ToolRecipeForInsert {
                tool_id: id,
                recipe_id,
                quantity: tools.get(idx).map_or(1, |x| x.quantity),
                tool_order: idx as i16,
            })
            .collect::<Vec<_>>();

        diesel::insert_into(schema::tools_recipes::table)
            .values(&tool_recipes)
            .execute(&mut conn)
            .await?;
    }

    Ok(())
}

pub(super) async fn insert_videos<C>(
    mut conn: &mut C,
    videos: &[VideoForCreate],
    recipe_id: i64,
) -> Result<()>
where
    C: AsyncConnection<Backend = diesel::pg::Pg>,
{
    if !videos.is_empty() {
        diesel::insert_into(schema::videos_recipes::table)
            .values(
                &videos
                    .iter()
                    .map(|video| VideoForInsert {
                        video: video.video,
                        recipe_id,
                        duration: video.duration.map(|duration| {
                            PgInterval::new(
                                duration.num_microseconds().unwrap_or_default(),
                                duration.num_days() as i32,
                                (duration.num_weeks() as f64 / 4.34524) as i32,
                            )
                        }),
                        content_url: video.content_url.clone(),
                        embed_url: video.embed_url.clone(),
                    })
                    .collect::<Vec<_>>(),
            )
            .execute(&mut conn)
            .await?;
    }

    Ok(())
}

pub(super) async fn update_category<C>(
    mut conn: &mut C,
    user_id: i64,
    recipe_id: i64,
    category: &Option<String>,
) -> Result<()>
where
    C: AsyncConnection<Backend = diesel::pg::Pg>,
{
    let category_id = get_category_id(conn, category).await?;

    diesel::update(schema::categories_recipes::table)
        .filter(schema::categories_recipes::recipe_id.eq(recipe_id))
        .set(schema::categories_recipes::category_id.eq(category_id))
        .execute(&mut conn)
        .await?;

    diesel::insert_into(schema::users_categories::table)
        .values((
            schema::users_categories::user_id.eq(user_id),
            schema::users_categories::category_id.eq(category_id),
        ))
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await?;

    Ok(())
}
