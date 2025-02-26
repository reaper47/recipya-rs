use std::collections::{BTreeSet, HashMap};

use diesel::data_types::PgInterval;
use diesel::prelude::*;
use diesel::upsert::excluded;
use diesel_async::{AsyncConnection, RunQueryDsl};

use super::structs::*;
use crate::core::model::{Error, Result};
use crate::core::repository::ModelManager;

impl Recipe {
    /// Creates a new recipe in the database for a given user.
    ///
    /// # Returns
    ///
    /// Returns `Ok(i64)` with the ID of the newly created recipe on success, or an `Err(Error)`
    /// if the operation fails.
    ///
    /// # Notes
    ///
    /// - If images are provided, the first image is treated as the main image, and the rest are
    ///   stored as additional images.
    /// - The detected language of the recipe is used for further processing and localization.
    pub async fn create(
        mm: &ModelManager,
        user_id: i64,
        recipe_c: &RecipeForCreate,
    ) -> Result<i64> {
        use crate::core::repository::schema;

        let mut conn = mm.pool.get().await.unwrap();

        let recipe_id = conn
            .transaction::<i64, Error, _>(|mut conn| {
                Box::pin(async move {
                    let language = whatlang::detect_lang(
                        &[
                            recipe_c.name.as_str(),
                            recipe_c.description.as_deref().unwrap_or(""),
                            &recipe_c
                                .ingredients
                                .iter()
                                .chain(&recipe_c.instructions)
                                .flat_map(|(_, items)| items)
                                .cloned()
                                .collect::<Vec<_>>()
                                .join(" "),
                        ]
                        .join(" "),
                    )
                    .unwrap_or(whatlang::Lang::Eng);

                    // Images
                    let (main_image, additional_images) = match recipe_c.images.as_deref() {
                        Some([first, rest @ ..]) => (Some(*first), rest.to_vec()),
                        _ => (None, Vec::new()),
                    };

                    let recipe_id = diesel::insert_into(schema::recipes::table)
                        .values(&RecipeForInsert {
                            name: recipe_c.name.to_string(),
                            description: recipe_c.description.clone(),
                            image: main_image,
                            yield_: recipe_c.yield_,
                            language: language.code().to_string(),
                            source: recipe_c.source.clone(),
                            user_id,
                        })
                        .returning(schema::recipes::id)
                        .get_result::<i64>(&mut conn)
                        .await
                        .map_err(|_| Error::DuplicateEntity)?;

                    // Category
                    let category_id = diesel::insert_into(schema::categories::table)
                        .values(&CategoryForInsert {
                            name: recipe_c.category.clone(),
                        })
                        .on_conflict(schema::categories::name)
                        .do_update()
                        .set(
                            schema::categories::name.eq(recipe_c
                                .category
                                .clone()
                                .unwrap_or(String::from("uncategorized"))),
                        )
                        .returning(schema::categories::id)
                        .get_result::<i64>(&mut conn)
                        .await?;

                    diesel::insert_into(schema::categories_recipes::table)
                        .values(&CategoryRecipe {
                            category_id,
                            recipe_id,
                        })
                        .execute(&mut conn)
                        .await?;

                    // Cuisine
                    if let Some(cuisine) = recipe_c.cuisine.as_ref() {
                        let cuisine_id = diesel::insert_into(schema::cuisines::table)
                            .values(&CuisineForInsert {
                                name: Some(cuisine.clone()),
                            })
                            .on_conflict(schema::cuisines::name)
                            .do_update()
                            .set(schema::cuisines::name.eq(excluded(schema::cuisines::name)))
                            .returning(schema::cuisines::id)
                            .get_result::<i64>(&mut conn)
                            .await?;

                        diesel::insert_into(schema::cuisines_recipes::table)
                            .values(&CuisineRecipe {
                                cuisine_id,
                                recipe_id,
                            })
                            .execute(&mut conn)
                            .await?;
                    }

                    // Sections for ingredients and instructions
                    let values: Vec<SectionForInsert> = recipe_c
                        .ingredients
                        .iter()
                        .chain(recipe_c.instructions.iter())
                        .map(|(name, _)| name.clone())
                        .collect::<BTreeSet<_>>()
                        .into_iter()
                        .rev()
                        .map(|name| SectionForInsert { name })
                        .collect();

                    let sections_map: HashMap<String, i64> =
                        diesel::insert_into(schema::sections::table)
                            .values(&values)
                            .on_conflict(schema::sections::name)
                            .do_update()
                            .set(schema::sections::name.eq(excluded(schema::sections::name)))
                            .returning((schema::sections::id, schema::sections::name))
                            .get_results::<(i64, String)>(&mut conn)
                            .await?
                            .into_iter()
                            .map(|(id, name)| (name, id))
                            .collect();

                    // Ingredients
                    for (section, ingredients) in recipe_c.ingredients.iter() {
                        let ingredient_recipes: Vec<_> =
                            diesel::insert_into(schema::ingredients::table)
                                .values(
                                    ingredients
                                        .iter()
                                        .map(|name| IngredientForInsert {
                                            name: name.to_string(),
                                        })
                                        .collect::<Vec<_>>(),
                                )
                                .on_conflict(schema::ingredients::name)
                                .do_update()
                                .set(
                                    schema::ingredients::name
                                        .eq(excluded(schema::ingredients::name)),
                                )
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

                    // Instructions
                    for (section, instructions) in recipe_c.instructions.iter() {
                        let instruction_recipes: Vec<InstructionRecipeForInsert> =
                            diesel::insert_into(schema::instructions::table)
                                .values(
                                    instructions
                                        .iter()
                                        .map(|name| InstructionForInsert {
                                            name: String::from(name),
                                        })
                                        .collect::<Vec<_>>(),
                                )
                                .on_conflict(schema::instructions::name)
                                .do_update()
                                .set(
                                    schema::instructions::name
                                        .eq(excluded(schema::instructions::name)),
                                )
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

                    // Keywords
                    for keyword in recipe_c.keywords.iter() {
                        let keyword_id = diesel::insert_into(schema::keywords::table)
                            .values(&KeywordForInsert {
                                name: Some(keyword.clone()),
                            })
                            .on_conflict(schema::keywords::name)
                            .do_update()
                            .set(schema::keywords::name.eq(keyword.clone()))
                            .returning(schema::keywords::id)
                            .get_result(&mut conn)
                            .await?;

                        diesel::insert_into(schema::keywords_recipes::table)
                            .values(&KeywordRecipe {
                                keyword_id,
                                recipe_id,
                            })
                            .execute(&mut conn)
                            .await?;
                    }

                    // Nutrition
                    if let Some(nutrition) = &recipe_c.nutrition {
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
                    }

                    // Times
                    let times = recipe_c.times.clone().unwrap_or_else(|| TimesForCreate {
                        prep_seconds: 15 * 60,
                        cook_seconds: 0,
                    });

                    diesel::insert_into(schema::times::table)
                        .values(&TimesForInsert {
                            recipe_id,
                            prep_seconds: times.prep_seconds,
                            cook_seconds: times.cook_seconds,
                        })
                        .execute(&mut conn)
                        .await?;

                    // Tools
                    if !recipe_c.tools.is_empty() {
                        let tool_recipes: Vec<_> = diesel::insert_into(schema::tools::table)
                            .values(
                                recipe_c
                                    .tools
                                    .iter()
                                    .map(|tool| ToolForInsert {
                                        name: tool.name.as_str().into(),
                                    })
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
                                quantity: recipe_c.tools.get(idx).map_or(1, |x| x.quantity),
                                tool_order: idx as i16,
                            })
                            .collect::<Vec<_>>();

                        diesel::insert_into(schema::tools_recipes::table)
                            .values(&tool_recipes)
                            .execute(&mut conn)
                            .await?;
                    }

                    // Additional Images
                    for image in additional_images.into_iter() {
                        diesel::insert_into(schema::additional_images_recipe::table)
                            .values(&AdditionalImageForInsert { recipe_id, image })
                            .execute(&mut conn)
                            .await?;
                    }

                    // Videos
                    if !recipe_c.videos.is_empty() {
                        diesel::insert_into(schema::videos_recipes::table)
                            .values(
                                &recipe_c
                                    .videos
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

                    Ok(recipe_id)
                })
            })
            .await?;

        diesel::insert_into(schema::users_recipes::table)
            .values((
                schema::users_recipes::user_id.eq(user_id),
                schema::users_recipes::recipe_id.eq(recipe_id),
            ))
            .execute(&mut conn)
            .await?;

        Ok(recipe_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::server::test_utils::{a_complete_recipe, insert_user, TestDb};
    use crate::server::AppState;

    pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    fn a_bare_minimum_recipe() -> RecipeForCreate {
        RecipeForCreate {
            name: String::from("Best Chinese Kale"),
            description: None,
            images: None,
            yield_: Some(4),
            source: None,
            videos: vec![],
            category: Some(String::from("uncategorized")),
            cuisine: None,
            ingredients: Sections::from([
                (
                    String::from("Sauce"),
                    vec![
                        String::from("1 cup blue spinach"),
                        String::from("1/2 tbsp cinnamon"),
                    ],
                ),
                (
                    String::from("Main"),
                    vec![
                        String::from("4 pounds top quality chicken filet"),
                        String::from("1/8 cup lemon juice"),
                    ],
                ),
            ]),
            instructions: Sections::from([
                (
                    String::from("Sauce"),
                    vec![String::from("Mix all these ingredients")],
                ),
                (
                    String::from("Chicken"),
                    vec![
                        String::from("Turn the oven at 300 F"),
                        String::from("Soak the chicken in the lemon juice"),
                        String::from("Bake for 35 minutes"),
                    ],
                ),
            ]),
            keywords: vec![],
            nutrition: None,
            times: None,
            tools: vec![],
        }
    }

    fn recipe_for_create_to_recipe_with_data(
        recipe_id: i64,
        recipe: RecipeForCreate,
        got: &RecipeDetails,
    ) -> RecipeDetails {
        let (main_image, additional_images) = match recipe.images.as_deref() {
            Some([first, rest @ ..]) => (Some(*first), rest.to_vec()),
            _ => (None, Vec::new()),
        };

        let times = recipe.times.unwrap_or(TimesForCreate {
            prep_seconds: 15 * 60,
            cook_seconds: 0,
        });

        let nutrition = match recipe.nutrition {
            None => None,
            Some(n) => Some(Nutrition {
                id: recipe_id,
                recipe_id,
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
                serving_size: n.serving_size,
            }),
        };

        RecipeDetails {
            recipe: Recipe {
                id: recipe_id,
                name: recipe.name,
                description: recipe.description,
                image: main_image,
                yield_: recipe.yield_.unwrap_or(4),
                language: String::from("eng"),
                source: recipe.source,
                user_id: 1,
                created_at: got.recipe.created_at,
                updated_at: got.recipe.updated_at,
            },
            additional_images,
            category: recipe.category.unwrap_or(String::from("uncategorized")),
            cuisine: recipe.cuisine,
            ingredients: recipe.ingredients,
            instructions: recipe.instructions,
            keywords: recipe.keywords,
            nutrition,
            times: Times {
                id: recipe_id,
                recipe_id,
                prep_seconds: times.prep_seconds,
                cook_seconds: times.cook_seconds,
                total_seconds: times.prep_seconds + times.cook_seconds,
            },
            tools: recipe
                .tools
                .into_iter()
                .enumerate()
                .map(|(idx, t)| ToolRecipe {
                    name: t.name,
                    quantity: t.quantity,
                    tool_order: (idx + 1) as i16,
                })
                .collect::<Vec<_>>(),
            videos: recipe
                .videos
                .into_iter()
                .enumerate()
                .map(|(idx, v)| Video {
                    video: v.video,
                    duration: v.duration,
                    content_url: v.content_url,
                    embed_url: v.embed_url,
                    created_at: got
                        .videos
                        .get(idx)
                        .unwrap_or(&Video {
                            video: Default::default(),
                            duration: None,
                            content_url: None,
                            embed_url: None,
                            created_at: Default::default(),
                        })
                        .created_at,
                })
                .collect::<Vec<_>>(),
        }
    }

    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = AppState::new(config.clone()).await?;
        let user = insert_user(config.clone()).await?;
        let recipe = a_complete_recipe();

        let got_recipe_id = Recipe::create(&state.mm, user.id, &recipe).await?;

        let got = Recipe::get(&state.mm, user.id, got_recipe_id).await?;
        let want = recipe_for_create_to_recipe_with_data(got_recipe_id, recipe, &got);
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    async fn test_create_duplicates_err() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = AppState::new(config.clone()).await?;
        let user = insert_user(config.clone()).await?;
        let mut recipe = a_complete_recipe();
        let _ = Recipe::create(&state.mm, user.id, &recipe).await?;
        recipe.name = String::from("Duplicate");

        let got_recipe_id = Recipe::create(&state.mm, user.id, &recipe).await?;

        let got = Recipe::get(&state.mm, user.id, got_recipe_id).await?;
        let want = recipe_for_create_to_recipe_with_data(got_recipe_id, recipe, &got);
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    async fn test_create_duplicate_name_err() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = AppState::new(config.clone()).await?;
        let user = insert_user(config.clone()).await?;
        let recipe = a_complete_recipe();
        let _ = Recipe::create(&state.mm, user.id, &recipe).await?;

        let got = Recipe::create(&state.mm, user.id, &recipe).await;

        pretty_assertions::assert_eq!(got, Err(Error::DuplicateEntity));
        Ok(())
    }

    #[tokio::test]
    async fn test_create_bare_minimum_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = AppState::new(config.clone()).await?;
        let user = insert_user(config.clone()).await?;
        let recipe = a_bare_minimum_recipe();

        let got_recipe_id = Recipe::create(&state.mm, user.id, &recipe).await?;

        let got = Recipe::get(&state.mm, user.id, got_recipe_id).await?;
        let want = recipe_for_create_to_recipe_with_data(got_recipe_id, recipe, &got);
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    async fn test_create_some_fields_are_lowercase_ok() -> Result<()> {
        Ok(())
    }
}
