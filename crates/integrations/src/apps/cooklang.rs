use std::io::{Read, Seek};

use cooklang::{Content, CooklangParser, Item, Value};
use tracing::{error, warn};
use url::Url;

use schema_org::enums::RestrictedDietEnum;
use schema_org::field::{
    HowToToolRequiredQuantityFieldEnum, RecipeAuthorFieldEnum, RecipeDescriptionFieldEnum,
    RecipeImageFieldEnum, RecipeIsBasedOnFieldEnum, RecipeKeywordsFieldEnum,
    RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum, RecipeToolFieldEnum,
};
use schema_org::{AtType, HowToTool, Recipe};
use support::time::parse_duration;

use super::helpers::read_file;
use crate::Result;
use crate::common::{Times, Tool};
use crate::helpers::{seconds_to_duration, to_yield};

/// A wrapper around the Cooklang parser that provides a consistent interface for parsing
/// and executing Cooklang code.
#[derive(Default)]
pub struct CookLang {
    parser: CooklangParser,
}

#[allow(dead_code)]
struct CooklangRecipe {
    author: Option<String>,
    category: Option<String>,
    cuisine: Option<String>,
    description: Option<String>,
    diet: Option<Vec<String>>,
    difficulty: Option<String>,
    locale: Option<String>,
    images: Vec<String>,
    ingredients: Vec<RecipeRecipeIngredientFieldEnum>,
    instructions: Vec<RecipeRecipeInstructionsFieldEnum>,
    name: String,
    servings: Option<i16>,
    source: Option<String>,
    tags: Vec<String>,
    times: Times,
    tools: Vec<Tool>,
}

