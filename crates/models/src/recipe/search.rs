use std::str::FromStr;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use diesel_full_text_search::{TsVectorExtensions, plainto_tsquery, to_tsquery, ts_rank};
use uuid::Uuid;
use winnow::Parser;
use winnow::combinator::alt;
use winnow::token::literal;

use repository::extensions::pagination::Paginate;
use repository::{ModelManager, schema};

use crate::recipe::get::fetch_recipe_details;
use crate::recipe::structs::time::Times;
use crate::{Error, Recipe, RecipeDetails, Result};

const DEFAULT_RECIPES_PER_PAGE: i64 = 15;

pub struct RecipeSearch {
    filters: SearchFilters,
    page: i64,
    is_favourites: bool,
    user_id: Uuid,
}

impl RecipeSearch {
    pub fn new(query: &str, page: i64, is_favourites: bool, user_id: Uuid) -> Result<Self> {
        Ok(Self {
            filters: SearchFilters::from_str(query)?,
            page,
            is_favourites,
            user_id,
        })
    }

    pub async fn search(&self, mm: &ModelManager) -> Result<Vec<RecipeDetails>> {
        use schema::recipes::dsl::{
            created_at, description, fts_category, fts_combined, fts_cuisine, fts_ingredients,
            fts_instructions, fts_keywords, fts_tools, id, image, is_favourite, language,
            measurement_system_id, name, notes, rating, source, updated_at, user_id, yield_,
        };

        let mut conn = mm.pool.get().await?;

        let mut query = schema::recipes::table
            .inner_join(schema::users_recipes::table.on(schema::users_recipes::recipe_id.eq(id)))
            .filter(schema::users_recipes::user_id.eq(self.user_id))
            .filter(schema::recipes::user_id.eq(self.user_id))
            .inner_join(schema::categories_recipes::table.inner_join(schema::categories::table))
            .left_join(schema::cuisines_recipes::table.left_join(schema::cuisines::table))
            .left_join(schema::keywords_recipes::table.left_join(schema::keywords::table))
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
                    notes,
                    source,
                    is_favourite,
                    rating,
                    created_at,
                    updated_at,
                    user_id,
                ),
                schema::categories::name,
                schema::cuisines::name.nullable(),
                schema::keywords::name.nullable(),
                schema::times::all_columns,
            ))
            .distinct_on(id)
            .into_boxed();

        if self.is_favourites {
            query = query.filter(is_favourite.eq(true));
        }

        if let Some(text) = &self.filters.category {
            query = query.filter(fts_category.matches(to_tsquery(text)));
        }

        if let Some(text) = &self.filters.cuisine {
            query = query.filter(fts_cuisine.matches(to_tsquery(text)));
        }

        if let Some(text) = &self.filters.ingredients {
            query = query.filter(fts_ingredients.matches(to_tsquery(text)));
        }

        if let Some(text) = &self.filters.instructions {
            query = query.filter(fts_instructions.matches(to_tsquery(text)));
        }

        if let Some(text) = &self.filters.keywords {
            query = query.filter(fts_keywords.matches(to_tsquery(text)));
        }

        if let Some(text) = &self.filters.name {
            let sanitized = text.replace("<->", " ");
            query = query.filter(schema::recipes::name.ilike(format!("%{}%", sanitized.trim())));
        }

        if let Some(n) = self.filters.rating {
            query = query.filter(rating.eq(n));
        }

        if let Some(text) = &self.filters.tools {
            query = query.filter(fts_tools.matches(to_tsquery(text)));
        }

        if let Some(text) = &self.filters.unclassified {
            let ts_query = plainto_tsquery(text);
            query = query
                .filter(fts_combined.matches(ts_query))
                .order_by((id, ts_rank(fts_combined, ts_query).desc()));
        } else {
            query = query.order_by(id);
        }

        let fetched_recipes = query
            .paginate(self.page.max(1), DEFAULT_RECIPES_PER_PAGE)
            .load::<(Recipe, String, Option<String>, Option<String>, Times)>(&mut conn)
            .await?;

        let mut all_recipes = Vec::with_capacity(fetched_recipes.len());
        for (recipe, category, cuisine, keywords, times) in fetched_recipes {
            all_recipes.push(
                fetch_recipe_details(&mut conn, recipe, category, cuisine, keywords, times).await?,
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
    is_favourites: bool,
    keywords: Option<String>,
    name: Option<String>,
    rating: Option<i16>,
    tools: Option<String>,
    unclassified: Option<String>,
}

const PREFIX_CATEGORY: &str = "cat:";
const PREFIX_CUISINE: &str = "cui:";
const PREFIX_INGREDIENTS: &str = "ing:";
const PREFIX_INSTRUCTIONS: &str = "ins:";
const PREFIX_KEYWORDS: &str = "kw:";
const PREFIX_NAME: &str = "name:";
const PREFIX_RATING: &str = "stars:";
const PREFIX_TOOLS: &str = "tool:";

impl FromStr for SearchFilters {
    type Err = Error;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Self::default());
        }

        let input = s.to_lowercase();
        let mut input = input.as_str();

        let prefixes = [
            PREFIX_CATEGORY,
            PREFIX_CUISINE,
            PREFIX_INGREDIENTS,
            PREFIX_INSTRUCTIONS,
            PREFIX_KEYWORDS,
            PREFIX_NAME,
            PREFIX_RATING,
            PREFIX_TOOLS,
        ];
        let first_prefix_pos = prefixes.iter().filter_map(|p| input.find(p)).min();
        let mut unclassified: Option<String> = None;

        if let Some(pos) = first_prefix_pos {
            if pos > 0 {
                unclassified = Some(input[0..pos].trim().to_string());
            }
            input = &input[pos..];
        } else {
            return Ok(Self {
                unclassified: Some(s.to_string()),
                ..Default::default()
            });
        }

        let mut category: Option<String> = None;
        let mut cuisine: Option<String> = None;
        let mut ingredients: Option<String> = None;
        let mut instructions: Option<String> = None;
        let mut keywords: Option<String> = None;
        let mut name: Option<String> = None;
        let mut rating: Option<i16> = None;
        let mut tools: Option<String> = None;

        while !input.is_empty() {
            match parse_any_section(&mut input) {
                Ok((prefix, text)) => match prefix {
                    PREFIX_CATEGORY => category = Some(text),
                    PREFIX_CUISINE => cuisine = Some(text),
                    PREFIX_INGREDIENTS => ingredients = Some(text),
                    PREFIX_INSTRUCTIONS => instructions = Some(text),
                    PREFIX_KEYWORDS => keywords = Some(text),
                    PREFIX_NAME => name = Some(text),
                    PREFIX_RATING => {
                        rating = text
                            .trim()
                            .parse::<i16>()
                            .ok()
                            .filter(|r| *r >= 1 && *r <= 5);
                    }
                    PREFIX_TOOLS => tools = Some(text),
                    _ => unreachable!(),
                },
                Err(_) => break,
            }
        }

        Ok(Self {
            category: category.map(|s| normalize_to_ts_query(&s, "|", "<->")),
            cuisine: cuisine.map(|s| normalize_to_ts_query(&s, "|", "<->")),
            ingredients: ingredients.map(|s| normalize_to_ts_query(&s, "&", "<->")),
            instructions: instructions.map(|s| normalize_to_ts_query(&s, "&", "&")),
            is_favourites: false,
            keywords: keywords.map(|s| normalize_to_ts_query(&s, "&", "<->")),
            name: name.map(|s| normalize_to_ts_query(&s, "&", "<->")),
            rating,
            tools: tools.map(|s| normalize_to_ts_query(&s, "&", "<->")),
            unclassified: unclassified.map(|s| normalize_to_ts_query(&s, "|", "&")),
        })
    }
}

