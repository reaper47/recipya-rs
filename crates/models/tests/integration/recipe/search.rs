use time::PrimitiveDateTime;
use uuid::Uuid;

use models::{
    Recipe, RecipeDetails,
    recipe::structs::{
        media::Video,
        nutrition::NutritionDetails,
        recipe::RecipeForCreate,
        section::{Item, SectionComponents},
        tool::{ToolForCreate, ToolRecipe},
    },
    recipe::{RecipeSearch, structs::time::Times},
    settings::UserSettingDetails,
};
use repository::ModelManager;
use test_db::default_config;
use test_fixtures::insert_user;
use test_harness::create_app_state;

use crate::recipe::utils::a_complete_recipe_for_create;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

mod tests_search {
    use super::*;

    fn to_recipe_details(id: i64, mut recipe_c: RecipeForCreate, user_id: Uuid) -> RecipeDetails {
        let mut keywords = recipe_c.keywords;
        keywords.sort();

        let times = recipe_c.times.unwrap_or_default();
        let prep_seconds = times.prep_seconds;
        let cook_seconds = times.cook_seconds;

        recipe_c
            .instructions
            .iter_mut()
            .enumerate()
            .for_each(|(idx, item)| item.id = Some(i64::try_from(idx + 1).unwrap_or_default()));

        RecipeDetails {
            recipe: Recipe {
                id,
                name: recipe_c.name,
                description: recipe_c.description,
                image: if recipe_c.images.is_empty() {
                    None
                } else {
                    Some(recipe_c.images[0]).filter(|u| !u.is_nil())
                },
                r#yield: recipe_c.r#yield.unwrap_or_default(),
                language: "eng".to_string(),
                measurement_system_id: 2,
                notes: recipe_c.notes,
                source: recipe_c.source,
                is_favourite: false,
                rating: recipe_c.rating,
                created_at: PrimitiveDateTime::MIN,
                updated_at: PrimitiveDateTime::MIN,
                user_id,
            },
            additional_images: if recipe_c.images.len() > 1 {
                recipe_c.images[1..]
                    .iter()
                    .copied()
                    .filter(|u| !u.is_nil())
                    .collect()
            } else {
                vec![]
            },
            category: recipe_c.category.unwrap_or_default(),
            cuisine: recipe_c.cuisine,
            ingredients: recipe_c.ingredients,
            instructions: recipe_c.instructions,
            keywords,
            nutrition: NutritionDetails::from(&recipe_c.nutrition),
            times: Times {
                id,
                recipe_id: id,
                prep_seconds,
                cook_seconds,
                total_seconds: prep_seconds + cook_seconds,
            },
            tools: recipe_c
                .tools
                .into_iter()
                .enumerate()
                .map(|(i, t)| ToolRecipe {
                    name: t.name,
                    quantity: t.quantity,
                    tool_order: i16::try_from(i).unwrap_or_default() + 1,
                })
                .collect(),
            videos: recipe_c
                .videos
                .into_iter()
                .map(|v| Video {
                    video: v.video,
                    duration: v.duration,
                    content_url: v.content_url,
                    embed_url: v.embed_url,
                    created_at: PrimitiveDateTime::MIN,
                })
                .collect(),
        }
    }

    fn adjust_recipe(mut recipe: RecipeDetails, other_recipe: RecipeDetails) -> RecipeDetails {
        if let Some(n) = recipe.nutrition.per_100g.as_mut() {
            let other_n = other_recipe.nutrition.per_100g.unwrap();
            n.id = other_n.id;
            n.is_precalculated_by_source = other_n.is_precalculated_by_source;
        }

        if let Some(n) = recipe.nutrition.per_serving.as_mut() {
            let other_n = other_recipe.nutrition.per_serving.unwrap().nutrition;
            n.nutrition.id = other_n.id;
            n.nutrition.is_precalculated_by_source = other_n.is_precalculated_by_source;
        }

        remove_id_from_recipes(&mut recipe);

        recipe.recipe.id = other_recipe.recipe.id;
        recipe.recipe.name = other_recipe.recipe.name;
        recipe.recipe.created_at = other_recipe.recipe.created_at;
        recipe.recipe.updated_at = other_recipe.recipe.updated_at;
        recipe.times.id = other_recipe.times.id;
        recipe.times.recipe_id = other_recipe.times.recipe_id;
        recipe.videos = other_recipe.videos;

        recipe
    }

