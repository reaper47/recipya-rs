#[cfg(test)]
mod tests {
    use axum::http::HeaderValue;
    use axum::http::Method;
    use axum_test::TestResponse;
    use uuid::Uuid;

    use models::recipe::structs::{
        media::VideoForCreate,
        nutrition::{
            Nutrition, NutritionDetails, NutritionDetailsForCreate, NutritionForCreate,
            NutritionPerServingDetails, NutritionPerServingDetailsForCreate,
        },
        recipe::RecipeForCreate,
        section::{Item, SectionComponents, SectionItem},
        time::{Times, TimesForCreate},
        tool::{ToolForCreate, ToolRecipe},
        types::Source,
    };
    use models::user::User;
    use models::{Recipe, RecipeDetails};
    use test_db::default_config;
    use test_utils::{assert_must_be_logged_in, build_server_logged_in};

    use crate::recipes_router::tests::helpers::create_form;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    const BASE_URI: &str = "/recipes/add/manual";

    #[tokio::test]
    async fn test_must_be_logged_in_ok() -> Result<()> {
        let _ = assert_must_be_logged_in(Method::GET, BASE_URI).await;
        assert_must_be_logged_in(Method::POST, BASE_URI).await
    }

    mod tests_get {
        use super::*;

        #[tokio::test]
        async fn test_logged_in_htmx_ok() -> Result<()> {
            let (mut server, _) = build_server_logged_in(default_config()).await?;
            server.add_header(axum_htmx::HX_REQUEST, HeaderValue::from_static("true"));

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_recipe_form(&res);
            Ok(())
        }

        #[tokio::test]
        async fn test_logged_in_ok() -> Result<()> {
            let (server, _) = build_server_logged_in(default_config()).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_recipe_form(&res);
            Ok(())
        }
    }

    mod tests_post {
        use reqwest::StatusCode;
        use time::Duration;

        use super::*;

