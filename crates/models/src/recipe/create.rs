use diesel::prelude::*;
use diesel_async::{AsyncConnection, RunQueryDsl};

use repository::{ModelManager, schema};

use super::helpers::{
    get_category_id, get_cuisine_id, insert_additional_images, insert_ingredients,
    insert_instructions, insert_keywords, insert_nutrition, insert_sections, insert_tools,
    insert_videos,
};
use super::structs::*;
use crate::user::UserCategory;
use crate::{Error, Result};

impl Recipe {
    /// Adds a recipe category into the database.
    pub async fn add_category(mm: &ModelManager, category: &str, user_id: i64) -> Result<()> {
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
    pub async fn create(
        mm: &ModelManager,
        user_id: i64,
        recipe_c: &RecipeForCreate,
    ) -> Result<i64> {
        let mut conn = mm.pool.get().await?;

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
                    let (main_image, additional_images) = recipe_c.first_and_rest_images();

                    let recipe_id = diesel::insert_into(schema::recipes::table)
                        .values(&RecipeForInsert {
                            name: recipe_c.name.to_string(),
                            description: recipe_c.description.clone(),
                            image: main_image,
                            yield_: recipe_c.yield_,
                            language: language.code().to_string(),
                            source: recipe_c.source.clone(),
                            is_favourite: recipe_c.is_favourite,
                            user_id,
                        })
                        .returning(schema::recipes::id)
                        .get_result::<i64>(&mut conn)
                        .await
                        .map_err(|_| Error::DuplicateEntity)?;

                    // Additional Images
                    insert_additional_images(&mut conn, recipe_id, additional_images).await?;

                    // Category
                    let category_id = get_category_id(conn, &recipe_c.category).await?;

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
                    insert_ingredients(conn, &sections_map, &recipe_c.ingredients, recipe_id)
                        .await?;

                    // Instructions
                    insert_instructions(conn, &sections_map, &recipe_c.instructions, recipe_id)
                        .await?;

                    // Keywords
                    insert_keywords(&mut conn, &recipe_c.keywords, user_id, recipe_id).await?;

                    // Nutrition
                    if let Some(nutrition) = &recipe_c.nutrition {
                        insert_nutrition(conn, nutrition, recipe_id).await?;
                    }

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
    use self::test_utils::a_complete_recipe_for_create;
    use super::*;

    use app::state::AppState;
    use recipe_schema::Sections;
    use testing::utils::{TestDb, build_server_logged_in, create_app_state, insert_user};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_add_category {
        use super::*;

        #[tokio::test]
        async fn test_create_new_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let _ = build_server_logged_in(config.clone()).await?;
            let category = "fish";

            Recipe::add_category(&state.mm, category, 1).await?;

            assert_category(state, category).await?;
            Ok(())
        }

        #[tokio::test]
        async fn test_create_new_duplicate_err() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let _ = build_server_logged_in(config.clone()).await?;
            let category = "fish";
            Recipe::add_category(&state.mm, category, 1).await?;

            let res = Recipe::add_category(&state.mm, category, 1).await;

            match res {
                Ok(_) => panic!("Should not succeed"),
                Err(_) => {
                    assert_category(state, category).await?;
                    Ok(())
                }
            }
        }

        async fn assert_category(state: AppState, category: &str) -> Result<()> {
            let mut conn = state.mm.pool.get().await?;
            let category_id = schema::categories::table
                .filter(schema::categories::name.eq(category))
                .select(schema::categories::id)
                .first::<i64>(&mut conn)
                .await?;
            assert!(category_id > 0);

            let count = schema::users_categories::table
                .filter(schema::users_categories::category_id.eq(category_id))
                .count()
                .get_result::<i64>(&mut conn)
                .await?;
            pretty_assertions::assert_eq!(count, 1);
            Ok(())
        }
    }

