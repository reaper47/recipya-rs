use std::io::Read;

use cooklang::{Content, CooklangParser, Item, ScalableValue};
use tracing::{error, warn};
use url::Url;

use crate::core::integrations::Result;
use crate::core::integrations::helpers::{
    seconds_to_duration, sections_to_itemlist, sections_to_vec, to_is_based_on,
    to_organization_type, to_text, to_yield,
};
use crate::core::model::recipe::{Sections, TimesForCreate, ToolForCreate};
use crate::core::scraper::schema::{
    AtType, DefinedTermOrTextOrUrl, HowToToolOrText, HowToToolType, ImageObjectOrUrl,
    RecipeCategory, RecipeCuisine, RecipeSchema, RestrictedDiet,
};
use crate::core::support::time::parse_duration;

/// A wrapper around the Cooklang parser that provides a consistent interface for parsing
/// and executing Cooklang code.
#[derive(Default)]
pub struct CookLang {
    parser: CooklangParser,
}

struct CooklangRecipe {
    author: Option<String>,
    category: Option<String>,
    cuisine: Option<String>,
    description: Option<String>,
    diet: Option<Vec<String>>,
    difficulty: Option<String>,
    locale: Option<String>,
    images: Vec<String>,
    ingredients: Sections,
    instructions: Sections,
    name: String,
    servings: Option<i16>,
    source: Option<String>,
    tags: Vec<String>,
    times: TimesForCreate,
    tools: Vec<ToolForCreate>,
}

impl From<CooklangRecipe> for RecipeSchema {
    fn from(r: CooklangRecipe) -> Self {
        Self {
            at_context: Default::default(),
            at_type: Some(AtType::Recipe),
            author: to_organization_type(r.author.unwrap_or_default()),
            cook_time: seconds_to_duration(r.times.cook_seconds),
            description: to_text(r.description.unwrap_or_default()),
            is_based_on: to_is_based_on(r.source.to_owned().unwrap()),
            keywords: Some(DefinedTermOrTextOrUrl::Text(r.tags.join(","))),
            name: Some(r.name).filter(|s| !s.is_empty()),
            prep_time: seconds_to_duration(r.times.prep_seconds),
            recipe_category: RecipeCategory::Text(r.category.unwrap_or_default()),
            recipe_cuisine: Some(RecipeCuisine::Text(r.cuisine.unwrap_or_default())).filter(|s| {
                match s {
                    RecipeCuisine::Text(s) => !s.is_empty(),
                }
            }),
            image: Some(
                r.images
                    .into_iter()
                    .filter_map(|image| Url::parse(&image).ok())
                    .map(ImageObjectOrUrl::Url)
                    .collect::<Vec<_>>(),
            ),
            recipe_ingredient: sections_to_vec(r.ingredients),
            recipe_instructions: sections_to_itemlist(r.instructions),
            recipe_yield: to_yield(r.servings.map(i64::from).unwrap_or_default()),
            suitable_for_diet: r
                .diet
                .map(|diets| {
                    diets
                        .into_iter()
                        .map(RestrictedDiet::from)
                        .filter(|diet| diet != &RestrictedDiet::UnspecifiedDiet)
                        .collect()
                })
                .unwrap_or_default(),
            tool: Some(
                r.tools
                    .into_iter()
                    .map(|t| {
                        HowToToolOrText::HowToTool(HowToToolType {
                            r#type: AtType::HowToTool,
                            name: t.name,
                            ..Default::default()
                        })
                    })
                    .collect::<Vec<_>>(),
            )
            .filter(|v| !v.is_empty()),
            url: Url::parse(&r.source.unwrap_or_default()).ok(),
            ..Default::default()
        }
    }
}

impl CookLang {
    /// Parses a Cooklang recipe from the file's content.
    pub fn parse<R>(&self, mut r: R, file_name: &str) -> Result<Vec<RecipeSchema>>
    where
        R: Read,
    {
        let mut content = String::new();
        r.read_to_string(&mut content)?;

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
            ingredients: Sections::from([(
                "".into(),
                recipe
                    .ingredients
                    .into_iter()
                    .map(|ing| {
                        let name = ing.display_name().to_string();
                        match ing.quantity {
                            None => name,
                            Some(q) => {
                                let unit = q.unit().unwrap_or_default();
                                match q.value() {
                                    ScalableValue::Fixed(v) => format!("{v} {unit} {name}"),
                                    ScalableValue::Linear(v) => format!("{v} {unit} {name}"),
                                }
                            }
                        }
                    })
                    .collect(),
            )]),
            instructions: recipe
                .sections
                .into_iter()
                .map(|section| {
                    (
                        section.name.unwrap_or_default(),
                        section
                            .content
                            .into_iter()
                            .map(|content| match content {
                                Content::Step(s) => s
                                    .items
                                    .into_iter()
                                    .map(|item| match item {
                                        Item::Text { value } => value,
                                        _ => "".to_string(),
                                    })
                                    .collect(),
                                Content::Text(s) => s,
                            })
                            .collect(),
                    )
                })
                .collect(),
            servings: recipe
                .metadata
                .servings()
                .map(|v| v.first().map(|&v| v as i16))
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
            times: TimesForCreate {
                prep_seconds: (prep.0 * 60 * 60 + prep.1 * 60) as i32,
                cook_seconds: (cook.0 * 60 * 60 + cook.1 * 60) as i32,
            },
            tools: recipe
                .cookware
                .into_iter()
                .map(|cookware| ToolForCreate {
                    name: cookware.name,
                    quantity: match cookware.quantity {
                        None => 1,
                        Some(ScalableValue::Fixed(v)) => v.to_string().parse().unwrap_or(1),
                        Some(ScalableValue::Linear(v)) => v.to_string().parse().unwrap_or(1),
                    },
                })
                .collect(),
        };

