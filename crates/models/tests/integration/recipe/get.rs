use ingredient::IngredientParser;
use models::{
    Recipe, RecipeDetails,
    params::SearchParams,
    recipe::structs::{
        media::Video,
        nutrition::{Nutrition, NutritionDetails, NutritionPerServingDetails},
        section::{Item, SectionComponents, SectionItem},
        time::Times,
        tool::ToolRecipe,
        types::Source,
    },
    settings::UserSettingDetails,
    user::User,
};
use test_db::default_config;
use test_harness::{build_server_anonymous, create_app_state};
use time::Duration;
use uuid::Uuid;

use crate::recipe::utils::a_complete_recipe_for_create;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

mod tests_all {
    use super::*;

    #[tokio::test]
    async fn test_all_ok() -> Result<()> {
        let (_, state) = build_server_anonymous(default_config()).await?;
        let all_users = User::all(&state.mm).await?;
        let user1_id = all_users[0].id;
        let user2_id = all_users[1].id;
        let settings1 = UserSettingDetails::get(&state.mm, user1_id).await?;
        let settings2 = UserSettingDetails::get(&state.mm, user2_id).await?;
        for i in 0..5 {
            let (mut recipe, _) = a_complete_recipe_for_create();
            recipe.name.push_str(i.to_string().as_str());
            let _ = Recipe::create(
                &state.mm,
                if i % 2 == 0 { user1_id } else { user2_id },
                &recipe,
                if i % 2 == 0 { &settings1 } else { &settings2 },
            )
            .await?;
        }

        let recipes = Recipe::all(&state.mm, user1_id).await?;
        assert_eq!(recipes.len(), 3);
        Ok(())
    }
}

mod tests_count {
    use super::*;

    #[tokio::test]
    async fn test_count_ok() -> Result<()> {
        let (_, state) = build_server_anonymous(default_config()).await?;
        let all_users = User::all(&state.mm).await?;
        let user1_id = all_users[0].id;
        let user2_id = all_users[1].id;
        let settings1 = UserSettingDetails::get(&state.mm, user1_id).await?;
        let settings2 = UserSettingDetails::get(&state.mm, user2_id).await?;
        for i in 0..5 {
            let (mut recipe, _) = a_complete_recipe_for_create();
            recipe.name.push_str(i.to_string().as_str());
            let _ = Recipe::create(&state.mm, user1_id, &recipe, &settings1).await?;
        }
        for i in 0..10 {
            let (mut recipe, _) = a_complete_recipe_for_create();
            recipe.name.push_str((i + 1002).to_string().as_str());
            let _ = Recipe::create(&state.mm, user2_id, &recipe, &settings2).await?;
        }

        let count_user1 = Recipe::count(&state.mm, user1_id).await?;
        let count_user2 = Recipe::count(&state.mm, user2_id).await?;

        assert_eq!(count_user1, 5);
        assert_eq!(count_user2, 10);
        Ok(())
    }
}

mod tests_get_page {
    use super::*;

    #[tokio::test]
    async fn test_get_page_no_recipes_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;

        let recipes = Recipe::get_page(
            &state.mm,
            Uuid::new_v4(),
            &SearchParams {
                page: Some(1),
                ..Default::default()
            },
        )
        .await?;

        pretty_assertions::assert_eq!(recipes, Vec::new());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_page_first_page_ok() -> Result<()> {
        let (_, state) = build_server_anonymous(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let mut expected = Vec::with_capacity(15);
        for i in 0..15 {
            let (mut recipe, _) = a_complete_recipe_for_create();
            recipe.name.push_str(i.to_string().as_str());
            let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
            expected.push(recipe.name);
        }

        let recipes = Recipe::get_page(
            &state.mm,
            user_id,
            &SearchParams {
                page: Some(1),
                ..Default::default()
            },
        )
        .await?;

        let got = recipes
            .into_iter()
            .map(|r| r.recipe.name)
            .collect::<Vec<_>>();
        pretty_assertions::assert_eq!(got, expected);
        Ok(())
    }
}

#[tokio::test]
#[allow(clippy::too_many_lines)]
async fn test_get_many_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let settings = UserSettingDetails::get(&state.mm, user_id).await?;
    let (recipe1, images1) = a_complete_recipe_for_create();
    let (mut recipe2, images2) = a_complete_recipe_for_create();
    recipe2.name = "Hello".to_string();
    let id1 = Recipe::create(&state.mm, user_id, &recipe1, &settings).await?;
    let id2 = Recipe::create(&state.mm, user_id, &recipe2, &settings).await?;

