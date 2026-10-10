use axum::http::HeaderValue;
use axum_test::TestResponse;
use reqwest::Method;
use time::Duration;
use uuid::Uuid;

use models::{
    Recipe, RecipeDetails,
    recipe::structs::{
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
    },
    settings::UserSettingDetails,
    user::User,
};
use test_db::default_config;
use test_fixtures::RecipeImages;
use test_harness::assert_sse_message;
use test_harness::{assert_must_be_logged_in, build_server_logged_in, build_server_sse};
use test_models::a_complete_recipe_for_create;

use crate::handlers::helpers::create_form;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

fn base_uri(recipe_id: i64) -> String {
    format!("/recipes/{recipe_id}/edit")
}

#[tokio::test]
async fn test_must_be_logged_in_ok() -> Result<()> {
    let _ = assert_must_be_logged_in(Method::GET, &base_uri(1)).await;
    assert_must_be_logged_in(Method::PUT, &base_uri(1)).await
}

mod tests_get {
    use super::*;

    #[tokio::test]
    async fn test_edit_recipe_not_exist_ok() -> Result<()> {
        let (server, mut ws_server, _) = build_server_sse(default_config()).await?;

        let res = server.get(&base_uri(1)).await;

        res.assert_status_not_found();
        assert_sse_message(&mut ws_server, r#"{"notification":{"type":"toast","message":"Recipe not found.","status":"alert-error","title":"Operation failed"}}"# ).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_edit_recipe_exists_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (recipe, images) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

        let res = server.get(&base_uri(recipe_id)).await;

        assert_recipe_form(&res, &images, recipe_id);
        Ok(())
    }

    #[tokio::test]
    async fn test_recipe_exists_htmx_request_ok() -> Result<()> {
        let (mut server, state) = build_server_logged_in(default_config()).await?;
        server.add_header(axum_htmx::HX_REQUEST, HeaderValue::from_static("true"));
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (recipe, images) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

        let res = server.get(&base_uri(recipe_id)).await;

        assert_recipe_form(&res, &images, recipe_id);
        Ok(())
    }
}

mod tests_put {
    use super::*;

    #[tokio::test]
    async fn test_missing_required_title_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let recipe_id = Recipe::all(&state.mm, user_id)
            .await?
            .last()
            .map_or(1, |r| r.id);
        let recipe = RecipeForCreate {
            ingredients: SectionComponents::Flat(vec![Item::new("1 apple")]),
            instructions: SectionComponents::Flat(vec![Item::new("Mix the apples")]),
            ..Default::default()
        };

        let res = server
            .put(&base_uri(recipe_id))
            .multipart(create_form(&recipe))
            .await;

        res.assert_status_bad_request();
        Ok(())
    }

    #[tokio::test]
    async fn test_missing_required_ingredients_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let recipe_id = Recipe::all(&state.mm, user_id)
            .await?
            .last()
            .map_or(1, |r| r.id);
        let recipe = RecipeForCreate {
            name: "Best Chinese Kale".to_string(),
            instructions: SectionComponents::Flat(vec![Item::new("Mix the apples")]),
            ..Default::default()
        };

        let res = server
            .put(&base_uri(recipe_id))
            .multipart(create_form(&recipe))
            .await;

