use std::collections::{BTreeSet, HashMap, HashSet};
use std::env::temp_dir;
use std::path::PathBuf;

use axum::extract::multipart::{Field, InvalidBoundary};
use diesel::data_types::PgInterval;
use diesel::prelude::*;
use diesel::upsert::excluded;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use repository::schema;
use support::regexp::time::TimeParser;
use support::strings::normalise_vulgar_fractions;
use uuid::Uuid;

use crate::Result;
use crate::nutrition::NutritionDataSource;
use crate::recipe::structs::media::{AdditionalImageForInsert, VideoForCreate, VideoForInsert};
use crate::recipe::structs::nutrition::{
    NutritionDetailsForCreate, NutritionForInsert, NutritionPer100gForInsert,
    NutritionPerServingForInsert,
};
use crate::recipe::structs::recipe::{
    CategoryForInsert, CuisineForInsert, IngredientForInsert, IngredientRecipeForInsert,
    InstructionForInsert, InstructionRecipeForInsert, KeywordForInsert, KeywordRecipe,
    RecipeForCreate,
};
use crate::recipe::structs::section::{SectionComponents, SectionForInsert};
use crate::recipe::structs::tool::{ToolForCreate, ToolForInsert, ToolRecipeForInsert};
use crate::user::UserKeyword;

pub(crate) async fn get_category_id<C>(mut conn: &mut C, category: &Option<String>) -> Result<i64>
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

pub(crate) async fn get_cuisine_id<C>(mut conn: &mut C, cuisine: String) -> Result<i64>
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