    let mut got = Recipe::get_many(&state.mm, user_id, &[id1, id2]).await?;

    for r in &mut got {
        match &mut r.instructions {
            SectionComponents::Grouped(section_items) => section_items
                .iter_mut()
                .for_each(|i| i.items.iter_mut().for_each(|i| i.id = None)),
            SectionComponents::Flat(items) => items.iter_mut().for_each(|i| i.id = None),
        }
    }
    let want_recipe1 = RecipeDetails {
        recipe: Recipe {
            id: id1,
            name: "Best Chinese Kale".into(),
            description: Some("This is the most delicious recipe!".into()),
            r#yield: 4,
            language: "eng".into(),
            measurement_system_id: 2,
            notes: Some("# My Recipe Notes\n\nThis dish should be served medium-cold".into()),
            source: Source::Url(
                "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/".into(),
            ),
            rating: Some(4),
            created_at: got[0].recipe.created_at,
            updated_at: got[0].recipe.updated_at,
            user_id,
            image: Some(images1.main),
            ..Default::default()
        },
        additional_images: vec![images1.additional],
        category: "dinner".into(),
        cuisine: Some("thai".into()),
        ingredients: SectionComponents::Grouped(vec![
            SectionItem {
                title: "Sauce".into(),
                items: vec![
                    Item::new("1 cup blue spinach"),
                    Item::new("1/2 tbsp cinnamon"),
                ],
            },
            SectionItem {
                title: "Main".into(),
                items: vec![
                    Item::new("4 pounds top quality chicken filet"),
                    Item::new("1/8 cup lemon juice"),
                ],
            },
        ]),
        instructions: SectionComponents::Grouped(vec![
            SectionItem {
                title: "Sauce".into(),
                items: vec![Item::new("Mix all these ingredients")],
            },
            SectionItem {
                title: "Chicken".into(),
                items: vec![
                    Item::new("Turn the oven at 300 F"),
                    Item::new("Soak the chicken in the lemon juice"),
                    Item::new("Bake for 35 minutes").with_duration(2100),
                ],
            },
        ]),
        keywords: vec!["tofu".into(), "vegetarian".into()],
        nutrition: NutritionDetails {
            per_100g: Some(Nutrition {
                id: got[0].nutrition.clone().per_100g.map_or(1, |n| n.id),
                is_precalculated_by_source: true,
                calories_kcal: Some(300),
                total_carbohydrates: Some(55.0),
                sugars_g: Some(43.0),
                protein_g: Some(7.0),
                total_fat_g: Some(6.0),
                saturated_fat_g: Some(1.0),
                unsaturated_fat_g: Some(2.0),
                cholesterol_mg: Some(5.0),
                sodium_mg: Some(12.0),
                fiber_g: Some(10.0),
                trans_fat_g: Some(3.0),
            }),
            per_serving: Some(NutritionPerServingDetails {
                nutrition: Nutrition {
                    id: got[0]
                        .nutrition
                        .clone()
                        .per_serving
                        .map_or(1, |n| n.nutrition.id),
                    is_precalculated_by_source: true,
                    calories_kcal: Some(240),
                    total_carbohydrates: Some(30.0),
                    sugars_g: Some(24.0),
                    protein_g: Some(4.0),
                    total_fat_g: Some(6.0),
                    saturated_fat_g: Some(2.0),
                    unsaturated_fat_g: Some(7.0),
                    cholesterol_mg: Some(12.0),
                    sodium_mg: Some(100.0),
                    fiber_g: Some(18.0),
                    trans_fat_g: Some(2.0),
                },
                serving_size: "2 buns".into(),
            }),
        },
        times: Times {
            id: got[0].times.id,
            recipe_id: id1,
            prep_seconds: 120,
            cook_seconds: 3600,
            total_seconds: 3720,
        },
        tools: vec![
            ToolRecipe {
                name: "wok".into(),
                quantity: 1,
                tool_order: 1,
            },
            ToolRecipe {
                name: "frying pan".into(),
                quantity: 1,
                tool_order: 2,
            },
        ],
        videos: vec![Video {
            video: images1.video,
            duration: Some(Duration::seconds(420)),
            content_url: Some("https://example.com/best-food.mp4".into()),
            embed_url: Some("https://example.com/embed/j43yfe3.mp4".into()),
            created_at: got[0].videos[0].created_at,
        }],
    };
    let want_recipe2 = RecipeDetails {
        recipe: Recipe {
            id: id2,
            name: "Hello".into(),
            description: want_recipe1.recipe.description.clone(),
            r#yield: want_recipe1.recipe.r#yield,
            language: want_recipe1.recipe.language.clone(),
            measurement_system_id: 2,
            notes: want_recipe1.recipe.notes.clone(),
            source: want_recipe1.recipe.source.clone(),
            rating: want_recipe1.recipe.rating,
            created_at: got[1].recipe.created_at,
            updated_at: got[1].recipe.updated_at,
            user_id,
            image: Some(images2.main),
            ..Default::default()
        },
        additional_images: vec![images2.additional],
        category: want_recipe1.category.clone(),
        cuisine: want_recipe1.cuisine.clone(),
        ingredients: want_recipe1.ingredients.clone(),
        instructions: want_recipe1.instructions.clone(),
        keywords: want_recipe1.keywords.clone(),
        nutrition: NutritionDetails {
            per_100g: Some(Nutrition {
                id: got[1].nutrition.clone().per_100g.map_or(1, |n| n.id),
                is_precalculated_by_source: true,
                calories_kcal: Some(300),
                total_carbohydrates: Some(55.0),
                sugars_g: Some(43.0),
                protein_g: Some(7.0),
                total_fat_g: Some(6.0),
                saturated_fat_g: Some(1.0),
                unsaturated_fat_g: Some(2.0),
                cholesterol_mg: Some(5.0),
                sodium_mg: Some(12.0),
                fiber_g: Some(10.0),
                trans_fat_g: Some(3.0),
            }),
            per_serving: Some(NutritionPerServingDetails {
                nutrition: Nutrition {
                    id: got[1]
                        .nutrition
                        .clone()
                        .per_serving
                        .map_or(1, |n| n.nutrition.id),
                    is_precalculated_by_source: true,
                    calories_kcal: Some(240),
                    total_carbohydrates: Some(30.0),
                    sugars_g: Some(24.0),
                    protein_g: Some(4.0),
                    total_fat_g: Some(6.0),
                    saturated_fat_g: Some(2.0),
                    unsaturated_fat_g: Some(7.0),
                    cholesterol_mg: Some(12.0),
                    sodium_mg: Some(100.0),
                    fiber_g: Some(18.0),
                    trans_fat_g: Some(2.0),
                },
                serving_size: "2 buns".into(),
            }),
        },
        times: Times {
            id: got[1].times.id,
            recipe_id: id2,
            prep_seconds: 120,
            cook_seconds: 3600,
            total_seconds: 3720,
        },
        tools: want_recipe1.tools.clone(),
        videos: vec![Video {
            video: images2.video,
            duration: Some(Duration::seconds(420)),
            content_url: Some("https://example.com/best-food.mp4".into()),
            embed_url: Some("https://example.com/embed/j43yfe3.mp4".into()),
            created_at: got[1].videos[0].created_at,
        }],
    };
    pretty_assertions::assert_eq!(got, vec![want_recipe1, want_recipe2]);
    Ok(())
}

