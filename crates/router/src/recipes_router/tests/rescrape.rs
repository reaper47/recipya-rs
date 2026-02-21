#[cfg(test)]
mod tests {
    use axum::http::Method;

    use axum_test::{TestServer, TestWebSocket};
    use models::{
        Recipe,
        recipe::structs::{
            recipe::RecipeForCreate, test_utils::a_complete_recipe_for_create, types::Source,
        },
        user::User,
    };
    use recipya_scraper::tests::support::scraper::scrape_test_websites;
    use reqwest::StatusCode;
    use testing::utils::{
        TestDb, assert_html, assert_must_be_logged_in, assert_ws_message, build_server_ws,
        create_app_state,
    };

    use crate::recipes_router::params::RecipeScrapeForm;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    fn base_uri(recipe_id: i64) -> String {
        format!("/recipes/{recipe_id}/rescrape")
    }

    #[tokio::test]
    async fn test_must_be_logged_in() -> Result<()> {
        assert_must_be_logged_in(Method::GET, &base_uri(1)).await?;
        assert_must_be_logged_in(Method::POST, &base_uri(1)).await
    }

    mod tests_get {
        use models::recipe::structs::{
            nutrition::{Nutrition, NutritionDetails, NutritionPerServingDetails},
            section::SectionComponents,
            time::Times,
        };

        use super::*;

        #[tokio::test]
        async fn test_recipe_not_found_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config).await?;

            let res = server.get(&base_uri(99)).await;

            res.assert_status_not_found();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Recipe not found.","status":"alert-error","title":"Operation Failed"}}"# ).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_recipe_url_not_valid_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let mut recipe = a_complete_recipe_for_create();
            recipe.name = "Not a valid URL".into();
            recipe.source = Source::new("a magazine");
            let _ = Recipe::create(&state.mm, user_id, &recipe).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_internal_server_error();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Error scraping recipe or recipe source is not a URL.","status":"alert-error","title":"Operation Failed"}}"# ).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_scraped_recipe_same_as_old_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            insert_zweigles_recipe(&server, &mut ws_server).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Recipe has not changed.","status":"alert-warning","title":"Attention"}}"# ).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_scraped_recipe_one_updated_field_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            insert_zweigles_recipe(&server, &mut ws_server).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let mut recipe = Recipe::get(&state.mm, user_id, 1).await?;
            recipe.keywords = vec!["potatoes".into(), "fish".into()];
            Recipe::update(
                &state.mm,
                user_id,
                1,
                &mut RecipeForCreate::from(recipe.clone()),
            )
            .await?;

            let res = server
                .get(&base_uri(1))
                .add_header(axum_htmx::HX_REQUEST, "true")
                .await;