        res.assert_status_bad_request();
        Ok(())
    }

    #[tokio::test]
    async fn test_missing_required_instructions_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let recipe_id = Recipe::all(&state.mm, user_id)
            .await?
            .last()
            .map_or(1, |r| r.id);
        let recipe = RecipeForCreate {
            name: "Best Chinese Kale".to_string(),
            ingredients: SectionComponents::Flat(vec![Item::new("8 apples")]),
            ..Default::default()
        };

        let res = server
            .put(&base_uri(recipe_id))
            .multipart(create_form(&recipe))
            .await;

        res.assert_status_bad_request();
        Ok(())
    }

    #[tokio::test]
    async fn test_update_image_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (mut recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
        recipe.images = Vec::new();
        recipe.videos = Vec::new();

        let res = server
            .put(&base_uri(recipe_id))
            .multipart(create_form(&recipe))
            .await;

        res.assert_status_see_other();
        let got = Recipe::get(&state.mm, user_id, recipe_id).await?;
        let mut expected = recipe_for_create_to_details(recipe, &got, user_id);
        expected.ingredients = got.ingredients.clone();
        expected.instructions = got.instructions.clone();
        pretty_assertions::assert_eq!(got, expected);
        assert!(got.recipe.image.is_none());
        pretty_assertions::assert_eq!(got.additional_images.len(), 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_missing_fields_defaults_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (mut recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
        recipe.name = "Maple Syrup Korean Chicken".into();
        recipe.ingredients = SectionComponents::Flat(vec![Item::new("4 apples")]);
        recipe.instructions = SectionComponents::Flat(vec![Item::new("Drink juice")]);

        let res = server
            .put(&base_uri(recipe_id))
            .multipart(create_form(&recipe))
            .await;

        res.assert_status_see_other();
        let mut got = Recipe::get(&state.mm, user_id, recipe_id).await?;
        if let SectionComponents::Flat(instructions) = &mut got.instructions {
            for i in instructions.iter_mut() {
                i.id = None;
            }
        }
        let expected = recipe_for_create_to_details(recipe, &got, user_id);
        pretty_assertions::assert_eq!(got, expected);
        assert!(got.recipe.image.is_some());
        pretty_assertions::assert_eq!(
            got.additional_images.len(),
            expected.additional_images.len()
        );
        Ok(())
    }

    #[tracing_test::traced_test]
    #[tokio::test]
    async fn test_can_only_be_one_category_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (mut recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
        recipe.category = Some("breakfast,dinner".into());

        let res = server
            .put(&base_uri(recipe_id))
            .multipart(create_form(&recipe))
            .await;

        res.assert_status_see_other();
        let got = Recipe::get(&state.mm, user_id, recipe_id).await?;
        pretty_assertions::assert_eq!(got.category, "breakfast");
        Ok(())
    }

    #[tokio::test]
    async fn test_subcategories_are_possible() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let mut recipe = RecipeForCreate {
            name: "Best Chinese Kale".to_string(),
            instructions: SectionComponents::Flat(vec![Item::new("Mix the apples")]),
            ingredients: SectionComponents::Flat(vec![Item::new("8 apples")]),
            ..Default::default()
        };
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
        recipe.category = Some("drinks:vodka".into());

        let res = server
            .put(&base_uri(recipe_id))
            .multipart(create_form(&recipe))
            .await;

        res.assert_status_see_other();
        let got = Recipe::get(&state.mm, user_id, recipe_id).await?;
        pretty_assertions::assert_eq!(got.category, "drinks:vodka");
        Ok(())
    }

    #[tokio::test]
    async fn test_submit_recipe_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (mut recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
        recipe = RecipeForCreate {
            name: "Crepes".into(),
            description: Some("Trust me. They're delicious.".into()),
            images: vec![Uuid::new_v4(), Uuid::new_v4()],
            measurement_system_id: 2,
            r#yield: Some(12),
            source: Source::new("My father's maple syrup recipes cookbook"),
            is_favourite: false,
            rating: Some(4),
            videos: vec![VideoForCreate {
                video: Uuid::new_v4(),
                duration: Some(Duration::minutes(30)),
                content_url: Some("https://www.youtube.com/watch?v=2".into()),
                embed_url: Some("https://www.youtube.com/embed/embeded".into()),
            }],
            category: Some("breakfast".into()),
            instructions: SectionComponents::Grouped(vec![
                SectionItem::new(
                    "",
                    vec![
                        Item::new("Mix the blueberries"),
                        Item::new("Mix the strawberries"),
                    ],
                ),
                SectionItem::new("Finalize", vec![Item::new("Whisk the fruits until smooth")]),
            ]),
            keywords: vec!["blueberries".into(), "vegan".into()],
            notes: Some("# Ze notes\n\nbip bop".into()),
            nutrition: NutritionDetailsForCreate {
                per_100g: Some(NutritionForCreate {
                    calories_kcal: Some(100),
                    total_carbohydrates: Some(20.),
                    sugars_g: Some(30.),
                    protein_g: Some(40.),
                    total_fat_g: Some(50.),
                    saturated_fat_g: Some(60.),
                    unsaturated_fat_g: Some(70.),
                    cholesterol_mg: Some(80.),
                    sodium_mg: Some(90.),
                    fibre_g: Some(100.),
                    trans_fat_g: Some(110.),
                }),
                per_serving: Some(NutritionPerServingDetailsForCreate {
                    nutrition: NutritionForCreate {
                        calories_kcal: Some(100),
                        total_carbohydrates: Some(20.),
                        sugars_g: Some(30.),
                        protein_g: Some(40.),
                        total_fat_g: Some(50.),
                        saturated_fat_g: Some(60.),
                        unsaturated_fat_g: Some(70.),
                        cholesterol_mg: Some(80.),
                        sodium_mg: Some(90.),
                        fibre_g: Some(100.),
                        trans_fat_g: Some(110.),
                    },
                    serving_size: "2 buns".into(),
                }),
            },
            times: Some(TimesForCreate {
                prep_seconds: 2000,
                cook_seconds: 800,
            }),
            ingredients: SectionComponents::Flat(vec![
                Item::new("8 lbs blueberries"),
                Item::new("12 lbs strawberries"),
            ]),
            cuisine: Some("quebec".into()),
            tools: vec![ToolForCreate {
                name: "medium bowl".into(),
                quantity: 1,
            }],
        };

        let res = server
            .put(&base_uri(recipe_id))
            .multipart(create_form(&recipe))
            .await;

        res.assert_status_see_other();
        let got = Recipe::get(&state.mm, user_id, recipe_id).await?;
        let mut expected = recipe_for_create_to_details(recipe, &got, user_id);
        expected.ingredients = got.ingredients.clone();
        expected.instructions = got.instructions.clone();
        pretty_assertions::assert_eq!(got, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_submit_recipe_groups_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (mut recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
        recipe = RecipeForCreate {
            name: "Crepes".into(),
            measurement_system_id: 2,
            category: Some("breakfast".into()),
            instructions: SectionComponents::Flat(vec![
                Item::new("Mix the blueberries"),
                Item::new("Mix the strawberries"),
            ]),
            ingredients: SectionComponents::Grouped(vec![
                SectionItem::new(
                    "Prepare",
                    vec![
                        Item::new("8 pints of strawberries"),
                        Item::new("4 pounds blueberries"),
                    ],
                ),
                SectionItem::new("Finalize", vec![Item::new("1 cup of sugar")]),
            ]),
            nutrition: NutritionDetailsForCreate {
                per_100g: Some(NutritionForCreate {
                    calories_kcal: Some(100),
                    total_carbohydrates: Some(20.),
                    sugars_g: Some(30.),
                    protein_g: Some(40.),
                    total_fat_g: Some(50.),
                    saturated_fat_g: Some(60.),
                    unsaturated_fat_g: Some(70.),
                    cholesterol_mg: Some(80.),
                    sodium_mg: Some(90.),
                    fibre_g: Some(100.),
                    trans_fat_g: Some(110.),
                }),
                per_serving: Some(NutritionPerServingDetailsForCreate {
                    nutrition: NutritionForCreate {
                        calories_kcal: Some(100),
                        total_carbohydrates: Some(20.),
                        sugars_g: Some(30.),
                        protein_g: Some(40.),
                        total_fat_g: Some(50.),
                        saturated_fat_g: Some(60.),
                        unsaturated_fat_g: Some(70.),
                        cholesterol_mg: Some(80.),
                        sodium_mg: Some(90.),
                        fibre_g: Some(100.),
                        trans_fat_g: Some(110.),
                    },
                    serving_size: "2 buns".into(),
                }),
            },
            ..Default::default()
        };

        let res = server
            .put(&base_uri(recipe_id))
            .multipart(create_form(&recipe))
            .await;

        res.assert_status_see_other();
        let mut got = Recipe::get(&state.mm, user_id, recipe_id).await?;
        if let SectionComponents::Flat(instructions) = &mut got.instructions {
            for i in instructions.iter_mut() {
                i.id = None;
            }
        }
        let expected = recipe_for_create_to_details(recipe, &got, user_id);
        pretty_assertions::assert_eq!(got.ingredients, expected.ingredients);
        pretty_assertions::assert_eq!(got.instructions, expected.instructions);
        Ok(())
    }

    #[tokio::test]
    async fn test_duplicates_are_removed_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (mut recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
        recipe.instructions = SectionComponents::Flat(vec![
            Item::new("Mix the apples"),
            Item::new("Eat"),
            Item::new("Mix the apples"),
        ]);
        recipe.ingredients = SectionComponents::Flat(vec![
            Item::new("8 apples"),
            Item::new("4 oranges"),
            Item::new("8 apples"),
        ]);
        recipe.keywords = vec!["drinks".into(), "vodka".into(), "drinks".into()];
        recipe.tools = vec![
            ToolForCreate {
                quantity: 1,
                name: "1 wok".into(),
            },
            ToolForCreate {
                quantity: 1,
                name: "1 wok".into(),
            },
        ];

        let res = server
            .put(&base_uri(recipe_id))
            .multipart(create_form(&recipe))
            .await;

        res.assert_status_see_other();
        let mut got = Recipe::get(&state.mm, user_id, recipe_id).await?;
        if let SectionComponents::Flat(instructions) = &mut got.instructions {
            for i in instructions.iter_mut() {
                i.id = None;
            }
        }
        pretty_assertions::assert_eq!(
            got.keywords,
            vec!["drinks".to_string(), "vodka".to_string()]
        );
        pretty_assertions::assert_eq!(
            got.ingredients,
            SectionComponents::Flat(vec![Item::new("8 apples"), Item::new("4 oranges")])
        );
        pretty_assertions::assert_eq!(
            got.instructions,
            SectionComponents::Flat(vec![Item::new("Mix the apples"), Item::new("Eat"),])
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
}

fn assert_recipe_form(res: &TestResponse, images: &RecipeImages, recipe_id: i64) {
    let body = res.text();
    let normalized = body.split_whitespace().collect::<Vec<_>>().join(" ");

    let expected = [
        "<title hx-swap-oob=\"true\">Edit \u{2068}Best Chinese Kale\u{2069} | Recipya</title>",
        &format!(
            r##"<form class="card-body contents" style="padding: 0" enctype="multipart/form-data" hx-put="/recipes/{recipe_id}/edit" hx-indicator="#fullscreen-loader">"##
        ),
        r#"<input required type="text" name="title" placeholder="Title of the recipe*" autocomplete="off" class="input w-full text-center rounded-t-lg rounded-b-none bg-base-200" value="Best Chinese Kale">"#,
        &format!(
            "<img src=\"/data/images/{}.webp\" alt=\"Image \u{2068}1\u{2069} of the recipe\" class=\"block w-full h-full object-contain\"><input type=\"hidden\" name=\"media-existing-image\" value=\"/data/images/{}.webp\">",
            images.main, images.main
        ),
        &format!(
            "<img src=\"/data/images/{}.webp\" alt=\"Image \u{2068}2\u{2069} of the recipe\" class=\"block w-full h-full object-contain\"><input type=\"hidden\" name=\"media-existing-image\" value=\"/data/images/{}.webp\">",
            images.additional, images.additional
        ),
        &format!(
            r#"<img src="" alt="" class="mb-2"><video controls class="mb-2" src="/data/videos/{}.webm" type="video/webm"></video><span class="grid gap-1 max-w-sm" style="margin: auto auto 0.25rem;"><div class="mr-1 hidden"><input type="file" accept="image/*,video/*" name="media" class="file-input file-input-sm file-input-bordered w-full max-w-sm" value="/data/videos/{}.webm"#,
            images.video, images.video
        ),
        r#"<input id="servings" type="number" min="1" name="yield" value="4" class="input input-sm w-11/12">"#,
        r#"<input id="category" type="text" list="categories" name="category" class="input input-sm w-11/12" placeholder="breakfast" autocomplete="off" value="dinner"><datalist id="categories"><option>uncategorized</option><option>appetizers</option><option>bread</option><option>breakfasts</option><option>condiments</option><option>dessert</option><option>lunch</option><option>main dish</option><option>salad</option><option>side dish</option><option>snacks</option><option>soups</option><option>stews</option><option>dinner</option></datalist>"#,
        r#"<textarea name="description" placeholder="This Thai curry chicken will make you drool." class="textarea w-full h-full resize-none rounded-none focus:outline-none">This is the most delicious recipe!</textarea>"#,
        r#"<input type="text" name="tool" placeholder="1 frying pan" class="input input-bordered input-sm w-full" value="1 frying pan" _="on keydown if event.key is 'Enter' halt the event then call addItem(event)">"#,
        r#"<h2 class="font-semibold text-center pb-2"><span class="underline">Ingredients</span><sup class="text-red-600">*</sup></h2>"#,
        "<ol class=\"pl-4\" id=\"ingredients-list\"><li class=\"list-none\"><div class=\"divider\"><div class=\"grid grid-flow-col gap-2 w-full\"><input type=\"text\" name=\"section-ingredient\" placeholder=\"Section name\" class=\"input input-sm w-fit\" value=\"Sauce\" onfocusout=\"renumberSections(this.closest('ol'), 'ingredient')\"><btn class=\"btn btn-xs btn-square\" onclick=\"deleteSection(this, 'ingredient')\"><svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"m9.75 9.75 4.5 4.5m0-4.5-4.5 4.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z\"></path></svg></btn></div></div></li><li class=\"pb-2\" data-drag-row><div class=\"grid grid-flow-col items-center\"><label class=\"flex gap-1\"><div class=\"inline-flex size-6 cursor-grab handle mt-1 items-center justify-center text-2xl\" data-drageable draggable=\"true\" data-drag-handle>⠿</div><input required type=\"text\" name=\"ingredient&lt;&gt;Sauce\" value=\"1 cup blue spinach\" placeholder=\"1 cup of chopped onions\" class=\"input input-sm\" _=\"on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))\"></label><div class=\"ml-2 flex gap-2\"><button type=\"button\" class=\"btn btn-sm btn-outline btn-success\" title=\"Shortcut: \u{2068}Enter\u{2069}\" onclick=\"addItem(event)\"><svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"M12 4.5v15m7.5-7.5h-15\"></svg>Add</button><button type=\"button\" class=\"delete-button btn btn-square btn-sm btn-outline btn-error\" onclick=\"deleteItem(this, 'ingredient')\"><svg xmlns=\"http://www.w3.org/2000/svg\" class=\"size-6 hover:text-red-600\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0\"></path></svg></button></div></div></li><li class=\"pb-2\" data-drag-row><div class=\"grid grid-flow-col items-center\"><label class=\"flex gap-1\"><div class=\"inline-flex size-6 cursor-grab handle mt-1 items-center justify-center text-2xl\" data-drageable draggable=\"true\" data-drag-handle>⠿</div><input required type=\"text\" name=\"ingredient&lt;&gt;Sauce\" value=\"1/2 tbsp cinnamon\" placeholder=\"1 cup of chopped onions\" class=\"input input-sm\" _=\"on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))\"></label><div class=\"ml-2 flex gap-2\"><button type=\"button\" class=\"btn btn-sm btn-outline btn-success\" title=\"Shortcut: \u{2068}Enter\u{2069}\" onclick=\"addItem(event)\"><svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"M12 4.5v15m7.5-7.5h-15\"></svg>Add</button><button type=\"button\" class=\"delete-button btn btn-square btn-sm btn-outline btn-error\" onclick=\"deleteItem(this, 'ingredient')\"><svg xmlns=\"http://www.w3.org/2000/svg\" class=\"size-6 hover:text-red-600\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0\"></path></svg></button></div></div></li><li class=\"list-none\"><div class=\"divider\"><div class=\"grid grid-flow-col gap-2 w-full\"><input type=\"text\" name=\"section-ingredient\" placeholder=\"Section name\" class=\"input input-sm w-fit\" value=\"Main\" onfocusout=\"renumberSections(this.closest('ol'), 'ingredient')\"><btn class=\"btn btn-xs btn-square\" onclick=\"deleteSection(this, 'ingredient')\"><svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"m9.75 9.75 4.5 4.5m0-4.5-4.5 4.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z\"></path></svg></btn></div></div></li><li class=\"pb-2\" data-drag-row><div class=\"grid grid-flow-col items-center\"><label class=\"flex gap-1\"><div class=\"inline-flex size-6 cursor-grab handle mt-1 items-center justify-center text-2xl\" data-drageable draggable=\"true\" data-drag-handle>⠿</div><input required type=\"text\" name=\"ingredient&lt;&gt;Main\" value=\"4 pounds top quality chicken filet\" placeholder=\"1 cup of chopped onions\" class=\"input input-sm\" _=\"on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))\"></label><div class=\"ml-2 flex gap-2\"><button type=\"button\" class=\"btn btn-sm btn-outline btn-success\" title=\"Shortcut: \u{2068}Enter\u{2069}\" onclick=\"addItem(event)\"><svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"M12 4.5v15m7.5-7.5h-15\"></svg>Add</button><button type=\"button\" class=\"delete-button btn btn-square btn-sm btn-outline btn-error\" onclick=\"deleteItem(this, 'ingredient')\"><svg xmlns=\"http://www.w3.org/2000/svg\" class=\"size-6 hover:text-red-600\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0\"></path></svg></button></div></div></li><li class=\"pb-2\" data-drag-row><div class=\"grid grid-flow-col items-center\"><label class=\"flex gap-1\"><div class=\"inline-flex size-6 cursor-grab handle mt-1 items-center justify-center text-2xl\" data-drageable draggable=\"true\" data-drag-handle>⠿</div><input required type=\"text\" name=\"ingredient&lt;&gt;Main\" value=\"1/8 cup lemon juice\" placeholder=\"1 cup of chopped onions\" class=\"input input-sm\" _=\"on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))\"></label><div class=\"ml-2 flex gap-2\"><button type=\"button\" class=\"btn btn-sm btn-outline btn-success\" title=\"Shortcut: \u{2068}Enter\u{2069}\" onclick=\"addItem(event)\"><svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"M12 4.5v15m7.5-7.5h-15\"></svg>Add</button><button type=\"button\" class=\"delete-button btn btn-square btn-sm btn-outline btn-error\" onclick=\"deleteItem(this, 'ingredient')\"><svg xmlns=\"http://www.w3.org/2000/svg\" class=\"size-6 hover:text-red-600\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0\"></path></svg></button></div></div></li></ol>",
        r#"<input required type="text" name="ingredient&lt;&gt;Sauce" value="1/2 tbsp cinnamon" placeholder="1 cup of chopped onions" class="input input-sm" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))">"#,
        r#"<input required type="text" name="ingredient&lt;&gt;Main" value="4 pounds top quality chicken filet" placeholder="1 cup of chopped onions" class="input input-sm" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))">"#,
        r#"<input required type="text" name="ingredient&lt;&gt;Main" value="1/8 cup lemon juice" placeholder="1 cup of chopped onions" class="input input-sm" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))">"#,
        r#"<h2 class="font-semibold text-center pb-2"><span class="underline">Instructions</span><sup class="text-red-600">*</sup></h2>"#,
        r#"<textarea required name="instruction&lt;&gt;Sauce" rows="4" class="textarea textarea-bordered rounded-none w-full" placeholder="Mix all ingredients together" _="on keydown if event.ctrlKey and event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))">Mix all these ingredients</textarea>"#,
        "<textarea required name=\"instruction&lt;&gt;Chicken\" rows=\"4\" class=\"textarea textarea-bordered rounded-none w-full\" placeholder=\"Mix all ingredients together\" _=\"on keydown if event.ctrlKey and event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))\">Turn the oven at 300 F</textarea></label><div class=\"grid gap-2 p-2 grid-flow-col\"><div class=\"inline-flex size-6 cursor-grab mt-1 items-center justify-center text-2xl\" data-drag-handle>⠿</div><div class=\"flex gap-2 justify-end\"><button type=\"button\" class=\"btn btn-sm btn-outline btn-success\" title=\"Shortcut: \u{2068}CTRL + Enter\u{2069}\" onclick=\"addItem(event)\"><svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"M12 4.5v15m7.5-7.5h-15\"></svg>Add</button><button type=\"button\" class=\"delete-button btn btn-square btn-sm btn-outline btn-error\" onclick=\"deleteItem(this, 'instruction')\"><svg xmlns=\"http://www.w3.org/2000/svg\" class=\"size-6 hover:text-red-600\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0\"></path></svg></button></div></div></div></li><li class=\"flex items-start gap-2 pt-2 md:pl-0 [counter-increment:steps] before:content-[counter(steps)_'.'] before:pt-2 before:font-medium\" data-drag-row><div class=\"w-full bg-base-300\" data-drageable draggable=\"true\"><label class=\"w-11/12\">",
        "<textarea required name=\"instruction&lt;&gt;Chicken\" rows=\"4\" class=\"textarea textarea-bordered rounded-none w-full\" placeholder=\"Mix all ingredients together\" _=\"on keydown if event.ctrlKey and event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))\">Soak the chicken in the lemon juice</textarea></label><div class=\"grid gap-2 p-2 grid-flow-col\"><div class=\"inline-flex size-6 cursor-grab mt-1 items-center justify-center text-2xl\" data-drag-handle>⠿</div><div class=\"flex gap-2 justify-end\"><button type=\"button\" class=\"btn btn-sm btn-outline btn-success\" title=\"Shortcut: \u{2068}CTRL + Enter\u{2069}\" onclick=\"addItem(event)\"><svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"M12 4.5v15m7.5-7.5h-15\"></svg>Add</button><button type=\"button\" class=\"delete-button btn btn-square btn-sm btn-outline btn-error\" onclick=\"deleteItem(this, 'instruction')\"><svg xmlns=\"http://www.w3.org/2000/svg\" class=\"size-6 hover:text-red-600\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0\"></path></svg></button></div></div></div></li><li class=\"flex items-start gap-2 pt-2 md:pl-0 [counter-increment:steps] before:content-[counter(steps)_'.'] before:pt-2 before:font-medium\" data-drag-row><div class=\"w-full bg-base-300\" data-drageable draggable=\"true\"><label class=\"w-11/12\">",
        r#"<textarea required name="instruction&lt;&gt;Chicken" rows="4" class="textarea textarea-bordered rounded-none w-full" placeholder="Mix all ingredients together" _="on keydown if event.ctrlKey and event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))">Bake for 35 minutes</textarea>"#,
        "<div class=\"rating\"><input class=\"rating-hidden\" type=\"radio\" name=\"rating\" value=\"\" aria-label=\"Clear\"><input type=\"radio\" name=\"rating\" class=\"mask mask-star-2\" value=\"1\" aria-label=\"\u{2068}1\u{2069} star\"><input type=\"radio\" name=\"rating\" class=\"mask mask-star-2\" value=\"2\" aria-label=\"\u{2068}2\u{2069} stars\"><input type=\"radio\" name=\"rating\" class=\"mask mask-star-2\" value=\"3\" aria-label=\"\u{2068}3\u{2069} stars\"><input type=\"radio\" name=\"rating\" class=\"mask mask-star-2\" value=\"4\" aria-label=\"\u{2068}4\u{2069} stars\" checked><input type=\"radio\" name=\"rating\" class=\"mask mask-star-2\" value=\"5\" aria-label=\"\u{2068}5\u{2069} stars\"></div></div>",
        r#"<button class="btn btn-primary btn-block btn-sm">Submit</button>"#,
    ];
    for want in expected {
        assert!(
            normalized.contains(want),
            "Expected to find:\n{want}\n\nIn:\n{normalized}"
        );
    }
}

fn recipe_for_create_to_details(
    recipe_c: RecipeForCreate,
    reference: &RecipeDetails,
    user_id: Uuid,
) -> RecipeDetails {
    let mut keywords = recipe_c.keywords;
    keywords.sort();

    let per_100g = recipe_c.nutrition.per_100g.unwrap_or_default();
    let per_serving = recipe_c.nutrition.per_serving.unwrap_or_default().nutrition;

    RecipeDetails {
        recipe: Recipe {
            id: reference.recipe.id,
            name: recipe_c.name,
            description: recipe_c.description,
            image: reference.recipe.image,
            r#yield: recipe_c.r#yield.unwrap_or(4),
            language: "eng".into(),
            measurement_system_id: 2,
            notes: recipe_c.notes,
            source: recipe_c.source,
            is_favourite: false,
            rating: Some(4),
            created_at: reference.recipe.created_at,
            updated_at: reference.recipe.updated_at,
            user_id,
        },
        additional_images: reference.additional_images.clone(),
        category: recipe_c.category.unwrap_or_else(|| "uncategorized".into()),
        cuisine: recipe_c.cuisine,
        ingredients: recipe_c.ingredients,
        instructions: recipe_c.instructions,
        keywords,
        nutrition: NutritionDetails {
            per_100g: Some(Nutrition {
                id: reference.nutrition.per_100g.as_ref().unwrap().id,
                is_precalculated_by_source: !per_100g.is_empty(),
                calories_kcal: per_100g.calories_kcal,
                total_carbohydrates: per_100g.total_carbohydrates,
                sugars_g: per_100g.sugars_g,
                protein_g: per_100g.protein_g,
                total_fat_g: per_100g.total_fat_g,
                saturated_fat_g: per_100g.saturated_fat_g,
                unsaturated_fat_g: per_100g.unsaturated_fat_g,
                cholesterol_mg: per_100g.cholesterol_mg,
                sodium_mg: per_100g.sodium_mg,
                fiber_g: per_100g.fibre_g,
                trans_fat_g: per_100g.trans_fat_g,
            }),
            per_serving: Some(NutritionPerServingDetails {
                nutrition: Nutrition {
                    id: reference
                        .nutrition
                        .per_serving
                        .as_ref()
                        .unwrap()
                        .nutrition
                        .id,
                    is_precalculated_by_source: !per_serving.is_empty(),
                    calories_kcal: per_serving.calories_kcal,
                    total_carbohydrates: per_serving.total_carbohydrates,
                    sugars_g: per_serving.sugars_g,
                    protein_g: per_serving.protein_g,
                    total_fat_g: per_serving.total_fat_g,
                    saturated_fat_g: per_serving.saturated_fat_g,
                    unsaturated_fat_g: per_serving.unsaturated_fat_g,
                    cholesterol_mg: per_serving.cholesterol_mg,
                    sodium_mg: per_serving.sodium_mg,
                    fiber_g: per_serving.fibre_g,
                    trans_fat_g: per_serving.trans_fat_g,
                },
                serving_size: "2 buns".into(),
            }),
        },
        times: Times {
            id: reference.times.id,
            recipe_id: reference.recipe.id,
            prep_seconds: recipe_c.times.clone().unwrap_or_default().prep_seconds,
            cook_seconds: recipe_c.times.clone().unwrap_or_default().cook_seconds,
            total_seconds: recipe_c.times.clone().unwrap_or_default().prep_seconds
                + recipe_c.times.clone().unwrap_or_default().cook_seconds,
        },
        tools: recipe_c
            .tools
            .into_iter()
            .enumerate()
            .map(|(idx, t)| ToolRecipe {
                name: t.name,
                quantity: t.quantity,
                tool_order: i16::try_from(idx + 1).unwrap_or_default(),
            })
            .collect(),
        videos: vec![],
    }
}