#[tokio::test]
async fn test_fetch_categories_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let settings = UserSettingDetails::get(&state.mm, user_id).await?;
    let category1 = String::from("late snack");
    let category2 = String::from("dinner");
    let (mut recipe, _) = a_complete_recipe_for_create();
    recipe.category = Some(category1.clone());
    let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
    let (mut recipe, _) = a_complete_recipe_for_create();
    recipe.name = "Hello".to_string();
    recipe.category = Some(category2.clone());
    let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

    let categories = Recipe::fetch_categories(&state.mm, user_id).await?;

    pretty_assertions::assert_eq!(categories, vec![category2, category1]);
    Ok(())
}

#[tokio::test]
async fn test_fetch_cuisines_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let settings = UserSettingDetails::get(&state.mm, user_id).await?;
    let cuisine1 = String::from("italian");
    let cuisine2 = String::from("mexican");
    let (mut recipe, _) = a_complete_recipe_for_create();
    recipe.cuisine = Some(cuisine1.clone());
    let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
    let (mut recipe, _) = a_complete_recipe_for_create();
    recipe.name = "Hello".to_string();
    recipe.cuisine = Some(cuisine2.clone());
    let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

    let got = Recipe::fetch_cuisines(&state.mm, user_id).await?;

    pretty_assertions::assert_eq!(got, vec![cuisine1, cuisine2]);
    Ok(())
}

