use std::io::Read;

use cooklang::{Content, CooklangParser, Item, ScalableValue};
use tracing::{error, warn};

use crate::core::integrations::error::Result;
use crate::core::model::recipe::{Sections, TimesForCreate, ToolForCreate};
use crate::core::support::time::parse_duration;

/// A wrapper around the Cooklang parser that provides a consistent interface for parsing
/// and executing Cooklang code.
#[derive(Default)]
pub struct CookLang {
    parser: CooklangParser,
}

impl CookLang {
    /// Parses a Cooklang recipe from the file's content.
    pub fn parse<R>(&self, mut r: R, file_name: &str) -> Result<CooklangRecipe>
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

        Ok(CooklangRecipe {
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
        })
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

#[derive(Debug, PartialEq)]
pub struct CooklangRecipe {
    author: Option<String>,
    name: String,
    category: Option<String>,
    cuisine: Option<String>,
    description: Option<String>,
    diet: Option<Vec<String>>,
    difficulty: Option<String>,
    locale: Option<String>,
    images: Vec<String>,
    ingredients: Sections,
    instructions: Sections,
    servings: Option<i16>,
    source: Option<String>,
    tags: Vec<String>,
    times: TimesForCreate,
    tools: Vec<ToolForCreate>,
}

#[cfg(test)]
mod tests {
    use super::*;

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
            CooklangRecipe {
                author: Some("John Doe".into()),
                name: "Spaghetti Carbonara".into(),
                category: Some("dinner".into()),
                cuisine: Some("French".into()),
                description: Some("This is the best recipe!".into()),
                diet: Some(vec!["gluten-free".into()]),
                difficulty: Some("easy".into()),
                locale: Some("es".into()),
                images: vec!["https://example.org/recipe_image.jpg".into(), "https://example.org/recipe_image2.jpg".into()],
                ingredients: Sections::from([(
                    "".into(),
                    vec![
                        "100 g potatoes".into(),
                        "50 g onions".into(),
                        "200 g mushrooms".into(),
                        "oil".into(),
                        "4 g salt".into(),
                        "1/4 tsp pepper".into(),
                        "1/4 tsp rosemary".into(),
                        "50 g double cream".into(),
                        "salt".into(),
                    ]
                ),]),
                instructions: Sections::from([(
                    "".into(),
                    vec![
                        "Peel and chop the ,  and  into chunks. The potatoes will need to be cut a bit smaller.".into(),
                        "Heat a  over a medium heat with a little  and sauté the vegetables until golden. Season with ,  and chopped fresh .".into(),
                        "Put the sauted vegetables into a saucepan and pour water over them until just covered. Bring to the boil over a medium heat, then lower the heat and leave at a low simmer until the potatoes are tender.".into(),
                        "Remove the soup from the heat and blend with a , add the  and  to taste. Garnish with freshly cracked black pepper.".into(),
                    ],
                ),]),
                servings: Some(1),
                source: Some("https://example.org/recipe".into()),
                tags: vec!["2022".into(), "baking".into(), "summer".into()],
                times: TimesForCreate {
                    prep_seconds: 2*60*60+30*60,
                    cook_seconds: 60*60,
                },
                tools: vec![
                    ToolForCreate {
                        name: "frying pan".into(),
                        quantity: 1,
                    },
                    ToolForCreate {
                        name: "blender".into(),
                        quantity: 1,
                    },
                ],
            }
        );
        Ok(())
    }
}