            res.assert_status_ok();
            res.assert_header(axum_htmx::HX_PUSH_URL, "/recipes/1/rescrape");
            res.assert_header(axum_htmx::HX_RESWAP, "innerHTML transition:true");
            assert_html(
                &res,
                vec![
                    // Title
                    r#"<input type="hidden" name="title-source" value="old">"#,
                    r#"<input type="hidden" name="title-old" value="Polish Kielbasa Sheet Pan and Potatoes">"#,
                    // Category
                    r#"<input type="hidden" name="category-source" value="old">"#,
                    r#"<input type="hidden" name="category-old" value="uncategorized">"#,
                    // Description
                    r#"<input type="hidden" name="description-source" value="old">"#,
                    r#"<input type="hidden" name="description-old" value="Want a hearty and comforting meal you can make in 35 minutes? This Polish Kielbasa Sheet Pan and Potatoes is your answer. Heat up some sauerkraut or throw together a salad to go with your sausage and potatoes and enjoy!">"#,
                    // Ingredients
                    r#"<input type="hidden" name="ingredients-source" value="old">"#,
                    r#"<input type="hidden" name="ingredients-old" value="1 lb Zweigle’s Smoked Polish Kielbasa">"#,
                    r#"<input type="hidden" name="ingredients-old" value="1.5 lb potatoes (Yukon gold tastes best)">"#,
                    r#"<input type="hidden" name="ingredients-old" value="1 large white onion">"#,
                    r#"<input type="hidden" name="ingredients-old" value="Seasoning (paprika, garlic powder, kosher salt, black pepper)">"#,
                    r#"<input type="hidden" name="ingredients-old" value="2 tbsp olive oil">"#,
                    r#"<input type="hidden" name="ingredients-old" value="2 tbsp fresh Italian parsley">"#,
                    // Instructions
                    r#"<input type="hidden" name="instructions-source" value="old">"#,
                    r#"<input type="hidden" name="instructions-old" value="1. Preheat oven to 400° and line a baking sheet with parchment paper.">"#,
                    r#"<input type="hidden" name="instructions-old" value="2. Chop potatoes into uniform small pieces, place in a bowl and cover with water. Microwave until potatoes are soft enough to pierce with a fork. Drain and rinse.">"#,
                    r#"<input type="hidden" name="instructions-old" value="3. Roughly chop onion and add to large bowl.">"#,
                    r#"<input type="hidden" name="instructions-old" value="4. Cut Polish Kielbasa into 1 inch diagonal pieces and add to bowl.">"#,
                    r#"<input type="hidden" name="instructions-old" value="5. Add potatoes to the bowl and add seasoning and olive oil. Stir to coat and spread out an even layer on the baking sheet.">"#,
                    r#"<input type="hidden" name="instructions-old" value="6. Bake until sausage and potatoes are browned (around 20 mins).">"#,
                    r#"<input type="hidden" name="instructions-old" value="7. Garnish with sour cream and chopped Italian parsley.">"#,
                    // Keywords
                    r#"<input type="radio" name="keywords-source" value="new" class="radio radio-sm radio-error mx-2" checked>"#,
                    r#"<input type="hidden" name="keywords-old" value="potatoes">"#,
                    r#"<input type="hidden" name="keywords-old" value="fish">"#,
                    // Media
                    r#"<input type="hidden" name="media-source" value="old">"#,
                    &format!(
                        r#"<input type="hidden" name="media-old-image" value="/data/images/{}.webp">"#,
                        recipe.all_images()[0]
                    ),
                    // Notes
                    r#"<input type="hidden" name="notes-source" value="old">"#,
                    r#"<input type="hidden" name="notes-old" value="">"#,
                    // Nutrition
                    r#"<input type="hidden" name="nutrition-source" value="old">"#,
                    r#"<input type="hidden" name="nutrition-old-calories-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-old-total-carbohydrates-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-old-sugars-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-old-protein-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-old-total-fat-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-old-saturated-fat-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-old-unsaturated-fat-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-old-trans-fat-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-old-cholesterol-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-old-sodium-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-old-fiber-per-100g" value="-">"#,
                    // Rating
                    r#"<input type="hidden" name="rating-source" value="old">"#,
                    r#"<input type="hidden" name="rating-old" value="0">"#,
                    // Times
                    r#"<input type="hidden" name="times-source" value="old">"#,
                    r#"<input type="hidden" name="time-cook-old" value="00:20:00">"#,
                    r#"<input type="hidden" name="time-prep-old" value="00:15:00">"#,
                    // Yield
                    r#"<input type="hidden" name="yield-source" value="old">"#,
                    r#"<input type="hidden" name="yield-old" value="4">"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        #[allow(clippy::too_many_lines)]
        async fn test_scraped_recipe_all_fields_updated_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            insert_zweigles_recipe(&server, &mut ws_server).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let mut recipe = Recipe::get(&state.mm, user_id, 1).await?;
            recipe.recipe.name = "Borsch".into();
            recipe.category = "soup".into();
            recipe.recipe.description = Some("Borsch is a traditional Ukrainian soup made with beets, cabbage, and meat or vegetables.".into());
            recipe.ingredients =
                SectionComponents::new(vec!["Beets".into(), "Cabbage".into(), "Meat".into()]);
            recipe.keywords = vec!["potatoes".into(), "fish".into()];
            recipe.instructions = SectionComponents::new(vec![
                "Boil beets".into(),
                "Cook cabbage".into(),
                "Add meat".into(),
            ]);
            recipe.recipe.notes = Some("Some notes".into());
            recipe.nutrition = NutritionDetails {
                per_100g: Some(Nutrition {
                    id: 1,
                    is_precalculated_by_source: true,
                    calories_kcal: Some(45),
                    total_carbohydrates: Some(8.2),
                    sugars_g: Some(2.5),
                    protein_g: Some(1.2),
                    total_fat_g: Some(0.6),
                    saturated_fat_g: Some(0.1),
                    unsaturated_fat_g: Some(0.4),
                    cholesterol_mg: Some(0.0),
                    sodium_mg: Some(280.0),
                    fiber_g: Some(1.8),
                    trans_fat_g: Some(0.0),
                }),
                per_serving: Some(NutritionPerServingDetails {
                    nutrition: Nutrition {
                        id: 2,
                        is_precalculated_by_source: true,
                        calories_kcal: Some(432),
                        total_carbohydrates: Some(78.7),
                        sugars_g: Some(24.0),
                        protein_g: Some(11.5),
                        total_fat_g: Some(5.8),
                        saturated_fat_g: Some(1.0),
                        unsaturated_fat_g: Some(3.8),
                        cholesterol_mg: Some(0.0),
                        sodium_mg: Some(2688.0),
                        fiber_g: Some(17.3),
                        trans_fat_g: Some(0.0),
                    },
                    serving_size: "4 cups".into(),
                }),
            };
            recipe.recipe.rating = Some(4);
            recipe.times = Times {
                id: 1,
                recipe_id: 1,
                prep_seconds: 300,
                cook_seconds: 600,
                total_seconds: 900,
            };
            recipe.recipe.r#yield = 12;
            Recipe::update(
                &state.mm,
                user_id,
                1,
                &mut RecipeForCreate::from(recipe.clone()),
            )
            .await?;

            let res = server
                .get(&base_uri(1))
                .add_header(axum_htmx::HX_REQUEST, "true")
                .await;

            res.assert_status_ok();
            res.assert_header(axum_htmx::HX_PUSH_URL, "/recipes/1/rescrape");
            res.assert_header(axum_htmx::HX_RESWAP, "innerHTML transition:true");
            assert_html(
                &res,
                vec![
                    // Title
                    r#"<input type="radio" name="title-source" value="old" class="radio radio-sm radio-error">"#,
                    r#"<input type="radio" name="title-source" value="new" class="radio radio-sm radio-success" checked>"#,
                    r#"<input type="text" readonly class="input w-full text-center bg-base-200 pointer-events-none focus:outline-none" name="title-old" value="Borsch">"#,
                    r#"<input type="text" readonly class="input w-full text-center bg-base-200 pointer-events-none focus:outline-none" name="title-new" value="Polish Kielbasa Sheet Pan and Potatoes">"#,
                    // Category
                    r#"<input type="radio" name="category-source" value="old" class="radio radio-sm radio-error mx-2">"#,
                    r#"<input type="radio" name="category-source" value="new" class="radio radio-sm radio-success mx-2" checked>"#,
                    r#"<input type="hidden" name="category-old" value="soup">"#,
                    r#"<input type="hidden" name="category-new" value="uncategorized">"#,
                    // Description
                    r#"<input type="radio" name="description-source" value="old" class="radio radio-sm radio-error mx-2">"#,
                    r#"<input type="radio" name="description-source" value="new" class="radio radio-sm radio-error mx-2" checked>"#,
                    r#"<input type="hidden" name="description-old" value="Borsch is a traditional Ukrainian soup made with beets, cabbage, and meat or vegetables.">"#,
                    r#"<input type="hidden" name="description-new" value="Want a hearty and comforting meal you can make in 35 minutes? This Polish Kielbasa Sheet Pan and Potatoes is your answer. Heat up some sauerkraut or throw together a salad to go with your sausage and potatoes and enjoy!">"#,
                    // Ingredients
                    r#"<input type="radio" name="ingredients-source" value="old" class="radio radio-sm radio-error mx-2">"#,
                    r#"<input type="radio" name="ingredients-source" value="new" class="radio radio-sm radio-error mx-2" checked>"#,
                    r#"<input type="hidden" name="ingredients-old" value="Beets">"#,
                    r#"<input type="hidden" name="ingredients-old" value="Cabbage">"#,
                    r#"<input type="hidden" name="ingredients-old" value="Meat">"#,
                    r#"<input type="hidden" name="ingredients-new" value="1 lb Zweigle’s Smoked Polish Kielbasa">"#,
                    r#"<input type="hidden" name="ingredients-new" value="1.5 lb potatoes (Yukon gold tastes best)">"#,
                    r#"<input type="hidden" name="ingredients-new" value="1 large white onion">"#,
                    r#"<input type="hidden" name="ingredients-new" value="Seasoning (paprika, garlic powder, kosher salt, black pepper)">"#,
                    r#"<input type="hidden" name="ingredients-new" value="2 tbsp olive oil">"#,
                    r#"<input type="hidden" name="ingredients-new" value="2 tbsp fresh Italian parsley">"#,
                    // Instructions
                    r#"<input type="radio" name="instructions-source" value="old" class="radio radio-sm radio-error mx-2">"#,
                    r#"<input type="radio" name="instructions-source" value="new" class="radio radio-sm radio-error mx-2" checked>"#,
                    r#"<input type="hidden" name="instructions-old" value="Boil beets">"#,
                    r#"<input type="hidden" name="instructions-old" value="Cook cabbage">"#,
                    r#"<input type="hidden" name="instructions-old" value="Add meat">"#,
                    r#"<input type="hidden" name="instructions-new" value="1. Preheat oven to 400° and line a baking sheet with parchment paper.">"#,
                    r#"<input type="hidden" name="instructions-new" value="2. Chop potatoes into uniform small pieces, place in a bowl and cover with water. Microwave until potatoes are soft enough to pierce with a fork. Drain and rinse.">"#,
                    r#"<input type="hidden" name="instructions-new" value="3. Roughly chop onion and add to large bowl.">"#,
                    r#"<input type="hidden" name="instructions-new" value="4. Cut Polish Kielbasa into 1 inch diagonal pieces and add to bowl.">"#,
                    r#"<input type="hidden" name="instructions-new" value="5. Add potatoes to the bowl and add seasoning and olive oil. Stir to coat and spread out an even layer on the baking sheet.">"#,
                    r#"<input type="hidden" name="instructions-new" value="6. Bake until sausage and potatoes are browned (around 20 mins).">"#,
                    r#"<input type="hidden" name="instructions-new" value="7. Garnish with sour cream and chopped Italian parsley.">"#,
                    // Keywords
                    r#"<input type="radio" name="keywords-source" value="new" class="radio radio-sm radio-error mx-2" checked>"#,
                    r#"<input type="hidden" name="keywords-old" value="potatoes">"#,
                    r#"<input type="hidden" name="keywords-old" value="fish">"#,
                    // Media
                    r#"<input type="hidden" name="media-source" value="old">"#,
                    &format!(
                        r#"<input type="hidden" name="media-old-image" value="/data/images/{}.webp">"#,
                        recipe.all_images()[0]
                    ),
                    // Notes
                    r#"<input type="radio" name="notes-source" value="old" class="radio radio-sm radio-error mx-2">"#,
                    r#"<input type="radio" name="notes-source" value="new" class="radio radio-sm radio-success mx-2" checked>"#,
                    r#"<input type="hidden" name="notes-old" value="Some notes">"#,
                    r#"<input type="hidden" name="notes-new" value="">"#,
                    // Nutrition
                    r#"<input type="radio" name="nutrition-source" value="old" class="radio radio-sm radio-error mx-2">"#,
                    r#"<input type="hidden" name="nutrition-old-calories-per-100g" value="45 kcal">"#,
                    r#"<input type="hidden" name="nutrition-old-total-carbohydrates-per-100g" value="8g">"#,
                    r#"<input type="hidden" name="nutrition-old-sugars-per-100g" value="2g">"#,
                    r#"<input type="hidden" name="nutrition-old-protein-per-100g" value="1g">"#,
                    r#"<input type="hidden" name="nutrition-old-total-fat-per-100g" value="0.60g">"#,
                    r#"<input type="hidden" name="nutrition-old-saturated-fat-per-100g" value="0.10g">"#,
                    r#"<input type="hidden" name="nutrition-old-unsaturated-fat-per-100g" value="0.10g">"#,
                    r#"<input type="hidden" name="nutrition-old-trans-fat-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-old-cholesterol-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-old-sodium-per-100g" value="280mg">"#,
                    r#"<input type="hidden" name="nutrition-old-fiber-per-100g" value="2g">"#,
                    r#"<input type="hidden" name="serving-size" value="4 cups">"#,
                    r#"<input type="hidden" name="nutrition-old-calories-per-serving" value="432 kcal">"#,
                    r#"<input type="hidden" name="nutrition-old-total-carbohydrates-per-serving" value="79g">"#,
                    r#"<input type="hidden" name="nutrition-old-sugars-per-serving" value="24g">"#,
                    r#"<input type="hidden" name="nutrition-old-protein-per-serving" value="12g">"#,
                    r#"<input type="hidden" name="nutrition-old-total-fat-per-serving" value="6g">"#,
                    r#"<input type="hidden" name="nutrition-old-saturated-fat-per-serving" value="1g">"#,
                    r#"<input type="hidden" name="nutrition-old-unsaturated-fat-per-serving" value="1g">"#,
                    r#"<input type="hidden" name="nutrition-old-trans-fat-per-serving" value="-">"#,
                    r#"<input type="hidden" name="nutrition-old-cholesterol-per-serving" value="-">"#,
                    r#"<input type="hidden" name="nutrition-old-sodium-per-serving" value="2688mg">"#,
                    r#"<input type="hidden" name="nutrition-old-fiber-per-serving" value="17g">"#,
                    r#"<input type="radio" name="nutrition-source" value="new" class="radio radio-sm radio-success mx-2" checked>"#,
                    r#"<input type="hidden" name="nutrition-new-calories-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-new-total-carbohydrates-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-new-sugars-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-new-protein-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-new-total-fat-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-new-saturated-fat-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-new-unsaturated-fat-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-new-trans-fat-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-new-cholesterol-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-new-sodium-per-100g" value="-">"#,
                    r#"<input type="hidden" name="nutrition-new-fiber-per-100g" value="-">"#,
                    // Rating
                    r#"<input type="radio" name="rating-source" value="old" class="radio radio-sm radio-error mx-2">"#,
                    r#"<input type="radio" name="rating-source" value="new" class="radio radio-sm radio-success mx-2" checked>"#,
                    r#"<input type="hidden" name="rating-old" value="4">"#,
                    r#"<input type="hidden" name="rating-new" value="0">"#,
                    // Times
                    r#"<input type="radio" name="times-source" value="new" class="radio radio-sm radio-success mx-2" checked>"#,
                    r#"<input type="radio" name="times-source" value="old" class="radio radio-sm radio-error mx-2">"#,
                    r#"<input type="hidden" name="time-cook-old" value="00:10:00">"#,
                    r#"<input type="hidden" name="time-cook-new" value="00:20:00">"#,
                    r#"<input type="hidden" name="time-prep-old" value="00:05:00">"#,
                    r#"<input type="hidden" name="time-prep-new" value="00:15:00">"#,
                    // Yield
                    r#"<input type="radio" name="yield-source" value="old" class="radio radio-sm radio-error">"#,
                    r#"<input type="radio" name="yield-source" value="new" class="radio radio-sm radio-error" readonly checked>"#,
                    r#"<input type="hidden" name="yield-old" value="12">"#,
                    r#"<input type="hidden" name="yield-new" value="4">"#,
                ],
            );
            Ok(())
        }
    }

