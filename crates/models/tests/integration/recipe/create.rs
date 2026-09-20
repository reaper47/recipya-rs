use std::assert_matches;

use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use time::PrimitiveDateTime;
use uuid::Uuid;

use app::state::AppState;
use models::{
    Error, Recipe, RecipeDetails,
    recipe::structs::{
        media::Video,
        nutrition::NutritionDetails,
        recipe::RecipeForCreate,
        section::{Item, SectionComponents, SectionItem},
        time::Times,
        tool::{ToolForCreate, ToolRecipe},
    },
    settings::UserSettingDetails,
    user::User,
};
use repository::schema;
use test_db::default_config;
use test_fixtures::insert_user;
use test_harness::{build_server_logged_in, create_app_state};

use crate::recipe::utils::a_complete_recipe_for_create;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

mod tests_add_category {
    use super::*;

    #[tokio::test]
    async fn test_create_new_ok() -> Result<()> {
        let (_, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let category = "fish";

        Recipe::add_category(&state.mm, category, user_id).await?;

        assert_category(state, category).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_create_new_duplicate_err() -> Result<()> {
        let (_, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let category = "fish";
        Recipe::add_category(&state.mm, category, user_id).await?;

        let res = Recipe::add_category(&state.mm, category, user_id).await;

        assert_matches!(res, Err(Error::Diesel(_)));
        Ok(())
    }

    async fn assert_category(state: AppState, category: &str) -> Result<()> {
        let category_id = schema::categories::table
            .filter(schema::categories::name.eq(category))
            .select(schema::categories::id)
            .first::<i64>(&mut state.mm.pool.get().await?)
            .await?;
        assert!(category_id > 0);

        let count = schema::users_categories::table
            .filter(schema::users_categories::category_id.eq(category_id))
            .count()
            .get_result::<i64>(&mut state.mm.pool.get().await?)
            .await?;
        pretty_assertions::assert_eq!(count, 1);
        Ok(())
    }
}

pub fn a_bare_minimum_recipe() -> RecipeForCreate {
    RecipeForCreate {
        name: "Best Chinese Kale".into(),
        r#yield: Some(4),
        rating: Some(4),
        category: Some("uncategorized".into()),
        ingredients: SectionComponents::Grouped(vec![
            SectionItem::new(
                "Sauce",
                vec![
                    Item::new("1 cup blue spinach"),
                    Item::new("1/2 tbsp cinnamon"),
                ],
            ),
            SectionItem::new(
                "Main",
                vec![
                    Item::new("4 pounds top quality chicken filet"),
                    Item::new("1/8 cup lemon juice"),
                ],
            ),
        ]),
        instructions: SectionComponents::Grouped(vec![
            SectionItem::new("Sauce", vec![Item::new("Mix all these ingredients")]),
            SectionItem::new(
                "Chicken",
                vec![
                    Item::new("Turn the oven at 300 F"),
                    Item::new("Soak the chicken in the lemon juice"),
                    Item::new("Bake for 35 minutes").with_duration(2100),
                ],
            ),
        ]),
        measurement_system_id: 2,
        ..Default::default()
    }
}

fn recipe_for_create_to_recipe_with_data(
    recipe_id: i64,
    user_id: Uuid,
    recipe: RecipeForCreate,
    got: &mut RecipeDetails,
) -> RecipeDetails {
    let mut keywords = recipe.keywords.clone();
    keywords.sort();

    let (main_image, additional_images) = recipe.first_and_rest_images();

    let times = recipe.times.unwrap_or_default();

    let mut nutrition = NutritionDetails::from(&recipe.nutrition);
    if let Some(n) = nutrition.per_100g.as_mut() {
        let other_n = got.nutrition.per_100g.as_ref().unwrap();
        n.id = other_n.id;
        n.is_precalculated_by_source = other_n.is_precalculated_by_source;
    }
    if let Some(n) = nutrition.per_serving.as_mut() {
        let other_n = got.nutrition.per_serving.as_ref().unwrap();
        n.nutrition.id = other_n.nutrition.id;
        n.nutrition.is_precalculated_by_source = other_n.nutrition.is_precalculated_by_source;
    }

    got.instructions.iter_mut().for_each(|item| item.id = None);

    RecipeDetails {
        recipe: Recipe {
            id: recipe_id,
            name: recipe.name,
            description: recipe.description,
            image: main_image,
            r#yield: recipe.r#yield.unwrap_or(4),
            language: "eng".into(),
            source: recipe.source,
            measurement_system_id: 2,
            notes: recipe.notes,
            user_id,
            created_at: got.recipe.created_at,
            updated_at: got.recipe.updated_at,
            is_favourite: false,
            rating: Some(4),
        },
        additional_images,
        category: recipe.category.unwrap_or_else(|| "uncategorized".into()),
        cuisine: recipe.cuisine,
        ingredients: recipe.ingredients,
        instructions: recipe.instructions,
        keywords,
        nutrition,
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
            .map(|t| ToolRecipe {
                name: t.name.clone(),
                quantity: t.quantity,
                tool_order: got
                    .tools
                    .iter()
                    .find(|t2| t2.name == t.name)
                    .unwrap()
                    .tool_order,
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
                    .map_or_else(|| PrimitiveDateTime::MIN, |v| v.created_at),
            })
            .collect::<Vec<_>>(),
    }
}

#[tokio::test]
async fn test_create_ok() -> Result<()> {
    let state = create_app_state(default_config()).await;
    let user = insert_user(&state.mm).await?;
    let (recipe, _) = a_complete_recipe_for_create();
    let user_settings = UserSettingDetails::get(&state.mm, user.id).await?;

    let got_recipe_id = Recipe::create(&state.mm, user.id, &recipe, &user_settings).await?;

    let mut got = Recipe::get(&state.mm, user.id, got_recipe_id).await?;
    let want = recipe_for_create_to_recipe_with_data(got_recipe_id, user.id, recipe, &mut got);
    pretty_assertions::assert_eq!(got, want);
    Ok(())
}

#[tokio::test]
async fn test_create_duplicates_err() -> Result<()> {
    let state = create_app_state(default_config()).await;
    let user = insert_user(&state.mm).await?;
    let (mut recipe, _) = a_complete_recipe_for_create();
    let user_settings = UserSettingDetails::get(&state.mm, user.id).await?;
    let _ = Recipe::create(&state.mm, user.id, &recipe, &user_settings).await?;
    recipe.name = "Duplicate".into();

    let got_recipe_id = Recipe::create(&state.mm, user.id, &recipe, &user_settings).await?;

    let mut got = Recipe::get(&state.mm, user.id, got_recipe_id).await?;
    let want = recipe_for_create_to_recipe_with_data(got_recipe_id, user.id, recipe, &mut got);
    pretty_assertions::assert_eq!(got, want);
    Ok(())
}

#[tokio::test]
async fn test_create_duplicate_name_err() -> Result<()> {
    let state = create_app_state(default_config()).await;
    let user = insert_user(&state.mm).await?;
    let (recipe, _) = a_complete_recipe_for_create();
    let user_settings = UserSettingDetails::get(&state.mm, user.id).await?;
    let _ = Recipe::create(&state.mm, user.id, &recipe, &user_settings).await?;

    let got = Recipe::create(&state.mm, user.id, &recipe, &user_settings).await;

    match got {
        Ok(_) => Err("Should have returned an error".into()),
        Err(Error::DuplicateEntityWithID(_)) => Ok(()),
        Err(err) => Err(format!("Wrong error occurred: {err}").into()),
    }
}

#[tokio::test]
async fn test_create_bare_minimum_ok() -> Result<()> {
    let state = create_app_state(default_config()).await;
    let user = insert_user(&state.mm).await?;
    let user_settings = UserSettingDetails::get(&state.mm, user.id).await?;
    let recipe = a_bare_minimum_recipe();

    let got_recipe_id = Recipe::create(&state.mm, user.id, &recipe, &user_settings).await?;

    let mut got = Recipe::get(&state.mm, user.id, got_recipe_id).await?;
    let want = recipe_for_create_to_recipe_with_data(got_recipe_id, user.id, recipe, &mut got);
    pretty_assertions::assert_eq!(got, want);
    Ok(())
}

#[tokio::test]
async fn test_create_some_fields_are_lowercase_ok() -> Result<()> {
    let state = create_app_state(default_config()).await;
    let user = insert_user(&state.mm).await?;
    let user_settings = UserSettingDetails::get(&state.mm, user.id).await?;
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

    let got_recipe_id = Recipe::create(&state.mm, user.id, &recipe, &user_settings).await?;

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

#[tokio::test]
async fn test_create_parse_duration_seconds_ok() -> Result<()> {
    let state = create_app_state(default_config()).await;
    let user = insert_user(&state.mm).await?;
    let user_settings = UserSettingDetails::get(&state.mm, user.id).await?;
    let mut recipe = a_bare_minimum_recipe();
    let instruction1 = "Heat oil on medium heat in a large";
    let instruction2 = "When tomatoes have softened and have started to release their juices (about 4-5 min) add basil";
    let instruction3 = "Simmer on low for at least 1 hour, or up to 6 hours, stirring occasionally. The longer you simmer, the better.";
    recipe.instructions = SectionComponents::Flat(vec![
        Item::new(instruction1),
        Item::new(instruction2),
        Item::new(instruction3),
    ]);

    let got_recipe_id = Recipe::create(&state.mm, user.id, &recipe, &user_settings).await?;

    let got = Recipe::get(&state.mm, user.id, got_recipe_id).await?;
    let get_id = |idx| match got.instructions {
        SectionComponents::Grouped(ref items) => items[0]
            .items
            .iter()
            .find(|&it| it.text == idx)
            .unwrap()
            .id
            .unwrap(),
        SectionComponents::Flat(ref items) => {
            items.iter().find(|&it| it.text == idx).unwrap().id.unwrap()
        }
    };
    pretty_assertions::assert_eq!(
        got.instructions,
        SectionComponents::Flat(vec![
            Item::new("Heat oil on medium heat in a large").with_id(get_id(instruction1)),
            Item::new(
                 "When tomatoes have softened and have started to release their juices (about 4-5 min) add basil",
            ).with_duration(5*60).with_id(get_id(instruction2)),
            Item::new(
                "Simmer on low for at least 1 hour, or up to 6 hours, stirring occasionally. The longer you simmer, the better.",
            ).with_duration(360*60).with_id(get_id(instruction3))
        ]));
    Ok(())
}