impl From<CooklangRecipe> for Recipe {
    fn from(r: CooklangRecipe) -> Self {
        Self {
            r#type: AtType::Recipe.to_opt(),
            author: r
                .author
                .map(|s| vec![RecipeAuthorFieldEnum::new_person(&s)])
                .unwrap_or_default(),
            cook_time: seconds_to_duration(r.times.cook_seconds),
            description: r
                .description
                .map(|s| vec![RecipeDescriptionFieldEnum::Text(s)])
                .unwrap_or_default(),
            is_based_on: r
                .source
                .clone()
                .map(|s| {
                    if s.parse::<Url>().is_ok() {
                        vec![RecipeIsBasedOnFieldEnum::URL(s)]
                    } else {
                        vec![RecipeIsBasedOnFieldEnum::new_creative_work_text(&s)]
                    }
                })
                .unwrap_or_default(),
            keywords: r
                .tags
                .into_iter()
                .map(RecipeKeywordsFieldEnum::TextOrURL)
                .collect(),
            name: Some(r.name)
                .filter(|s| !s.is_empty())
                .map(|s| vec![s])
                .unwrap_or_default(),
            prep_time: seconds_to_duration(r.times.prep_seconds),
            recipe_category: r.category.map(|s| vec![s]).unwrap_or_default(),
            recipe_cuisine: r
                .cuisine
                .filter(|s| !s.is_empty())
                .map(|s| vec![s])
                .unwrap_or_default(),
            image: r
                .images
                .into_iter()
                .map(RecipeImageFieldEnum::URL)
                .collect(),
            recipe_ingredient: r.ingredients,
            recipe_instructions: r.instructions,
            recipe_yield: to_yield(r.servings.map(i64::from).unwrap_or_default()),
            suitable_for_diet: r
                .diet
                .map(|diets| {
                    diets
                        .into_iter()
                        .map(RestrictedDietEnum::from)
                        .filter(|d| !matches!(d, RestrictedDietEnum::UnspecifiedDiet))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
            tool: r
                .tools
                .into_iter()
                .map(|t| {
                    if t.quantity == 0 {
                        RecipeToolFieldEnum::Text(t.name)
                    } else {
                        RecipeToolFieldEnum::HowToTool(Box::from(HowToTool {
                            r#type: AtType::HowToTool.to_opt(),
                            name: vec![t.name],
                            required_quantity: vec![HowToToolRequiredQuantityFieldEnum::Number(
                                t.quantity as f32,
                            )],
                            ..Default::default()
                        }))
                    }
                })
                .collect(),
            url: r.source.into_iter().collect(),
            ..Default::default()
        }
    }
}

impl CookLang {
    /// Parses a Cooklang recipe from the file's content.
    pub fn parse<R>(&self, r: R, file_name: &str) -> Result<Vec<Recipe>>
    where
        R: Read + Seek,
    {
        let content = read_file(r)?;

        let (recipe, report) = self.parser.parse(&content).into_result()?;
        report
            .warnings()
            .for_each(|w| warn!("Cooklang parsing warning: {w}"));
        report
            .errors()
            .for_each(|err| error!("Cooklang parsing error: {err}"));

        let metadata = recipe.metadata.clone().map;

        let category = metadata
            .get("category")
            .and_then(|c| c.as_str())
            .map(String::from);

        let course = metadata
            .get("course")
            .and_then(|c| c.as_str())
            .map(String::from);

        let image = metadata
            .get("image")
            .and_then(|c| c.as_str())
            .map(String::from);

        let images = metadata
            .get("images")
            .and_then(|c| c.as_str())
            .map(String::from);

        let picture = metadata
            .get("picture")
            .and_then(|c| c.as_str())
            .map(String::from);

        let pictures = metadata
            .get("pictures")
            .and_then(|c| c.as_str())
            .map(String::from);

        let time = metadata
            .get("time")
            .and_then(|c| c.as_str())
            .map(String::from);

        let time_required = metadata
            .get("time required")
            .and_then(|c| c.as_str())
            .map(String::from);

        let duration = metadata
            .get("duration")
            .and_then(|c| c.as_str())
            .map(String::from);

        let total_time = time
            .clone()
            .or(time.clone())
            .or(time_required.clone())
            .or(duration.clone())
            .filter(|_| time.is_some() || time_required.is_some() || duration.is_some())
            .map(|v| {
                let (_, duration) = parse_duration(&v).unwrap_or(("", (0, 15)));
                duration
            })
            .unwrap_or((0, 15));

        let prep_time = metadata
            .get("prep time")
            .and_then(|c| c.as_str())
            .map(String::from);

        let time_prep = metadata
            .get("time.prep")
            .and_then(|c| c.as_str())
            .map(String::from);

        let prep = prep_time
            .clone()
            .or(time_prep.clone())
            .filter(|_| prep_time.is_some() || time_prep.is_some())
            .map(|v| {
                let (_, duration) = parse_duration(&v).unwrap_or(("", total_time));
                duration
            })
            .unwrap_or(total_time);

        let cook_time = metadata
            .get("cook time")
            .and_then(|c| c.as_str())
            .map(String::from);

        let time_cook = metadata
            .get("time.cook")
            .and_then(|c| c.as_str())
            .map(String::from);

        let cook = cook_time
            .clone()
            .or(time_cook.clone())
            .filter(|_| cook_time.is_some() || time_cook.is_some())
            .map(|v| {
                let (_, duration) = parse_duration(&v).unwrap_or(("", (0, 30)));
                duration
            })
            .unwrap_or((0, 30));

        let cooklang_recipe = CooklangRecipe {
            author: recipe
                .metadata
                .author()
                .map(|s| s.name().unwrap_or_default().into()),
            name: recipe.metadata.title().unwrap_or(file_name).to_string(),
            category: category
                .clone()
                .or(course.clone())
                .filter(|_| category.is_some() || course.is_some()),
            cuisine: metadata
                .get("cuisine")
                .and_then(|c| c.as_str())
                .map(String::from),
            description: recipe.metadata.description().map(String::from),
            diet: metadata
                .get("diet")
                .and_then(|v| v.as_str())
                .map(|s| s.split(',').map(str::trim).map(String::from).collect()),
            difficulty: metadata
                .get("difficulty")
                .and_then(|c| c.as_str())
                .map(String::from),
            locale: recipe.metadata.locale().map(|(lang, _)| String::from(lang)),
            images: image
                .clone()
                .or(images.clone())
                .or(picture.clone())
                .or(pictures.clone())
                .filter(|_| {
                    image.is_some() || images.is_some() || picture.is_some() || pictures.is_some()
                })
                .map(|s| {
                    s.trim_end_matches(']')
                        .to_owned()
                        .trim_start_matches('[')
                        .split(',')
                        .map(str::trim)
                        .map(String::from)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
            ingredients: recipe
                .ingredients
                .into_iter()
                .map(|ing| {
                    let name = ing.display_name().to_string();
                    let s = match ing.quantity {
                        None => name,
                        Some(q) => {
                            let unit = q.unit().unwrap_or_default();
                            match q.value() {
                                Value::Number(v) => format!("{} {unit} {name}", v.value()),
                                Value::Range { start, end } => {
                                    format!("{}-{} {unit} {name}", start.value(), end.value())
                                }
                                Value::Text(s) => format!("{s} {unit} {name}"),
                            }
                        }
                    };
                    RecipeRecipeIngredientFieldEnum::Text(s)
                })
                .collect(),
            instructions: recipe
                .sections
                .into_iter()
                .flat_map(|section| {
                    let content = section.content.into_iter().filter_map(clean_content);

                    match section.name {
                        Some(name) => {
                            vec![RecipeRecipeInstructionsFieldEnum::new_section(
                                &name,
                                content.collect(),
                            )]
                        }
                        None => content
                            .map(RecipeRecipeInstructionsFieldEnum::Text)
                            .collect(),
                    }
                })
                .collect(),
            servings: recipe
                .metadata
                .servings()
                .map(|v| v.as_number().map(|v| v as i16))
                .unwrap_or_default(),
            source: recipe
                .metadata
                .source()
                .map(|v| v.url().or(v.name()).map(String::from))
                .unwrap_or_default(),
            tags: recipe
                .metadata
                .tags()
                .unwrap_or_default()
                .iter()
                .map(|s| {
                    s.trim_end_matches(']')
                        .to_owned()
                        .trim_start_matches('[')
                        .to_string()
                })
                .collect::<Vec<_>>(),
            times: Times {
                prep_seconds: (prep.0 * 60 * 60 + prep.1 * 60) as i32,
                cook_seconds: (cook.0 * 60 * 60 + cook.1 * 60) as i32,
            },
            tools: recipe
                .cookware
                .into_iter()
                .map(|cookware| Tool {
                    name: cookware.name,
                    quantity: match cookware.quantity {
                        None => 1,
                        Some(q) => match q.value() {
                            Value::Number(v) => v.value() as i16,
                            Value::Range { start, end: _ } => start.value() as i16,
                            Value::Text(s) => s.parse().unwrap_or(1),
                        },
                    },
                })
                .collect(),
        };

        Ok(vec![cooklang_recipe.into()])
    }
}

fn clean_content(content: Content) -> Option<String> {
    let s = match content {
        Content::Step(s) => {
            let parts: Vec<&str> = s
                .items
                .iter()
                .filter_map(|item| match item {
                    Item::Text { value } => {
                        let trimmed = value.trim();
                        (!trimmed.is_empty()).then_some(trimmed)
                    }
                    _ => None,
                })
                .collect();

            if parts.is_empty() {
                return None;
            }
            parts.join(" ")
        }
        Content::Text(s) => {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                return None;
            }
            trimmed.to_string()
        }
    };

    Some(if s.contains(" ,") {
        s.replace(" ,", ",")
    } else {
        s
    })
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use schema_org::Recipe;
    use schema_org::field::{RecipeRecipeIngredientFieldEnum, RecipeRecipeYieldFieldEnum};

    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_complete_recipe_ok() -> Result<()> {
        let file = r##">> title: Spaghetti Carbonara
>> description: This is the best recipe!
>> servings: 1
>> produce: 550%g
>> calories: 340%kkal
>> protein: 12.5%g
>> total fat: 22%g
>> total carb.: 23.5%g
>> source: https://example.org/recipe
>> author: John Doe
>> course: dinner
>> locale: es_VE
>> time required: 1 hour 30 minutes
>> time.prep: 2h 30
>> time.cook: 1 hour
>> difficulty: easy
>> cuisine: French
>> diet: gluten-free
>> tags: [2022, baking, summer]
>> images: [https://example.org/recipe_image.jpg, https://example.org/recipe_image2.jpg]

Peel and chop the @potatoes{100%g}, @onions{50%g} and @mushrooms{200%g} into chunks. The potatoes will need to be cut a bit smaller.

Heat a #frying pan{} over a medium heat with a little @oil and sauté the vegetables until golden. Season with @salt{4%g}, @pepper{1/4%tsp} and chopped fresh @rosemary{1/4%tsp}.

Put the sauted vegetables into a saucepan and pour water over them until just covered. Bring to the boil over a medium heat, then lower the heat and leave at a low simmer until the potatoes are tender.

Remove the soup from the heat and blend with a #blender, add the @double cream{50%g} and @salt to taste. Garnish with freshly cracked black pepper.
"##;
        let buf = Cursor::new(file.as_bytes());
        let parser = CookLang::default();

        let got = parser.parse(buf, "Spaghetti Carbonara")?;

        pretty_assertions::assert_eq!(
            got,
            vec![Recipe {
                r#type: AtType::Recipe.to_opt(),
                author: vec![RecipeAuthorFieldEnum::new_person("John Doe")],
                cook_time: seconds_to_duration(60 * 60),
                description: vec![RecipeDescriptionFieldEnum::Text(
                    "This is the best recipe!".into()
                )],
                image: vec![
                    RecipeImageFieldEnum::URL("https://example.org/recipe_image.jpg".into()),
                    RecipeImageFieldEnum::URL("https://example.org/recipe_image2.jpg".into()),
                ],
                is_based_on: vec![RecipeIsBasedOnFieldEnum::URL(
                    "https://example.org/recipe".into()
                )],
                keywords: vec![
                    RecipeKeywordsFieldEnum::TextOrURL("2022".into()),
                    RecipeKeywordsFieldEnum::TextOrURL("baking".into()),
                    RecipeKeywordsFieldEnum::TextOrURL("summer".into()),
                ],
                name: vec!["Spaghetti Carbonara".into()],
                prep_time: seconds_to_duration(2 * 60 * 60 + 30 * 60),
                recipe_category: vec!["dinner".into()],
                recipe_cuisine: vec!["French".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("100 g potatoes".into()),
                    RecipeRecipeIngredientFieldEnum::Text("50 g onions".into()),
                    RecipeRecipeIngredientFieldEnum::Text("200 g mushrooms".into()),
                    RecipeRecipeIngredientFieldEnum::Text("oil".into()),
                    RecipeRecipeIngredientFieldEnum::Text("4 g salt".into()),
                    RecipeRecipeIngredientFieldEnum::Text("0.25 tsp pepper".into()),
                    RecipeRecipeIngredientFieldEnum::Text("0.25 tsp rosemary".into()),
                    RecipeRecipeIngredientFieldEnum::Text("50 g double cream".into()),
                    RecipeRecipeIngredientFieldEnum::Text("salt".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text(
                        "Peel and chop the, and into chunks. The potatoes will need to be cut a bit smaller.".into()
                    ),
                    RecipeRecipeInstructionsFieldEnum::Text(
                        "Heat a over a medium heat with a little and sauté the vegetables until golden. Season with, and chopped fresh .".into()
                    ),
                    RecipeRecipeInstructionsFieldEnum::Text(
                        "Put the sauted vegetables into a saucepan and pour water over them until just covered. Bring to the boil over a medium heat, then lower the heat and leave at a low simmer until the potatoes are tender.".into()
                    ),
                    RecipeRecipeInstructionsFieldEnum::Text(
                        "Remove the soup from the heat and blend with a, add the and to taste. Garnish with freshly cracked black pepper.".into()
                    ),
                ],
                recipe_yield: vec![RecipeRecipeYieldFieldEnum::new_quantitative_value(1.0)],
                suitable_for_diet: vec![RestrictedDietEnum::GlutenFreeDiet],
                tool: vec![
                    RecipeToolFieldEnum::new_tool("frying pan", 1.0),
                    RecipeToolFieldEnum::new_tool("blender", 1.0),
                ],
                url: vec!["https://example.org/recipe".into()],
                ..Default::default()
            }]
        );
        Ok(())
    }
}