        #[tokio::test]
        async fn test_missing_required_title_ok() -> Result<()> {
            let (server, _) = build_server_logged_in(default_config()).await?;
            let recipe = RecipeForCreate {
                ingredients: SectionComponents::Flat(vec![Item::new("1 apple")]),
                instructions: SectionComponents::Flat(vec![Item::new("Mix the apples")]),
                ..Default::default()
            };
            let form = create_form(&recipe);

            let res = server.post(BASE_URI).multipart(form).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_missing_required_ingredients_ok() -> Result<()> {
            let (server, _) = build_server_logged_in(default_config()).await?;
            let recipe = RecipeForCreate {
                name: "Best Chinese Kale".to_string(),
                instructions: SectionComponents::Flat(vec![Item::new("Mix the apples")]),
                ..Default::default()
            };
            let form = create_form(&recipe);

            let res = server.post(BASE_URI).multipart(form).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_missing_required_instructions_ok() -> Result<()> {
            let (server, _) = build_server_logged_in(default_config()).await?;
            let recipe = RecipeForCreate {
                name: "Best Chinese Kale".to_string(),
                ingredients: SectionComponents::Flat(vec![Item::new("8 apples")]),
                ..Default::default()
            };
            let form = create_form(&recipe);

            let res = server.post(BASE_URI).multipart(form).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_missing_fields_defaults_ok() -> Result<()> {
            let (server, state) = build_server_logged_in(default_config()).await?;
            let recipe = RecipeForCreate {
                name: "Best Chinese Kale".to_string(),
                instructions: SectionComponents::Flat(vec![Item::new("Mix the apples")]),
                ingredients: SectionComponents::Flat(vec![Item::new("8 apples")]),
                ..Default::default()
            };
            let form = create_form(&recipe);

            let res = server.post(BASE_URI).multipart(form).await;

            res.assert_status(StatusCode::CREATED);
            let users = User::all(&state.mm).await?;
            let user_id = users[0].id;
            let recipe_id = Recipe::all(&state.mm, user_id).await?.last().unwrap().id;
            let got = Recipe::get(&state.mm, user_id, recipe_id).await?;
            pretty_assertions::assert_eq!(got.category, "uncategorized");
            pretty_assertions::assert_eq!(got.recipe.r#yield, 1);
            Ok(())
        }

        #[tokio::test]
        async fn test_can_only_be_one_category_ok() -> Result<()> {
            let (server, state) = build_server_logged_in(default_config()).await?;
            let recipe = RecipeForCreate {
                name: "Best Chinese Kale".to_string(),
                instructions: SectionComponents::Flat(vec![Item::new("Mix the apples")]),
                ingredients: SectionComponents::Flat(vec![Item::new("8 apples")]),
                category: Some("breakfast,dinner".into()),
                ..Default::default()
            };
            let form = create_form(&recipe);

            let res = server.post(BASE_URI).multipart(form).await;

            res.assert_status(StatusCode::CREATED);
            let users = User::all(&state.mm).await?;
            let user_id = users[0].id;
            let recipe_id = Recipe::all(&state.mm, user_id).await?.last().unwrap().id;
            let mut got = Recipe::get(&state.mm, user_id, recipe_id).await?;
            if let SectionComponents::Flat(instructions) = &mut got.instructions {
                for i in instructions.iter_mut() { i.id = None; }
            }
            pretty_assertions::assert_eq!(got.category, "breakfast");
            Ok(())
        }

        #[tokio::test]
        async fn test_duplicates_are_removed_ok() -> Result<()> {
            let (server, state) = build_server_logged_in(default_config()).await?;
            let recipe = RecipeForCreate {
                name: "Best Chinese Kale".to_string(),
                instructions: SectionComponents::Flat(vec![
                    Item::new("Mix the apples"),
                    Item::new("Eat"),
                    Item::new("Mix the apples"),
                ]),
                ingredients: SectionComponents::Flat(vec![
                    Item::new("8 apples"),
                    Item::new("4 oranges"),
                    Item::new("8 apples"),
                ]),
                keywords: vec!["drinks".into(), "vodka".into(), "drinks".into()],
                tools: vec![
                    ToolForCreate {
                        quantity: 1,
                        name: "1 wok".into(),
                    },
                    ToolForCreate {
                        quantity: 1,
                        name: "1 wok".into(),
                    },
                ],
                ..Default::default()
            };
            let form = create_form(&recipe);

            let res = server.post(BASE_URI).multipart(form).await;

            res.assert_status(StatusCode::CREATED);
            let users = User::all(&state.mm).await?;
            let user_id = users[0].id;
            let recipe_id = Recipe::all(&state.mm, user_id).await?.last().unwrap().id;
            let mut got = Recipe::get(&state.mm, user_id, recipe_id).await?;
            if let SectionComponents::Flat(instructions) = &mut got.instructions {
                for i in instructions.iter_mut() { i.id = None; }
            }
            pretty_assertions::assert_eq!(
                got.keywords,
                vec!["drinks".to_string(), "vodka".to_string()]
            );
            pretty_assertions::assert_eq!(
                got.ingredients,
                SectionComponents::Flat(vec![Item::new("8 apples"), Item::new("4 oranges")]),
            );
            pretty_assertions::assert_eq!(
                got.instructions,
                SectionComponents::Flat(vec![
                    Item::new("Mix the apples"),
                    Item::new("Eat"),
                ]),
            );
            pretty_assertions::assert_eq!(
                got.tools,
                vec![ToolRecipe {
                    name: "wok".into(),
                    tool_order: 1,
                    quantity: 1,
                }]
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_manual_subcategories_are_possible() -> Result<()> {
            let (server, state) = build_server_logged_in(default_config()).await?;
            let recipe = RecipeForCreate {
                name: "Best Chinese Kale".to_string(),
                instructions: SectionComponents::Flat(vec![Item::new("Mix the apples")]),
                ingredients: SectionComponents::Flat(vec![Item::new("8 apples")]),
                category: Some("drinks:vodka".into()),
                ..Default::default()
            };
            let form = create_form(&recipe);

            let res = server.post(BASE_URI).multipart(form).await;

            res.assert_status(StatusCode::CREATED);
            let user_id = User::all(&state.mm).await?[0].id;
            let recipe_id = Recipe::all(&state.mm, user_id).await?.last().unwrap().id;
            let got = Recipe::get(&state.mm, user_id, recipe_id).await?;
            pretty_assertions::assert_eq!(got.category, "drinks:vodka");
            Ok(())
        }

        #[tokio::test]
        #[allow(clippy::too_many_lines)]
        async fn test_submit_recipe_ok() -> Result<()> {
            let (server, state) = build_server_logged_in(default_config()).await?;
            let recipe = RecipeForCreate {
                name: "Best Chinese Kale".into(),
                description: Some("Your mouth will drool like never before".into()),
                images: vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()],
                measurement_system_id: 2,
                r#yield: Some(6),
                source: Source::new("My mother's maple syrup recipes cookbook"),
                is_favourite: false,
                rating: Some(4),
                videos: vec![VideoForCreate {
                    video: Uuid::new_v4(),
                    duration: Some(Duration::hours(1)),
                    content_url: Some("https://www.youtube.com/watch?v=1".into()),
                    embed_url: Some("https://www.youtube.com/embed/embed".into()),
                }],
                category: Some("dinner".into()),
                instructions: SectionComponents::Flat(vec![
                    Item::new("Mix the apples"),
                    Item::new("Mix the blueberries"),
                    Item::new("Add whip cream and whisk the fruits until smooth"),
                ]),
                keywords: vec!["fruits".into(), "strawberries".into(), "healthy".into()],
                notes: Some("# Test\n\nSome notes".into()),
                nutrition: NutritionDetailsForCreate {
                    per_100g: Some(NutritionForCreate {
                        calories_kcal: Some(1),
                        total_carbohydrates: Some(2.),
                        sugars_g: Some(3.),
                        protein_g: Some(4.),
                        total_fat_g: Some(5.),
                        saturated_fat_g: Some(6.),
                        unsaturated_fat_g: Some(7.),
                        cholesterol_mg: Some(8.),
                        sodium_mg: Some(9.),
                        fiber_g: Some(10.),
                        trans_fat_g: Some(11.),
                    }),
                    per_serving: Some(NutritionPerServingDetailsForCreate {
                        nutrition: NutritionForCreate {
                            calories_kcal: Some(1),
                            total_carbohydrates: Some(2.),
                            sugars_g: Some(3.),
                            protein_g: Some(4.),
                            total_fat_g: Some(5.),
                            saturated_fat_g: Some(6.),
                            unsaturated_fat_g: Some(7.),
                            cholesterol_mg: Some(8.),
                            sodium_mg: Some(9.),
                            fiber_g: Some(10.),
                            trans_fat_g: Some(11.),
                        },
                        serving_size: "4 cups".into(),
                    }),
                },
                times: Some(TimesForCreate {
                    prep_seconds: 3600,
                    cook_seconds: 72000,
                }),
                ingredients: SectionComponents::Grouped(vec![
                    SectionItem::new(
                        "Prepare",
                        vec![Item::new("8 apples"), Item::new("5 lbs blueberries")],
                    ),
                    SectionItem::new("Execution", vec![Item::new("200g 30% whip cream")]),
                ]),
                cuisine: Some("japanese".into()),
                tools: vec![
                    ToolForCreate {
                        name: "large bowl".into(),
                        quantity: 1,
                    },
                    ToolForCreate {
                        name: "whisk".into(),
                        quantity: 2,
                    },
                ],
            };
            let form = create_form(&recipe);

            let res = server.post(BASE_URI).multipart(form).await;

            res.assert_status(StatusCode::CREATED);
            let user_id = User::all(&state.mm).await?[0].id;
            let recipe_id = Recipe::all(&state.mm, user_id).await?.last().unwrap().id;
            let mut got = Recipe::get(&state.mm, user_id, recipe_id).await?;
            if let SectionComponents::Flat(instructions) = &mut got.instructions {
                for item in instructions.iter_mut() { item.id = None; }
            }

            let times = recipe.times.expect("Should have times");
            pretty_assertions::assert_eq!(
                got,
                RecipeDetails {
                    recipe: Recipe {
                        id: recipe_id,
                        name: recipe.name,
                        description: recipe.description,
                        image: Some(got.recipe.image.expect("A main image")),
                        r#yield: 6,
                        language: "eng".into(),
                        measurement_system_id: 2,
                        notes: Some("# Test\n\nSome notes".into()),
                        source: recipe.source,
                        is_favourite: false,
                        rating: Some(4),
                        created_at: got.recipe.created_at,
                        updated_at: got.recipe.updated_at,
                        user_id,
                    },
                    additional_images: got.additional_images.clone(),
                    category: recipe.category.expect("Should have category"),
                    cuisine: recipe.cuisine,
                    ingredients: match recipe.ingredients {
                        SectionComponents::Grouped(section_items) =>
                            SectionComponents::Grouped(section_items),
                        SectionComponents::Flat(items) => SectionComponents::Flat(items),
                    },
                    instructions: match recipe.instructions {
                        SectionComponents::Grouped(section_items) =>
                            SectionComponents::Grouped(section_items),
                        SectionComponents::Flat(items) => SectionComponents::Flat(items),
                    },
                    keywords: vec!["fruits".into(), "healthy".into(), "strawberries".into()],
                    nutrition: NutritionDetails {
                        per_100g: Some(Nutrition {
                            id: got.nutrition.clone().per_100g.unwrap().id,
                            calories_kcal: Some(1),
                            is_precalculated_by_source: true,
                            total_carbohydrates: Some(2.),
                            sugars_g: Some(3.),
                            protein_g: Some(4.),
                            total_fat_g: Some(5.),
                            saturated_fat_g: Some(6.),
                            unsaturated_fat_g: Some(7.),
                            cholesterol_mg: Some(8.),
                            sodium_mg: Some(9.),
                            fiber_g: Some(10.),
                            trans_fat_g: Some(11.),
                        }),
                        per_serving: Some(NutritionPerServingDetails {
                            nutrition: Nutrition {
                                id: got.nutrition.clone().per_serving.unwrap().nutrition.id,
                                calories_kcal: Some(1),
                                is_precalculated_by_source: true,
                                total_carbohydrates: Some(2.),
                                sugars_g: Some(3.),
                                protein_g: Some(4.),
                                total_fat_g: Some(5.),
                                saturated_fat_g: Some(6.),
                                unsaturated_fat_g: Some(7.),
                                cholesterol_mg: Some(8.),
                                sodium_mg: Some(9.),
                                fiber_g: Some(10.),
                                trans_fat_g: Some(11.),
                            },
                            serving_size: "4 cups".into(),
                        }),
                    },
                    times: Times {
                        id: got.times.id,
                        recipe_id,
                        prep_seconds: times.prep_seconds,
                        cook_seconds: times.cook_seconds,
                        total_seconds: times.prep_seconds + times.cook_seconds,
                    },
                    tools: recipe
                        .tools
                        .into_iter()
                        .enumerate()
                        .map(|(idx, tool_c)| ToolRecipe {
                            name: tool_c.name,
                            quantity: tool_c.quantity,
                            tool_order: i16::try_from(idx + 1).unwrap_or_default(),
                        })
                        .collect(),
                    videos: vec![],
                }
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_submit_ingredients_instructions_sections_ok() -> Result<()> {
            let (server, state) = build_server_logged_in(default_config()).await?;
            let recipe = RecipeForCreate {
                name: "Best Chinese Kale".into(),
                measurement_system_id: 2,
                category: Some("dinner".into()),
                instructions: SectionComponents::Grouped(vec![
                    SectionItem::new(
                        "Prepare",
                        vec![
                            Item::new("Mix the apples"),
                            Item::new("Mix the blueberries"),
                        ],
                    ),
                    SectionItem::new(
                        "Execution",
                        vec![
                            Item::new("Add whip cream and whisk the fruits until smooth")
                        ],
                    ),
                ]),
                ingredients: SectionComponents::Grouped(vec![
                    SectionItem::new(
                        "Prepare",
                        vec![Item::new("8 apples"), Item::new("5 lbs blueberries")],
                    ),
                    SectionItem::new("Execution", vec![Item::new("200g 30% whip cream")]),
                ]),
                ..Default::default()
            };
            let form = create_form(&recipe);

            let res = server.post(BASE_URI).multipart(form).await;

            res.assert_status(StatusCode::CREATED);
            let users = User::all(&state.mm).await?;
            let user_id = users[0].id;
            let recipe_id = Recipe::all(&state.mm, user_id).await?.last().unwrap().id;
            let mut got = Recipe::get(&state.mm, user_id, recipe_id).await?;
            if let SectionComponents::Grouped(instructions) = &mut got.instructions {
                for section in instructions.iter_mut() { section.items.iter_mut().for_each(|item| item.id = None); }
            }
            pretty_assertions::assert_eq!(
                got,
                RecipeDetails {
                    recipe: Recipe {
                        id: recipe_id,
                        name: recipe.name,
                        description: recipe.description,
                        r#yield: 1,
                        language: "eng".into(),
                        measurement_system_id: 2,
                        source: recipe.source,
                        is_favourite: false,
                        created_at: got.recipe.created_at,
                        updated_at: got.recipe.updated_at,
                        user_id,
                        ..Default::default()
                    },
                    category: recipe.category.expect("Should have category"),
                    ingredients: match recipe.ingredients {
                        SectionComponents::Grouped(section_items) =>
                            SectionComponents::Grouped(section_items),
                        SectionComponents::Flat(items) => SectionComponents::Flat(items),
                    },
                    instructions: match recipe.instructions {
                        SectionComponents::Grouped(section_items) =>
                            SectionComponents::Grouped(section_items),
                        SectionComponents::Flat(items) => SectionComponents::Flat(items),
                    },
                    times: Times {
                        id: got.times.id,
                        recipe_id,
                        prep_seconds: 900,
                        cook_seconds: 1800,
                        total_seconds: 2700,
                    },
                    ..Default::default()
                }
            );
            Ok(())
        }
    }

    fn assert_recipe_form(res: &TestResponse) {
        let body = res.text();
        let normalized = body.split_whitespace().collect::<Vec<_>>().join(" ");

        let expected = [
            r#"<title hx-swap-oob="true">Add Recipe Manually | Recipya</title>"#,
            r##"<form class="card-body contents" style="padding: 0" enctype="multipart/form-data" hx-encoding="multipart/form-data" hx-post="/recipes/add/manual" hx-indicator="#fullscreen-loader">"##,
            r#"<input required type="text" name="title" placeholder="Title of the recipe*" autocomplete="off" class="input w-full text-center rounded-t-lg rounded-b-none bg-base-200">"#,
            r#"<img src="" alt="Image #1 of the recipe" class="block w-full h-full object-contain"></div><span class="grid gap-1"><div class="mr-1 image-selector p-4"><input type="file" accept="image/*,video/*" name="media" class="file-input file-input-sm file-input-bordered w-full max-w-sm""#,
            r#"<input id="servings" type="number" min="1" name="yield" value="1" class="input input-sm w-11/12">"#,
            r#"<input id="category" type="text" list="categories" name="category" class="input input-sm w-11/12" placeholder="Breakfast" autocomplete="off" value=""><datalist id="categories"><option>uncategorized</option><option>appetizers</option><option>bread</option><option>breakfasts</option><option>condiments</option><option>dessert</option><option>lunch</option><option>main dish</option><option>salad</option><option>side dish</option><option>snacks</option><option>soups</option><option>stews</option></datalist>"#,
            r#"<textarea name="description" placeholder="This Thai curry chicken will make you drool." class="textarea textarea-ghost w-full h-full resize-none rounded-none focus:outline-none"></textarea>"#,
            r#"<div class="flex justify-self-center items-center gap-1 cursor-default" title="Cooking time"><span data-tip="Cook time" class="tooltip tooltip-left"><svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="24px" height="23px" viewBox="0 0 24 23" version="1.1"><g id="surface1"><path style=" stroke:none;fill-rule:nonzero;fill:rgb(62.745098%,64.705882%,65.882353%);fill-opacity:1;" d="M 4.636719 10.984375 L 4.636719 20.417969 C 4.636719 21.007812 5.078125 21.527344 5.667969 21.527344 L 18.257812 21.527344 C 18.847656 21.527344 19.363281 21.007812 19.363281 20.417969 L 19.363281 10.984375 Z M 4.636719 10.984375 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(76.862745%,76.862745%,76.862745%);fill-opacity:1;" d="M 19.289062 9.953125 C 18.992188 8.699219 17.964844 7.8125 16.710938 7.8125 L 7.214844 7.8125 C 5.964844 7.8125 4.933594 8.699219 4.710938 9.953125 Z M 19.289062 9.953125 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(54.509804%,69.411765%,81.960784%);fill-opacity:1;" d="M 16.710938 7.8125 L 13.914062 7.8125 C 14.945312 7.8125 15.828125 8.476562 16.269531 9.363281 C 16.417969 9.730469 16.785156 9.953125 17.226562 9.953125 L 19.289062 9.953125 C 18.992188 8.699219 17.964844 7.8125 16.710938 7.8125 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(0.392157%,26.666667%,38.823529%);fill-opacity:1;" d="M 22.160156 9.953125 L 20.320312 9.953125 C 20.097656 8.183594 18.550781 6.78125 16.710938 6.78125 L 15.535156 6.78125 L 15.3125 5.898438 C 15.09375 5.160156 14.503906 4.644531 13.765625 4.644531 L 11.191406 4.644531 C 10.453125 4.644531 9.792969 5.160156 9.644531 5.898438 L 9.421875 6.78125 L 7.214844 6.78125 C 5.449219 6.78125 3.902344 8.183594 3.605469 9.953125 L 1.765625 9.953125 C 1.03125 9.953125 0.441406 10.542969 0.441406 11.277344 C 0.441406 11.5 0.441406 11.722656 0.589844 11.941406 L 1.25 13.269531 C 1.546875 13.785156 2.0625 14.152344 2.648438 14.152344 L 3.605469 14.152344 L 3.605469 20.417969 C 3.605469 21.597656 4.492188 22.558594 5.667969 22.558594 L 18.257812 22.558594 C 19.4375 22.558594 20.394531 21.597656 20.394531 20.417969 L 20.394531 14.152344 L 21.351562 14.152344 C 21.9375 14.152344 22.453125 13.859375 22.75 13.269531 L 23.410156 11.867188 C 23.558594 11.648438 23.558594 11.5 23.558594 11.277344 C 23.558594 10.542969 22.894531 9.953125 22.160156 9.953125 M 3.605469 13.121094 L 2.648438 13.121094 C 2.429688 13.121094 2.28125 12.972656 2.136719 12.753906 L 1.472656 11.5 L 1.472656 11.277344 C 1.472656 11.132812 1.621094 10.984375 1.765625 10.984375 L 3.605469 10.984375 Z M 10.75 6.117188 C 10.75 5.972656 10.96875 5.75 11.265625 5.75 L 13.839844 5.75 C 14.0625 5.75 14.28125 5.898438 14.355469 6.117188 L 14.503906 6.78125 L 10.601562 6.78125 Z M 7.214844 7.8125 L 16.710938 7.8125 C 17.964844 7.8125 18.992188 8.699219 19.289062 9.953125 L 4.710938 9.953125 C 4.933594 8.699219 5.964844 7.8125 7.214844 7.8125 M 19.363281 13.636719 L 19.363281 20.417969 C 19.363281 21.007812 18.847656 21.527344 18.257812 21.527344 L 5.667969 21.527344 C 5.078125 21.527344 4.636719 21.007812 4.636719 20.417969 L 4.636719 10.984375 L 19.363281 10.984375 Z M 22.453125 11.5 L 21.71875 12.828125 C 21.644531 12.972656 21.496094 13.121094 21.277344 13.121094 L 20.394531 13.121094 L 20.394531 11.058594 L 22.160156 11.058594 C 22.308594 11.058594 22.453125 11.207031 22.453125 11.351562 L 22.453125 11.5 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(92.54902%,94.117647%,97.647059%);fill-opacity:1;" d="M 7.804688 17.324219 C 8.089844 17.324219 8.320312 17.554688 8.320312 17.839844 C 8.320312 18.125 8.089844 18.355469 7.804688 18.355469 C 7.519531 18.355469 7.289062 18.125 7.289062 17.839844 C 7.289062 17.554688 7.519531 17.324219 7.804688 17.324219 M 7.804688 16.808594 C 7.4375 16.808594 7.214844 16.585938 7.214844 16.21875 L 7.214844 13.121094 C 7.214844 12.753906 7.4375 12.53125 7.804688 12.53125 C 8.097656 12.53125 8.320312 12.753906 8.320312 13.121094 L 8.320312 16.21875 C 8.320312 16.585938 8.097656 16.808594 7.804688 16.808594 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(54.509804%,69.411765%,81.960784%);fill-opacity:1;" d="M 16.195312 12.015625 L 16.195312 19.902344 C 16.195312 20.492188 15.679688 21.007812 15.09375 21.007812 L 6.699219 21.007812 C 6.183594 21.007812 5.667969 20.492188 5.667969 19.902344 L 5.667969 12.015625 C 5.667969 11.5 5.226562 10.984375 4.636719 10.984375 L 4.636719 20.417969 C 4.636719 21.007812 5.078125 21.527344 5.667969 21.527344 L 18.257812 21.527344 C 18.847656 21.527344 19.363281 21.007812 19.363281 20.417969 L 19.363281 10.984375 L 17.226562 10.984375 C 16.636719 10.984375 16.195312 11.5 16.195312 12.015625 M 8.246094 7.8125 L 7.214844 7.8125 C 5.964844 7.8125 4.933594 8.699219 4.710938 9.953125 L 4.933594 9.953125 C 5.375 9.953125 5.742188 9.730469 5.890625 9.363281 C 6.332031 8.476562 7.214844 7.8125 8.246094 7.8125 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(0.392157%,26.666667%,38.823529%);fill-opacity:1;" d="M 18.773438 5.75 C 18.625 5.75 18.480469 5.675781 18.40625 5.527344 C 18.183594 5.308594 18.257812 4.9375 18.40625 4.792969 C 18.699219 4.644531 18.773438 4.421875 18.773438 4.128906 C 18.773438 3.90625 18.699219 3.6875 18.480469 3.539062 C 18.257812 3.316406 18.257812 3.023438 18.40625 2.800781 C 18.625 2.582031 18.992188 2.507812 19.140625 2.726562 C 19.582031 3.097656 19.878906 3.613281 19.878906 4.128906 C 19.878906 4.71875 19.582031 5.234375 19.140625 5.601562 L 18.773438 5.75 M 18.773438 1.03125 C 19.058594 1.03125 19.289062 1.261719 19.289062 1.546875 C 19.289062 1.832031 19.058594 2.0625 18.773438 2.0625 C 18.488281 2.0625 18.257812 1.832031 18.257812 1.546875 C 18.257812 1.261719 18.488281 1.03125 18.773438 1.03125 M 16.710938 5.75 L 16.269531 5.527344 C 16.050781 5.308594 16.121094 4.9375 16.34375 4.792969 C 16.5625 4.644531 16.710938 4.421875 16.710938 4.128906 C 16.710938 3.90625 16.5625 3.6875 16.417969 3.539062 C 15.902344 3.097656 15.679688 2.652344 15.679688 2.0625 C 15.679688 1.472656 15.902344 1.03125 16.417969 0.589844 C 16.5625 0.367188 16.933594 0.441406 17.152344 0.664062 C 17.300781 0.8125 17.300781 1.179688 17.078125 1.402344 C 16.785156 1.546875 16.710938 1.769531 16.710938 2.0625 C 16.710938 2.285156 16.785156 2.507812 17.007812 2.652344 C 17.519531 3.097656 17.742188 3.613281 17.742188 4.128906 C 17.742188 4.71875 17.519531 5.234375 17.007812 5.601562 L 16.710938 5.75 "></path></g></svg></span><label><input type="text" name="time-cook" value="00:30:00" class="input input-sm max-w-24 html-duration-picker"></label></div>"#,
            r#"<div class="flex justify-self-center items-center gap-1 cursor-default" title="Prep time"><span data-tip="Prep time" class="tooltip tooltip-left"><svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="24px" height="14px" viewBox="0 0 23 14" version="1.1"><defs><linearGradient id="linear0" gradientUnits="userSpaceOnUse" x1="-125.300003" y1="85.900002" x2="-64.599998" y2="85.900002" gradientTransform="matrix(0.000000000000000013,-0.225806,0.219048,0.000000000000000014,-7.447619,-14.451613)"><stop offset="0.1" style="stop-color:rgb(67.058824%,23.921569%,8.235294%);stop-opacity:1;"></stop><stop offset="0.5" style="stop-color:rgb(78.431373%,51.372549%,30.588235%);stop-opacity:1;"></stop><stop offset="0.8" style="stop-color:rgb(90.196078%,77.254902%,53.333333%);stop-opacity:1;"></stop><stop offset="1" style="stop-color:rgb(94.509804%,87.45098%,62.352941%);stop-opacity:1;"></stop></linearGradient></defs><g id="surface1"><path style=" stroke:none;fill-rule:evenodd;fill:rgb(95.294118%,82.745099%,64.705884%);fill-opacity:1;" d="M 0 8.128906 L 0.21875 6.324219 C 0.4375 5.644531 0.65625 5.195312 1.3125 4.96875 L 8.542969 2.484375 L 8.542969 1.804688 L 8.980469 0.675781 L 9.855469 0.453125 L 10.734375 0.453125 L 10.953125 0.675781 L 12.265625 1.128906 C 13.003906 1 13.761719 1.078125 14.457031 1.355469 C 15.125 1.453125 15.785156 1.601562 16.429688 1.804688 L 17.523438 1.804688 L 18.617188 0.902344 L 20.589844 0.453125 C 21.246094 0.675781 21.6875 0.902344 21.90625 1.355469 C 22.34375 1.804688 22.5625 2.710938 22.34375 4.066406 L 22.125 4.515625 C 22.5625 4.742188 22.78125 5.195312 22.78125 5.644531 L 22.78125 7.675781 L 22.125 8.804688 L 21.027344 9.484375 L 17.304688 11.289062 L 16.210938 11.742188 L 12.921875 13.324219 L 12.046875 13.773438 L 11.390625 14 L 10.078125 14 L 8.542969 13.546875 C 6.269531 12.441406 4.007812 11.308594 1.753906 10.160156 L 0.65625 9.257812 C 0.21875 9.03125 0 8.582031 0 8.128906 Z M 0 8.128906 "></path><path style=" stroke:none;fill-rule:evenodd;fill:url(#linear0);" d="M 1.3125 4.742188 L 8.542969 2.03125 L 8.542969 1.582031 L 8.980469 0.453125 C 9.199219 0.226562 9.636719 0 9.855469 0.226562 L 10.953125 0.226562 L 12.265625 0.902344 C 13.003906 0.773438 13.761719 0.851562 14.457031 1.128906 L 15.769531 1.355469 C 16.308594 1.65625 16.933594 1.738281 17.523438 1.582031 L 17.523438 1.355469 C 17.742188 1.128906 18.179688 0.675781 18.617188 0.675781 C 19.277344 0.226562 19.933594 0.226562 20.589844 0.226562 C 21.027344 0.453125 21.6875 0.675781 21.90625 1.128906 C 22.34375 1.582031 22.5625 2.484375 22.34375 3.839844 L 22.125 4.289062 C 22.5625 4.515625 22.78125 4.96875 22.78125 5.417969 L 22.78125 6.546875 L 22.5625 7.453125 L 22.125 8.582031 L 21.027344 9.257812 L 17.085938 11.289062 L 16.210938 11.515625 L 12.921875 13.097656 L 12.046875 13.546875 L 11.390625 13.773438 L 9.855469 13.773438 L 8.324219 13.324219 C 6.121094 12.21875 3.929688 11.089844 1.753906 9.933594 L 1.535156 9.933594 L 0.4375 9.03125 L 0 7.902344 C 0 7.292969 0.0742188 6.6875 0.21875 6.097656 C 0.21875 5.417969 0.65625 4.96875 1.3125 4.742188 Z M 1.3125 4.742188 "></path><path style="fill:none;stroke-width:0.3;stroke-linecap:butt;stroke-linejoin:miter;stroke:rgb(95.294118%,82.745099%,64.705884%);stroke-opacity:1;stroke-miterlimit:4;" d="M 5.991848 21.001116 L 39.999151 8.995536 L 39.00051 6.00279 L 40.997792 2.006696 L 46.008832 0 L 47.007473 0 L 49.004755 1.003348 L 50.003397 1.003348 L 55.995245 3.996094 C 59.579654 2.923549 63.413723 2.923549 66.998132 3.996094 L 73.007812 6.00279 C 75.272588 6.763951 77.733526 6.763951 79.998302 6.00279 L 84.991508 2.006696 C 88.005265 1.003348 91.001189 0 93.997113 1.003348 C 96.993037 1.003348 99.008152 2.006696 101.005435 3.996094 C 103.002717 6.00279 103.002717 9.998884 102.004076 16.001674 L 102.004076 18.008371 L 104.001359 23.007812 L 105 28.007254 L 104.001359 33.006696 L 101.005435 37.00279 L 95.994395 40.998884 L 78.99966 49.008371 L 75.005095 50.997768 L 74.006454 50.997768 L 58.991168 58.003906 L 54.996603 59.993304 L 52.000679 59.993304 L 51.002038 60.996652 L 46.008832 60.996652 L 39.00051 59.007254 C 28.60394 54.128906 18.278702 49.129464 8.006963 43.991629 L 8.006963 43.00558 L 2.995924 39.995536 L 0 33.992746 C 0 31.294085 0.338825 28.612723 0.998641 26.000558 C 1.997283 23.993862 2.995924 22.004464 5.991848 21.001116 Z M 5.991848 21.001116 " transform="matrix(0.219048,0,0,0.225806,0,0)"></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(25.882354%,9.411765%,1.568628%);fill-opacity:1;" d="M 1.3125 8.128906 L 1.09375 7.675781 L 1.3125 6.097656 L 1.753906 5.644531 L 9.855469 2.933594 L 9.636719 2.257812 C 9.710938 1.882812 9.785156 1.503906 9.855469 1.128906 L 10.078125 1.128906 L 10.515625 1.355469 L 10.734375 1.355469 L 12.265625 2.03125 C 12.914062 1.859375 13.589844 1.859375 14.238281 2.03125 C 14.910156 2.207031 15.570312 2.433594 16.210938 2.710938 L 16.648438 2.710938 L 17.960938 2.484375 L 18.398438 2.03125 L 19.058594 1.582031 L 20.371094 1.355469 L 21.246094 1.804688 L 21.246094 3.839844 L 21.027344 4.066406 L 20.371094 4.515625 L 20.589844 4.515625 L 21.464844 4.96875 L 21.90625 5.417969 L 21.90625 6.324219 L 21.6875 7 L 21.464844 7.675781 L 20.589844 8.128906 C 19.933594 8.582031 18.839844 9.257812 16.867188 9.933594 L 12.484375 11.96875 L 11.828125 12.417969 C 11.617188 12.515625 11.394531 12.589844 11.171875 12.644531 L 10.296875 12.644531 L 8.980469 12.195312 C 6.703125 11.09375 4.441406 9.964844 2.191406 8.804688 Z M 1.3125 8.128906 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(65.490198%,51.372552%,26.274511%);fill-opacity:1;" d="M 6.351562 5.195312 C 8.921875 4.820312 11.476562 4.371094 14.019531 3.839844 L 15.332031 4.289062 L 16.429688 4.515625 C 17.304688 4.515625 17.742188 4.289062 17.960938 4.066406 L 18.179688 3.613281 L 18.617188 3.160156 L 19.933594 2.484375 L 21.027344 2.03125 L 21.027344 3.839844 L 20.808594 4.066406 C 20.371094 4.289062 19.933594 4.515625 19.277344 4.289062 L 17.960938 4.742188 L 19.496094 4.96875 L 21.464844 5.644531 L 21.464844 7.226562 L 21.246094 7.453125 L 20.371094 8.128906 C 17.839844 9.550781 15.203125 10.757812 12.484375 11.742188 L 11.171875 12.417969 C 10.515625 12.417969 9.636719 12.417969 8.980469 11.96875 C 6.324219 10.59375 3.695312 9.164062 1.09375 7.675781 L 1.3125 7 L 1.535156 6.324219 Z M 6.351562 5.195312 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(56.078434%,44.313726%,22.352941%);fill-opacity:1;" d="M 4.382812 7.902344 L 3.503906 7.902344 L 3.285156 9.257812 L 3.066406 9.03125 L 3.285156 7.675781 L 2.847656 7.226562 L 2.628906 7.453125 L 2.410156 8.804688 L 2.191406 8.582031 L 1.972656 8.582031 L 2.410156 7.226562 L 1.972656 7 L 1.753906 8.355469 L 1.3125 8.128906 L 1.535156 6.546875 L 6.351562 6.324219 L 10.078125 8.128906 L 10.953125 10.839844 L 11.171875 12.417969 L 10.296875 12.417969 L 10.515625 10.839844 L 9.417969 10.613281 L 9.199219 12.195312 L 8.542969 11.96875 L 8.761719 10.386719 L 8.105469 10.160156 L 7.886719 11.515625 L 7.449219 11.289062 L 7.449219 9.710938 L 7.230469 9.03125 L 6.570312 9.257812 L 6.132812 10.613281 L 5.476562 10.386719 L 5.914062 9.03125 L 4.820312 8.128906 L 4.601562 8.355469 L 4.382812 9.710938 L 4.160156 9.484375 Z M 4.382812 7.902344 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(52.941179%,41.176471%,18.82353%);fill-opacity:1;" d="M 19.933594 2.484375 L 19.933594 2.710938 C 18.839844 3.160156 18.617188 3.839844 19.496094 4.289062 L 18.617188 4.289062 L 17.960938 4.742188 L 19.496094 4.96875 L 21.464844 5.644531 L 21.464844 7.226562 L 21.246094 7.453125 L 20.371094 8.128906 C 17.839844 9.550781 15.203125 10.757812 12.484375 11.742188 L 11.828125 12.195312 L 10.515625 12.417969 L 10.734375 12.195312 L 10.734375 9.710938 L 10.515625 8.804688 C 13.609375 6.628906 16.75 4.519531 19.933594 2.484375 Z M 19.933594 2.484375 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(47.450981%,36.470589%,16.862746%);fill-opacity:1;" d="M 21.027344 7.453125 C 21.464844 7 21.464844 6.546875 21.464844 5.644531 L 21.464844 7.226562 L 21.246094 7.453125 L 20.371094 8.128906 C 17.839844 9.550781 15.203125 10.757812 12.484375 11.742188 L 11.828125 12.195312 L 10.515625 12.417969 L 10.734375 12.195312 L 11.171875 12.195312 L 12.046875 11.96875 C 15.042969 10.46875 18.035156 8.960938 21.027344 7.453125 Z M 21.027344 7.453125 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(87.450981%,71.764708%,40.784314%);fill-opacity:1;" d="M 1.753906 6.097656 C 5.445312 4.722656 9.167969 3.441406 12.921875 2.257812 L 14.238281 2.484375 L 15.550781 2.933594 L 16.648438 3.160156 C 17.523438 3.160156 17.960938 2.933594 18.179688 2.710938 L 18.617188 2.257812 L 19.058594 2.03125 C 19.714844 1.582031 20.152344 1.582031 20.808594 1.804688 C 21.246094 2.03125 21.246094 2.484375 20.808594 2.710938 L 19.496094 3.160156 L 18.179688 3.613281 L 19.058594 4.289062 C 19.855469 4.75 20.660156 5.203125 21.464844 5.644531 C 21.6875 5.871094 21.246094 6.324219 20.589844 6.773438 C 17.578125 8.257812 14.511719 9.613281 11.390625 10.839844 C 10.734375 11.066406 9.855469 10.839844 8.980469 10.613281 C 6.472656 9.3125 3.988281 7.957031 1.535156 6.546875 L 1.535156 6.097656 Z M 1.753906 6.097656 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(79.215688%,64.313728%,33.333334%);fill-opacity:1;" d="M 2.628906 7.226562 L 2.410156 7.226562 L 4.820312 6.097656 L 6.351562 4.742188 L 8.105469 3.839844 C 9.554688 3.546875 11.015625 3.320312 12.484375 3.160156 C 12.921875 3.160156 13.582031 2.933594 14.019531 2.484375 L 14.457031 2.484375 C 12.945312 3.640625 11.078125 4.203125 9.199219 4.066406 C 8.105469 4.066406 7.230469 4.289062 6.570312 4.742188 L 5.039062 6.097656 C 4.601562 6.546875 3.722656 7 2.628906 7.226562 Z M 2.628906 7.226562 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(79.215688%,64.313728%,33.333334%);fill-opacity:1;" d="M 5.257812 7 C 4.601562 7 3.941406 7.226562 3.503906 7.675781 L 3.285156 7.675781 C 4.5625 6.878906 5.976562 6.34375 7.449219 6.097656 L 10.296875 5.417969 L 11.609375 4.742188 L 12.484375 4.066406 L 14.675781 2.484375 L 14.894531 2.710938 L 12.921875 4.289062 L 10.953125 5.644531 C 9.855469 6.324219 7.886719 6.773438 5.257812 7 Z M 1.972656 6.324219 L 3.066406 5.871094 L 4.160156 5.195312 L 6.132812 4.515625 L 5.476562 4.96875 L 3.722656 6.097656 L 2.191406 6.773438 L 1.972656 6.773438 L 1.535156 6.546875 Z M 9.855469 3.160156 L 11.609375 2.710938 L 13.363281 2.257812 L 13.582031 2.257812 C 13.144531 2.710938 12.484375 2.933594 11.828125 2.933594 Z M 18.617188 7.675781 C 19.277344 6.546875 20.152344 5.871094 21.246094 5.417969 L 21.464844 5.644531 C 20.808594 5.871094 20.152344 6.324219 19.714844 7 Z M 9.199219 7.902344 C 10.078125 7.902344 10.953125 7.675781 12.046875 7 L 14.019531 5.417969 L 15.550781 4.066406 C 16.210938 3.613281 16.648438 3.160156 17.304688 3.160156 L 18.179688 2.710938 L 20.589844 1.804688 L 20.808594 1.804688 L 20.589844 2.03125 L 18.398438 2.933594 L 16.210938 3.839844 L 14.457031 5.417969 C 13.800781 6.324219 13.144531 6.773438 12.703125 7 C 12.265625 7.453125 11.390625 7.902344 10.078125 8.128906 L 7.886719 8.582031 L 6.570312 9.257812 L 5.914062 8.804688 L 7.230469 8.355469 Z M 16.867188 6.097656 C 17.304688 5.195312 17.960938 4.742188 18.839844 4.289062 L 19.496094 4.515625 L 17.742188 5.871094 L 15.992188 7.902344 C 15.113281 8.582031 14.457031 9.03125 13.582031 9.257812 L 10.953125 9.710938 L 9.417969 10.613281 L 8.761719 10.386719 L 10.515625 9.484375 L 12.703125 9.03125 C 14.382812 8.582031 15.851562 7.542969 16.867188 6.097656 Z M 16.867188 6.097656 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(79.215688%,64.313728%,33.333334%);fill-opacity:1;" d="M 6.789062 7.675781 C 5.914062 7.675781 5.257812 7.902344 4.601562 8.355469 L 4.160156 8.128906 C 4.820312 7.675781 5.914062 7.226562 7.230469 7.226562 L 10.953125 6.097656 C 12.046875 5.644531 12.921875 4.96875 13.582031 4.289062 L 15.332031 2.710938 L 15.992188 2.933594 L 14.019531 4.515625 L 12.265625 5.871094 L 9.636719 7.226562 Z M 20.152344 4.742188 L 20.589844 4.96875 L 18.839844 6.324219 C 18.179688 6.773438 17.742188 7.453125 17.304688 8.355469 C 14.960938 9.164062 12.625 9.992188 10.296875 10.839844 L 12.703125 9.933594 L 14.894531 9.03125 C 15.992188 8.582031 17.304688 7.453125 18.617188 5.644531 Z M 13.144531 7.453125 C 14.671875 6.238281 16.203125 5.035156 17.742188 3.839844 C 17.960938 3.386719 19.058594 2.933594 21.027344 2.03125 L 21.027344 2.484375 C 20.402344 2.570312 19.804688 2.800781 19.277344 3.160156 L 18.179688 3.613281 L 18.398438 3.839844 L 15.769531 6.324219 L 13.582031 8.128906 L 10.515625 8.804688 C 9.417969 9.03125 8.761719 9.484375 8.105469 9.933594 L 7.449219 9.710938 C 9.289062 8.8125 11.191406 8.058594 13.144531 7.453125 Z M 6.570312 5.871094 L 6.570312 5.644531 L 6.789062 5.195312 L 7.230469 4.515625 L 8.761719 4.289062 L 10.515625 4.289062 C 10.734375 4.515625 10.734375 4.742188 10.296875 4.96875 L 8.761719 5.644531 L 7.449219 5.871094 L 7.449219 5.195312 L 8.324219 4.742188 L 9.199219 4.515625 L 9.417969 4.742188 L 8.761719 5.195312 L 8.980469 4.96875 L 8.324219 4.96875 L 8.105469 5.195312 C 7.886719 5.417969 8.105469 5.417969 8.324219 5.417969 L 9.199219 5.195312 L 9.855469 4.742188 L 9.855469 4.515625 L 8.761719 4.515625 L 7.449219 4.96875 L 6.789062 5.417969 Z M 6.570312 5.871094 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(41.960785%,25.098041%,14.117648%);fill-opacity:1;" d="M 9.855469 3.160156 L 11.171875 2.710938 L 12.265625 3.839844 L 14.238281 5.417969 L 15.550781 7 L 16.210938 8.582031 L 15.992188 9.484375 L 15.332031 9.710938 L 14.894531 9.710938 L 14.894531 9.257812 L 15.113281 9.03125 L 15.332031 8.582031 L 14.675781 7.675781 L 14.457031 7.453125 L 14.457031 7.226562 L 14.238281 6.773438 L 14.019531 6.773438 C 13.304688 7.003906 12.570312 7.15625 11.828125 7.226562 L 11.171875 7.226562 L 10.734375 6.097656 L 10.078125 4.289062 Z M 9.855469 3.160156 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(47.843137%,47.843137%,47.843137%);fill-opacity:1;" d="M 11.171875 7 L 11.390625 5.871094 L 12.265625 4.96875 L 13.582031 4.96875 L 14.894531 5.195312 L 15.113281 5.871094 L 14.894531 6.097656 L 12.921875 6.773438 L 11.609375 7 Z M 11.171875 7 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(59.215689%,59.215689%,59.215689%);fill-opacity:1;" d="M 12.265625 6.097656 L 12.046875 6.546875 L 12.265625 7 L 11.171875 7 L 11.390625 6.324219 Z M 12.265625 6.097656 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(92.54902%,92.54902%,92.54902%);fill-opacity:1;" d="M 9.855469 1.582031 L 10.734375 1.804688 L 12.921875 2.933594 C 13.75 3.667969 14.488281 4.5 15.113281 5.417969 L 14.894531 5.417969 L 14.019531 4.289062 L 12.484375 2.710938 L 10.734375 1.804688 Z M 9.855469 1.582031 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(78.039217%,78.039217%,78.039217%);fill-opacity:1;" d="M 9.855469 1.582031 L 10.078125 1.582031 L 10.515625 1.804688 C 10.609375 3.429688 11.140625 4.992188 12.046875 6.324219 L 11.171875 7 L 10.953125 6.546875 C 10.421875 5.246094 10.054688 3.882812 9.855469 2.484375 Z M 9.855469 1.582031 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(89.411765%,89.411765%,89.411765%);fill-opacity:1;" d="M 10.078125 3.386719 L 10.078125 2.933594 L 10.734375 2.933594 L 10.734375 3.160156 Z M 9.855469 2.03125 L 10.515625 2.03125 L 10.515625 2.710938 L 9.855469 2.710938 Z M 11.828125 6.097656 L 11.171875 6.773438 L 10.953125 6.324219 L 11.609375 5.871094 Z M 10.296875 4.515625 L 10.078125 3.839844 L 10.734375 3.613281 L 10.953125 4.066406 Z M 11.390625 5.195312 L 10.734375 5.644531 L 10.515625 4.96875 L 10.953125 4.515625 Z M 11.390625 5.195312 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(56.470591%,56.470591%,56.470591%);fill-opacity:1;" d="M 14.238281 6.546875 L 14.019531 6.324219 L 14.019531 5.871094 L 14.675781 5.871094 L 15.113281 6.097656 L 15.113281 6.324219 L 14.894531 6.546875 Z M 14.238281 6.546875 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(70.19608%,70.19608%,70.19608%);fill-opacity:1;" d="M 14.019531 5.871094 L 14.238281 5.644531 L 14.894531 5.644531 L 15.113281 5.871094 L 15.113281 6.097656 L 14.238281 6.097656 Z M 14.019531 5.871094 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(55.686277%,35.686275%,17.254902%);fill-opacity:1;" d="M 14.238281 6.773438 L 14.238281 6.097656 L 14.894531 6.097656 L 15.550781 6.773438 L 16.429688 8.128906 L 16.429688 8.582031 L 15.769531 9.484375 C 15.113281 9.710938 14.675781 9.710938 14.894531 9.257812 L 15.113281 8.582031 L 15.113281 8.128906 L 14.894531 7.675781 L 14.675781 7.453125 L 14.457031 6.773438 Z M 14.238281 6.773438 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(43.137255%,27.058825%,13.725491%);fill-opacity:1;" d="M 15.769531 8.355469 L 16.210938 7.675781 L 16.429688 8.128906 L 16.429688 8.582031 C 16.429688 9.03125 16.210938 9.484375 15.769531 9.484375 L 14.894531 9.484375 L 14.894531 9.03125 L 15.113281 8.582031 L 15.332031 8.355469 Z M 15.769531 8.355469 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(64.313728%,44.705883%,26.666668%);fill-opacity:1;" d="M 14.675781 6.324219 L 14.457031 6.097656 L 14.457031 5.871094 L 15.113281 5.871094 L 15.769531 6.097656 L 16.429688 7.226562 L 16.429688 8.582031 C 15.992188 8.804688 15.550781 9.03125 15.113281 8.804688 L 15.113281 8.355469 L 15.550781 7.902344 L 15.332031 7.453125 L 14.894531 7.226562 L 14.675781 6.773438 Z M 14.675781 6.324219 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 15.113281 8.128906 L 15.550781 7.902344 L 15.332031 8.355469 L 15.332031 8.804688 L 15.113281 8.804688 Z M 14.457031 5.871094 L 14.894531 6.546875 L 15.332031 7.453125 L 15.113281 7.453125 L 14.894531 6.773438 L 14.894531 6.546875 L 14.675781 6.546875 L 14.238281 6.097656 Z M 16.429688 7.675781 L 16.429688 7.453125 C 16.648438 7.902344 16.648438 8.355469 16.210938 8.582031 Z M 16.429688 7.675781 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 14.894531 6.097656 L 15.332031 6.546875 L 15.550781 6.773438 L 15.550781 7 L 15.769531 7.453125 L 15.769531 8.128906 L 15.550781 8.804688 L 15.550781 7 L 14.675781 5.871094 Z M 14.894531 6.097656 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 15.550781 6.324219 L 15.992188 6.546875 L 16.210938 7.226562 L 16.210938 8.128906 L 15.992188 8.804688 L 15.992188 6.546875 C 15.695312 6.324219 15.402344 6.101562 15.113281 5.871094 L 15.332031 5.871094 Z M 15.550781 6.324219 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 15.113281 5.871094 L 15.332031 6.324219 L 15.550781 6.773438 L 15.769531 7.226562 L 15.992188 7.453125 L 15.992188 8.128906 L 15.769531 8.804688 L 15.769531 7 L 15.550781 6.773438 L 15.332031 6.324219 L 14.894531 5.871094 Z M 15.332031 5.871094 L 15.769531 6.324219 L 15.992188 6.546875 L 15.550781 6.324219 Z M 15.332031 5.871094 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(43.137255%,27.058825%,13.725491%);fill-opacity:1;" d="M 16.210938 8.355469 C 15.992188 8.582031 15.769531 8.804688 15.550781 8.582031 L 15.332031 8.355469 L 15.992188 8.128906 Z M 16.210938 8.355469 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(69.411767%,69.411767%,69.411767%);fill-opacity:1;" d="M 16.210938 8.355469 L 15.992188 8.582031 L 15.332031 8.582031 L 15.769531 8.355469 Z M 16.210938 8.355469 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(49.019608%,49.019608%,49.019608%);fill-opacity:1;" d="M 16.210938 8.355469 L 15.992188 8.582031 L 15.332031 8.582031 L 15.769531 8.582031 L 15.992188 8.355469 Z M 16.210938 8.355469 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 15.992188 6.773438 L 15.769531 6.773438 L 15.992188 7 L 15.992188 6.773438 L 15.992188 7 L 15.769531 7 L 15.769531 6.773438 Z M 15.992188 6.773438 "></path></g></svg></span><label><input type="text" name="time-prep" value="00:15:00" class="input input-sm max-w-24 html-duration-picker"></label></div>"#,
            r#"<table class="table table-zebra table-xs"><thead><tr><th><select class="select select-sm" onchange="filterNutritionRows(this, this.value)"><option value="per-100g">Nutrition (per 100g)</option><option value="per-serving">Nutrition (per serving)</option></select></th></tr></thead><tbody><tr data-nutrition-type="per-100g"><td>Calories</td><td><label><input type="text" name="calories-per-100g" autocomplete="off" placeholder="368kcal" class="input input-xs max-w-24"></label></td></tr><tr data-nutrition-type="per-100g"><td>Total carbs</td><td><label><input type="text" name="total-carbohydrates-per-100g" autocomplete="off" placeholder="35g" class="input input-xs max-w-24"></label></td></tr><tr data-nutrition-type="per-100g"><td>Sugars</td><td><label><input type="text" name="sugars-per-100g" autocomplete="off" placeholder="3g" class="input input-xs max-w-24"></label></td></tr><tr data-nutrition-type="per-100g"><td>Protein</td><td><label><input type="text" name="protein-per-100g" autocomplete="off" placeholder="21g" class="input input-xs max-w-24"></label></td></tr><tr data-nutrition-type="per-100g"><td>Total fat</td><td><label><input type="text" name="total-fat-per-100g" autocomplete="off" placeholder="15g" class="input input-xs max-w-24"></label></td></tr><tr data-nutrition-type="per-100g"><td>Saturated fat</td><td><label><input type="text" name="saturated-fat-per-100g" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24"></label></td></tr><tr data-nutrition-type="per-100g"><td>Unsaturated fat</td><td><label><input type="text" name="unsaturated-fat-per-100g" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24"></label></td></tr><tr data-nutrition-type="per-100g"><td>Trans fat</td><td><label><input type="text" name="trans-fat-per-100g" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24"></label></td></tr><tr data-nutrition-type="per-100g"><td>Cholesterol</td><td><label><input type="text" name="cholesterol-per-100g" autocomplete="off" placeholder="1.1mg" class="input input-xs max-w-24"></label></td></tr><tr data-nutrition-type="per-100g"><td>Sodium</td><td><label><input type="text" name="sodium-per-100g" autocomplete="off" placeholder="100mg" class="input input-xs max-w-24"></label></td></tr><tr data-nutrition-type="per-100g"><td>Fiber</td><td><label><input type="text" name="fiber-per-100g" autocomplete="off" placeholder="8g" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Serving size</td><td><label><input type="text" name="serving-size" autocomplete="off" placeholder="1/4 cup (45g)" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Calories</td><td><label><input type="text" name="calories-per-serving" autocomplete="off" placeholder="128kcal" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Total carbs</td><td><label><input type="text" name="total-carbohydrates-per-serving" autocomplete="off" placeholder="12g" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Sugars</td><td><label><input type="text" name="sugars-per-serving" autocomplete="off" placeholder="2g" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Protein</td><td><label><input type="text" name="protein-per-serving" autocomplete="off" placeholder="5g" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Total fat</td><td><label><input type="text" name="total-fat-per-serving" autocomplete="off" placeholder="15g" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Saturated fat</td><td><label><input type="text" name="saturated-fat-per-serving" autocomplete="off" placeholder="3.8g" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Unsaturated fat</td><td><label><input type="text" name="unsaturated-fat-per-serving" autocomplete="off" placeholder="3.2g" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Trans fat</td><td><label><input type="text" name="trans-fat-per-serving" autocomplete="off" placeholder="0g" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Cholesterol</td><td><label><input type="text" name="cholesterol-per-serving" autocomplete="off" placeholder="100mg" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Sodium</td><td><label><input type="text" name="sodium-per-serving" autocomplete="off" placeholder="25mg" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Fiber</td><td><label><input type="text" name="fiber-per-serving" autocomplete="off" placeholder="6g" class="input input-xs max-w-24"></label></td></tr></tbody></table>"#,
            r#"<h2 class="font-semibold text-center pb-2"><span class="underline">Tools</span></h2>"#,
            r#"<ol id="tools-list" class="pl-4"><li class="pb-2" data-drag-row><div class="grid grid-flow-col items-center"><label class="flex gap-1"><div class="inline-flex size-6 cursor-grab mt-1 items-center justify-center text-2xl"><data-drageable draggable="true" data-drag-handle>⠿</data-drageable></div><input type="text" name="tool" placeholder="1 frying pan" class="input input-bordered input-sm w-full" value="" _="on keydown if event.key is 'Enter' halt the event then call addItem(event)"></label><div class="ml-2 flex gap-2"><button type="button" class="btn btn-sm btn-outline btn-success" title="Shortcut: Enter" onclick="addItem(event)"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15"></svg>Add</button><button type="button" class="delete-button btn btn-square btn-sm btn-outline btn-error""#,
            r#"<h2 class="font-semibold text-center pb-2"><span class="underline">Ingredients</span><sup class="text-red-600">*</sup></h2>"#,
            r#"<ol id="ingredients-list" class="pl-4"><li data-drag-row><div class="flex items-center" data-drageable draggable="true"><div class="inline-flex size-6 cursor-grab items-center justify-center text-2xl" data-drag-handle>⠿</div><div class="divider flex-1"><div class="flex gap-2"><btn class="btn btn-xs btn-outline" _="on click make an &lt;input/&gt; called newInput set newInput.type to 'text' set newInput.name to 'section-ingredient' set newInput.placeholder to 'Section name' set newInput.className to 'input input-sm w-fit' set list to closest &lt;ol/&gt; set sectionName to 'ingredient' put newInput before me remove me js(newInput, list, sectionName) newInput.addEventListener('keydown', function(event) { if (event.key === 'Enter') { event.preventDefault(); newInput.blur(); } }); newInput.addEventListener('focusout', function() { renumberSections(list, sectionName) }); end newInput.focus()">Add section</btn><btn class="btn btn-xs btn-square" _="on click set list to closest &lt;ol/&gt; set sectionName to 'ingredient' remove closest &lt;li/&gt; call renumberSections(list, sectionName)"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m9.75 9.75 4.5 4.5m0-4.5-4.5 4.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"></path></svg></btn></div></div></div></li><li class="pb-2" data-drag-row><div class="grid grid-flow-col items-center"><label class="flex gap-1"><div class="inline-flex size-6 cursor-grab handle mt-1 items-center justify-center text-2xl" data-drageable draggable="true" data-drag-handle>⠿</div><input required type="text" name="ingredient" value="" placeholder="1 cup of chopped onions" class="input input-sm" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))"></label><div class="ml-2 flex gap-2"><button type="button" class="btn btn-sm btn-outline btn-success" title="Shortcut: Enter" onclick="addItem(event)"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15"></svg>Add</button><button type="button" class="delete-button btn btn-square btn-sm btn-outline btn-error" onclick="deleteItem(this, 'ingredient')"><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"></path></svg></button></div></div></li></ol>"#,
            r#"<h2 class="font-semibold text-center pb-2"><span class="underline">Instructions</span><sup class="text-red-600">*</sup></h2>"#,
            r#"<ol id="instructions-list" class="grid [counter-reset:steps]"><li data-drag-row><div class="flex items-center" data-drageable draggable="true"><div class="inline-flex size-6 cursor-grab items-center justify-center text-2xl" data-drag-handle>⠿</div><div class="divider flex-1"><div class="flex gap-2"><btn class="btn btn-xs btn-outline" _="on click make an &lt;input/&gt; called newInput set newInput.type to 'text' set newInput.name to 'section-instruction' set newInput.placeholder to 'Section name' set newInput.className to 'input input-sm w-fit' set list to closest &lt;ol/&gt; set sectionName to 'instruction' put newInput before me remove me js(newInput, list, sectionName) newInput.addEventListener('keydown', function(event) { if (event.key === 'Enter') { event.preventDefault(); newInput.blur(); } }); newInput.addEventListener('focusout', function() { renumberSections(list, sectionName) }); end newInput.focus()">Add section</btn><btn class="btn btn-xs btn-square" _="on click set list to closest &lt;ol/&gt; set sectionName to 'instruction' remove closest &lt;li/&gt; call renumberSections(list, sectionName)"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m9.75 9.75 4.5 4.5m0-4.5-4.5 4.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"></path></svg></btn></div></div></div></li><li class="flex items-start gap-2 pt-2 md:pl-0 [counter-increment:steps] before:content-[counter(steps)_'.'] before:pt-2 before:font-medium" data-drag-row><div class="w-full bg-base-300" data-drageable draggable="true"><label class="w-11/12"><textarea required name="instruction" rows="4" class="textarea textarea-bordered rounded-none w-full" placeholder="Mix all ingredients together" _="on keydown if event.ctrlKey and event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))"></textarea></label><div class="grid gap-2 p-2 grid-flow-col"><div class="inline-flex size-6 cursor-grab mt-1 items-center justify-center text-2xl" data-drag-handle>⠿</div><div class="flex gap-2 justify-end"><button type="button" class="btn btn-sm btn-outline btn-success" title="Shortcut: CTRL + Enter" onclick="addItem(event)"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15"></svg>Add</button><button type="button" class="delete-button btn btn-square btn-sm btn-outline btn-error" onclick="deleteItem(this, 'instruction')"><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"></path></svg></button></div></div></div></li></ol>"#,
            r#"<button class="btn btn-primary btn-block btn-sm">Submit</button>"#,
        ];
        for want in expected {
            assert!(
                normalized.contains(want),
                "Expected to find:\n{want}\n\nIn:\n{normalized}"
            );
        }
    }
}