        Ok(vec![cooklang_recipe.into()])
    }
}

/// A builder pattern for constructing a `CookLang` parser.
pub struct ParserBuilder {
    parser: Option<CooklangParser>,
}

impl ParserBuilder {
    /// Creates a new `ParserBuilder` with no parser initialized.
    pub fn new() -> Self {
        Self { parser: None }
    }

    /// Adds a parser to this builder, making it available for use during the build process.
    pub fn with_parser(mut self, parser: CooklangParser) -> Self {
        self.parser = Some(parser);
        self
    }

    /// Builds a new `Parser` using the configured options or defaults.
    pub fn build(self) -> CookLang {
        CookLang {
            parser: self.parser.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::scraper::schema::{
        OrganizationType, QuantitativeValueOrText, QuantitativeValueType, RecipeCuisine,
        TextOrTextObject,
    };
    use std::io::Cursor;

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
            vec![RecipeSchema {
                at_context: Default::default(),
                at_type: Some(AtType::Recipe),
                author: Some(OrganizationType {
                    name: Some("John Doe".into()),
                    ..Default::default()
                }),
                cook_time: seconds_to_duration(60*60),
                description: Some(TextOrTextObject::Text("This is the best recipe!".into())),
                image: Some(vec![
                    ImageObjectOrUrl::Url(Url::parse("https://example.org/recipe_image.jpg")?),
                    ImageObjectOrUrl::Url(Url::parse("https://example.org/recipe_image2.jpg")?),
                ]),
                is_based_on: to_is_based_on("https://example.org/recipe".into()),
                keywords: Some(DefinedTermOrTextOrUrl::Text(["2022", "baking", "summer"].join(","))),
                name: Some("Spaghetti Carbonara".into()),
                prep_time: seconds_to_duration(2*60*60+30*60),
                recipe_category: RecipeCategory::Text("dinner".into()),
                recipe_cuisine: Some(RecipeCuisine::Text("French".into())),
                recipe_ingredient: Some(vec![
                    "<section></section>".into(),
                    "100 g potatoes".into(),
                    "50 g onions".into(),
                    "200 g mushrooms".into(),
                    "oil".into(),
                    "4 g salt".into(),
                    "1/4 tsp pepper".into(),
                    "1/4 tsp rosemary".into(),
                    "50 g double cream".into(),
                    "salt".into(),
                ]),
                recipe_instructions: sections_to_itemlist(Sections::from([(
                    "".into(), vec![
                        "Peel and chop the ,  and  into chunks. The potatoes will need to be cut a bit smaller.".into(),
                        "Heat a  over a medium heat with a little  and sauté the vegetables until golden. Season with ,  and chopped fresh .".into(),
                        "Put the sauted vegetables into a saucepan and pour water over them until just covered. Bring to the boil over a medium heat, then lower the heat and leave at a low simmer until the potatoes are tender.".into(),
                        "Remove the soup from the heat and blend with a , add the  and  to taste. Garnish with freshly cracked black pepper.".into(),
                    ]),
                ])),
                recipe_yield: QuantitativeValueOrText::QuantitativeValue(QuantitativeValueType {
                    value: 1
                }),
                suitable_for_diet: vec![RestrictedDiet::GlutenFreeDiet],
                tool: Some(vec![
                    HowToToolOrText::HowToTool(HowToToolType {
                        r#type: AtType::HowToTool,
                        name: "frying pan".into(),
                        ..Default::default()
                    }),
                    HowToToolOrText::HowToTool(HowToToolType {
                        r#type: AtType::HowToTool,
                        name: "blender".into(),
                        ..Default::default()
                    }),
                ]),
                url: Url::parse("https://example.org/recipe").ok(),
                ..Default::default()
            }
        ]);
        Ok(())
    }
}
