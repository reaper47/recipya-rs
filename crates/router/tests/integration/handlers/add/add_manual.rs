use axum::http::HeaderValue;
use axum::http::Method;
use axum_test::TestResponse;
use reqwest::StatusCode;
use time::Duration;
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
};
use models::types::RecipeYield;
use models::types::{Rating, Source};
use models::user::User;
use models::{Recipe, RecipeDetails};
use test_db::default_config;
use test_harness::{assert_must_be_logged_in, build_server_logged_in};

use crate::handlers::helpers::create_form;

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
    async fn test_add_manual_missing_fields_defaults_ok() -> Result<()> {
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
        pretty_assertions::assert_eq!(got.recipe.r#yield.into_inner(), 4);
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
            for i in instructions.iter_mut() {
                i.id = None;
            }
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
            SectionComponents::Flat(vec![Item::new("8 apples"), Item::new("4 oranges")]),
        );
        pretty_assertions::assert_eq!(
            got.instructions,
            SectionComponents::Flat(vec![Item::new("Mix the apples"), Item::new("Eat"),]),
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
    async fn test_manual_submit_recipe_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let recipe = RecipeForCreate {
            name: "Best Chinese Kale".into(),
            description: Some("Your mouth will drool like never before".into()),
            images: vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()],
            measurement_system_id: 2,
            r#yield: RecipeYield::from(6),
            source: Source::new("My mother's maple syrup recipes cookbook"),
            is_favourite: false,
            rating: Rating::from(4),
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
                    fibre_g: Some(10.),
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
                        fibre_g: Some(10.),
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
            for item in instructions.iter_mut() {
                item.id = None;
            }
        }

        let times = recipe.times.expect("should have times");
        pretty_assertions::assert_eq!(
            got,
            RecipeDetails {
                recipe: Recipe {
                    id: recipe_id,
                    name: recipe.name,
                    description: recipe.description,
                    image: Some(got.recipe.image.expect("main image")),
                    r#yield: RecipeYield::from(6),
                    language: "eng".into(),
                    measurement_system_id: 2,
                    notes: Some("# Test\n\nSome notes".into()),
                    source: recipe.source,
                    is_favourite: false,
                    rating: Rating::from(4),
                    created_at: got.recipe.created_at,
                    updated_at: got.recipe.updated_at,
                    user_id,
                },
                additional_images: got.additional_images.clone(),
                category: recipe.category.expect("should have category"),
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
                    vec![Item::new(
                        "Add whip cream and whisk the fruits until smooth",
                    )],
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
            for section in instructions.iter_mut() {
                section.items.iter_mut().for_each(|item| item.id = None);
            }
        }
        pretty_assertions::assert_eq!(
            got,
            RecipeDetails {
                recipe: Recipe {
                    id: recipe_id,
                    name: recipe.name,
                    description: recipe.description,
                    r#yield: RecipeYield::from(1),
                    language: "eng".into(),
                    measurement_system_id: 2,
                    source: recipe.source,
                    is_favourite: false,
                    created_at: got.recipe.created_at,
                    updated_at: got.recipe.updated_at,
                    user_id,
                    ..Default::default()
                },
                category: recipe.category.expect("should have category"),
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
        r#"<title hx-swap-oob="true">Add recipe manually | Recipya</title>"#,
        r##"<form class="card-body contents" style="padding: 0" enctype="multipart/form-data" hx-encoding="multipart/form-data" hx-post="/recipes/add/manual" hx-indicator="#fullscreen-loader">"##,
        r#"<input required type="text" name="title" placeholder="Title of the recipe*" autocomplete="off" class="input w-full text-center rounded-t-lg rounded-b-none bg-base-200">"#,
        "<img src=\"\" alt=\"Image \u{2068}1\u{2069} of the recipe\" class=\"block w-full h-full object-contain\"></div><span class=\"grid gap-1\"><div class=\"mr-1 image-selector p-4\"><input type=\"file\" accept=\"image/*,video/*\" name=\"media\" class=\"file-input file-input-sm file-input-bordered w-full max-w-sm\"",
        r#"<input id="servings" type="number" min="1" name="yield" value="1" class="input input-sm w-11/12">"#,
        r#"<input id="category" type="text" list="categories" name="category" class="input input-sm w-11/12" placeholder="breakfast" autocomplete="off" value=""><datalist id="categories"><option>uncategorized</option><option>appetizers</option><option>bread</option><option>breakfasts</option><option>condiments</option><option>dessert</option><option>lunch</option><option>main dish</option><option>salad</option><option>side dish</option><option>snacks</option><option>soups</option><option>stews</option></datalist>"#,
        r#"<textarea name="description" placeholder="This Thai curry chicken will make you drool." class="textarea textarea-ghost w-full h-full resize-none rounded-none focus:outline-none"></textarea>"#,
        r#"<div class="flex justify-self-center items-center gap-1 cursor-default" title="Cook time"><span data-tip="Cook time" class="tooltip tooltip-left"><svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="24px" height="23px" viewBox="0 0 24 23" version="1.1"><g id="surface1"><path style=" stroke:none;fill-rule:nonzero;fill:rgb(62.745098%,64.705882%,65.882353%);fill-opacity:1;" d="M 4.636719 10.984375 L 4.636719 20.417969 C 4.636719 21.007812 5.078125 21.527344 5.667969 21.527344 L 18.257812 21.527344 C 18.847656 21.527344 19.363281 21.007812 19.363281 20.417969 L 19.363281 10.984375 Z M 4.636719 10.984375 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(76.862745%,76.862745%,76.862745%);fill-opacity:1;" d="M 19.289062 9.953125 C 18.992188 8.699219 17.964844 7.8125 16.710938 7.8125 L 7.214844 7.8125 C 5.964844 7.8125 4.933594 8.699219 4.710938 9.953125 Z M 19.289062 9.953125 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(54.509804%,69.411765%,81.960784%);fill-opacity:1;" d="M 16.710938 7.8125 L 13.914062 7.8125 C 14.945312 7.8125 15.828125 8.476562 16.269531 9.363281 C 16.417969 9.730469 16.785156 9.953125 17.226562 9.953125 L 19.289062 9.953125 C 18.992188 8.699219 17.964844 7.8125 16.710938 7.8125 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(0.392157%,26.666667%,38.823529%);fill-opacity:1;" d="M 22.160156 9.953125 L 20.320312 9.953125 C 20.097656 8.183594 18.550781 6.78125 16.710938 6.78125 L 15.535156 6.78125 L 15.3125 5.898438 C 15.09375 5.160156 14.503906 4.644531 13.765625 4.644531 L 11.191406 4.644531 C 10.453125 4.644531 9.792969 5.160156 9.644531 5.898438 L 9.421875 6.78125 L 7.214844 6.78125 C 5.449219 6.78125 3.902344 8.183594 3.605469 9.953125 L 1.765625 9.953125 C 1.03125 9.953125 0.441406 10.542969 0.441406 11.277344 C 0.441406 11.5 0.441406 11.722656 0.589844 11.941406 L 1.25 13.269531 C 1.546875 13.785156 2.0625 14.152344 2.648438 14.152344 L 3.605469 14.152344 L 3.605469 20.417969 C 3.605469 21.597656 4.492188 22.558594 5.667969 22.558594 L 18.257812 22.558594 C 19.4375 22.558594 20.394531 21.597656 20.394531 20.417969 L 20.394531 14.152344 L 21.351562 14.152344 C 21.9375 14.152344 22.453125 13.859375 22.75 13.269531 L 23.410156 11.867188 C 23.558594 11.648438 23.558594 11.5 23.558594 11.277344 C 23.558594 10.542969 22.894531 9.953125 22.160156 9.953125 M 3.605469 13.121094 L 2.648438 13.121094 C 2.429688 13.121094 2.28125 12.972656 2.136719 12.753906 L 1.472656 11.5 L 1.472656 11.277344 C 1.472656 11.132812 1.621094 10.984375 1.765625 10.984375 L 3.605469 10.984375 Z M 10.75 6.117188 C 10.75 5.972656 10.96875 5.75 11.265625 5.75 L 13.839844 5.75 C 14.0625 5.75 14.28125 5.898438 14.355469 6.117188 L 14.503906 6.78125 L 10.601562 6.78125 Z M 7.214844 7.8125 L 16.710938 7.8125 C 17.964844 7.8125 18.992188 8.699219 19.289062 9.953125 L 4.710938 9.953125 C 4.933594 8.699219 5.964844 7.8125 7.214844 7.8125 M 19.363281 13.636719 L 19.363281 20.417969 C 19.363281 21.007812 18.847656 21.527344 18.257812 21.527344 L 5.667969 21.527344 C 5.078125 21.527344 4.636719 21.007812 4.636719 20.417969 L 4.636719 10.984375 L 19.363281 10.984375 Z M 22.453125 11.5 L 21.71875 12.828125 C 21.644531 12.972656 21.496094 13.121094 21.277344 13.121094 L 20.394531 13.121094 L 20.394531 11.058594 L 22.160156 11.058594 C 22.308594 11.058594 22.453125 11.207031 22.453125 11.351562 L 22.453125 11.5 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(92.54902%,94.117647%,97.647059%);fill-opacity:1;" d="M 7.804688 17.324219 C 8.089844 17.324219 8.320312 17.554688 8.320312 17.839844 C 8.320312 18.125 8.089844 18.355469 7.804688 18.355469 C 7.519531 18.355469 7.289062 18.125 7.289062 17.839844 C 7.289062 17.554688 7.519531 17.324219 7.804688 17.324219 M 7.804688 16.808594 C 7.4375 16.808594 7.214844 16.585938 7.214844 16.21875 L 7.214844 13.121094 C 7.214844 12.753906 7.4375 12.53125 7.804688 12.53125 C 8.097656 12.53125 8.320312 12.753906 8.320312 13.121094 L 8.320312 16.21875 C 8.320312 16.585938 8.097656 16.808594 7.804688 16.808594 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(54.509804%,69.411765%,81.960784%);fill-opacity:1;" d="M 16.195312 12.015625 L 16.195312 19.902344 C 16.195312 20.492188 15.679688 21.007812 15.09375 21.007812 L 6.699219 21.007812 C 6.183594 21.007812 5.667969 20.492188 5.667969 19.902344 L 5.667969 12.015625 C 5.667969 11.5 5.226562 10.984375 4.636719 10.984375 L 4.636719 20.417969 C 4.636719 21.007812 5.078125 21.527344 5.667969 21.527344 L 18.257812 21.527344 C 18.847656 21.527344 19.363281 21.007812 19.363281 20.417969 L 19.363281 10.984375 L 17.226562 10.984375 C 16.636719 10.984375 16.195312 11.5 16.195312 12.015625 M 8.246094 7.8125 L 7.214844 7.8125 C 5.964844 7.8125 4.933594 8.699219 4.710938 9.953125 L 4.933594 9.953125 C 5.375 9.953125 5.742188 9.730469 5.890625 9.363281 C 6.332031 8.476562 7.214844 7.8125 8.246094 7.8125 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(0.392157%,26.666667%,38.823529%);fill-opacity:1;" d="M 18.773438 5.75 C 18.625 5.75 18.480469 5.675781 18.40625 5.527344 C 18.183594 5.308594 18.257812 4.9375 18.40625 4.792969 C 18.699219 4.644531 18.773438 4.421875 18.773438 4.128906 C 18.773438 3.90625 18.699219 3.6875 18.480469 3.539062 C 18.257812 3.316406 18.257812 3.023438 18.40625 2.800781 C 18.625 2.582031 18.992188 2.507812 19.140625 2.726562 C 19.582031 3.097656 19.878906 3.613281 19.878906 4.128906 C 19.878906 4.71875 19.582031 5.234375 19.140625 5.601562 L 18.773438 5.75 M 18.773438 1.03125 C 19.058594 1.03125 19.289062 1.261719 19.289062 1.546875 C 19.289062 1.832031 19.058594 2.0625 18.773438 2.0625 C 18.488281 2.0625 18.257812 1.832031 18.257812 1.546875 C 18.257812 1.261719 18.488281 1.03125 18.773438 1.03125 M 16.710938 5.75 L 16.269531 5.527344 C 16.050781 5.308594 16.121094 4.9375 16.34375 4.792969 C 16.5625 4.644531 16.710938 4.421875 16.710938 4.128906 C 16.710938 3.90625 16.5625 3.6875 16.417969 3.539062 C 15.902344 3.097656 15.679688 2.652344 15.679688 2.0625 C 15.679688 1.472656 15.902344 1.03125 16.417969 0.589844 C 16.5625 0.367188 16.933594 0.441406 17.152344 0.664062 C 17.300781 0.8125 17.300781 1.179688 17.078125 1.402344 C 16.785156 1.546875 16.710938 1.769531 16.710938 2.0625 C 16.710938 2.285156 16.785156 2.507812 17.007812 2.652344 C 17.519531 3.097656 17.742188 3.613281 17.742188 4.128906 C 17.742188 4.71875 17.519531 5.234375 17.007812 5.601562 L 16.710938 5.75 "></path></g></svg></span><label><input type="text" name="time-cook" value="00:30:00" class="input input-sm max-w-24 html-duration-picker"></label></div>"#,
        r#"<div class="flex justify-self-center items-center gap-1 cursor-default" title="Prep time"><span data-tip="Prep time" class="tooltip tooltip-left">"#,
        r#"<input type="text" name="time-prep" value="00:15:00" class="input input-sm max-w-24 html-duration-picker"></label></div>"#,
        r#"<table class="table table-zebra table-xs"><thead><tr><th><select class="select select-sm" onchange="filterNutritionRows(this, this.value)"><option value="per-100g">Nutrition (per 100g)</option><option value="per-serving">Nutrition (per serving)</option></select></th></tr></thead><tbody><tr data-nutrition-type="per-100g"><td>Calories</td><td><label><input type="text" name="calories-per-100g" autocomplete="off" placeholder="368kcal" class="input input-xs max-w-24"></label></td></tr><tr data-nutrition-type="per-100g"><td>Total carbs</td><td><label><input type="text" name="total-carbohydrates-per-100g" autocomplete="off" placeholder="35g" class="input input-xs max-w-24"></label></td></tr><tr data-nutrition-type="per-100g"><td>Sugars</td><td><label><input type="text" name="sugars-per-100g" autocomplete="off" placeholder="3g" class="input input-xs max-w-24"></label></td></tr><tr data-nutrition-type="per-100g"><td>Protein</td><td><label><input type="text" name="protein-per-100g" autocomplete="off" placeholder="21g" class="input input-xs max-w-24"></label></td></tr><tr data-nutrition-type="per-100g"><td>Total fat</td><td><label><input type="text" name="total-fat-per-100g" autocomplete="off" placeholder="15g" class="input input-xs max-w-24"></label></td></tr><tr data-nutrition-type="per-100g"><td>Saturated fat</td><td><label><input type="text" name="saturated-fat-per-100g" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24"></label></td></tr><tr data-nutrition-type="per-100g"><td>Unsaturated fat</td><td><label><input type="text" name="unsaturated-fat-per-100g" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24"></label></td></tr><tr data-nutrition-type="per-100g"><td>Trans fat</td><td><label><input type="text" name="trans-fat-per-100g" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24"></label></td></tr><tr data-nutrition-type="per-100g"><td>Cholesterol</td><td><label><input type="text" name="cholesterol-per-100g" autocomplete="off" placeholder="1.1mg" class="input input-xs max-w-24"></label></td></tr><tr data-nutrition-type="per-100g"><td>Sodium</td><td><label><input type="text" name="sodium-per-100g" autocomplete="off" placeholder="100mg" class="input input-xs max-w-24"></label></td></tr><tr data-nutrition-type="per-100g"><td>Fibre</td><td><label><input type="text" name="fibre-per-100g" autocomplete="off" placeholder="8g" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Serving size</td><td><label><input type="text" name="serving-size" autocomplete="off" placeholder="1/4 cup (45g)" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Calories</td><td><label><input type="text" name="calories-per-serving" autocomplete="off" placeholder="128kcal" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Total carbs</td><td><label><input type="text" name="total-carbohydrates-per-serving" autocomplete="off" placeholder="12g" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Sugars</td><td><label><input type="text" name="sugars-per-serving" autocomplete="off" placeholder="2g" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Protein</td><td><label><input type="text" name="protein-per-serving" autocomplete="off" placeholder="5g" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Total fat</td><td><label><input type="text" name="total-fat-per-serving" autocomplete="off" placeholder="15g" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Saturated fat</td><td><label><input type="text" name="saturated-fat-per-serving" autocomplete="off" placeholder="3.8g" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Unsaturated fat</td><td><label><input type="text" name="unsaturated-fat-per-serving" autocomplete="off" placeholder="3.2g" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Trans fat</td><td><label><input type="text" name="trans-fat-per-serving" autocomplete="off" placeholder="0g" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Cholesterol</td><td><label><input type="text" name="cholesterol-per-serving" autocomplete="off" placeholder="100mg" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Sodium</td><td><label><input type="text" name="sodium-per-serving" autocomplete="off" placeholder="25mg" class="input input-xs max-w-24"></label></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Fibre</td><td><label><input type="text" name="fibre-per-serving" autocomplete="off" placeholder="6g" class="input input-xs max-w-24"></label></td></tr></tbody></table>"#,
        r#"<h2 class="font-semibold text-center pb-2"><span class="underline">Tools</span></h2>"#,
        "<ol class=\"pl-4\" id=\"tools-list\"><li class=\"pb-2\" data-drag-row><div class=\"grid grid-flow-col items-center\"><label class=\"flex gap-1\"><div class=\"inline-flex size-6 cursor-grab mt-1 items-center justify-center text-2xl\"><data-drageable draggable=\"true\" data-drag-handle>⠿</data-drageable></div><input type=\"text\" name=\"tool\" placeholder=\"1 frying pan\" class=\"input input-bordered input-sm w-full\" value=\"\" _=\"on keydown if event.key is 'Enter' halt the event then call addItem(event)\"></label><div class=\"ml-2 flex gap-2\"><button type=\"button\" class=\"btn btn-sm btn-outline btn-success\" title=\"Shortcut: \u{2068}Enter\u{2069}\" onclick=\"addItem(event)\"><svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"M12 4.5v15m7.5-7.5h-15\"></svg>Add</button><button type=\"button\" class=\"delete-button btn btn-square btn-sm btn-outline btn-error\"",
        r#"<h2 class="font-semibold text-center pb-2"><span class="underline">Ingredients</span><sup class="text-red-600">*</sup></h2>"#,
        "<ol class=\"pl-4\" id=\"ingredients-list\"><li data-drag-row><div class=\"flex items-center\" data-drageable draggable=\"true\"><div class=\"inline-flex size-6 cursor-grab items-center justify-center text-2xl\" data-drag-handle>⠿</div><div class=\"divider flex-1\"><div class=\"flex gap-2\"><btn class=\"btn btn-xs btn-outline\" _=\"on click make an &lt;input/&gt; called newInput set newInput.type to 'text' set newInput.name to 'section-ingredient' set newInput.placeholder to 'Section name' set newInput.className to 'input input-sm w-fit' set list to closest &lt;ol/&gt; set sectionName to 'ingredient' put newInput before me remove me js(newInput, list, sectionName) newInput.addEventListener('keydown', function(event) { if (event.key === 'Enter') { event.preventDefault(); newInput.blur(); } }); newInput.addEventListener('focusout', function() { renumberSections(list, sectionName) }); end newInput.focus()\">New section</btn><btn class=\"btn btn-xs btn-square\" _=\"on click set list to closest &lt;ol/&gt; set sectionName to 'ingredient' remove closest &lt;li/&gt; call renumberSections(list, sectionName)\"><svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"m9.75 9.75 4.5 4.5m0-4.5-4.5 4.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z\"></path></svg></btn></div></div></div></li><li class=\"pb-2\" data-drag-row><div class=\"grid grid-flow-col items-center\"><label class=\"flex gap-1\"><div class=\"inline-flex size-6 cursor-grab handle mt-1 items-center justify-center text-2xl\" data-drageable draggable=\"true\" data-drag-handle>⠿</div><input required type=\"text\" name=\"ingredient\" value=\"\" placeholder=\"1 cup of chopped onions\" class=\"input input-sm\" _=\"on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))\"></label><div class=\"ml-2 flex gap-2\"><button type=\"button\" class=\"btn btn-sm btn-outline btn-success\" title=\"Shortcut: \u{2068}Enter\u{2069}\" onclick=\"addItem(event)\"><svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"M12 4.5v15m7.5-7.5h-15\"></svg>Add</button><button type=\"button\" class=\"delete-button btn btn-square btn-sm btn-outline btn-error\" onclick=\"deleteItem(this, 'ingredient')\"><svg xmlns=\"http://www.w3.org/2000/svg\" class=\"size-6 hover:text-red-600\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0\"></path></svg></button></div></div></li></ol>",
        r#"<h2 class="font-semibold text-center pb-2"><span class="underline">Instructions</span><sup class="text-red-600">*</sup></h2>"#,
        "<ol id=\"instructions-list\" class=\"grid [counter-reset:steps]\"><li data-drag-row><div class=\"flex items-center\" data-drageable draggable=\"true\"><div class=\"inline-flex size-6 cursor-grab items-center justify-center text-2xl\" data-drag-handle>⠿</div><div class=\"divider flex-1\"><div class=\"flex gap-2\"><btn class=\"btn btn-xs btn-outline\" _=\"on click make an &lt;input/&gt; called newInput set newInput.type to 'text' set newInput.name to 'section-instruction' set newInput.placeholder to 'Section name' set newInput.className to 'input input-sm w-fit' set list to closest &lt;ol/&gt; set sectionName to 'instruction' put newInput before me remove me js(newInput, list, sectionName) newInput.addEventListener('keydown', function(event) { if (event.key === 'Enter') { event.preventDefault(); newInput.blur(); } }); newInput.addEventListener('focusout', function() { renumberSections(list, sectionName) }); end newInput.focus()\">New section</btn><btn class=\"btn btn-xs btn-square\" _=\"on click set list to closest &lt;ol/&gt; set sectionName to 'instruction' remove closest &lt;li/&gt; call renumberSections(list, sectionName)\"><svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"m9.75 9.75 4.5 4.5m0-4.5-4.5 4.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z\"></path></svg></btn></div></div></div></li><li class=\"flex items-start gap-2 pt-2 md:pl-0 [counter-increment:steps] before:content-[counter(steps)_'.'] before:pt-2 before:font-medium\" data-drag-row><div class=\"w-full bg-base-300\" data-drageable draggable=\"true\"><label class=\"w-11/12\"><textarea required name=\"instruction\" rows=\"4\" class=\"textarea textarea-bordered rounded-none w-full\" placeholder=\"Mix all ingredients together\" _=\"on keydown if event.ctrlKey and event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))\"></textarea></label><div class=\"grid gap-2 p-2 grid-flow-col\"><div class=\"inline-flex size-6 cursor-grab mt-1 items-center justify-center text-2xl\" data-drag-handle>⠿</div><div class=\"flex gap-2 justify-end\"><button type=\"button\" class=\"btn btn-sm btn-outline btn-success\" title=\"Shortcut: \u{2068}CTRL + Enter\u{2069}\" onclick=\"addItem(event)\"><svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"M12 4.5v15m7.5-7.5h-15\"></svg>Add</button><button type=\"button\" class=\"delete-button btn btn-square btn-sm btn-outline btn-error\" onclick=\"deleteItem(this, 'instruction')\"><svg xmlns=\"http://www.w3.org/2000/svg\" class=\"size-6 hover:text-red-600\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"size-6\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0\"></path></svg></button></div></div></div></li></ol>",
        r#"<button class="btn btn-primary btn-block btn-sm">Submit</button>"#,
    ];
    for want in expected {
        assert!(
            normalized.contains(want),
            "Expected to find:\n{want}\n\nIn:\n{normalized}"
        );
    }
}