    mod tests_put {
        use models::recipe::structs::{
            nutrition::{Nutrition, NutritionDetails, NutritionPerServingDetails},
            section::{Item, SectionComponents, SectionItem},
            time::Times,
            tool::ToolRecipe,
        };
        use uuid::Uuid;

        use super::*;

        #[tokio::test]
        async fn test_scraped_recipe_diff_title_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            insert_zweigles_recipe(&server, &mut ws_server).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let form: Vec<(&str, &str)> =
                vec![("title-source", "new"), ("title-new", "Bluerry Pie")];

            let res = server
                .put(&base_uri(1))
                .form(&form)
                .add_header(axum_htmx::HX_REQUEST, "true")
                .await;

            res.assert_status_see_other();
            res.assert_header("hx-redirect", "/recipes/1");
            let got = Recipe::get(&state.mm, user_id, 1).await?;
            assert_eq!(got.recipe.name, "Bluerry Pie");
            Ok(())
        }

        #[tokio::test]
        async fn test_scraped_recipe_diff_description_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            insert_zweigles_recipe(&server, &mut ws_server).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let form: Vec<(&str, &str)> = vec![
                ("description-source", "new"),
                ("description-new", "Bluerry description"),
            ];

            let res = server
                .put(&base_uri(1))
                .form(&form)
                .add_header(axum_htmx::HX_REQUEST, "true")
                .await;