    fn a_bare_minimum_recipe() -> RecipeForCreate {
        RecipeForCreate {
            name: "Best Chinese Kale".into(),
            description: None,
            images: vec![],
            yield_: Some(4),
            source: None,
            is_favourite: false,
            videos: vec![],
            category: Some("uncategorized".into()),
            cuisine: None,
            ingredients: Sections::from([
                (
                    "Sauce".into(),
                    Vec::<String>::from(["1 cup blue spinach".into(), "1/2 tbsp cinnamon".into()]),
                ),
                (
                    "Main".into(),
                    Vec::<String>::from([
                        "4 pounds top quality chicken filet".into(),
                        "1/8 cup lemon juice".into(),
                    ]),
                ),
            ]),
            instructions: Sections::from([
                (
                    "Sauce".into(),
                    Vec::<String>::from(["Mix all these ingredients".into()]),
                ),
                (
                    "Chicken".into(),
                    Vec::<String>::from([
                        "Turn the oven at 300 F".into(),
                        "Soak the chicken in the lemon juice".into(),
                        "Bake for 35 minutes".into(),
                    ]),
                ),
            ]),
            keywords: vec![],
            measurement_system_id: 2,
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
        let mut keywords = recipe.keywords.clone();
        keywords.sort();

        let (main_image, additional_images) = recipe.first_and_rest_images();

        let times = recipe.times.unwrap_or_default();

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
                language: "eng".into(),
                source: recipe.source,
                measurement_system_id: 2,
                user_id: 1,
                created_at: got.recipe.created_at,
                updated_at: got.recipe.updated_at,
                is_favourite: false,
            },
            additional_images,
            category: recipe.category.unwrap_or("uncategorized".into()),
            cuisine: recipe.cuisine,
            ingredients: recipe.ingredients,
            instructions: recipe.instructions,
            keywords,
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
        let state = create_app_state(config.clone()).await;
        let user = insert_user(config.clone()).await?;
        let recipe = a_complete_recipe_for_create();

        let got_recipe_id = Recipe::create(&state.mm, user.id, &recipe).await?;

        let got = Recipe::get(&state.mm, user.id, got_recipe_id).await?;
        let want = recipe_for_create_to_recipe_with_data(got_recipe_id, recipe, &got);
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    async fn test_create_duplicates_err() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let user = insert_user(config.clone()).await?;
        let mut recipe = a_complete_recipe_for_create();
        let _ = Recipe::create(&state.mm, user.id, &recipe).await?;
        recipe.name = "Duplicate".into();

        let got_recipe_id = Recipe::create(&state.mm, user.id, &recipe).await?;

        let got = Recipe::get(&state.mm, user.id, got_recipe_id).await?;
        let want = recipe_for_create_to_recipe_with_data(got_recipe_id, recipe, &got);
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    async fn test_create_duplicate_name_err() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let user = insert_user(config.clone()).await?;
        let recipe = a_complete_recipe_for_create();
        let _ = Recipe::create(&state.mm, user.id, &recipe).await?;

        let got = Recipe::create(&state.mm, user.id, &recipe).await;

        match got {
            Ok(_) => Err("Should have returned an error".into()),
            Err(Error::DuplicateEntity) => Ok(()),
            Err(err) => Err(format!("Wrong error occurred: {err}").into()),
        }
    }

    #[tokio::test]
    async fn test_create_bare_minimum_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
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
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let user = insert_user(config.clone()).await?;
        let mut recipe = a_bare_minimum_recipe();
        recipe.keywords = vec!["CHICKEN".into(), "MEAT".into()];
        recipe.category = Some("KVELDSMAT".into());
        recipe.cuisine = Some("NORWEGIAN".into());
        recipe.tools = vec![
            ToolForCreate {
                name: "FRYING PAN".into(),
                quantity: 1,
            },
            ToolForCreate {
                name: "WOK".into(),
                quantity: 1,
            },
        ];

        let got_recipe_id = Recipe::create(&state.mm, user.id, &recipe).await?;

        let got = Recipe::get(&state.mm, user.id, got_recipe_id).await?;
        pretty_assertions::assert_eq!(
            got.keywords,
            vec!["chicken".to_string(), "meat".to_string()]
        );
        pretty_assertions::assert_eq!(got.category, "kveldsmat".to_string());
        pretty_assertions::assert_eq!(got.cuisine, Some("norwegian".to_string()));
        pretty_assertions::assert_eq!(
            got.tools,
            vec![
                ToolRecipe {
                    name: "frying pan".to_string(),
                    quantity: 1,
                    tool_order: 1
                },
                ToolRecipe {
                    name: "wok".to_string(),
                    quantity: 1,
                    tool_order: 2
                }
            ]
        );
        Ok(())
    }
}