#[tokio::test]
async fn test_fetch_ingredients_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let settings = UserSettingDetails::get(&state.mm, user_id).await?;
    let (recipe, _) = a_complete_recipe_for_create();
    let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

    let ingredients = Recipe::fetch_ingredients(&state.mm, user_id).await?;

    pretty_assertions::assert_eq!(
        ingredients,
        vec![
            "blue spinach".to_string(),
            "cinnamon".to_string(),
            "lemon juice".to_string(),
            "top quality chicken filet".to_string(),
        ]
    );
    Ok(())
}

#[tokio::test]
async fn test_fetch_keywords_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let settings = UserSettingDetails::get(&state.mm, user_id).await?;
    let (recipe, _) = a_complete_recipe_for_create();
    let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

    let keywords = Recipe::fetch_keywords(&state.mm, user_id).await?;

    pretty_assertions::assert_eq!(
        keywords,
        vec!["tofu".to_string(), "vegetarian".to_string(),]
    );
    Ok(())
}

#[tokio::test]
async fn test_fetch_tools_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let settings = UserSettingDetails::get(&state.mm, user_id).await?;
    let (recipe, _) = a_complete_recipe_for_create();
    let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

    let tools = Recipe::fetch_tools(&state.mm, user_id).await?;

    pretty_assertions::assert_eq!(tools, vec!["frying pan".to_string(), "wok".to_string(),]);
    Ok(())
}

#[tokio::test]
async fn test_fetch_sources_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let settings = UserSettingDetails::get(&state.mm, user_id).await?;
    let (recipe, _) = a_complete_recipe_for_create();
    let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

    let sources = Recipe::fetch_sources(&state.mm, user_id).await?;

    pretty_assertions::assert_eq!(sources, vec!["www.allrecipes.com".to_string(),]);
    Ok(())
}

#[tokio::test]
async fn test_ingredients_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let (recipe, _) = a_complete_recipe_for_create();
    let settings = UserSettingDetails::get(&state.mm, user_id).await?;
    let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

    let got = Recipe::ingredients(&state.mm, recipe_id).await?;

    pretty_assertions::assert_eq!(
        got,
        vec![
            IngredientParser::new(false).from_str("1 cup blue spinach"),
            IngredientParser::new(false).from_str("1/2 tbsp cinnamon"),
            IngredientParser::new(false).from_str("4 pounds top quality chicken filet"),
            IngredientParser::new(false).from_str("1/8 cup lemon juice"),
        ]
    );
    Ok(())
}
