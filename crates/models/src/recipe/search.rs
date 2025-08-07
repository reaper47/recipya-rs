use std::str::FromStr;

use diesel::prelude::*;
use diesel::{JoinOnDsl, NullableExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use diesel_full_text_search::{TsVectorExtensions, to_tsquery, ts_rank};
use repository::extensions::pagination::Paginate;
use repository::{ModelManager, schema};
use winnow::Parser;
use winnow::combinator::alt;
use winnow::token::literal;

use crate::recipe::get::fetch_recipe_details;
use crate::recipe::{Nutrition, Times};
use crate::{Error, Recipe, RecipeDetails, Result};

pub struct RecipeSearch {
    filters: SearchFilters,
    page: i64,
    user_id: i64,
}

impl RecipeSearch {
    pub fn new(query: &str, page: i64, user_id: i64) -> Result<Self> {
        Ok(Self {
            filters: SearchFilters::from_str(query)?,
            page,
            user_id,
        })
    }

    pub async fn search(&self, mm: &ModelManager) -> Result<Vec<RecipeDetails>> {
        use schema::recipes::dsl::*;

        let mut conn = mm.pool.get().await?;

        let mut query = schema::recipes::table
            .inner_join(schema::users_recipes::table.on(schema::users_recipes::recipe_id.eq(id)))
            .filter(schema::users_recipes::user_id.eq(self.user_id))
            .inner_join(schema::categories_recipes::table.inner_join(schema::categories::table))
            .left_join(schema::cuisines_recipes::table.left_join(schema::cuisines::table))
            .left_join(schema::keywords_recipes::table.left_join(schema::keywords::table))
            .left_join(schema::nutrition::table.on(schema::nutrition::recipe_id.eq(id)))
            .inner_join(schema::times::table.on(schema::times::recipe_id.eq(id)))
            .select((
                (
                    id,
                    name,
                    description,
                    image,
                    yield_,
                    language,
                    measurement_system_id,
                    source,
                    created_at,
                    updated_at,
                    user_id,
                ),
                schema::categories::name,
                schema::cuisines::name.nullable(),
                schema::keywords::name.nullable(),
                schema::nutrition::all_columns.nullable(),
                schema::times::all_columns,
            ))
            .distinct_on(id)
            .into_boxed();

        if let Some(text) = &self.filters.category {
            let ts_query = to_tsquery(text);
            query = query.filter(fts_category.matches(ts_query));
        }

        if let Some(text) = &self.filters.cuisine {
            let ts_query = to_tsquery(text);
            query = query.filter(fts_cuisine.matches(ts_query));
        }

        if let Some(text) = &self.filters.ingredients {
            let ts_query = to_tsquery(text);
            query = query.filter(fts_ingredients.matches(ts_query));
        }

        if let Some(text) = &self.filters.instructions {
            let ts_query = to_tsquery(text);
            query = query.filter(fts_instructions.matches(ts_query));
        }

        if let Some(text) = &self.filters.keywords {
            let ts_query = to_tsquery(text);
            query = query.filter(fts_keywords.matches(ts_query));
        }

        if let Some(text) = &self.filters.tools {
            let ts_query = to_tsquery(text);
            query = query.filter(fts_tools.matches(ts_query));
        }

        if let Some(text) = &self.filters.unclassified {
            let ts_query = to_tsquery(text);
            query = query
                .filter(fts_combined.matches(ts_query))
                .order_by((id, ts_rank(fts_combined, ts_query).desc()));
        }

        let page = if self.page < 1 { 1 } else { self.page };

        let fetched_recipes = query
            .paginate(page)
            .load::<(
                Recipe,
                String,
                Option<String>,
                Option<String>,
                Option<Nutrition>,
                Times,
            )>(&mut conn)
            .await?;

        let mut all_recipes = Vec::with_capacity(fetched_recipes.len());
        for (recipe, category, cuisine, keywords, nutrition, times) in fetched_recipes {
            all_recipes.push(
                fetch_recipe_details(
                    &mut conn, recipe, category, cuisine, keywords, nutrition, times,
                )
                .await?,
            );
        }

        Ok(all_recipes)
    }
}

#[derive(Debug, Default, PartialEq)]
struct SearchFilters {
    category: Option<String>,
    cuisine: Option<String>,
    ingredients: Option<String>,
    instructions: Option<String>,
    keywords: Option<String>,
    tools: Option<String>,
    unclassified: Option<String>,
}

impl FromStr for SearchFilters {
    type Err = Error;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(SearchFilters::default());
        }

        let input = s.to_lowercase();
        let mut input = input.as_str();

        let prefixes = ["cat:", "cui:", "ing:", "ins:", "kw:", "tool:"];
        let first_prefix_pos = prefixes.iter().filter_map(|p| input.find(p)).min();
        let mut unclassified: Option<String> = None;

        if let Some(pos) = first_prefix_pos {
            if pos > 0 {
                unclassified = Some(input[0..pos].trim().to_string());
            }
            input = &input[pos..];
        } else {
            return Ok(SearchFilters {
                unclassified: Some(s.to_string()),
                ..Default::default()
            });
        }

        let mut category: Option<String> = None;
        let mut cuisine: Option<String> = None;
        let mut ingredients: Option<String> = None;
        let mut instructions: Option<String> = None;
        let mut keywords: Option<String> = None;
        let mut tools: Option<String> = None;

        while !input.is_empty() {
            match parse_any_section(&mut input) {
                Ok((prefix, text)) => match prefix {
                    "cat:" => category = Some(text),
                    "cui:" => cuisine = Some(text),
                    "ing:" => ingredients = Some(text),
                    "ins:" => instructions = Some(text),
                    "kw:" => keywords = Some(text),
                    "tool:" => tools = Some(text),
                    _ => unreachable!(),
                },
                Err(_) => break,
            }
        }

        Ok(SearchFilters {
            category: category.map(|s| normalize_to_ts_query(&s, "|", "<->")),
            cuisine: cuisine.map(|s| normalize_to_ts_query(&s, "|", "<->")),
            ingredients: ingredients.map(|s| normalize_to_ts_query(&s, "&", "<->")),
            instructions: instructions.map(|s| normalize_to_ts_query(&s, "&", "&")),
            keywords: keywords.map(|s| normalize_to_ts_query(&s, "&", "<->")),
            tools: tools.map(|s| normalize_to_ts_query(&s, "&", "<->")),
            unclassified: unclassified.map(|s| normalize_to_ts_query(&s, "|", "&")),
        })
    }
}