            res.assert_status_see_other();
            res.assert_header("hx-redirect", "/recipes/1");
            let got = Recipe::get(&state.mm, user_id, 1).await?;
            assert_eq!(got.recipe.description, Some("Bluerry description".into()));
            Ok(())
        }

        #[tokio::test]
        async fn test_scraped_recipe_diff_media_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            insert_zweigles_recipe(&server, &mut ws_server).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;

            let res = server
                .put(&base_uri(1))
                .form(&vec![
                    ("media-source", "new"),
                    (
                        "media-new-image",
                        &format!("/example.com/{}.webp", Uuid::new_v4()),
                    ),
                    (
                        "media-new-image",
                        &format!("/example.com/{}.webp", Uuid::new_v4()),
                    ),
                    (
                        "media-new-video",
                        &format!("/example.com/{}.webm", Uuid::new_v4()),
                    ),
                ])
                .add_header(axum_htmx::HX_REQUEST, "true")
                .await;

            res.assert_status_see_other();
            res.assert_header("hx-redirect", "/recipes/1");
            let got = Recipe::get(&state.mm, user_id, 1).await?;
            assert_eq!(got.all_images().len(), 2);
            assert_eq!(got.num_videos(), 1);
            Ok(())
        }

        #[tokio::test]
        async fn test_scraped_recipe_diff_notes_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            insert_zweigles_recipe(&server, &mut ws_server).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let form: Vec<(&str, &str)> =
                vec![("notes-source", "new"), ("notes-new", "The best notes")];

            let res = server
                .put(&base_uri(1))
                .form(&form)
                .add_header(axum_htmx::HX_REQUEST, "true")
                .await;

            res.assert_status_see_other();
            res.assert_header("hx-redirect", "/recipes/1");
            let got = Recipe::get(&state.mm, user_id, 1).await?;
            assert_eq!(got.recipe.notes, Some("The best notes".into()));
            Ok(())
        }

        #[tokio::test]
        async fn test_scraped_recipe_diff_source_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            insert_zweigles_recipe(&server, &mut ws_server).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let form: Vec<(&str, &str)> = vec![];

            let res = server
                .put(&base_uri(1))
                .form(&form)
                .add_header(axum_htmx::HX_REQUEST, "true")
                .await;

            res.assert_status_see_other();
            res.assert_header("hx-redirect", "/recipes/1");
            let got = Recipe::get(&state.mm, user_id, 1).await?;
            assert_eq!(
                got.recipe.source,
                Source::Url(
                    "https://zweigles.com/recipes/polish-kielbasa-sheet-pan-and-potatoes".into()
                )
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_scraped_recipe_diff_rating_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            insert_zweigles_recipe(&server, &mut ws_server).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let form: Vec<(&str, &str)> = vec![("rating-source", "new"), ("rating-new", "3")];

            let res = server
                .put(&base_uri(1))
                .form(&form)
                .add_header(axum_htmx::HX_REQUEST, "true")
                .await;

            res.assert_status_see_other();
            res.assert_header("hx-redirect", "/recipes/1");
            let got = Recipe::get(&state.mm, user_id, 1).await?;
            assert_eq!(got.recipe.rating, Some(3));
            Ok(())
        }

        #[tokio::test]
        async fn test_scraped_recipe_diff_yield_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            insert_zweigles_recipe(&server, &mut ws_server).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let form: Vec<(&str, &str)> = vec![("yield-source", "new"), ("yield-new", "20")];

            let res = server
                .put(&base_uri(1))
                .form(&form)
                .add_header(axum_htmx::HX_REQUEST, "true")
                .await;

            res.assert_status_see_other();
            res.assert_header("hx-redirect", "/recipes/1");
            let got = Recipe::get(&state.mm, user_id, 1).await?;
            assert_eq!(got.recipe.r#yield, 20);
            Ok(())
        }

        #[tokio::test]
        async fn test_scraped_recipe_diff_category_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            insert_zweigles_recipe(&server, &mut ws_server).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let form: Vec<(&str, &str)> =
                vec![("category-source", "new"), ("category-new", "lunch")];

            let res = server
                .put(&base_uri(1))
                .form(&form)
                .add_header(axum_htmx::HX_REQUEST, "true")
                .await;

            res.assert_status_see_other();
            res.assert_header("hx-redirect", "/recipes/1");
            let got = Recipe::get(&state.mm, user_id, 1).await?;
            assert_eq!(got.category, "lunch".to_string());
            Ok(())
        }

        #[tokio::test]
        async fn test_scraped_recipe_diff_cuisine_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            insert_zweigles_recipe(&server, &mut ws_server).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let form: Vec<(&str, &str)> =
                vec![("cuisine-source", "new"), ("cuisine-new", "tunisian")];

            let res = server
                .put(&base_uri(1))
                .form(&form)
                .add_header(axum_htmx::HX_REQUEST, "true")
                .await;

            res.assert_status_see_other();
            res.assert_header("hx-redirect", "/recipes/1");
            let got = Recipe::get(&state.mm, user_id, 1).await?;
            assert_eq!(got.cuisine, Some("tunisian".to_string()));
            Ok(())
        }

        #[tokio::test]
        async fn test_scraped_recipe_diff_ingredients_flat_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            insert_zweigles_recipe(&server, &mut ws_server).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let new_ingredients = ["4 chicken legs".to_string(), "3 potatoes".to_string()];

            let mut form: Vec<(&str, &str)> = vec![("ingredients-source", "new")];
            for v in &new_ingredients {
                form.push(("ingredients-new", v.as_str()));
            }

            let res = server
                .put(&base_uri(1))
                .form(&form)
                .add_header(axum_htmx::HX_REQUEST, "true")
                .await;

            res.assert_status_see_other();
            res.assert_header("hx-redirect", "/recipes/1");
            let got = Recipe::get(&state.mm, user_id, 1).await?;
            assert_eq!(
                got.ingredients,
                SectionComponents::Flat(
                    new_ingredients
                        .into_iter()
                        .map(Item::new)
                        .collect::<Vec<_>>()
                )
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_scraped_recipe_diff_ingredients_grouped_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            insert_zweigles_recipe(&server, &mut ws_server).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let sections = [
                ("Marinade", vec!["4 chicken legs", "2 tbsp olive oil"]),
                ("Sides", vec!["3 potatoes", "1 cup rice"]),
            ];

            let mut form: Vec<(String, String)> = vec![("ingredients-source".into(), "new".into())];
            for (section_title, items) in &sections {
                for item in items {
                    form.push((
                        format!("ingredients-new<>{section_title}"),
                        item.to_string(),
                    ));
                }
            }

            let res = server
                .put(&base_uri(1))
                .form(&form)
                .add_header(axum_htmx::HX_REQUEST, "true")
                .await;

            res.assert_status_see_other();
            res.assert_header("hx-redirect", "/recipes/1");
            let got = Recipe::get(&state.mm, user_id, 1).await?;
            assert_eq!(
                got.ingredients,
                SectionComponents::Grouped(vec![
                    SectionItem {
                        title: "Marinade".into(),
                        items: vec![Item::new("4 chicken legs"), Item::new("2 tbsp olive oil")],
                    },
                    SectionItem {
                        title: "Sides".into(),
                        items: vec![Item::new("3 potatoes"), Item::new("1 cup rice")],
                    },
                ])
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_scraped_recipe_diff_instructions_flat_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            insert_zweigles_recipe(&server, &mut ws_server).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let new_instructions = [
                "Mix all ingredients".to_string(),
                "Bake for a long time".to_string(),
            ];

            let mut form: Vec<(&str, &str)> = vec![("instructions-source", "new")];
            for v in &new_instructions {
                form.push(("instructions-new", v.as_str()));
            }

            let res = server
                .put(&base_uri(1))
                .form(&form)
                .add_header(axum_htmx::HX_REQUEST, "true")
                .await;

            res.assert_status_see_other();
            res.assert_header("hx-redirect", "/recipes/1");
            let got = Recipe::get(&state.mm, user_id, 1).await?;
            assert_eq!(
                got.instructions,
                SectionComponents::Flat(
                    new_instructions
                        .into_iter()
                        .map(Item::new)
                        .collect::<Vec<_>>()
                )
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_scraped_recipe_diff_instructions_grouped_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            insert_zweigles_recipe(&server, &mut ws_server).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let sections = [
                (
                    "Marinade",
                    vec!["Mix all marinade ingredients", "Let soak overnight"],
                ),
                (
                    "Sides",
                    vec![
                        "Cut potatoes",
                        "Bake them in the oven",
                        "Mix marinade with baked potatoes",
                    ],
                ),
            ];

            let mut form: Vec<(String, String)> =
                vec![("instructions-source".into(), "new".into())];
            for (section_title, items) in &sections {
                for item in items {
                    form.push((
                        format!("instructions-new<>{section_title}"),
                        item.to_string(),
                    ));
                }
            }

            let res = server
                .put(&base_uri(1))
                .form(&form)
                .add_header(axum_htmx::HX_REQUEST, "true")
                .await;

            res.assert_status_see_other();
            res.assert_header("hx-redirect", "/recipes/1");
            let got = Recipe::get(&state.mm, user_id, 1).await?;
            assert_eq!(
                got.instructions,
                SectionComponents::Grouped(vec![
                    SectionItem {
                        title: "Marinade".into(),
                        items: vec![
                            Item::new("Mix all marinade ingredients"),
                            Item::new("Let soak overnight")
                        ],
                    },
                    SectionItem {
                        title: "Sides".into(),
                        items: vec![
                            Item::new("Cut potatoes"),
                            Item::new("Bake them in the oven"),
                            Item::new("Mix marinade with baked potatoes")
                        ],
                    },
                ])
            );
            Ok(())
        }

        #[tokio::test]
        #[tracing_test::traced_test]
        async fn test_scraped_recipe_diff_keywords_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            insert_zweigles_recipe(&server, &mut ws_server).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let new_keywords = ["fish".to_string(), "potatoes".to_string()];

            let mut form: Vec<(&str, &str)> = vec![("keywords-source", "new")];
            for kw in &new_keywords {
                form.push(("keywords-new", kw.as_str()));
            }

            let res = server
                .put(&base_uri(1))
                .form(&form)
                .add_header(axum_htmx::HX_REQUEST, "true")
                .await;

            res.assert_status_see_other();
            res.assert_header("hx-redirect", "/recipes/1");
            let got = Recipe::get(&state.mm, user_id, 1).await?;
            assert_eq!(got.keywords.as_slice(), new_keywords);
            Ok(())
        }

        #[tokio::test]
        #[ignore = "search for a scraped recipe that precalculates nutrition"]
        async fn test_scraped_recipe_diff_nutrition_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            insert_zweigles_recipe(&server, &mut ws_server).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;

            let res = server
                .put(&base_uri(1))
                .form(&[
                    ("nutrition-source", "new"),
                    ("nutrition-new-calories-per-100g", "1g"),
                    ("nutrition-new-calories-per-serving", "2g"),
                    ("nutrition-new-total-carbohydrates-per-100g", "3g"),
                    ("nutrition-new-total-carbohydrates-per-serving", "4g"),
                    ("nutrition-new-sugars-per-100g", "5g"),
                    ("nutrition-new-sugars-per-serving", "6g"),
                    ("nutrition-new-protein-per-100g", "7g"),
                    ("nutrition-new-protein-per-serving", "8g"),
                    ("nutrition-new-total-fat-per-100g", "9g"),
                    ("nutrition-new-total-fat-per-serving", "10g"),
                    ("nutrition-new-saturated-fat-per-100g", "11g"),
                    ("nutrition-new-saturated-fat-per-serving", "12g"),
                    ("nutrition-new-unsaturated-fat-per-100g", "13g"),
                    ("nutrition-new-unsaturated-fat-per-serving", "14g"),
                    ("nutrition-new-trans-fat-per-100g", "15g"),
                    ("nutrition-new-trans-fat-per-serving", "16g"),
                    ("nutrition-new-cholesterol-per-100g", "17g"),
                    ("nutrition-new-cholesterol-per-serving", "18g"),
                    ("nutrition-new-sodium-per-100g", "19g"),
                    ("nutrition-new-sodium-per-serving", "20g"),
                    ("nutrition-new-fiber-per-100g", "21g"),
                    ("nutrition-new-fiber-per-serving", "22g"),
                ])
                .add_header(axum_htmx::HX_REQUEST, "true")
                .await;

            res.assert_status_see_other();
            res.assert_header("hx-redirect", "/recipes/1");
            let got = Recipe::get(&state.mm, user_id, 1).await?;
            assert_eq!(
                got.nutrition,
                NutritionDetails {
                    per_100g: Some(Nutrition {
                        id: 1,
                        is_precalculated_by_source: true,
                        calories_kcal: Some(1),
                        total_carbohydrates: Some(3.),
                        sugars_g: Some(5.),
                        protein_g: Some(7.),
                        total_fat_g: Some(9.),
                        saturated_fat_g: Some(11.),
                        unsaturated_fat_g: Some(13.),
                        cholesterol_mg: Some(17.),
                        sodium_mg: Some(19.),
                        fiber_g: Some(21.),
                        trans_fat_g: Some(15.),
                    }),
                    per_serving: Some(NutritionPerServingDetails {
                        nutrition: Nutrition {
                            id: 2,
                            is_precalculated_by_source: true,
                            calories_kcal: Some(2),
                            total_carbohydrates: Some(4.),
                            sugars_g: Some(6.),
                            protein_g: Some(8.),
                            total_fat_g: Some(10.),
                            saturated_fat_g: Some(12.),
                            unsaturated_fat_g: Some(14.),
                            cholesterol_mg: Some(18.),
                            sodium_mg: Some(20.),
                            fiber_g: Some(22.),
                            trans_fat_g: Some(16.),
                        },
                        serving_size: String::new(),
                    })
                }
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_scraped_recipe_diff_times_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            insert_zweigles_recipe(&server, &mut ws_server).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let form: Vec<(&str, &str)> = vec![
                ("times-source", "new"),
                ("prep-new", "00:23:00"),
                ("cook-new", "00:55:00"),
            ];

            let res = server
                .put(&base_uri(1))
                .form(&form)
                .add_header(axum_htmx::HX_REQUEST, "true")
                .await;

            res.assert_status_see_other();
            res.assert_header("hx-redirect", "/recipes/1");
            let got = Recipe::get(&state.mm, user_id, 1).await?;
            assert_eq!(
                got.times,
                Times {
                    id: 1,
                    recipe_id: 1,
                    prep_seconds: 1380,
                    cook_seconds: 3300,
                    total_seconds: 4680,
                }
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_scraped_recipe_diff_tools_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            insert_zweigles_recipe(&server, &mut ws_server).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let new_tools = ["1 wok".to_string(), "big knife".to_string()];

            let mut form: Vec<(&str, &str)> = vec![("tools-source", "new")];
            for v in &new_tools {
                form.push(("tools-new", v.as_str()));
            }

            let res = server
                .put(&base_uri(1))
                .form(&form)
                .add_header(axum_htmx::HX_REQUEST, "true")
                .await;

            res.assert_status_see_other();
            res.assert_header("hx-redirect", "/recipes/1");
            let got = Recipe::get(&state.mm, user_id, 1).await?;
            assert_eq!(
                got.tools.as_slice(),
                new_tools
                    .into_iter()
                    .enumerate()
                    .map(|(idx, s)| ToolRecipe {
                        name: s,
                        quantity: 0,
                        tool_order: i16::try_from(idx + 1).unwrap_or_default(),
                    })
                    .collect::<Vec<_>>()
            );
            Ok(())
        }
    }

    async fn insert_zweigles_recipe(
        server: &TestServer,
        ws_server: &mut TestWebSocket,
    ) -> Result<()> {
        let form = RecipeScrapeForm {
            urls: "https://zweigles.com/recipes/polish-kielbasa-sheet-pan-and-potatoes".into(),
        };
        scrape_test_websites(1).await?;

        let res = server.post("/recipes/add/website").form(&form).await;

        res.assert_status(StatusCode::ACCEPTED);
        assert_ws_message(ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetching recipes</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">0 of 1</span><span class="font-semibold">0.0%</span></div><div id="export-progress"><progress max="100" value="0.00"></progress></div></div></div>"# ).await;
        assert_ws_message(ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default hidden"><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1"></p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">-1 of -1</span><span class="font-semibold">0.0%</span></div><div id="export-progress"><progress max="100" value="0.00"></progress></div></div></div>"# ).await;
        assert_ws_message(ws_server, r#"{"showMessageHtmx":{"type":"toast","action":"View /recipes/1","message":"Recipe has been added to your collection.","status":"alert-info","title":"Operation Successful"}}"# ).await;
        Ok(())
    }
}