    fn remove_id_from_recipes(recipe: &mut RecipeDetails) {
        match &mut recipe.instructions {
            SectionComponents::Grouped(section_items) => {
                for item in section_items.iter_mut() {
                    item.items.iter_mut().for_each(|i| i.id = None);
                }
            }
            SectionComponents::Flat(items) => items.iter_mut().for_each(|i| i.id = None),
        }
    }

    #[tokio::test]
    async fn test_search_by_name() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let (mut a_recipe, _) = a_complete_recipe_for_create();
        let _ = Recipe::create(&state.mm, user.id, &a_recipe, &settings).await?;
        a_recipe.name = "Taco Tuesday".to_string();
        let _ = Recipe::create(&state.mm, user.id, &a_recipe, &settings).await?;

        let recipe_search = RecipeSearch::new("chinese", 1, false, user.id)?;
        let mut results = recipe_search.search(&state.mm).await?;

        results.iter_mut().for_each(remove_id_from_recipes);
        pretty_assertions::assert_eq!(
            results,
            vec![adjust_recipe(
                to_recipe_details(1, a_recipe, user.id),
                results[0].clone()
            )]
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_search_by_rank() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let (recipe1, _) = a_complete_recipe_for_create();
        let (mut recipe2, _) = a_complete_recipe_for_create();
        recipe2.name = "Taco Tuesday".to_string();
        recipe2.description = Some("The most authentic Chinese recipe ever".to_string());
        let (mut recipe3, _) = a_complete_recipe_for_create();
        recipe3.name = "Chicken Jersey".to_string();
        recipe3.description = Some("The most authentic tacos recipe ever".to_string());
        insert_recipes(
            &state.mm,
            user.id,
            &settings,
            vec![&recipe1, &recipe2, &recipe3],
        )
        .await?;

        let recipe_search = RecipeSearch::new("chinese", 1, false, user.id)?;
        let mut results = recipe_search.search(&state.mm).await?;

        results.iter_mut().for_each(remove_id_from_recipes);
        pretty_assertions::assert_eq!(
            results,
            vec![
                adjust_recipe(to_recipe_details(1, recipe1, user.id), results[0].clone()),
                adjust_recipe(to_recipe_details(2, recipe2, user.id), results[1].clone()),
            ]
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_search_by_category() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let (recipe1, _) = a_complete_recipe_for_create();
        let (mut recipe2, _) = a_complete_recipe_for_create();
        recipe2.name = "Taco Tuesday".to_string();
        recipe2.category = Some("Meat".to_string());
        let (mut recipe3, _) = a_complete_recipe_for_create();
        recipe3.name = "Chicken Jersey".to_string();
        recipe3.category = Some("breakfast".to_string());
        insert_recipes(
            &state.mm,
            user.id,
            &settings,
            vec![&recipe1, &recipe2, &recipe3],
        )
        .await?;

        let recipe_search = RecipeSearch::new("cat:Breakfast", 1, false, user.id)?;
        let mut results = recipe_search.search(&state.mm).await?;

        for result in &mut results {
            match &mut result.instructions {
                SectionComponents::Grouped(section_items) => {
                    for item in section_items.iter_mut() {
                        item.items.iter_mut().for_each(|i| i.id = None);
                    }
                }
                SectionComponents::Flat(items) => items.iter_mut().for_each(|i| i.id = None),
            }
        }
        pretty_assertions::assert_eq!(
            results,
            vec![adjust_recipe(
                to_recipe_details(3, recipe3, user.id),
                results[0].clone()
            )]
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_search_by_category_and_unclassified() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let (recipe1, _) = a_complete_recipe_for_create();
        let (mut recipe2, _) = a_complete_recipe_for_create();
        recipe2.name = "Taco Tuesday".to_string();
        recipe2.category = Some("breakfast".to_string());
        let (mut recipe3, _) = a_complete_recipe_for_create();
        recipe3.name = "Chicken Jersey".to_string();
        recipe3.category = Some("breakfast".to_string());
        insert_recipes(
            &state.mm,
            user.id,
            &settings,
            vec![&recipe1, &recipe2, &recipe3],
        )
        .await?;

        let recipe_search = RecipeSearch::new("tacos cat:Breakfast", 1, false, user.id)?;
        let mut results = recipe_search.search(&state.mm).await?;

        results.iter_mut().for_each(remove_id_from_recipes);
        pretty_assertions::assert_eq!(
            results,
            vec![adjust_recipe(
                to_recipe_details(2, recipe2, user.id),
                results[0].clone()
            ),]
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_search_by_cuisine() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let (recipe1, _) = a_complete_recipe_for_create();
        let (mut recipe2, _) = a_complete_recipe_for_create();
        recipe2.name = "Taco Tuesday".to_string();
        recipe2.cuisine = Some("thai".to_string());
        let (mut recipe3, _) = a_complete_recipe_for_create();
        recipe3.name = "Chicken Jersey".to_string();
        recipe3.cuisine = Some("Chinese".to_string());
        insert_recipes(
            &state.mm,
            user.id,
            &settings,
            vec![&recipe1, &recipe2, &recipe3],
        )
        .await?;

        let recipe_search = RecipeSearch::new("cui:THAI", 1, false, user.id)?;
        let mut results = recipe_search.search(&state.mm).await?;

        results.iter_mut().for_each(remove_id_from_recipes);
        pretty_assertions::assert_eq!(
            results,
            vec![
                adjust_recipe(to_recipe_details(1, recipe1, user.id), results[0].clone()),
                adjust_recipe(to_recipe_details(2, recipe2, user.id), results[1].clone()),
            ]
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_search_by_ingredients() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let (recipe1, _) = a_complete_recipe_for_create();
        let (mut recipe2, _) = a_complete_recipe_for_create();
        recipe2.name = "Taco Tuesday".to_string();
        recipe2.ingredients =
            SectionComponents::Flat(vec![Item::new("tomato"), Item::new("1/2 cups of lettuce")]);
        let (mut recipe3, _) = a_complete_recipe_for_create();
        recipe3.name = "Chicken Jersey".to_string();
        recipe3.ingredients = SectionComponents::Flat(vec![
            Item::new("1 tbsp of hot cayenne pepper"),
            Item::new("3 lbs of chicken breasts"),
        ]);
        insert_recipes(
            &state.mm,
            user.id,
            &settings,
            vec![&recipe1, &recipe2, &recipe3],
        )
        .await?;

        let recipe_search = RecipeSearch::new("ing:cayenne pepper,chicken", 1, false, user.id)?;
        let mut results = recipe_search.search(&state.mm).await?;

        results.iter_mut().for_each(remove_id_from_recipes);
        pretty_assertions::assert_eq!(
            results,
            vec![adjust_recipe(
                to_recipe_details(3, recipe3, user.id),
                results[0].clone()
            )]
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_search_by_instructions() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let (recipe1, _) = a_complete_recipe_for_create();
        let (mut recipe2, _) = a_complete_recipe_for_create();
        recipe2.name = "Taco Tuesday".to_string();
        recipe2.instructions = SectionComponents::Flat(vec![
            Item::new("Sauté veggies: In a large pot, melt butter over medium heat. Add onions and garlic, cooking until soft (about 5 minutes). Add mushrooms and cook until they release moisture and begin to brown").with_duration(300),
            Item::new("Make roux: Sprinkle flour over the mushrooms and stir well to coat. Cook for 1–2 minutes to eliminate the raw flour taste.").with_duration(120),
        ]);
        let (mut recipe3, _) = a_complete_recipe_for_create();
        recipe3.name = "Chicken Jersey".to_string();
        recipe3.instructions = SectionComponents::Flat(vec![
            Item::new("Boil pasta: Bring a large pot of salted water to a boil. Add spaghetti and cook until al dente according to package directions. Reserve 1 cup of pasta water before draining."),
            Item::new("Sauté garlic: While pasta cooks, heat olive oil in a large skillet over medium heat. Add sliced garlic and red pepper flakes. Cook until garlic is golden (1–2 minutes), stirring constantly to prevent burning.").with_duration(120),
        ]);
        insert_recipes(
            &state.mm,
            user.id,
            &settings,
            vec![&recipe1, &recipe2, &recipe3],
        )
        .await?;

        let recipe_search = RecipeSearch::new("ins:melt butter medium heat", 1, false, user.id)?;
        let mut results = recipe_search.search(&state.mm).await?;

        results.iter_mut().for_each(remove_id_from_recipes);
        let expected = adjust_recipe(to_recipe_details(2, recipe2, user.id), results[0].clone());
        pretty_assertions::assert_eq!(results, vec![expected]);
        Ok(())
    }

    #[tokio::test]
    async fn test_search_by_keywords() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let (recipe1, _) = a_complete_recipe_for_create();
        let (mut recipe2, _) = a_complete_recipe_for_create();
        recipe2.name = "Taco Tuesday".to_string();
        recipe2.keywords = vec!["healthy".to_string(), "vegan".to_string()];
        let (mut recipe3, _) = a_complete_recipe_for_create();
        recipe3.name = "Chicken Jersey".to_string();
        recipe3.keywords = vec!["very fat".to_string(), "air fryer".to_string()];
        insert_recipes(
            &state.mm,
            user.id,
            &settings,
            vec![&recipe1, &recipe2, &recipe3],
        )
        .await?;

        let recipe_search = RecipeSearch::new("kw:very fat,air fryer", 1, false, user.id)?;
        let mut results = recipe_search.search(&state.mm).await?;

        results.iter_mut().for_each(remove_id_from_recipes);
        pretty_assertions::assert_eq!(
            results,
            vec![adjust_recipe(
                to_recipe_details(3, recipe3, user.id),
                results[0].clone()
            )]
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_search_by_rating() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let (recipe1, _) = a_complete_recipe_for_create();
        let (mut recipe2, _) = a_complete_recipe_for_create();
        recipe2.name = "Taco Tuesday".to_string();
        recipe2.rating = Some(1);
        let (mut recipe3, _) = a_complete_recipe_for_create();
        recipe3.name = "Chicken Jersey".to_string();
        recipe3.rating = None;
        insert_recipes(
            &state.mm,
            user.id,
            &settings,
            vec![&recipe1, &recipe2, &recipe3],
        )
        .await?;

        let recipe_search = RecipeSearch::new("stars:1", 1, false, user.id)?;
        let mut results = recipe_search.search(&state.mm).await?;

        results.iter_mut().for_each(remove_id_from_recipes);
        pretty_assertions::assert_eq!(
            results,
            vec![adjust_recipe(
                to_recipe_details(2, recipe2, user.id),
                results[0].clone()
            ),]
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_search_by_tools() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let (recipe1, _) = a_complete_recipe_for_create();
        let (mut recipe2, _) = a_complete_recipe_for_create();
        recipe2.name = "Taco Tuesday".to_string();
        recipe2.tools = vec![ToolForCreate {
            name: "wok".to_string(),
            quantity: 1,
        }];
        let (mut recipe3, _) = a_complete_recipe_for_create();
        recipe3.name = "Chicken Jersey".to_string();
        recipe3.tools = vec![ToolForCreate {
            name: "frying pan".to_string(),
            quantity: 1,
        }];
        insert_recipes(
            &state.mm,
            user.id,
            &settings,
            vec![&recipe1, &recipe2, &recipe3],
        )
        .await?;

        let recipe_search = RecipeSearch::new("tool:wok", 1, false, user.id)?;
        let mut results = recipe_search.search(&state.mm).await?;

        results.iter_mut().for_each(remove_id_from_recipes);
        pretty_assertions::assert_eq!(
            results,
            vec![
                adjust_recipe(to_recipe_details(1, recipe1, user.id), results[0].clone()),
                adjust_recipe(to_recipe_details(2, recipe2, user.id), results[1].clone()),
            ]
        );
        Ok(())
    }

    async fn insert_recipes(
        mm: &ModelManager,
        user_id: Uuid,
        settings: &UserSettingDetails,
        recipes: Vec<&RecipeForCreate>,
    ) -> Result<()> {
        for recipe in recipes {
            let _ = Recipe::create(mm, user_id, recipe, settings).await?;
        }
        Ok(())
    }
}