fn normalize_to_ts_query(s: &str, comma_char: &str, space_char: &str) -> String {
    s.trim().replace(",", comma_char).replace(" ", space_char)
}

fn parse_any_section<'a>(input: &mut &'a str) -> winnow::Result<(&'a str, String)> {
    alt((
        parse_section("cat:"),
        parse_section("cui:"),
        parse_section("ing:"),
        parse_section("ins:"),
        parse_section("kw:"),
        parse_section("tool:"),
    ))
    .parse_next(input)
}

fn parse_section<'a>(
    prefix: &'a str,
) -> impl Parser<&'a str, (&'a str, String), winnow::error::ContextError> + 'a {
    move |input: &mut &'a str| {
        let _ = literal(prefix).parse_next(input)?;

        let remaining = *input;
        let prefixes = ["cat:", "cui:", "ing:", "ins:", "kw:", "tool:"];

        let end_pos = prefixes
            .iter()
            .filter_map(|p| remaining.find(p))
            .min()
            .unwrap_or(remaining.len());

        let content = &remaining[..end_pos];
        *input = &remaining[end_pos..];

        Ok((prefix, content.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Recipe;
    use crate::recipe::test_utils::a_complete_recipe_for_create;

    use crate::recipe::{RecipeForCreate, ToolRecipe, Video};
    use testing::utils::{TestDb, create_app_state, insert_user};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_filters {
        use super::*;

        #[test]
        fn test_category_only() {
            let filters = SearchFilters::from_str("cat:Breakfast,midnight dinner").unwrap();

            pretty_assertions::assert_eq!(
                filters,
                SearchFilters {
                    category: Some("breakfast|midnight<->dinner".to_string()),
                    ..Default::default()
                }
            );
        }

        #[test]
        fn test_cuisine_only() {
            let filters = SearchFilters::from_str("cui:indian,thai").unwrap();

            pretty_assertions::assert_eq!(
                filters,
                SearchFilters {
                    cuisine: Some("indian|thai".to_string()),
                    ..Default::default()
                }
            );
        }

        #[test]
        fn test_ingredients_only() {
            let filters = SearchFilters::from_str("ing:blue cheese,paprika").unwrap();

            pretty_assertions::assert_eq!(
                filters,
                SearchFilters {
                    ingredients: Some("blue<->cheese&paprika".to_string()),
                    ..Default::default()
                }
            );
        }

        #[test]
        fn test_instructions_only() {
            let filters =
                SearchFilters::from_str("ins:sprinkle some salt and pepper,mix everything and eat")
                    .unwrap();

            pretty_assertions::assert_eq!(
                filters,
                SearchFilters {
                    instructions: Some(
                        "sprinkle&some&salt&and&pepper&mix&everything&and&eat".to_string()
                    ),
                    ..Default::default()
                }
            );
        }

        #[test]
        fn test_keywords_only() {
            let filters = SearchFilters::from_str("kw:air fryer,healthy,very fat").unwrap();

            pretty_assertions::assert_eq!(
                filters,
                SearchFilters {
                    keywords: Some("air<->fryer&healthy&very<->fat".to_string()),
                    ..Default::default()
                }
            );
        }

        #[test]
        fn test_tools_only() {
            let filters = SearchFilters::from_str("tool:steel pan,wok").unwrap();

            pretty_assertions::assert_eq!(
                filters,
                SearchFilters {
                    tools: Some("steel<->pan&wok".to_string()),
                    ..Default::default()
                }
            );
        }

        #[test]
        fn test_all() {
            let filters =
                SearchFilters::from_str("rip Alexi Laiho cat:Breakfast,midnight dinner cui:thai ing:blue cheese,paprika ins:sprinkle some salt and pepper kw:air fryer,healthy tool:steel pan,wok").unwrap();

            pretty_assertions::assert_eq!(
                filters,
                SearchFilters {
                    category: Some("breakfast|midnight<->dinner".to_string()),
                    cuisine: Some("thai".to_string()),
                    ingredients: Some("blue<->cheese&paprika".to_string()),
                    instructions: Some("sprinkle&some&salt&and&pepper".to_string()),
                    keywords: Some("air<->fryer&healthy".to_string()),
                    tools: Some("steel<->pan&wok".to_string()),
                    unclassified: Some("rip&alexi&laiho".to_string()),
                }
            );
        }
    }

    mod tests_search {
        use super::*;
        use crate::recipe::ToolForCreate;
        use recipe_schema::Sections;

        fn to_recipe_details(id: i64, recipe_c: RecipeForCreate) -> RecipeDetails {
            let mut keywords = recipe_c.keywords;
            keywords.sort();

            let times = recipe_c.times.unwrap_or_default();
            let prep_seconds = times.prep_seconds;
            let cook_seconds = times.cook_seconds;

            RecipeDetails {
                recipe: Recipe {
                    id,
                    name: recipe_c.name,
                    description: recipe_c.description,
                    image: if !recipe_c.images.is_empty() {
                        Some(recipe_c.images[0])
                    } else {
                        None
                    },
                    yield_: recipe_c.yield_.unwrap_or_default(),
                    language: "eng".to_string(),
                    measurement_system_id: 2,
                    source: recipe_c.source,
                    created_at: Default::default(),
                    updated_at: Default::default(),
                    user_id: 1,
                },
                additional_images: if recipe_c.images.len() > 1 {
                    recipe_c.images[1..].to_vec()
                } else {
                    vec![]
                },
                category: recipe_c.category.unwrap_or_default(),
                cuisine: recipe_c.cuisine,
                ingredients: recipe_c.ingredients,
                instructions: recipe_c.instructions,
                keywords,
                nutrition: if let Some(n) = recipe_c.nutrition {
                    Some(Nutrition {
                        id,
                        recipe_id: id,
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
                    })
                } else {
                    None
                },
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
                        tool_order: (i as i16) + 1,
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
                        created_at: Default::default(),
                    })
                    .collect(),
            }
        }

        fn adjust_recipe(mut recipe: RecipeDetails, other_recipe: RecipeDetails) -> RecipeDetails {
            recipe.recipe.created_at = other_recipe.recipe.created_at;
            recipe.recipe.updated_at = other_recipe.recipe.updated_at;
            recipe.videos = other_recipe.videos.clone();
            recipe
        }

        #[tokio::test]
        async fn test_search_by_name() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let user = insert_user(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            let mut a_recipe = a_complete_recipe_for_create();
            let _ = Recipe::create(&state.mm, user.id, &a_recipe).await?;
            a_recipe.name = "Taco Tuesday".to_string();
            let _ = Recipe::create(&state.mm, user.id, &a_recipe).await?;

            let recipe_search = RecipeSearch::new("chinese", 1, user.id)?;
            let results = recipe_search.search(&state.mm).await?;

            pretty_assertions::assert_eq!(
                results,
                vec![adjust_recipe(
                    to_recipe_details(1, a_complete_recipe_for_create()),
                    results[0].clone()
                ),]
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_search_by_rank() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let user = insert_user(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            let recipe1 = a_complete_recipe_for_create();
            let mut recipe2 = a_complete_recipe_for_create();
            recipe2.name = "Taco Tuesday".to_string();
            recipe2.description = Some("The most authentic Chinese recipe ever".to_string());
            let mut recipe3 = a_complete_recipe_for_create();
            recipe3.name = "Chicken Jersey".to_string();
            recipe3.description = Some("The most authentic tacos recipe ever".to_string());
            insert_recipes(&state.mm, user.id, vec![&recipe1, &recipe2, &recipe3]).await?;

            let recipe_search = RecipeSearch::new("chinese", 1, user.id)?;
            let results = recipe_search.search(&state.mm).await?;

            pretty_assertions::assert_eq!(
                results,
                vec![
                    adjust_recipe(to_recipe_details(1, recipe1), results[0].clone()),
                    adjust_recipe(to_recipe_details(2, recipe2), results[1].clone()),
                ]
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_search_by_category() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let user = insert_user(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            let recipe1 = a_complete_recipe_for_create();
            let mut recipe2 = a_complete_recipe_for_create();
            recipe2.name = "Taco Tuesday".to_string();
            recipe2.category = Some("Meat".to_string());
            let mut recipe3 = a_complete_recipe_for_create();
            recipe3.name = "Chicken Jersey".to_string();
            recipe3.category = Some("breakfast".to_string());
            insert_recipes(&state.mm, user.id, vec![&recipe1, &recipe2, &recipe3]).await?;

            let recipe_search = RecipeSearch::new("cat:Breakfast", 1, user.id)?;
            let results = recipe_search.search(&state.mm).await?;

            pretty_assertions::assert_eq!(
                results,
                vec![adjust_recipe(
                    to_recipe_details(3, recipe3),
                    results[0].clone()
                ),]
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_search_by_category_and_unclassified() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let user = insert_user(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            let recipe1 = a_complete_recipe_for_create();
            let mut recipe2 = a_complete_recipe_for_create();
            recipe2.name = "Taco Tuesday".to_string();
            recipe2.category = Some("breakfast".to_string());
            let mut recipe3 = a_complete_recipe_for_create();
            recipe3.name = "Chicken Jersey".to_string();
            recipe3.category = Some("breakfast".to_string());
            insert_recipes(&state.mm, user.id, vec![&recipe1, &recipe2, &recipe3]).await?;

            let recipe_search = RecipeSearch::new("tacos cat:Breakfast", 1, user.id)?;
            let results = recipe_search.search(&state.mm).await?;

            pretty_assertions::assert_eq!(
                results,
                vec![adjust_recipe(
                    to_recipe_details(2, recipe2),
                    results[0].clone()
                ),]
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_search_by_cuisine() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let user = insert_user(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            let recipe1 = a_complete_recipe_for_create();
            let mut recipe2 = a_complete_recipe_for_create();
            recipe2.name = "Taco Tuesday".to_string();
            recipe2.cuisine = Some("thai".to_string());
            let mut recipe3 = a_complete_recipe_for_create();
            recipe3.name = "Chicken Jersey".to_string();
            recipe3.cuisine = Some("Chinese".to_string());
            insert_recipes(&state.mm, user.id, vec![&recipe1, &recipe2, &recipe3]).await?;

            let recipe_search = RecipeSearch::new("cui:THAI", 1, user.id)?;
            let results = recipe_search.search(&state.mm).await?;

            pretty_assertions::assert_eq!(
                results,
                vec![
                    adjust_recipe(to_recipe_details(1, recipe1), results[0].clone()),
                    adjust_recipe(to_recipe_details(2, recipe2), results[1].clone()),
                ]
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_search_by_ingredients() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let user = insert_user(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            let recipe1 = a_complete_recipe_for_create();
            let mut recipe2 = a_complete_recipe_for_create();
            recipe2.name = "Taco Tuesday".to_string();
            recipe2.ingredients = Sections::from([(
                "".into(),
                vec!["tomato".to_string(), "1/2 cups of lettuce".to_string()],
            )]);
            let mut recipe3 = a_complete_recipe_for_create();
            recipe3.name = "Chicken Jersey".to_string();
            recipe3.ingredients = Sections::from([(
                "".into(),
                vec![
                    "1 tbsp of hot cayenne pepper".to_string(),
                    "3 lbs of chicken breasts".to_string(),
                ],
            )]);
            insert_recipes(&state.mm, user.id, vec![&recipe1, &recipe2, &recipe3]).await?;

            let recipe_search = RecipeSearch::new("ing:cayenne pepper,chicken", 1, user.id)?;
            let results = recipe_search.search(&state.mm).await?;

            pretty_assertions::assert_eq!(
                results,
                vec![adjust_recipe(
                    to_recipe_details(3, recipe3),
                    results[0].clone()
                )]
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_search_by_instructions() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let user = insert_user(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            let recipe1 = a_complete_recipe_for_create();
            let mut recipe2 = a_complete_recipe_for_create();
            recipe2.name = "Taco Tuesday".to_string();
            recipe2.instructions = Sections::from([("".into(), vec![
                "Sauté veggies: In a large pot, melt butter over medium heat. Add onions and garlic, cooking until soft (about 5 minutes). Add mushrooms and cook until they release moisture and begin to brown ".to_string(),
                "Make roux: Sprinkle flour over the mushrooms and stir well to coat. Cook for 1–2 minutes to eliminate the raw flour taste.".to_string(),
            ])]);
            let mut recipe3 = a_complete_recipe_for_create();
            recipe3.name = "Chicken Jersey".to_string();
            recipe3.instructions = Sections::from([("".into(), vec![
                "Boil pasta: Bring a large pot of salted water to a boil. Add spaghetti and cook until al dente according to package directions. Reserve 1 cup of pasta water before draining.".to_string(),
                "Sauté garlic: While pasta cooks, heat olive oil in a large skillet over medium heat. Add sliced garlic and red pepper flakes. Cook until garlic is golden (1–2 minutes), stirring constantly to prevent burning.".to_string(),
            ])]);
            insert_recipes(&state.mm, user.id, vec![&recipe1, &recipe2, &recipe3]).await?;

            let recipe_search = RecipeSearch::new("ins:melt butter medium heat", 1, user.id)?;
            let results = recipe_search.search(&state.mm).await?;

            pretty_assertions::assert_eq!(
                results,
                vec![adjust_recipe(
                    to_recipe_details(2, recipe2),
                    results[0].clone()
                )]
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_search_by_keywords() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let user = insert_user(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            let recipe1 = a_complete_recipe_for_create();
            let mut recipe2 = a_complete_recipe_for_create();
            recipe2.name = "Taco Tuesday".to_string();
            recipe2.keywords = vec!["healthy".to_string(), "vegan".to_string()];
            let mut recipe3 = a_complete_recipe_for_create();
            recipe3.name = "Chicken Jersey".to_string();
            recipe3.keywords = vec!["very fat".to_string(), "air fryer".to_string()];
            insert_recipes(&state.mm, user.id, vec![&recipe1, &recipe2, &recipe3]).await?;

            let recipe_search = RecipeSearch::new("kw:very fat,air fryer", 1, user.id)?;
            let results = recipe_search.search(&state.mm).await?;

            pretty_assertions::assert_eq!(
                results,
                vec![adjust_recipe(
                    to_recipe_details(3, recipe3),
                    results[0].clone()
                )]
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_search_by_tools() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let user = insert_user(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            let recipe1 = a_complete_recipe_for_create();
            let mut recipe2 = a_complete_recipe_for_create();
            recipe2.name = "Taco Tuesday".to_string();
            recipe2.tools = vec![ToolForCreate {
                name: "wok".to_string(),
                quantity: 1,
            }];
            let mut recipe3 = a_complete_recipe_for_create();
            recipe3.name = "Chicken Jersey".to_string();
            recipe3.tools = vec![ToolForCreate {
                name: "frying pan".to_string(),
                quantity: 1,
            }];
            insert_recipes(&state.mm, user.id, vec![&recipe1, &recipe2, &recipe3]).await?;

            let recipe_search = RecipeSearch::new("tool:wok", 1, user.id)?;
            let results = recipe_search.search(&state.mm).await?;

            pretty_assertions::assert_eq!(
                results,
                vec![
                    adjust_recipe(to_recipe_details(1, recipe1), results[0].clone()),
                    adjust_recipe(to_recipe_details(2, recipe2), results[1].clone()),
                ]
            );
            Ok(())
        }

        async fn insert_recipes(
            mm: &ModelManager,
            user_id: i64,
            recipes: Vec<&RecipeForCreate>,
        ) -> Result<()> {
            for recipe in recipes {
                let _ = Recipe::create(mm, user_id, recipe).await?;
            }
            Ok(())
        }
    }
}