pub(crate) async fn insert_additional_images<C>(
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

pub(crate) async fn insert_ingredients<C>(
    conn: &mut C,
    sections_map: &HashMap<String, i64>,
    ingredients: &SectionComponents,
    recipe_id: i64,
) -> Result<()>
where
    C: AsyncConnection<Backend = diesel::pg::Pg>,
{
    // Collect all ingredients
    let mut all_ingredients = Vec::new();

    match ingredients {
        SectionComponents::Grouped(sections) => {
            for section in sections.iter() {
                let id = *sections_map.get(&section.title).unwrap_or(&1);
                for (idx, item) in section.items.iter().enumerate() {
                    all_ingredients.push((normalise_vulgar_fractions(&item.text), id, idx))
                }
            }
        }
        SectionComponents::Flat(items) => {
            for (idx, item) in items.iter().enumerate() {
                all_ingredients.push((normalise_vulgar_fractions(&item.text), 1, idx))
            }
        }
    }

    if all_ingredients.is_empty() {
        return Ok(());
    }

    // Remove duplicates
    let mut seen = HashSet::new();
    let mut unique_ingredients = Vec::new();

    for (name, section_id, idx) in all_ingredients {
        if seen.insert(name.clone()) {
            unique_ingredients.push((name, section_id, idx));
        }
    }

    let ingredient_names = unique_ingredients
        .iter()
        .map(|(name, _, _)| name.clone())
        .collect::<Vec<_>>();

    // Insert into ingredients table
    let ingredients_with_ids: Vec<(i64, String)> = diesel::insert_into(schema::ingredients::table)
        .values(
            &ingredient_names
                .iter()
                .map(|s| IngredientForInsert { name: s.clone() })
                .collect::<Vec<_>>(),
        )
        .on_conflict(schema::ingredients::name)
        .do_update()
        .set(schema::ingredients::name.eq(excluded(schema::ingredients::name)))
        .returning((schema::ingredients::id, schema::ingredients::name))
        .get_results(conn)
        .await?;

    // Insert into junctions table
    let name_to_id: HashMap<String, i64> = ingredients_with_ids
        .into_iter()
        .map(|(id, name)| (name, id))
        .collect();

    diesel::insert_into(schema::ingredients_recipes::table)
        .values(
            &unique_ingredients
                .iter()
                .filter_map(|(name, section_id, item_order)| {
                    name_to_id
                        .get(name)
                        .map(|&ingredient_id| IngredientRecipeForInsert {
                            ingredient_id,
                            recipe_id,
                            section_id: *section_id,
                            item_order: *item_order as i16,
                        })
                })
                .collect::<Vec<_>>(),
        )
        .execute(conn)
        .await?;

    Ok(())
}

pub(crate) async fn insert_instructions<C>(
    conn: &mut C,
    sections_map: &HashMap<String, i64>,
    instructions: &SectionComponents,
    recipe_id: i64,
) -> Result<()>
where
    C: AsyncConnection<Backend = diesel::pg::Pg>,
{
    let time_parser = TimeParser::new();
    let mut all_instructions = Vec::new();

    match instructions {
        SectionComponents::Grouped(sections) => {
            for section in sections.iter() {
                let section_id = *sections_map.get(&section.title).unwrap_or(&1);
                for (idx, item) in section.items.iter().enumerate() {
                    let duration = time_parser.parse_max_time_seconds(&item.text);
                    all_instructions.push((item.text.clone(), duration, section_id, idx));
                }
            }
        }
        SectionComponents::Flat(items) => {
            for (idx, item) in items.iter().enumerate() {
                let duration = time_parser.parse_max_time_seconds(&item.text);
                all_instructions.push((item.text.clone(), duration, 1, idx));
            }
        }
    }

    // Remove duplicates
    let mut seen = HashSet::new();
    let mut unique_instructions = Vec::new();

    for (name, duration, section_id, idx) in all_instructions {
        if seen.insert(name.clone()) {
            unique_instructions.push((name, duration, section_id, idx));
        }
    }

    // Insert into instructions table
    let instruction_ids: Vec<(i64, String)> = diesel::insert_into(schema::instructions::table)
        .values(
            &unique_instructions
                .iter()
                .map(|(name, duration, _, _)| InstructionForInsert {
                    name: name.clone(),
                    duration_seconds: *duration,
                })
                .collect::<Vec<_>>(),
        )
        .on_conflict(schema::instructions::name)
        .do_update()
        .set(schema::instructions::name.eq(excluded(schema::instructions::name)))
        .returning((schema::instructions::id, schema::instructions::name))
        .get_results(conn)
        .await?;

    let name_to_id: HashMap<String, i64> = instruction_ids
        .into_iter()
        .map(|(id, name)| (name, id))
        .collect();

    // Insert into junction table
    diesel::insert_into(schema::instructions_recipes::table)
        .values(
            &unique_instructions
                .iter()
                .filter_map(|(name, _, section_id, item_order)| {
                    name_to_id
                        .get(name)
                        .map(|&instruction_id| InstructionRecipeForInsert {
                            instruction_id,
                            recipe_id,
                            section_id: *section_id,
                            item_order: *item_order as i16,
                        })
                })
                .collect::<Vec<_>>(),
        )
        .execute(conn)
        .await?;

    Ok(())
}

pub(crate) async fn insert_keywords<C>(
    mut conn: &mut C,
    keywords: &[String],
    user_id: Uuid,
    recipe_id: i64,
) -> Result<()>
where
    C: AsyncConnection<Backend = diesel::pg::Pg>,
{
    if keywords.is_empty() {
        return Ok(());
    }

    let mut keywords = Vec::from(keywords);
    keywords.sort();
    keywords.dedup();

    let keyword_ids: Vec<i64> = diesel::insert_into(schema::keywords::table)
        .values(
            &keywords
                .iter()
                .map(|kw| KeywordForInsert {
                    name: Some(kw.to_lowercase()),
                })
                .collect::<Vec<_>>(),
        )
        .on_conflict(schema::keywords::name)
        .do_update()
        .set(schema::keywords::name.eq(excluded(schema::keywords::name)))
        .returning(schema::keywords::id)
        .get_results(&mut conn)
        .await?;

    diesel::insert_into(schema::users_keywords::table)
        .values(
            &keyword_ids
                .iter()
                .map(|&id| UserKeyword {
                    user_id,
                    keyword_id: id,
                })
                .collect::<Vec<_>>(),
        )
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await?;

    diesel::insert_into(schema::keywords_recipes::table)
        .values(
            &keyword_ids
                .iter()
                .map(|&id| KeywordRecipe {
                    keyword_id: id,
                    recipe_id,
                })
                .collect::<Vec<_>>(),
        )
        .execute(&mut conn)
        .await?;

    Ok(())
}

pub(crate) async fn insert_nutrition(
    conn: &mut AsyncPgConnection,
    recipe_id: i64,
    nutrition: &NutritionDetailsForCreate,
    ingredients: &[&str],
    nutrition_source: NutritionDataSource,
    num_servings: i16,
) -> Result<()> {
    let per_100g = &nutrition.per_100g;
    let per_serving = &nutrition.per_serving;

    let calculated_nutrition = if per_100g.is_some() || per_serving.is_some() {
        None
    } else {
        nutrition_source
            .calculate_nutrition(conn, ingredients, num_servings)
            .await
            .ok()
    };

    let nutrition_per_100g = match per_100g {
        Some(n) if !n.is_empty() => Some(NutritionForInsert {
            is_precalculated_by_source: true,
            calories_kcal: n.calories_kcal,
            total_carbohydrates: n.total_carbohydrates,
            sugars_g: n.sugars_g,
            protein_g: n.protein_g,
            total_fat_g: n.total_fat_g,
            saturated_fat_g: n.saturated_fat_g,
            unsaturated_fat_g: n.unsaturated_fat_g,
            cholesterol_mg: n.cholesterol_mg,
            sodium_mg: n.sodium_mg,
            fiber_g: n.fiber_g,
            trans_fat_g: n.trans_fat_g,
        }),
        Some(_) | None => calculated_nutrition
            .as_ref()
            .map(|n| NutritionForInsert::from(&n.per_100g)),
    };

    let nutrition_per_serving = match per_serving {
        Some(n) if !n.nutrition.is_empty() => Some(NutritionForInsert {
            is_precalculated_by_source: true,
            calories_kcal: n.nutrition.calories_kcal,
            total_carbohydrates: n.nutrition.total_carbohydrates,
            sugars_g: n.nutrition.sugars_g,
            protein_g: n.nutrition.protein_g,
            total_fat_g: n.nutrition.total_fat_g,
            saturated_fat_g: n.nutrition.saturated_fat_g,
            unsaturated_fat_g: n.nutrition.unsaturated_fat_g,
            cholesterol_mg: n.nutrition.cholesterol_mg,
            sodium_mg: n.nutrition.sodium_mg,
            fiber_g: n.nutrition.fiber_g,
            trans_fat_g: n.nutrition.trans_fat_g,
        }),
        Some(_) | None => calculated_nutrition
            .as_ref()
            .map(|n| NutritionForInsert::from(&n.per_serving)),
    };

    diesel::delete(schema::nutrition_per_100g::table)
        .filter(schema::nutrition_per_100g::recipe_id.eq(recipe_id))
        .execute(conn)
        .await?;

    diesel::delete(schema::nutrition_per_serving::table)
        .filter(schema::nutrition_per_serving::recipe_id.eq(recipe_id))
        .execute(conn)
        .await?;

    if let Some(n) = nutrition_per_100g {
        diesel::insert_into(schema::nutrition_per_100g::table)
            .values(&NutritionPer100gForInsert {
                recipe_id,
                nutrition_id: diesel::insert_into(schema::nutrition::table)
                    .values(&n)
                    .returning(schema::nutrition::id)
                    .get_result::<i64>(conn)
                    .await?,
            })
            .execute(conn)
            .await?;
    }

    if let Some(n) = nutrition_per_serving {
        diesel::insert_into(schema::nutrition_per_serving::table)
            .values(&NutritionPerServingForInsert {
                recipe_id,
                nutrition_id: diesel::insert_into(schema::nutrition::table)
                    .values(&n)
                    .returning(schema::nutrition::id)
                    .get_result::<i64>(conn)
                    .await?,
                serving_size: nutrition
                    .per_serving
                    .as_ref()
                    .map(|n| n.serving_size.clone())
                    .unwrap_or_default(),
            })
            .execute(conn)
            .await?;
    }

    Ok(())
}

pub(crate) async fn insert_sections<C>(
    mut conn: &mut C,
    recipe_c: &RecipeForCreate,
) -> Result<HashMap<String, i64>>
where
    C: AsyncConnection<Backend = diesel::pg::Pg>,
{
    let sections_for_insert = recipe_c
        .ingredients
        .titles()
        .chain(recipe_c.instructions.titles())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .rev()
        .map(|name| SectionForInsert {
            name: name.to_string(),
        })
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

pub(crate) async fn insert_tools<C>(
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

pub(crate) async fn insert_videos<C>(
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

pub async fn save_media_field(
    field: Field<'_>,
    images: &mut HashMap<String, PathBuf>,
    videos: &mut HashMap<String, PathBuf>,
) -> std::result::Result<(), InvalidBoundary> {
    let filename = Uuid::new_v4();

    let bytes = field.bytes().await.unwrap_or_default();
    if bytes.is_empty() {
        return Ok(());
    }

    let mime = infer::get(&bytes)
        .map(|k| k.mime_type())
        .unwrap_or("application/octet-stream");

    let path = temp_dir().join(filename.to_string());
    tokio::fs::write(&path, &bytes)
        .await
        .map_err(|_| InvalidBoundary::default())?;

    if mime.starts_with("video/") {
        videos.insert(filename.to_string(), path.clone());
    } else {
        images.insert(filename.to_string(), path.clone());
    }
    Ok(())
}

pub async fn text_trim(field: Field<'_>) -> Option<String> {
    match field.text().await {
        Ok(s) => {
            let s = s.trim();
            if s.is_empty() {
                None
            } else {
                Some(s.to_owned())
            }
        }
        Err(_) => None,
    }
}

pub(crate) async fn update_category<C>(
    mut conn: &mut C,
    user_id: Uuid,
    recipe_id: i64,
    category: &Option<String>,
) -> Result<()>
where
    C: AsyncConnection<Backend = diesel::pg::Pg>,
{
    let category_id = get_category_id(conn, category).await?;

    diesel::update(
        schema::categories_recipes::table
            .filter(schema::categories_recipes::recipe_id.eq(recipe_id)),
    )
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