fn normalize_to_ts_query(s: &str, comma_char: &str, space_char: &str) -> String {
    s.trim().replace(',', comma_char).replace(' ', space_char)
}

fn parse_any_section<'a>(input: &mut &'a str) -> winnow::Result<(&'a str, String)> {
    alt((
        parse_section(PREFIX_CATEGORY),
        parse_section(PREFIX_CUISINE),
        parse_section(PREFIX_INGREDIENTS),
        parse_section(PREFIX_INSTRUCTIONS),
        parse_section(PREFIX_KEYWORDS),
        parse_section(PREFIX_NAME),
        parse_section(PREFIX_RATING),
        parse_section(PREFIX_TOOLS),
    ))
    .parse_next(input)
}

fn parse_section<'a>(
    prefix: &'a str,
) -> impl Parser<&'a str, (&'a str, String), winnow::error::ContextError> + 'a {
    move |input: &mut &'a str| {
        let _ = literal(prefix).parse_next(input)?;

        let remaining = *input;
        let prefixes = [
            PREFIX_CATEGORY,
            PREFIX_CUISINE,
            PREFIX_INGREDIENTS,
            PREFIX_INSTRUCTIONS,
            PREFIX_KEYWORDS,
            PREFIX_NAME,
            PREFIX_RATING,
            PREFIX_TOOLS,
        ];

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
        fn test_name_only() {
            let filters = SearchFilters::from_str("name:cute kitty").unwrap();

            pretty_assertions::assert_eq!(
                filters,
                SearchFilters {
                    name: Some("cute<->kitty".to_string()),
                    ..Default::default()
                }
            );
        }

        #[test]
        fn test_rating_only() {
            let filters = SearchFilters::from_str("stars:4").unwrap();

            pretty_assertions::assert_eq!(
                filters,
                SearchFilters {
                    rating: Some(4),
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
                SearchFilters::from_str("rip Alexi Laiho cat:Breakfast,midnight dinner cui:thai ing:blue cheese,paprika ins:sprinkle some salt and pepper stars:2 kw:air fryer,healthy tool:steel pan,wok name:cute kitty").unwrap();

            pretty_assertions::assert_eq!(
                filters,
                SearchFilters {
                    category: Some("breakfast|midnight<->dinner".to_string()),
                    cuisine: Some("thai".to_string()),
                    ingredients: Some("blue<->cheese&paprika".to_string()),
                    instructions: Some("sprinkle&some&salt&and&pepper".to_string()),
                    is_favourites: false,
                    keywords: Some("air<->fryer&healthy".to_string()),
                    name: Some("cute<->kitty".to_string()),
                    rating: Some(2),
                    tools: Some("steel<->pan&wok".to_string()),
                    unclassified: Some("rip&alexi&laiho".to_string()),
                }
            );
        }
    }
}
