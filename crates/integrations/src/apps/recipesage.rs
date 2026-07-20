use std::borrow::Cow;
use std::io::{Cursor, Read, Seek};
use std::str::FromStr;

use iso8601::DateTime;
use itertools::Itertools;
use serde::Deserialize;
use tracing::error;
use url::Url;
use winnow::ModalResult;
use winnow::Parser;
use winnow::ascii::{line_ending, multispace0, multispace1, till_line_ending};
use winnow::combinator::{
    alt, delimited, opt, peek, preceded, repeat, repeat_till, separated, seq, terminated,
};
use winnow::error::{ContextError, ErrMode};

use schema_org::field::{
    CreativeWorkIsBasedOnFieldEnum, QuantitativeValueValueFieldEnum, RecipeAuthorFieldEnum,
    RecipeDescriptionFieldEnum, RecipeImageFieldEnum, RecipeIsBasedOnFieldEnum,
    RecipeKeywordsFieldEnum, RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
    RecipeRecipeYieldFieldEnum,
};
use schema_org::{
    AggregateRating, AtType, Comment, DurationOrText, Energy, Mass, NutritionInformation, Recipe,
    at_context,
};
use support::numbers::float_to_i16_safe;
use support::strings::extract_number;
use winnow::token::{literal, take_till, take_until};

use crate::apps::helpers::read_file;
use crate::error::{Error, Result};
use crate::helpers::{seconds_to_duration, to_is_based_on, to_yield};

#[derive(Default, Debug, Deserialize)]
struct JsonRoot {
    recipes: Vec<Recipe>,
}

#[derive(Default)]
struct RecipeSage {
    category: Option<String>,
    description: Option<String>,
    image: Option<String>,
    ingredients: Vec<RecipeRecipeIngredientFieldEnum>,
    instructions: Vec<RecipeRecipeInstructionsFieldEnum>,
    keywords: Vec<String>,
    notes: Option<String>,
    nutrition: Option<NutritionInformation>,
    rating: Option<f32>,
    source: Option<String>,
    prep_time: Option<String>,
    total_time: Option<String>,
    title: String,
    r#yield: i16,
    url: Option<String>,
}

#[derive(Default)]
struct RecipeComponents<'a> {
    description: Option<&'a str>,
    image: Option<&'a str>,
    ingredients: Vec<&'a str>,
    instructions: &'a str,
    notes: Option<&'a str>,
    notes_other: Option<&'a str>,
    nutrition: Option<NutritionComponents<'a>>,
    prep_time: Option<&'a str>,
    total_time: Option<&'a str>,
    r#yield: Option<i16>,
    rating: Option<f32>,
    source: Option<&'a str>,
    title: &'a str,
    url: Option<&'a str>,
    labels: Vec<&'a str>,
}

struct NutritionComponents<'a> {
    serving_size: Option<&'a str>,
    calories: Option<f32>,
    fat: Option<f32>,
    sat_fat: Option<f32>,
    trans_fat: Option<f32>,
    poly_fat: Option<f32>,
    mono_fat: Option<f32>,
    cholesterol: Option<f32>,
    sodium: Option<f32>,
    carbs: Option<f32>,
    fibre: Option<f32>,
    sugars: Option<f32>,
    protein: Option<f32>,
}

impl From<NutritionComponents<'_>> for NutritionInformation {
    fn from(n: NutritionComponents) -> Self {
        let extract_mass = |nut: Option<f32>, unit: &str| {
            nut.filter(|n| (n - 0.0).abs() > 1e-5)
                .map_or(Vec::new(), |s| vec![Mass::new(format!("{s} {unit}"))])
        };

        Self {
            calories: n
                .calories
                .filter(|n| (n - 0.0).abs() > 1e-5)
                .map_or(Vec::new(), |s| vec![Energy::new(format!("{s} kcal"))]),
            carbohydrate_content: extract_mass(n.carbs, "g"),
            cholesterol_content: extract_mass(n.cholesterol, "mg"),
            context: at_context(),
            fat_content: extract_mass(n.fat, "g"),
            fiber_content: extract_mass(n.fibre, "g"),
            protein_content: extract_mass(n.protein, "g"),
            saturated_fat_content: extract_mass(n.sat_fat, "g"),
            serving_size: n.serving_size.map_or(Vec::new(), |s| {
                if s == "null" { vec![] } else { vec![s.into()] }
            }),
            sodium_content: extract_mass(n.sodium, "mg"),
            sugar_content: extract_mass(n.sugars, "g"),
            r#type: AtType::NutritionInformation.to_opt(),
            trans_fat_content: extract_mass(n.trans_fat, "g"),
            unsaturated_fat_content: extract_mass(
                n.poly_fat.map(|g| g + n.mono_fat.unwrap_or_default()),
                "g",
            ),
        }
    }
}

#[derive(Deserialize)]
struct RecipeSageXMLData<'a> {
    #[serde(rename = "recipe")]
    pub recipes: Vec<RecipeSageXMLRecipe<'a>>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecipeSageXMLRecipe<'a> {
    id: Cow<'a, str>,
    title: Cow<'a, str>,
    description: Cow<'a, str>,
    #[serde(rename = "yield")]
    r#yield: Cow<'a, str>,
    active_time: Cow<'a, str>,
    total_time: Cow<'a, str>,
    source: Cow<'a, str>,
    url: Cow<'a, str>,
    notes: Cow<'a, str>,
    ingredients: Cow<'a, str>,
    instructions: Cow<'a, str>,
    folder: Cow<'a, str>,
    created_at: Cow<'a, str>,
    updated_at: Cow<'a, str>,
    user_id: Cow<'a, str>,
    from_user: Cow<'a, str>,
    labels: Option<Vec<Label<'a>>>,
    images: Option<Images<'a>>,
}

#[derive(Debug, Deserialize)]
struct Label<'a> {
    title: Cow<'a, str>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Images<'a> {
    id: Cow<'a, str>,
    location: Cow<'a, str>,
}

impl From<RecipeComponents<'_>> for RecipeSage {
    fn from(r: RecipeComponents<'_>) -> Self {
        let items = r.labels.split_first();

        let url = r.url.map(str::trim).filter(|s| s != &":").map(Into::into);
        let source = r.source.filter(|s| s != &":\n").map(Into::into);

        let mut instructions = r
            .instructions
            .split_terminator("\n\n")
            .map(|s| RecipeRecipeInstructionsFieldEnum::Text(s.trim().replace('\n', " ")))
            .collect_vec();

        if instructions.len() == 1 {
            instructions = r
                .instructions
                .lines()
                .map(|s| RecipeRecipeInstructionsFieldEnum::Text(s.trim().replace('\n', " ")))
                .collect_vec();
        }

        Self {
            category: items.map(|(&a, _b)| a.into()),
            description: r.description.filter(|s| s != &":\n").map(Into::into),
            ingredients: r
                .ingredients
                .into_iter()
                .map(|s| RecipeRecipeIngredientFieldEnum::Text(s.into()))
                .collect(),
            instructions,
            keywords: items
                .map(|(_a, b)| b.to_vec())
                .unwrap_or_default()
                .into_iter()
                .map(str::trim)
                .map(String::from)
                .collect(),
            nutrition: r
                .nutrition
                .map(Into::into)
                .filter(|n: &NutritionInformation| !n.is_empty()),
            rating: r.rating.filter(|n| (n - 0.0).abs() > 1e-5),
            source: if source.is_none() && url.is_some() {
                url
            } else {
                source
            },
            prep_time: r.prep_time.map(Into::into),
            total_time: r.total_time.map(Into::into),
            notes: {
                let mut notes: Option<String> = r.notes.filter(|s| s != &":\n").map(Into::into);
                if notes.is_none() {
                    notes = r
                        .notes_other
                        .filter(|s| s != &":\n")
                        .map(|s| s.trim().into());
                }
                notes
            },
            title: r.title.into(),
            url: r.url.map(str::trim).filter(|s| s != &":").map(Into::into),
            r#yield: r.r#yield.unwrap_or_default(),
            image: r.image.filter(|s| s != &":\n").map(Into::into),
        }
    }
}

impl From<RecipeSageXMLRecipe<'_>> for RecipeSage {
    fn from(r: RecipeSageXMLRecipe) -> Self {
        let labels = r.labels.unwrap_or_default();
        let items = labels.split_first();

        Self {
            category: items.map(|(a, _b)| a.title.to_string()),
            description: Some(r.description.to_string()).filter(|s| !s.is_empty()),
            ingredients: r
                .ingredients
                .lines()
                .map(|s| RecipeRecipeIngredientFieldEnum::Text(s.into()))
                .collect(),
            instructions: r
                .instructions
                .split_terminator("\n\n")
                .map(|s| RecipeRecipeInstructionsFieldEnum::Text(s.trim().replace('\n', " ")))
                .collect(),
            keywords: items
                .map(|(_a, b)| b.iter().map(|s| s.title.to_string()).collect())
                .unwrap_or_default(),
            source: if r.source.is_empty() {
                Some(r.url.to_string()).filter(|s| !s.is_empty())
            } else {
                Some(r.source.to_string())
            },
            notes: Some(r.notes.to_string()).filter(|s| !s.is_empty()),
            nutrition: None,
            title: r.title.to_string(),
            r#yield: extract_number(&r.r#yield).unwrap_or_default(),
            url: if r.url.is_empty() {
                None
            } else {
                Some(r.url.to_string())
            },
            ..Default::default()
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
impl From<Recipe> for RecipeSage {
    fn from(r: Recipe) -> Self {
        Self {
            rating: None,
            category: r.recipe_category.first().cloned(),
            description: r
                .description
                .into_iter()
                .map(|s| match s {
                    RecipeDescriptionFieldEnum::Text(s) => s,
                    RecipeDescriptionFieldEnum::TextObject(obj) => {
                        obj.text.first().cloned().unwrap_or_default()
                    }
                })
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .first()
                .cloned(),
            ingredients: r.recipe_ingredient,
            instructions: r.recipe_instructions,
            source: r
                .is_based_on
                .first()
                .cloned()
                .map(|s| match s {
                    RecipeIsBasedOnFieldEnum::CreativeWork(obj) => {
                        obj.is_based_on.first().cloned().map(|s| match s {
                            CreativeWorkIsBasedOnFieldEnum::CreativeWork(_)
                            | CreativeWorkIsBasedOnFieldEnum::Product(_) => String::new(),
                            CreativeWorkIsBasedOnFieldEnum::URL(s) => s,
                        })
                    }
                    RecipeIsBasedOnFieldEnum::Product(_) => Some(String::new()),
                    RecipeIsBasedOnFieldEnum::URL(s) => s.into(),
                })
                .unwrap_or_default(),
            notes: r
                .comment
                .first()
                .cloned()
                .map(|v| v.text)
                .filter(|s| !s.is_empty())
                .unwrap_or_default()
                .first()
                .cloned(),
            nutrition: None,
            title: r.name.first().cloned().unwrap_or_default(),
            r#yield: r
                .recipe_yield
                .first()
                .map(|v| match v {
                    RecipeRecipeYieldFieldEnum::QuantitativeValue(v) => v
                        .value
                        .first()
                        .cloned()
                        .map(|v| match v {
                            QuantitativeValueValueFieldEnum::BooleanEnumOrText(s) => {
                                s.parse::<i16>().ok().unwrap_or_default()
                            }
                            QuantitativeValueValueFieldEnum::Number(i) => {
                                i.round().clamp(f64::from(i16::MIN), f64::from(i16::MAX)) as i16
                            }
                            QuantitativeValueValueFieldEnum::StructuredValue(_) => 0,
                            QuantitativeValueValueFieldEnum::QuantitativeValue(q) => q.to_number(),
                        })
                        .unwrap_or_default(),
                    RecipeRecipeYieldFieldEnum::Number(n) => float_to_i16_safe(*n),
                    RecipeRecipeYieldFieldEnum::Text(s) => {
                        s.parse::<i16>().ok().unwrap_or_default()
                    }
                })
                .unwrap_or_default(),
            keywords: r
                .keywords
                .into_iter()
                .map(|k| match k {
                    RecipeKeywordsFieldEnum::DefinedTerm(t) => {
                        t.name.first().cloned().unwrap_or_default()
                    }
                    RecipeKeywordsFieldEnum::TextOrURL(s) => s,
                })
                .collect(),
            url: r.url.first().map(Into::into),
            prep_time: r.prep_time.first().cloned().map(|s| match s {
                schema_org::DurationOrText::Text(s) => s,
                schema_org::DurationOrText::Duration(_) => String::new(),
            }),
            total_time: r.total_time.first().cloned().map(|s| match s {
                schema_org::DurationOrText::Text(s) => s,
                schema_org::DurationOrText::Duration(_) => String::new(),
            }),
            image: r.image.first().cloned().map(|s| match s {
                schema_org::field::FieldEnum22::URL(u) => u,
                schema_org::field::FieldEnum22::ImageObject(_) => String::new(),
            }),
        }
    }
}

impl From<RecipeSage> for Recipe {
    fn from(r: RecipeSage) -> Self {
        Self {
            aggregate_rating: r
                .rating
                .map_or(Vec::new(), |rating| vec![AggregateRating::new(rating, 1)]),
            comment: r
                .notes
                .filter(|s| !s.is_empty())
                .map(|s| Comment {
                    text: vec![s],
                    ..Default::default()
                })
                .map(|c| vec![c])
                .unwrap_or_default(),
            image: r
                .image
                .map_or(Vec::new(), |s| vec![RecipeImageFieldEnum::URL(s)]),
            keywords: r
                .keywords
                .into_iter()
                .map(RecipeKeywordsFieldEnum::TextOrURL)
                .collect(),
            name: vec![r.title],
            nutrition: r
                .nutrition
                .filter(|n| !n.is_empty())
                .map_or(Vec::new(), |n| vec![n]),
            prep_time: r
                .prep_time
                .filter(|s| s != ":\n")
                .map_or(Vec::new(), |s| vec![DurationOrText::Text(s)]),
            total_time: r
                .total_time
                .filter(|s| s != ":\n")
                .map_or(Vec::new(), |s| vec![DurationOrText::Text(s)]),
            recipe_category: r.category.map(|s| vec![s]).unwrap_or_default(),
            recipe_ingredient: r.ingredients,
            recipe_instructions: r.instructions,
            recipe_yield: to_yield(i64::from(r.r#yield)),
            description: r
                .description
                .map(|s| vec![RecipeDescriptionFieldEnum::Text(s)])
                .unwrap_or_default(),
            is_based_on: to_is_based_on(&r.source.unwrap_or_default()),
            url: r.url.map_or(Vec::new(), |s| vec![s]),
            ..Default::default()
        }
    }
}

impl From<RecipeSageXMLRecipe<'_>> for Recipe {
    fn from(r: RecipeSageXMLRecipe) -> Self {
        let categories = r.labels.unwrap_or_default();
        let categories = categories.split_first();

        let active_time_secs = match humantime::parse_duration(&r.active_time) {
            Ok(d) => i32::try_from(d.as_secs()).unwrap_or_default(),
            Err(err) => {
                error!("Failed to parse prep time of a RecipeSage recipe: {err}");
                15 * 60
            }
        };

        let total_time_secs = match humantime::parse_duration(&r.total_time) {
            Ok(d) => i32::try_from(d.as_secs()).unwrap_or_default(),
            Err(err) => {
                error!("Failed to total time of a RecipeSage recipe: {err}");
                30 * 60
            }
        };

        let url = Url::parse(&r.url).ok();

        Self {
            author: {
                if r.from_user.is_empty() {
                    vec![]
                } else {
                    vec![RecipeAuthorFieldEnum::new_person(&r.from_user)]
                }
            },
            comment: if r.notes.is_empty() {
                vec![]
            } else {
                vec![Comment {
                    text: vec![r.notes.to_string()],
                    ..Default::default()
                }]
            },
            cook_time: seconds_to_duration(total_time_secs - active_time_secs),
            date_created: DateTime::from_str(&r.created_at)
                .ok()
                .map(|d| vec![d.to_string()])
                .unwrap_or_default(),
            date_modified: DateTime::from_str(&r.updated_at)
                .ok()
                .map(|d| vec![d.to_string()])
                .unwrap_or_default(),
            description: if r.description.is_empty() {
                vec![]
            } else {
                vec![RecipeDescriptionFieldEnum::Text(r.description.to_string())]
            },
            is_based_on: if url.is_none() && !r.url.is_empty() {
                to_is_based_on(&r.url)
            } else {
                to_is_based_on(&r.source)
            },
            keywords: categories
                .map(|(_, b)| b.iter().map(|s| s.title.clone()).collect::<Vec<_>>())
                .map(|v| {
                    v.into_iter()
                        .map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            name: vec![r.title.to_string()],
            prep_time: seconds_to_duration(active_time_secs),
            recipe_category: categories
                .map(|(a, _b)| vec![a.title.to_string()])
                .unwrap_or_default(),
            recipe_ingredient: r
                .ingredients
                .lines()
                .map(|s| RecipeRecipeIngredientFieldEnum::Text(s.into()))
                .collect(),
            recipe_instructions: r
                .instructions
                .split_terminator("\n\n")
                .map(|s| RecipeRecipeInstructionsFieldEnum::Text(s.trim().replace('\n', " ")))
                .collect(),
            recipe_yield: to_yield(extract_number(&r.r#yield).unwrap_or_default()),
            url: url.map(|u| vec![u.to_string()]).unwrap_or_default(),
            ..Default::default()
        }
    }
}

/// Parses a `RecipeSage` recipes text file.
pub fn parse_txt<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;
    Ok(parse_text_file(content.as_str())?
        .into_iter()
        .map(Recipe::from)
        .collect())
}

/// Parses a `RecipeSage` recipes XML file.
pub fn parse_xml<R>(mut r: R) -> Result<Vec<Recipe>>
where
    R: Read,
{
    let mut buf = Vec::new();
    r.read_to_end(&mut buf)?;

    let root: RecipeSageXMLData = quick_xml::de::from_reader(Cursor::new(buf)).map_err(|err| {
        error!("Failed to read RecipeSage XML file: {err}");
        Error::Parse(err.to_string())
    })?;

    Ok(root.recipes.into_iter().map(Recipe::from).collect())
}

/// Parses a `RecipeSage` recipes JSON file.
pub fn parse_json<R>(mut r: R) -> Result<Vec<Recipe>>
where
    R: Read,
{
    let mut buf = String::new();
    r.read_to_string(&mut buf)
        .map_err(|err| Error::Parse(err.to_string()))?;

    match serde_json::from_str::<Vec<Recipe>>(&buf) {
        Ok(res) => Ok(res),
        Err(err) => match serde_json::from_str::<JsonRoot>(&buf) {
            Ok(root) => Ok(root.recipes.into()),
            Err(_) => {
                error!("Failed to read RecipeSage JSON file: {err}");
                Err(Error::Parse(err.to_string()))
            }
        },
    }
}

fn parse_text_file(input: &str) -> Result<Vec<RecipeSage>> {
    repeat(1.., parse_recipe.map(RecipeSage::from))
        .parse(input)
        .map_err(|err| Error::Parse(err.to_string()))
}

fn parse_recipe<'s>(input: &mut &'s str) -> ModalResult<RecipeComponents<'s>> {
    seq! {RecipeComponents {
        _: (opt(parse_header), opt(parse_id)),
        title: parse_title,
        description: opt(parse_description),
        r#yield: opt(parse_servings),
        prep_time: opt(parse_active_time),
        total_time: opt(parse_total_time),
        source: opt(parse_source),
        url: opt(parse_url),
        notes: opt(parse_notes),
        _: opt(parse_folder),
        ingredients: parse_ingredients,
        instructions: parse_instructions,
        notes_other: opt(parse_other_notes),
        _: (opt(parse_folder), opt(parse_created_at), opt(parse_updated_at), opt(parse_user_id)),
        rating: opt(parse_rating),
        nutrition: opt(parse_nutrition),
        labels: parse_labels,
        image: opt(parse_images),
        _: opt(line_ending)
    }}
    .parse_next(input)
}

fn parse_header<'s>(input: &mut &'s str) -> ModalResult<Option<&'s str>> {
    terminated(literal("==== Recipes ===="), multispace1)
        .map(|s: &str| if s.is_empty() { None } else { Some(s) })
        .parse_next(input)
}

fn parse_id<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    parse_required_tag("id").parse_next(input)
}

fn parse_title<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    parse_required_tag("title").parse_next(input)
}

fn parse_description<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    parse_required_tag("description").parse_next(input)
}

fn parse_servings(input: &mut &str) -> ModalResult<i16> {
    parse_required_tag("yield")
        .map(|s| extract_number(s).unwrap_or_default())
        .parse_next(input)
}

fn parse_active_time<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    parse_required_tag("activeTime").parse_next(input)
}

fn parse_total_time<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    parse_required_tag("totalTime").parse_next(input)
}

fn parse_source<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    parse_required_tag("source").parse_next(input)
}

fn parse_url<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    parse_required_tag("url").parse_next(input)
}

fn parse_notes<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    parse_required_tag("notes").parse_next(input)
}

fn parse_ingredients<'s>(input: &mut &'s str) -> ModalResult<Vec<&'s str>> {
    preceded(
        alt((literal("ingredients: "), literal("Ingredients: "))),
        repeat_till(
            0..,
            terminated(take_until(0.., "\n"), line_ending),
            peek(alt((literal("instructions:"), literal("Instructions:")))),
        )
        .map(|(lines, _)| lines),
    )
    .parse_next(input)
}

fn parse_instructions<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    preceded(
        alt((literal("instructions: "), literal("Instructions: "))),
        take_until(0.., ("folder:", "Folder:", "Notes: ")),
    )
    .parse_next(input)
}

fn parse_other_notes<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    preceded(literal("Notes: "), take_until(0.., "CreatedAt: ")).parse_next(input)
}

fn parse_folder<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    parse_required_tag("folder").parse_next(input)
}

fn parse_created_at<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    parse_required_tag("createdAt").parse_next(input)
}

fn parse_updated_at<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    parse_required_tag("updatedAt").parse_next(input)
}

fn parse_user_id<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    parse_required_tag("userId").parse_next(input)
}

fn parse_rating(input: &mut &str) -> ModalResult<f32> {
    let text = delimited(literal("Rating: "), till_line_ending, line_ending).parse_next(input)?;
    Ok(text.trim().parse::<f32>().unwrap_or_default())
}

fn parse_nutrition<'s>(input: &mut &'s str) -> ModalResult<NutritionComponents<'s>> {
    seq! {NutritionComponents {
        serving_size: opt(parse_serving_size),
        calories: opt(parse_calories),
        fat: opt(parse_fat),
        sat_fat: opt(parse_sat_fat),
        trans_fat: opt(parse_trans_fat),
        poly_fat: opt(parse_poly_fat),
        mono_fat: opt(parse_mono_fat),
        cholesterol: opt(parse_cholesterol),
        sodium: opt(parse_sodium),
        carbs: opt(parse_carbs),
        fibre: opt(parse_fibre),
        sugars: opt(parse_sugars),
        _: till_line_ending,
        protein: opt(parse_protein),
        _: skip_to_labels,
    }}
    .parse_next(input)
}

fn parse_serving_size<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    delimited(
        literal("NutritionServingSize: "),
        till_line_ending,
        line_ending,
    )
    .parse_next(input)
}

fn parse_calories(input: &mut &str) -> ModalResult<f32> {
    parse_extract_energy(input, "NutritionCalories: ")
}

fn parse_fat(input: &mut &str) -> ModalResult<f32> {
    parse_extract_mass(input, "NutritionTotalFat: ")
}

fn parse_sat_fat(input: &mut &str) -> ModalResult<f32> {
    parse_extract_mass(input, "NutritionSaturatedFat: ")
}

fn parse_trans_fat(input: &mut &str) -> ModalResult<f32> {
    parse_extract_mass(input, "NutritionTransFat: ")
}

fn parse_poly_fat(input: &mut &str) -> ModalResult<f32> {
    parse_extract_mass(input, "NutritionPolyunsaturatedFat: ")
}

fn parse_mono_fat(input: &mut &str) -> ModalResult<f32> {
    parse_extract_mass(input, "NutritionMonounsaturatedFat: ")
}

fn parse_cholesterol(input: &mut &str) -> ModalResult<f32> {
    parse_extract_mass(input, "NutritionCholesterol: ")
}

fn parse_sodium(input: &mut &str) -> ModalResult<f32> {
    parse_extract_mass(input, "NutritionSodium: ")
}

fn parse_carbs(input: &mut &str) -> ModalResult<f32> {
    parse_extract_mass(input, "NutritionTotalCarbs: ")
}

fn parse_fibre(input: &mut &str) -> ModalResult<f32> {
    parse_extract_mass(input, "NutritionDietaryFiber: ")
}

fn parse_sugars(input: &mut &str) -> ModalResult<f32> {
    parse_extract_mass(input, "NutritionTotalSugars: ")
}

fn parse_protein(input: &mut &str) -> ModalResult<f32> {
    parse_extract_mass(input, "NutritionProtein: ")
}

fn parse_extract_energy(input: &mut &str, prefix: &str) -> ModalResult<f32> {
    delimited(
        (multispace0, literal(prefix)),
        till_line_ending,
        line_ending,
    )
    .map(|s: &str| s.trim().parse::<f32>().unwrap_or_default())
    .parse_next(input)
}

fn parse_extract_mass(input: &mut &str, prefix: &str) -> ModalResult<f32> {
    delimited(
        (multispace0, literal(prefix)),
        till_line_ending,
        line_ending,
    )
    .map(|s: &str| s.trim().parse::<f32>().unwrap_or_default())
    .parse_next(input)
}

fn skip_to_labels<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    take_until(0.., "Labels").parse_next(input)
}

fn parse_labels<'s>(input: &mut &'s str) -> ModalResult<Vec<&'s str>> {
    preceded(
        alt((literal("labels: "), literal("Labels: "))),
        (
            separated(0.., take_till(0.., |c| c == ',' || c == '\n'), literal(",")),
            line_ending,
        ),
    )
    .map(|(s, _)| s)
    .parse_next(input)
}

fn parse_images<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    parse_required_tag("images").parse_next(input)
}

fn parse_required_tag<'a>(
    prefix: &'a str,
) -> impl Parser<&'a str, &'a str, ErrMode<ContextError>> + 'a {
    move |input: &mut &'a str| {
        let s = uppercase_first_letter(prefix);

        preceded(
            alt((literal(prefix), literal(s.trim()))),
            alt((
                literal(":\n"),
                delimited(literal(": "), take_till(0.., |c| c == '\n'), line_ending),
            )),
        )
        .parse_next(input)
    }
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    c.next()
        .map_or_else(String::new, |f| f.to_uppercase().chain(c).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod test_recipes {
        use super::*;

        use std::io::Cursor;

        #[test]
        fn test_txt_ok() -> Result<()> {
            let buf = Cursor::new(files::txt());

            let got = parse_txt(buf)?;

            pretty_assertions::assert_eq!(got, results::all_recipes());
            Ok(())
        }

        #[test]
        fn test_txt2_ok() -> Result<()> {
            let buf = Cursor::new(files::txt2());

            let got = parse_txt(buf)?;

            pretty_assertions::assert_eq!(got, results::other_recipes());
            Ok(())
        }

        #[test]
        fn test_xml_ok() -> Result<()> {
            let buf = Cursor::new(files::xml());

            let got = parse_xml(buf)?;

            let mut want = results::all_recipes();
            for r in &mut want {
                r.prep_time = seconds_to_duration(15 * 60);
                r.cook_time = seconds_to_duration(15 * 60);
                r.url.clear();
                r.image.clear();
            }
            pretty_assertions::assert_eq!(got, want);
            Ok(())
        }

        #[test]
        fn test_json_ok() -> Result<()> {
            let buf = Cursor::new(files::json());

            let got = parse_json(buf)?;

            pretty_assertions::assert_eq!(got.len(), results::all_recipes().len());
            Ok(())
        }

        #[test]
        fn test_json2_ok() -> Result<()> {
            let buf = Cursor::new(files::json2());

            let got = parse_json(buf)?;

            pretty_assertions::assert_eq!(got.len(), results::other_recipes().len());
            Ok(())
        }
    }

    mod files {
        #[allow(clippy::too_many_lines)]
        pub fn txt<'a>() -> &'a str {
            r#"==== Recipes ====

id: 5ba3144a-e86e-431e-b5bb-c4109ebb8c04
title: Asparagus Soup (Zuppa Di Asparagi)
description:
yield: 6 servings
activeTime:
totalTime:
source:
url: MMF
notes:
ingredients: 2 tb Extra-virgin olive oil 1 qt Chicken broth
2 Cloves garlic, minced 4 Eggs
2 lb Asparagus, trimmed, peeled 1/2 c Freshly grated Parmesan or
-and cut (1 inch pieces) -pecorino cheese
Salt and pepper 6 sl Italian bread, toasted
instructions: Heat the oil and garlic in a soup pot until the garlic is golden. Add the
asparagus and cook until they begin to color. Season with salt and pepper.
Add the broth and bring to a boil; reduce the heat and simmer for 15
minutes, or until the asparagus is tender.

Beat the eggs and cheese together. When the asparagus is tender, reduce
the heat so the soup is no longer simmering. Very slowly ladle some of the
hot soup into the beaten eggs, stirring continuously. After adding about 2
cups of the hot soup to the eggs, reverse the process and gradually stir
the eggs mixture into the soup pot. The soup must not boil or the eggs
will scramble. Heat until thickened.

Put one slice of toasted bread into each soup dish. Ladle the hot soup on
top and pass additional grated cheese.

Serves 6.

NOTE: To trim asparagus, hold the tip in one hand and the base of the
stalk in the other. Bend gently. The asparagus will snap, leaving the
tender part with the tip.

[ "We Called It Macaroni"; Nancy Verde Barr; Knopf; ISBN 0-394-55798-0 ]

Posted by Fred Peters.
folder: main
createdAt: Wed Apr 16 2025 00:12:39 GMT+0000 (Coordinated Universal Time)
updatedAt: Wed Apr 16 2025 00:12:39 GMT+0000 (Coordinated Universal Time)
userId: ac292a74-bb5d-4fbf-9c44-557560821f53
labels: italian, soups/stews, vegetables
images:

id: d646992d-4726-48b4-ae7a-a85260b27811
title: Aubergine and Sesame Pate
description:
yield: 2 servings
activeTime:
totalTime:
source:
url: MMF
notes:
ingredients: 1/2 md Aubergine 1/4 Juice of 1 lemon
1 Crushed garlic cloves 1 tb Olive oil
1 1/2 tb Tahini Seasoning
Toasted Sesame seeds Flatleaf Parsley
Cayenne Pepper
25-30 minutes until tender. Cool slightly , then peel and
instructions: 1> Preheat the oven to 200c/400f/Gas 6. Bake the aubergine for
puree the flesh in a blender or processor.

Add the garlic, tahini and lemon juice and process until mixed.
With the motor running, drizzle in the oil to make a smooth paste.
Season to taste.

Transfer to a serving dish, garnish and serve cold with pitta bread.
folder: main
createdAt: Wed Apr 16 2025 00:12:39 GMT+0000 (Coordinated Universal Time)
updatedAt: Wed Apr 16 2025 00:12:39 GMT+0000 (Coordinated Universal Time)
userId: ac292a74-bb5d-4fbf-9c44-557560821f53
labels: appetizers, greek, vegetarian
images:

id: adea4c61-fd42-4700-a2be-48338cc44818
title: Aubergines a la Toulousaine (Eggplant A La Toulouse)
description:
yield: 4 servings
activeTime:
totalTime:
source:
url: MMF
notes:
ingredients: 1 md Eggplant 2 tb Snipped parsley
1/4 c Salad oil 1 cl Galic, minced
3 lg Tomatoes, peeled 1 tb Salad oil
2 c Fresh bread cubes 1/4 c Grated Parmesan cheese
instructions: Cut eggplant into 1/2-inch thick slices: pared. Place slices on paper
towels; sprinkle each generously with salt. let stand for 30 minutes; then
blot dry with paper towels. Start heating oven to 400 deg. F. Saute
eggplant in 1/4 cup salad oil until golden. Add more oils as needed. Cut
tomatoes into 1/2-inch thick slices; saute in same skillet. In a 10x6x2
inch baking dish, arrange eggplant and tomatoes in alternate layers, (4 in
all), sprinkling each layer with 1/4 teaspoon salt and 1/8 teaspoon pepper.
Combine bread cubes with parsley, garlic, 1 tablespoon salad oil and
cheese. Toss well. Sprinkle over top layer. Bake 20 minutes or until bread
cubes are golden and eggplant is tender.

SOURCE: Good Houskeeping's Around The World Cookbook. Consolidated Book
Publishers Chicago 1, Illinois 1958
folder: main
createdAt: Wed Apr 16 2025 00:12:39 GMT+0000 (Coordinated Universal Time)
updatedAt: Wed Apr 16 2025 00:12:39 GMT+0000 (Coordinated Universal Time)
userId: ac292a74-bb5d-4fbf-9c44-557560821f53
labels: vegetables, french, casseroles
images:

id: 03237963-3eca-4731-8586-ddaee265b98b
title: August Goerg's Grilled Steak (Spiessbraten August Goerg)
description:
yield: 6 servings
activeTime:
totalTime:
source:
url: MMF
notes:
ingredients: 1 Shallot or small onion cut 1 pn Mace
-into small pieces 1 lg Steak (just over 1 lb), at
Freshly ground black pepper -least 1 1/4 inches
instructions: ((Note: Per Horst Scharfenberg, this recipe originated in the town of
Idar-Oberstein in the 19 th century, when gemstone prospectors returning
from South America created their own version of gaucho-grilled steaks. The
dish was then further refined by Scharfenberg's mentor August Goerg. K.B.))

Per person: thick, trimmed

Mix together the shallot or onion with the pepper and mace. Insert a few
shallot pieces into the steak using the point of a small knife. Coat the
steak with the shallot mixture, pressing it in so it will adhere.

Remove the loose shallot pieces and grill the steak (over a fire of oak
logs, says August Goerg, from which the bark has been removed).* Take the
steaks off the grill while they are still pink inside. Sprinkle them with
salt.

*Note: A special grill is used, suspended with 3 chains from an iron
tripod and constantly swinging through the flames.

From: THE CUISINES OF GERMANY by Horst Scharfenberg, Simon &
Schuster/Poseidon Press, New York. 1989 Posted by: Karin Brewer, Cooking
Echo, 8/92
folder: main
createdAt: Wed Apr 16 2025 00:12:39 GMT+0000 (Coordinated Universal Time)
updatedAt: Wed Apr 16 2025 00:12:39 GMT+0000 (Coordinated Universal Time)
userId: ac292a74-bb5d-4fbf-9c44-557560821f53
labels: german, beef
images:

id: 4c23ec92-fc6f-4636-bb69-e1edbfa965c3
title: Aunt Julia's Paella
description:
yield: 6 servings
activeTime:
totalTime:
source:
url: MMF
notes:
ingredients: 1 Chicken, cut up (Or 4 thighs 1 3/4 oz Jar sliced pimento
-and legs) 2 ts Capers, with juice
Salt and pepper to thaste 4 oz Jar pimento-stiffed green
1 lb Lean pork, cut into 1-inch -olives
-cubes 1/2 lb Calamari (squid), cleaned
1 md Onion, minced -and sliced
2 Toes garlic, minced 5 c Water
Cut into 1 1/2 inch julliene 4 Chicken bouillon cubes
-strips: 1 ts Saffron threads
1/2 lg Bell pepper 2 1/2 c Uncle Ben's (c) rice,
1 lg Carrot -uncooked
1 Stalk celery 3 Hard boiled eggs, sliced
1 c Frozen green peas 1/2 lb Unpeeled shrimp (heads on)
1 1/2 lb Peeled shrimp Oil for frying
instructions: { Submitted by Chiqui Collier, Cookery N'Orleans Restaurant }

In a large electric skillet or paella pan, brown the chicken pieces (that
have been seasoned with salt and pepper) in a little oil. Remove from the
pan. Add the pork cubes to the drippinfs and brown for about 5 minutes.
Remove from the pan. To the pan drippings (add a little more oil if
necessary) add the onion, garlic, bell pepper, celery and carrot. Stir-fry
for 2 minutes.

Add the peas, peeled shrimp, pimentos, capers, chicken, calamari and pork.
Stir. In a separate pot, bring the 5 cups of water to a boil; stir in the
bouillon cubes and saffron. Let it stand for 5 minutes until dissolved.

Gently stir the rice into the skillet mixture. Slowly pour in enough of
the bouillon mixture to cover the rice and chicken pieces. Cover and cook
over low heat for about 20 minutes. Uncover and decoaratively arrange the
egg slices and raw unpeeled shrimp on the top. (Add more broth as necessary
to keep the rice moist.

Cover and steam for another 10 minutes until the shrimp are cooked and the
rice is tender. (Paella should be moist but not wet!) Place the pan on a
hot pad on the serving table and let everyone help themselves.

Serve with a mixed green salad, red ripe tomatoes and some French bread.
Also mix up a pitcher of Sangria and enjoy!

Serves: 12.

[ The Legends of Louisisna Cookbook; Sheila Ainbinder; ISBN 0-671-70817-1 ]

Posted by Fred Peters
folder: main
createdAt: Wed Apr 16 2025 00:12:39 GMT+0000 (Coordinated Universal Time)
updatedAt: Wed Apr 16 2025 00:12:39 GMT+0000 (Coordinated Universal Time)
userId: ac292a74-bb5d-4fbf-9c44-557560821f53
labels: fish/sea, pork/ham, poultry, spanish
images: https://chefbook-prod.s3.us-west-2.amazonaws.com/1744762360133-537cea00530765
"#
        }

        #[allow(clippy::too_many_lines)]
        pub fn txt2<'a>() -> &'a str {
            r"==== Recipes ====

Title: Best Chocolate Chip Cookies Recipe (with Video)
Description: The best cookies ever!
Yield: Per Recipe
ActiveTime: 20 Mins
TotalTime: 30 Mins
Source: localhost
Url: https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/
Folder: main
Ingredients: Keep Screen Awake
Oops! Something went wrong. Our team is working on it.
This recipe was developed at its original yield. Ingredient amounts are automatically adjusted, but cooking times and steps remain unchanged. Note that not all recipes scale perfectly.
Original recipe (1X) yields 48 servings
1 cup butter, softened
1 cup white sugar
1 cup packed brown sugar
2 large eggs
2 teaspoons vanilla extract
1 teaspoon baking soda
2 teaspoons hot water
½ teaspoon salt
3 cups all-purpose flour
2 cups semisweet chocolate chips
1 cup chopped walnuts
Instructions: Gather your ingredients, making sure your butter is softened, and your eggs are room temperature.
Dotdash Meredith Food Studios
Preheat the oven to 350 degrees F (175 degrees C). Beat butter, white sugar, and brown sugar together in a large bowl with an electric mixer until smooth and creamy.
Dotdash Meredith Food Studios
Beat in eggs, one at a time, then stir in vanilla.
Dotdash Meredith Food Studios
Dissolve baking soda in hot water; add to batter along with salt and mix until combined.
Dotdash Meredith Food Studios
Stir in flour, chocolate chips, and walnuts until a soft dough forms.
Dotdash Meredith Food Studios
Drop rounded spoonfuls of cookie dough 2 inches apart onto ungreased baking sheets.
Dotdash Meredith Food Studios
Bake in the preheated oven until edges are lightly browned, about 10 minutes.
Dotdash Meredith Food Studios
Cool on the baking sheets briefly before transferring to a wire rack to cool completely.
Dotdash Meredith Food Studios
Store in an airtight container or serve immediately and enjoy!
Notes: These are some notes

And these are other notes.
CreatedAt: Sun Jul 19 2026 01:52:08 GMT+0000 (Coordinated Universal Time)
UpdatedAt: Sun Jul 19 2026 01:52:08 GMT+0000 (Coordinated Universal Time)
Rating: null
NutritionServingSize: null
NutritionCalories: null
NutritionTotalFat: null
NutritionSaturatedFat: null
NutritionTransFat: null
NutritionPolyunsaturatedFat: null
NutritionMonounsaturatedFat: null
NutritionCholesterol: null
NutritionSodium: null
NutritionTotalCarbs: null
NutritionDietaryFiber: null
NutritionTotalSugars: null
NutritionAddedSugars: null
NutritionProtein: null
NutritionVitaminD: null
NutritionCalcium: null
NutritionIron: null
NutritionPotassium: null
NutritionOtherDetails: null
Labels: cookies, dessert
Images:

Title: Oven Roasted Potatoes
Description: Oven roasted potatoes are an easy side dish that goes well with almost any meal!
Yield: 6 Servings
ActiveTime: 5 Minutes
TotalTime: 35 Minutes
Source: Spend With Pennies
Url: https://www.spendwithpennies.com/simple-herb-oven-roasted-potatoes/
Folder: main
Ingredients: 2 pounds red potatoes (or yellow or Yukon gold potatoes )
2 tablespoons olive oil
1 teaspoon garlic powder
3 tablespoons chopped fresh herbs (any combination of rosemary, parsley, thyme, or basil, or 2 teaspoons dried herbs)
½ teaspoon paprika
kosher salt (to taste)
black pepper (to taste)
Instructions: Preheat the oven to 425°F.
Scrub the potatoes and cut them into 1-inch cubes.
If time allows, soak potatoes in cold water for 20 minutes or up to 1 hour. This is optional, but it removes starch and makes for a fluffier potato. Drain and dry them well.
Toss the potatoes with olive oil, garlic powder, herbs, paprika, salt, and pepper.
Spread them in a single layer on a baking sheet and roast for 30 to 35 minutes or until browned and tender.
Notes: Use any combination of herbs you’d like.  Dry spices/herbs can be substituted; use 1-2 teaspoons dry herbs instead of fresh.
The high temperature can cause fresh garlic to burn, so garlic powder is best for roasted potatoes.
Any kind of potatoes will work in this recipe.
Peeling potatoes is optional.
CreatedAt: Sun Jul 19 2026 01:53:48 GMT+0000 (Coordinated Universal Time)
UpdatedAt: Sun Jul 19 2026 01:53:48 GMT+0000 (Coordinated Universal Time)
Rating: null
NutritionServingSize: 1 serving
NutritionCalories: 147
NutritionTotalFat: 4
NutritionSaturatedFat: null
NutritionTransFat: null
NutritionPolyunsaturatedFat: null
NutritionMonounsaturatedFat: null
NutritionCholesterol: null
NutritionSodium: 27
NutritionTotalCarbs: 24
NutritionDietaryFiber: 2
NutritionTotalSugars: 1
NutritionAddedSugars: null
NutritionProtein: 2
NutritionVitaminD: null
NutritionCalcium: null
NutritionIron: null
NutritionPotassium: null
NutritionOtherDetails: null
Labels: meat, dinner
Images: https://chefbook-prod.s3.us-west-2.amazonaws.com/1784425991105-a0799e041061cd

"
        }

        #[allow(clippy::too_many_lines)]
        pub fn xml<'a>() -> &'a str {
            r#"<data>
    <recipe>
        <id>5ba3144a-e86e-431e-b5bb-c4109ebb8c04</id>
        <title>Asparagus Soup (Zuppa Di Asparagi)</title>
        <description/>
        <yield>6 servings</yield>
        <activeTime/>
        <totalTime/>
        <source/>
        <url>MMF</url>
        <notes/>
        <ingredients>2 tb Extra-virgin olive oil 1 qt Chicken broth
2 Cloves garlic, minced 4 Eggs
2 lb Asparagus, trimmed, peeled 1/2 c Freshly grated Parmesan or
-and cut (1 inch pieces) -pecorino cheese
Salt and pepper 6 sl Italian bread, toasted</ingredients>
        <instructions>Heat the oil and garlic in a soup pot until the garlic is golden. Add the
asparagus and cook until they begin to color. Season with salt and pepper.
Add the broth and bring to a boil; reduce the heat and simmer for 15
minutes, or until the asparagus is tender.

Beat the eggs and cheese together. When the asparagus is tender, reduce
the heat so the soup is no longer simmering. Very slowly ladle some of the
hot soup into the beaten eggs, stirring continuously. After adding about 2
cups of the hot soup to the eggs, reverse the process and gradually stir
the eggs mixture into the soup pot. The soup must not boil or the eggs
will scramble. Heat until thickened.

Put one slice of toasted bread into each soup dish. Ladle the hot soup on
top and pass additional grated cheese.

Serves 6.

NOTE: To trim asparagus, hold the tip in one hand and the base of the
stalk in the other. Bend gently. The asparagus will snap, leaving the
tender part with the tip.

[ "We Called It Macaroni"; Nancy Verde Barr; Knopf; ISBN 0-394-55798-0 ]

Posted by Fred Peters.</instructions>
        <folder>main</folder>
        <createdAt/>
        <updatedAt/>
        <userId>ac292a74-bb5d-4fbf-9c44-557560821f53</userId>
        <fromUser/>
        <labels>
            <title>italian</title>
        </labels>
        <labels>
            <title>soups/stews</title>
        </labels>
        <labels>
            <title>vegetables</title>
        </labels>
    </recipe>
    <recipe>
        <id>d646992d-4726-48b4-ae7a-a85260b27811</id>
        <title>Aubergine and Sesame Pate</title>
        <description/>
        <yield>2 servings</yield>
        <activeTime/>
        <totalTime/>
        <source/>
        <url>MMF</url>
        <notes/>
        <ingredients>1/2 md Aubergine 1/4 Juice of 1 lemon
1 Crushed garlic cloves 1 tb Olive oil
1 1/2 tb Tahini Seasoning
Toasted Sesame seeds Flatleaf Parsley
Cayenne Pepper
25-30 minutes until tender. Cool slightly , then peel and</ingredients>
        <instructions>1&gt; Preheat the oven to 200c/400f/Gas 6. Bake the aubergine for
puree the flesh in a blender or processor.

Add the garlic, tahini and lemon juice and process until mixed.
With the motor running, drizzle in the oil to make a smooth paste.
Season to taste.

Transfer to a serving dish, garnish and serve cold with pitta bread.</instructions>
        <folder>main</folder>
        <createdAt/>
        <updatedAt/>
        <userId>ac292a74-bb5d-4fbf-9c44-557560821f53</userId>
        <fromUser/>
        <labels>
            <title>appetizers</title>
        </labels>
        <labels>
            <title>greek</title>
        </labels>
        <labels>
            <title>vegetarian</title>
        </labels>
    </recipe>
    <recipe>
        <id>adea4c61-fd42-4700-a2be-48338cc44818</id>
        <title>Aubergines a la Toulousaine (Eggplant A La Toulouse)</title>
        <description/>
        <yield>4 servings</yield>
        <activeTime/>
        <totalTime/>
        <source/>
        <url>MMF</url>
        <notes/>
        <ingredients>1 md Eggplant 2 tb Snipped parsley
1/4 c Salad oil 1 cl Galic, minced
3 lg Tomatoes, peeled 1 tb Salad oil
2 c Fresh bread cubes 1/4 c Grated Parmesan cheese</ingredients>
        <instructions>Cut eggplant into 1/2-inch thick slices: pared. Place slices on paper
towels; sprinkle each generously with salt. let stand for 30 minutes; then
blot dry with paper towels. Start heating oven to 400 deg. F. Saute
eggplant in 1/4 cup salad oil until golden. Add more oils as needed. Cut
tomatoes into 1/2-inch thick slices; saute in same skillet. In a 10x6x2
inch baking dish, arrange eggplant and tomatoes in alternate layers, (4 in
all), sprinkling each layer with 1/4 teaspoon salt and 1/8 teaspoon pepper.
Combine bread cubes with parsley, garlic, 1 tablespoon salad oil and
cheese. Toss well. Sprinkle over top layer. Bake 20 minutes or until bread
cubes are golden and eggplant is tender.

SOURCE: Good Houskeeping's Around The World Cookbook. Consolidated Book
Publishers Chicago 1, Illinois 1958</instructions>
        <folder>main</folder>
        <createdAt/>
        <updatedAt/>
        <userId>ac292a74-bb5d-4fbf-9c44-557560821f53</userId>
        <fromUser/>
        <labels>
            <title>vegetables</title>
        </labels>
        <labels>
            <title>french</title>
        </labels>
        <labels>
            <title>casseroles</title>
        </labels>
    </recipe>
    <recipe>
        <id>03237963-3eca-4731-8586-ddaee265b98b</id>
        <title>August Goerg's Grilled Steak (Spiessbraten August Goerg)</title>
        <description/>
        <yield>6 servings</yield>
        <activeTime/>
        <totalTime/>
        <source/>
        <url>MMF</url>
        <notes/>
        <ingredients>1 Shallot or small onion cut 1 pn Mace
-into small pieces 1 lg Steak (just over 1 lb), at
Freshly ground black pepper -least 1 1/4 inches</ingredients>
        <instructions>((Note: Per Horst Scharfenberg, this recipe originated in the town of
Idar-Oberstein in the 19 th century, when gemstone prospectors returning
from South America created their own version of gaucho-grilled steaks. The
dish was then further refined by Scharfenberg's mentor August Goerg. K.B.))

Per person: thick, trimmed

Mix together the shallot or onion with the pepper and mace. Insert a few
shallot pieces into the steak using the point of a small knife. Coat the
steak with the shallot mixture, pressing it in so it will adhere.

Remove the loose shallot pieces and grill the steak (over a fire of oak
logs, says August Goerg, from which the bark has been removed).* Take the
steaks off the grill while they are still pink inside. Sprinkle them with
salt.

*Note: A special grill is used, suspended with 3 chains from an iron
tripod and constantly swinging through the flames.

From: THE CUISINES OF GERMANY by Horst Scharfenberg, Simon &amp;
Schuster/Poseidon Press, New York. 1989 Posted by: Karin Brewer, Cooking
Echo, 8/92</instructions>
        <folder>main</folder>
        <createdAt/>
        <updatedAt/>
        <userId>ac292a74-bb5d-4fbf-9c44-557560821f53</userId>
        <fromUser/>
        <labels>
            <title>german</title>
        </labels>
        <labels>
            <title>beef</title>
        </labels>
    </recipe>
    <recipe>
        <id>4c23ec92-fc6f-4636-bb69-e1edbfa965c3</id>
        <title>Aunt Julia's Paella</title>
        <description/>
        <yield>6 servings</yield>
        <activeTime/>
        <totalTime/>
        <source/>
        <url>MMF</url>
        <notes/>
        <ingredients>1 Chicken, cut up (Or 4 thighs 1 3/4 oz Jar sliced pimento
-and legs) 2 ts Capers, with juice
Salt and pepper to thaste 4 oz Jar pimento-stiffed green
1 lb Lean pork, cut into 1-inch -olives
-cubes 1/2 lb Calamari (squid), cleaned
1 md Onion, minced -and sliced
2 Toes garlic, minced 5 c Water
Cut into 1 1/2 inch julliene 4 Chicken bouillon cubes
-strips: 1 ts Saffron threads
1/2 lg Bell pepper 2 1/2 c Uncle Ben's (c) rice,
1 lg Carrot -uncooked
1 Stalk celery 3 Hard boiled eggs, sliced
1 c Frozen green peas 1/2 lb Unpeeled shrimp (heads on)
1 1/2 lb Peeled shrimp Oil for frying</ingredients>
        <instructions>{ Submitted by Chiqui Collier, Cookery N'Orleans Restaurant }

In a large electric skillet or paella pan, brown the chicken pieces (that
have been seasoned with salt and pepper) in a little oil. Remove from the
pan. Add the pork cubes to the drippinfs and brown for about 5 minutes.
Remove from the pan. To the pan drippings (add a little more oil if
necessary) add the onion, garlic, bell pepper, celery and carrot. Stir-fry
for 2 minutes.

Add the peas, peeled shrimp, pimentos, capers, chicken, calamari and pork.
Stir. In a separate pot, bring the 5 cups of water to a boil; stir in the
bouillon cubes and saffron. Let it stand for 5 minutes until dissolved.

Gently stir the rice into the skillet mixture. Slowly pour in enough of
the bouillon mixture to cover the rice and chicken pieces. Cover and cook
over low heat for about 20 minutes. Uncover and decoaratively arrange the
egg slices and raw unpeeled shrimp on the top. (Add more broth as necessary
to keep the rice moist.

Cover and steam for another 10 minutes until the shrimp are cooked and the
rice is tender. (Paella should be moist but not wet!) Place the pan on a
hot pad on the serving table and let everyone help themselves.

Serve with a mixed green salad, red ripe tomatoes and some French bread.
Also mix up a pitcher of Sangria and enjoy!

Serves: 12.

[ The Legends of Louisisna Cookbook; Sheila Ainbinder; ISBN 0-671-70817-1 ]

Posted by Fred Peters</instructions>
        <folder>main</folder>
        <createdAt/>
        <updatedAt/>
        <userId>ac292a74-bb5d-4fbf-9c44-557560821f53</userId>
        <fromUser/>
        <labels>
            <title>fish/sea</title>
        </labels>
        <labels>
            <title>pork/ham</title>
        </labels>
        <labels>
            <title>poultry</title>
        </labels>
        <labels>
            <title>spanish</title>
        </labels>
        <images>
            <id>270f79d0-cd3e-4ac8-992c-b15b4c057dea</id>
            <location>https://chefbook-prod.s3.us-west-2.amazonaws.com/1744762360133-537cea00530765</location>
        </images>
    </recipe>
</data>"#
        }

        pub fn json<'a>() -> &'a str {
            r#"[{"@context":"http://schema.org","@type":"Recipe","identifier":"5ba3144a-e86e-431e-b5bb-c4109ebb8c04","datePublished":"2025-04-16T00:12:39.551Z","description":"","image":[],"name":"Asparagus Soup (Zuppa Di Asparagi)","prepTime":"","recipeIngredient":["2 tb Extra-virgin olive oil 1 qt Chicken broth","2 Cloves garlic, minced 4 Eggs","2 lb Asparagus, trimmed, peeled 1/2 c Freshly grated Parmesan or","-and cut (1 inch pieces) -pecorino cheese","Salt and pepper 6 sl Italian bread, toasted"],"recipeInstructions":[{"@type":"HowToStep","text":"Heat the oil and garlic in a soup pot until the garlic is golden. Add the"},{"@type":"HowToStep","text":"asparagus and cook until they begin to color. Season with salt and pepper."},{"@type":"HowToStep","text":"Add the broth and bring to a boil; reduce the heat and simmer for 15"},{"@type":"HowToStep","text":"minutes, or until the asparagus is tender."},{"@type":"HowToStep","text":"Beat the eggs and cheese together. When the asparagus is tender, reduce"},{"@type":"HowToStep","text":"the heat so the soup is no longer simmering. Very slowly ladle some of the"},{"@type":"HowToStep","text":"hot soup into the beaten eggs, stirring continuously. After adding about 2"},{"@type":"HowToStep","text":"cups of the hot soup to the eggs, reverse the process and gradually stir"},{"@type":"HowToStep","text":"the eggs mixture into the soup pot. The soup must not boil or the eggs"},{"@type":"HowToStep","text":"will scramble. Heat until thickened."},{"@type":"HowToStep","text":"Put one slice of toasted bread into each soup dish. Ladle the hot soup on"},{"@type":"HowToStep","text":"top and pass additional grated cheese."},{"@type":"HowToStep","text":"Serves 6."},{"@type":"HowToStep","text":"NOTE: To trim asparagus, hold the tip in one hand and the base of the"},{"@type":"HowToStep","text":"stalk in the other. Bend gently. The asparagus will snap, leaving the"},{"@type":"HowToStep","text":"tender part with the tip."},{"@type":"HowToSection","text":"[ \"We Called It Macaroni\"; Nancy Verde Barr; Knopf; ISBN 0-394-55798-0 ]"},{"@type":"HowToStep","text":"Posted by Fred Peters."}],"recipeYield":"6 servings","totalTime":"","recipeCategory":["italian","soups/stews","vegetables"],"creditText":"","isBasedOn":"MMF","comment":[{"@type":"Comment","name":"Author Notes","text":""}]},{"@context":"http://schema.org","@type":"Recipe","identifier":"d646992d-4726-48b4-ae7a-a85260b27811","datePublished":"2025-04-16T00:12:39.551Z","description":"","image":[],"name":"Aubergine and Sesame Pate","prepTime":"","recipeIngredient":["1/2 md Aubergine 1/4 Juice of 1 lemon","1 Crushed garlic cloves 1 tb Olive oil","1 1/2 tb Tahini Seasoning","Toasted Sesame seeds Flatleaf Parsley","Cayenne Pepper","25-30 minutes until tender. Cool slightly , then peel and"],"recipeInstructions":[{"@type":"HowToStep","text":"1> Preheat the oven to 200c/400f/Gas 6. Bake the aubergine for"},{"@type":"HowToStep","text":"puree the flesh in a blender or processor."},{"@type":"HowToStep","text":"Add the garlic, tahini and lemon juice and process until mixed."},{"@type":"HowToStep","text":"With the motor running, drizzle in the oil to make a smooth paste."},{"@type":"HowToStep","text":"Season to taste."},{"@type":"HowToStep","text":"Transfer to a serving dish, garnish and serve cold with pitta bread."}],"recipeYield":"2 servings","totalTime":"","recipeCategory":["appetizers","greek","vegetarian"],"creditText":"","isBasedOn":"MMF","comment":[{"@type":"Comment","name":"Author Notes","text":""}]},{"@context":"http://schema.org","@type":"Recipe","identifier":"adea4c61-fd42-4700-a2be-48338cc44818","datePublished":"2025-04-16T00:12:39.551Z","description":"","image":[],"name":"Aubergines a la Toulousaine (Eggplant A La Toulouse)","prepTime":"","recipeIngredient":["1 md Eggplant 2 tb Snipped parsley","1/4 c Salad oil 1 cl Galic, minced","3 lg Tomatoes, peeled 1 tb Salad oil","2 c Fresh bread cubes 1/4 c Grated Parmesan cheese"],"recipeInstructions":[{"@type":"HowToStep","text":"Cut eggplant into 1/2-inch thick slices: pared. Place slices on paper"},{"@type":"HowToStep","text":"towels; sprinkle each generously with salt. let stand for 30 minutes; then"},{"@type":"HowToStep","text":"blot dry with paper towels. Start heating oven to 400 deg. F. Saute"},{"@type":"HowToStep","text":"eggplant in 1/4 cup salad oil until golden. Add more oils as needed. Cut"},{"@type":"HowToStep","text":"tomatoes into 1/2-inch thick slices; saute in same skillet. In a 10x6x2"},{"@type":"HowToStep","text":"inch baking dish, arrange eggplant and tomatoes in alternate layers, (4 in"},{"@type":"HowToStep","text":"all), sprinkling each layer with 1/4 teaspoon salt and 1/8 teaspoon pepper."},{"@type":"HowToStep","text":"Combine bread cubes with parsley, garlic, 1 tablespoon salad oil and"},{"@type":"HowToStep","text":"cheese. Toss well. Sprinkle over top layer. Bake 20 minutes or until bread"},{"@type":"HowToStep","text":"cubes are golden and eggplant is tender."},{"@type":"HowToStep","text":"SOURCE: Good Houskeeping's Around The World Cookbook. Consolidated Book"},{"@type":"HowToStep","text":"Publishers Chicago 1, Illinois 1958"}],"recipeYield":"4 servings","totalTime":"","recipeCategory":["vegetables","french","casseroles"],"creditText":"","isBasedOn":"MMF","comment":[{"@type":"Comment","name":"Author Notes","text":""}]},{"@context":"http://schema.org","@type":"Recipe","identifier":"03237963-3eca-4731-8586-ddaee265b98b","datePublished":"2025-04-16T00:12:39.551Z","description":"","image":[],"name":"August Goerg's Grilled Steak (Spiessbraten August Goerg)","prepTime":"","recipeIngredient":["1 Shallot or small onion cut 1 pn Mace","-into small pieces 1 lg Steak (just over 1 lb), at","Freshly ground black pepper -least 1 1/4 inches"],"recipeInstructions":[{"@type":"HowToStep","text":"((Note: Per Horst Scharfenberg, this recipe originated in the town of"},{"@type":"HowToStep","text":"Idar-Oberstein in the 19 th century, when gemstone prospectors returning"},{"@type":"HowToStep","text":"from South America created their own version of gaucho-grilled steaks. The"},{"@type":"HowToStep","text":"dish was then further refined by Scharfenberg's mentor August Goerg. K.B.))"},{"@type":"HowToStep","text":"Per person: thick, trimmed"},{"@type":"HowToStep","text":"Mix together the shallot or onion with the pepper and mace. Insert a few"},{"@type":"HowToStep","text":"shallot pieces into the steak using the point of a small knife. Coat the"},{"@type":"HowToStep","text":"steak with the shallot mixture, pressing it in so it will adhere."},{"@type":"HowToStep","text":"Remove the loose shallot pieces and grill the steak (over a fire of oak"},{"@type":"HowToStep","text":"logs, says August Goerg, from which the bark has been removed).* Take the"},{"@type":"HowToStep","text":"steaks off the grill while they are still pink inside. Sprinkle them with"},{"@type":"HowToStep","text":"salt."},{"@type":"HowToStep","text":"*Note: A special grill is used, suspended with 3 chains from an iron"},{"@type":"HowToStep","text":"tripod and constantly swinging through the flames."},{"@type":"HowToStep","text":"From: THE CUISINES OF GERMANY by Horst Scharfenberg, Simon &"},{"@type":"HowToStep","text":"Schuster/Poseidon Press, New York. 1989 Posted by: Karin Brewer, Cooking"},{"@type":"HowToStep","text":"Echo, 8/92"}],"recipeYield":"6 servings","totalTime":"","recipeCategory":["german","beef"],"creditText":"","isBasedOn":"MMF","comment":[{"@type":"Comment","name":"Author Notes","text":""}]},{"@context":"http://schema.org","@type":"Recipe","identifier":"4c23ec92-fc6f-4636-bb69-e1edbfa965c3","datePublished":"2025-04-16T00:12:39.551Z","description":"","image":["https://chefbook-prod.s3.us-west-2.amazonaws.com/1744762360133-537cea00530765"],"name":"Aunt Julia's Paella","prepTime":"","recipeIngredient":["1 Chicken, cut up (Or 4 thighs 1 3/4 oz Jar sliced pimento","-and legs) 2 ts Capers, with juice","Salt and pepper to thaste 4 oz Jar pimento-stiffed green","1 lb Lean pork, cut into 1-inch -olives","-cubes 1/2 lb Calamari (squid), cleaned","1 md Onion, minced -and sliced","2 Toes garlic, minced 5 c Water","Cut into 1 1/2 inch julliene 4 Chicken bouillon cubes","-strips: 1 ts Saffron threads","1/2 lg Bell pepper 2 1/2 c Uncle Ben's (c) rice,","1 lg Carrot -uncooked","1 Stalk celery 3 Hard boiled eggs, sliced","1 c Frozen green peas 1/2 lb Unpeeled shrimp (heads on)","1 1/2 lb Peeled shrimp Oil for frying"],"recipeInstructions":[{"@type":"HowToStep","text":"{ Submitted by Chiqui Collier, Cookery N'Orleans Restaurant }"},{"@type":"HowToStep","text":"In a large electric skillet or paella pan, brown the chicken pieces (that"},{"@type":"HowToStep","text":"have been seasoned with salt and pepper) in a little oil. Remove from the"},{"@type":"HowToStep","text":"pan. Add the pork cubes to the drippinfs and brown for about 5 minutes."},{"@type":"HowToStep","text":"Remove from the pan. To the pan drippings (add a little more oil if"},{"@type":"HowToStep","text":"necessary) add the onion, garlic, bell pepper, celery and carrot. Stir-fry"},{"@type":"HowToStep","text":"for 2 minutes."},{"@type":"HowToStep","text":"Add the peas, peeled shrimp, pimentos, capers, chicken, calamari and pork."},{"@type":"HowToStep","text":"Stir. In a separate pot, bring the 5 cups of water to a boil; stir in the"},{"@type":"HowToStep","text":"bouillon cubes and saffron. Let it stand for 5 minutes until dissolved."},{"@type":"HowToStep","text":"Gently stir the rice into the skillet mixture. Slowly pour in enough of"},{"@type":"HowToStep","text":"the bouillon mixture to cover the rice and chicken pieces. Cover and cook"},{"@type":"HowToStep","text":"over low heat for about 20 minutes. Uncover and decoaratively arrange the"},{"@type":"HowToStep","text":"egg slices and raw unpeeled shrimp on the top. (Add more broth as necessary"},{"@type":"HowToStep","text":"to keep the rice moist."},{"@type":"HowToStep","text":"Cover and steam for another 10 minutes until the shrimp are cooked and the"},{"@type":"HowToStep","text":"rice is tender. (Paella should be moist but not wet!) Place the pan on a"},{"@type":"HowToStep","text":"hot pad on the serving table and let everyone help themselves."},{"@type":"HowToStep","text":"Serve with a mixed green salad, red ripe tomatoes and some French bread."},{"@type":"HowToStep","text":"Also mix up a pitcher of Sangria and enjoy!"},{"@type":"HowToStep","text":"Serves: 12."},{"@type":"HowToSection","text":"[ The Legends of Louisisna Cookbook; Sheila Ainbinder; ISBN 0-671-70817-1 ]"},{"@type":"HowToStep","text":"Posted by Fred Peters"}],"recipeYield":"6 servings","totalTime":"","recipeCategory":["fish/sea","pork/ham","poultry","spanish"],"creditText":"","isBasedOn":"MMF","comment":[{"@type":"Comment","name":"Author Notes","text":""}]}]"#
        }

        pub fn json2<'a>() -> &'a str {
            r#"{"recipes":[{"@context":"http://schema.org","@type":"Recipe","identifier":"790c3480-61c5-428e-9785-62a9c856183d","datePublished":"2026-07-19T01:52:08.558Z","description":"The best cookies ever!","image":[],"name":"Best Chocolate Chip Cookies Recipe (with Video)","prepTime":"PT20M","recipeIngredient":["Keep Screen Awake","Oops! Something went wrong. Our team is working on it.","This recipe was developed at its original yield. Ingredient amounts are automatically adjusted, but cooking times and steps remain unchanged. Note that not all recipes scale perfectly.","Original recipe (1X) yields 48 servings","1 cup butter, softened","1 cup white sugar","1 cup packed brown sugar","2 large eggs","2 teaspoons vanilla extract","1 teaspoon baking soda","2 teaspoons hot water","1/2 teaspoon salt","3 cups all-purpose flour","2 cups semisweet chocolate chips","1 cup chopped walnuts"],"recipeInstructions":[{"@type":"HowToStep","text":"Gather your ingredients, making sure your butter is softened, and your eggs are room temperature."},{"@type":"HowToStep","text":"Dotdash Meredith Food Studios"},{"@type":"HowToStep","text":"Preheat the oven to 350 degrees F (175 degrees C). Beat butter, white sugar, and brown sugar together in a large bowl with an electric mixer until smooth and creamy."},{"@type":"HowToStep","text":"Dotdash Meredith Food Studios"},{"@type":"HowToStep","text":"Beat in eggs, one at a time, then stir in vanilla."},{"@type":"HowToStep","text":"Dotdash Meredith Food Studios"},{"@type":"HowToStep","text":"Dissolve baking soda in hot water; add to batter along with salt and mix until combined."},{"@type":"HowToStep","text":"Dotdash Meredith Food Studios"},{"@type":"HowToStep","text":"Stir in flour, chocolate chips, and walnuts until a soft dough forms."},{"@type":"HowToStep","text":"Dotdash Meredith Food Studios"},{"@type":"HowToStep","text":"Drop rounded spoonfuls of cookie dough 2 inches apart onto ungreased baking sheets."},{"@type":"HowToStep","text":"Dotdash Meredith Food Studios"},{"@type":"HowToStep","text":"Bake in the preheated oven until edges are lightly browned, about 10 minutes."},{"@type":"HowToStep","text":"Dotdash Meredith Food Studios"},{"@type":"HowToStep","text":"Cool on the baking sheets briefly before transferring to a wire rack to cool completely."},{"@type":"HowToStep","text":"Dotdash Meredith Food Studios"},{"@type":"HowToStep","text":"Store in an airtight container or serve immediately and enjoy!"}],"recipeYield":"Per Recipe","totalTime":"PT30M","recipeCategory":["cookies","dessert"],"creditText":"localhost","isBasedOn":"https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/","comment":[{"@type":"Comment","name":"Author Notes","text":"These are some notes\n\nAnd these are other notes."}]},{"@context":"http://schema.org","@type":"Recipe","identifier":"2345536c-1db5-46a4-8aa8-f5d4a5454f94","datePublished":"2026-07-19T01:53:48.031Z","description":"Oven roasted potatoes are an easy side dish that goes well with almost any meal!","image":["https://chefbook-prod.s3.us-west-2.amazonaws.com/1784425991105-a0799e041061cd"],"name":"Oven Roasted Potatoes","prepTime":"PT5M","recipeIngredient":["2 pounds red potatoes (or yellow or Yukon gold potatoes )","2 tablespoons olive oil","1 teaspoon garlic powder","3 tablespoons chopped fresh herbs (any combination of rosemary, parsley, thyme, or basil, or 2 teaspoons dried herbs)","1/2 teaspoon paprika","kosher salt (to taste)","black pepper (to taste)"],"recipeInstructions":[{"@type":"HowToStep","text":"Preheat the oven to 425°F."},{"@type":"HowToStep","text":"Scrub the potatoes and cut them into 1-inch cubes."},{"@type":"HowToStep","text":"If time allows, soak potatoes in cold water for 20 minutes or up to 1 hour. This is optional, but it removes starch and makes for a fluffier potato. Drain and dry them well."},{"@type":"HowToStep","text":"Toss the potatoes with olive oil, garlic powder, herbs, paprika, salt, and pepper."},{"@type":"HowToStep","text":"Spread them in a single layer on a baking sheet and roast for 30 to 35 minutes or until browned and tender."}],"recipeYield":"6 Servings","totalTime":"PT35M","recipeCategory":["meat","dinner"],"creditText":"Spend With Pennies","isBasedOn":"https://www.spendwithpennies.com/simple-herb-oven-roasted-potatoes/","comment":[{"@type":"Comment","name":"Author Notes","text":"Use any combination of herbs you’d like.  Dry spices/herbs can be substituted; use 1-2 teaspoons dry herbs instead of fresh.\nThe high temperature can cause fresh garlic to burn, so garlic powder is best for roasted potatoes.\nAny kind of potatoes will work in this recipe.\nPeeling potatoes is optional."}],"nutrition":{"@type":"NutritionInformation","servingSize":"1 serving","calories":"147 kcal","fatContent":"4 g","sodiumContent":"27 mg","carbohydrateContent":"24 g","fiberContent":"2 g","sugarContent":"1 g","proteinContent":"2 g"}}]}"#
        }
    }

    mod results {
        use std::iter::once;

        use schema_org::{DurationOrText, field::RecipeImageFieldEnum};

        use super::*;

        #[allow(clippy::too_many_lines)]
        pub fn all_recipes() -> Vec<Recipe> {
            vec![
                Recipe {
                    keywords: ["soups/stews", "vegetables"].into_iter().map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.into())).collect(),
                    is_based_on: to_is_based_on("MMF"),
                    name: vec!["Asparagus Soup (Zuppa Di Asparagi)".into()],
                    recipe_category: vec!["italian".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("2 tb Extra-virgin olive oil 1 qt Chicken broth".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 Cloves garlic, minced 4 Eggs".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 lb Asparagus, trimmed, peeled 1/2 c Freshly grated Parmesan or".into()),
                        RecipeRecipeIngredientFieldEnum::Text("-and cut (1 inch pieces) -pecorino cheese".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Salt and pepper 6 sl Italian bread, toasted".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Heat the oil and garlic in a soup pot until the garlic is golden. Add the asparagus and cook until they begin to color. Season with salt and pepper. Add the broth and bring to a boil; reduce the heat and simmer for 15 minutes, or until the asparagus is tender.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Beat the eggs and cheese together. When the asparagus is tender, reduce the heat so the soup is no longer simmering. Very slowly ladle some of the hot soup into the beaten eggs, stirring continuously. After adding about 2 cups of the hot soup to the eggs, reverse the process and gradually stir the eggs mixture into the soup pot. The soup must not boil or the eggs will scramble. Heat until thickened.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Put one slice of toasted bread into each soup dish. Ladle the hot soup on top and pass additional grated cheese.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("Serves 6.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "NOTE: To trim asparagus, hold the tip in one hand and the base of the stalk in the other. Bend gently. The asparagus will snap, leaving the tender part with the tip.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "[ \"We Called It Macaroni\"; Nancy Verde Barr; Knopf; ISBN 0-394-55798-0 ]".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("Posted by Fred Peters.".into()),
                    ],
                    recipe_yield: to_yield(6),
                    url: vec!["MMF".into()],
                    ..Default::default()
                },
                Recipe {
                    keywords: ["greek", "vegetarian"].into_iter().map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.into())).collect(),
                    is_based_on: to_is_based_on("MMF"),
                    name: vec!["Aubergine and Sesame Pate".into()],
                    recipe_category: vec!["appetizers".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1/2 md Aubergine 1/4 Juice of 1 lemon".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Crushed garlic cloves 1 tb Olive oil".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 1/2 tb Tahini Seasoning".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Toasted Sesame seeds Flatleaf Parsley".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Cayenne Pepper".into()),
                        RecipeRecipeIngredientFieldEnum::Text("25-30 minutes until tender. Cool slightly , then peel and".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "1> Preheat the oven to 200c/400f/Gas 6. Bake the aubergine for puree the flesh in a blender or processor.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Add the garlic, tahini and lemon juice and process until mixed. With the motor running, drizzle in the oil to make a smooth paste. Season to taste.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Transfer to a serving dish, garnish and serve cold with pitta bread.".into(),
                        ),
                    ],
                    recipe_yield: to_yield(2),
                    url: vec!["MMF".into()],
                    ..Default::default()
                },
                Recipe {
                    keywords: ["french", "casseroles"].into_iter().map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.into())).collect(),
                    is_based_on: to_is_based_on("MMF"),
                    name: vec!["Aubergines a la Toulousaine (Eggplant A La Toulouse)".into()],
                    recipe_category: vec!["vegetables".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1 md Eggplant 2 tb Snipped parsley".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/4 c Salad oil 1 cl Galic, minced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3 lg Tomatoes, peeled 1 tb Salad oil".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 c Fresh bread cubes 1/4 c Grated Parmesan cheese".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Cut eggplant into 1/2-inch thick slices: pared. Place slices on paper towels; sprinkle each generously with salt. let stand for 30 minutes; then blot dry with paper towels. Start heating oven to 400 deg. F. Saute eggplant in 1/4 cup salad oil until golden. Add more oils as needed. Cut tomatoes into 1/2-inch thick slices; saute in same skillet. In a 10x6x2 inch baking dish, arrange eggplant and tomatoes in alternate layers, (4 in all), sprinkling each layer with 1/4 teaspoon salt and 1/8 teaspoon pepper. Combine bread cubes with parsley, garlic, 1 tablespoon salad oil and cheese. Toss well. Sprinkle over top layer. Bake 20 minutes or until bread cubes are golden and eggplant is tender.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "SOURCE: Good Houskeeping's Around The World Cookbook. Consolidated Book Publishers Chicago 1, Illinois 1958".into(),
                        ),
                    ],
                    recipe_yield: to_yield(4),
                    url: vec!["MMF".into()],
                    ..Default::default()
                },
                Recipe {
                    keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("beef".into())],
                    is_based_on: to_is_based_on("MMF"),
                    name: vec!["August Goerg's Grilled Steak (Spiessbraten August Goerg)".into()],
                    recipe_category: vec!["german".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1 Shallot or small onion cut 1 pn Mace".into()),
                        RecipeRecipeIngredientFieldEnum::Text("-into small pieces 1 lg Steak (just over 1 lb), at".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Freshly ground black pepper -least 1 1/4 inches".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "((Note: Per Horst Scharfenberg, this recipe originated in the town of Idar-Oberstein in the 19 th century, when gemstone prospectors returning from South America created their own version of gaucho-grilled steaks. The dish was then further refined by Scharfenberg's mentor August Goerg. K.B.))".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("Per person: thick, trimmed".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Mix together the shallot or onion with the pepper and mace. Insert a few shallot pieces into the steak using the point of a small knife. Coat the steak with the shallot mixture, pressing it in so it will adhere.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Remove the loose shallot pieces and grill the steak (over a fire of oak logs, says August Goerg, from which the bark has been removed).* Take the steaks off the grill while they are still pink inside. Sprinkle them with salt.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "*Note: A special grill is used, suspended with 3 chains from an iron tripod and constantly swinging through the flames.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "From: THE CUISINES OF GERMANY by Horst Scharfenberg, Simon & Schuster/Poseidon Press, New York. 1989 Posted by: Karin Brewer, Cooking Echo, 8/92".into(),
                        ),
                    ],
                    recipe_yield: to_yield(6),
                    url: vec!["MMF".into()],
                    ..Default::default()
                },
                Recipe {
                    keywords: ["pork/ham", "poultry", "spanish"].into_iter().map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.into())).collect(),
                    is_based_on: to_is_based_on("MMF"),
                    name: vec!["Aunt Julia's Paella".into()],
                    recipe_category: vec!["fish/sea".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1 Chicken, cut up (Or 4 thighs 1 3/4 oz Jar sliced pimento".into()),
                        RecipeRecipeIngredientFieldEnum::Text("-and legs) 2 ts Capers, with juice".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Salt and pepper to thaste 4 oz Jar pimento-stiffed green".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 lb Lean pork, cut into 1-inch -olives".into()),
                        RecipeRecipeIngredientFieldEnum::Text("-cubes 1/2 lb Calamari (squid), cleaned".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 md Onion, minced -and sliced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 Toes garlic, minced 5 c Water".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Cut into 1 1/2 inch julliene 4 Chicken bouillon cubes".into()),
                        RecipeRecipeIngredientFieldEnum::Text("-strips: 1 ts Saffron threads".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 lg Bell pepper 2 1/2 c Uncle Ben's (c) rice,".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 lg Carrot -uncooked".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Stalk celery 3 Hard boiled eggs, sliced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 c Frozen green peas 1/2 lb Unpeeled shrimp (heads on)".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 1/2 lb Peeled shrimp Oil for frying".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "{ Submitted by Chiqui Collier, Cookery N'Orleans Restaurant }".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "In a large electric skillet or paella pan, brown the chicken pieces (that have been seasoned with salt and pepper) in a little oil. Remove from the pan. Add the pork cubes to the drippinfs and brown for about 5 minutes. Remove from the pan. To the pan drippings (add a little more oil if necessary) add the onion, garlic, bell pepper, celery and carrot. Stir-fry for 2 minutes.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Add the peas, peeled shrimp, pimentos, capers, chicken, calamari and pork. Stir. In a separate pot, bring the 5 cups of water to a boil; stir in the bouillon cubes and saffron. Let it stand for 5 minutes until dissolved.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Gently stir the rice into the skillet mixture. Slowly pour in enough of the bouillon mixture to cover the rice and chicken pieces. Cover and cook over low heat for about 20 minutes. Uncover and decoaratively arrange the egg slices and raw unpeeled shrimp on the top. (Add more broth as necessary to keep the rice moist.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Cover and steam for another 10 minutes until the shrimp are cooked and the rice is tender. (Paella should be moist but not wet!) Place the pan on a hot pad on the serving table and let everyone help themselves.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Serve with a mixed green salad, red ripe tomatoes and some French bread. Also mix up a pitcher of Sangria and enjoy!".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("Serves: 12.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "[ The Legends of Louisisna Cookbook; Sheila Ainbinder; ISBN 0-671-70817-1 ]".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("Posted by Fred Peters".into()),
                    ],
                    recipe_yield: to_yield(6),
                    image: vec![RecipeImageFieldEnum::URL("https://chefbook-prod.s3.us-west-2.amazonaws.com/1744762360133-537cea00530765".into())],
                    url: vec!["MMF".into()],
                    ..Default::default()
                },
            ]
        }

        pub fn other_recipes() -> Vec<Recipe> {
            vec![
                Recipe {
                    comment: vec![
                        Comment {
                            text: vec!["These are some notes\n\nAnd these are other notes.".into()],
                            ..Default::default()
                        }
                    ],
                    description: vec![RecipeDescriptionFieldEnum::Text("The best cookies ever!".into())],
                    keywords: once("dessert").map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.into())).collect(),
                    is_based_on: to_is_based_on("localhost"),
                    name: vec!["Best Chocolate Chip Cookies Recipe (with Video)".into()],
                    prep_time: vec![DurationOrText::Text("20 Mins".into())],
                    total_time: vec![DurationOrText::Text("30 Mins".into())],
                    recipe_category: vec!["cookies".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text(
                            "Keep Screen Awake".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "Oops! Something went wrong. Our team is working on it.".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "This recipe was developed at its original yield. Ingredient amounts are automatically adjusted, but cooking times and steps remain unchanged. Note that not all recipes scale perfectly.".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "Original recipe (1X) yields 48 servings".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 cup butter, softened".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 cup white sugar".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 cup packed brown sugar".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "2 large eggs".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "2 teaspoons vanilla extract".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 teaspoon baking soda".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "2 teaspoons hot water".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "½ teaspoon salt".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "3 cups all-purpose flour".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "2 cups semisweet chocolate chips".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 cup chopped walnuts".into(),
                        ),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Gather your ingredients, making sure your butter is softened, and your eggs are room temperature.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Dotdash Meredith Food Studios".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Preheat the oven to 350 degrees F (175 degrees C). Beat butter, white sugar, and brown sugar together in a large bowl with an electric mixer until smooth and creamy.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Dotdash Meredith Food Studios".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Beat in eggs, one at a time, then stir in vanilla.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Dotdash Meredith Food Studios".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Dissolve baking soda in hot water; add to batter along with salt and mix until combined.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Dotdash Meredith Food Studios".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Stir in flour, chocolate chips, and walnuts until a soft dough forms.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Dotdash Meredith Food Studios".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Drop rounded spoonfuls of cookie dough 2 inches apart onto ungreased baking sheets.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Dotdash Meredith Food Studios".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Bake in the preheated oven until edges are lightly browned, about 10 minutes.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Dotdash Meredith Food Studios".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Cool on the baking sheets briefly before transferring to a wire rack to cool completely.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Dotdash Meredith Food Studios".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Store in an airtight container or serve immediately and enjoy!".into(),
                        ),
                    ],
                    url: vec!["https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/".into()],
                    ..Default::default()
                },
                Recipe {
                    comment: vec![
                        Comment {
                            text: vec!["Use any combination of herbs you’d like.  Dry spices/herbs can be substituted; use 1-2 teaspoons dry herbs instead of fresh.\nThe high temperature can cause fresh garlic to burn, so garlic powder is best for roasted potatoes.\nAny kind of potatoes will work in this recipe.\nPeeling potatoes is optional.".into()],
                            ..Default::default()
                        }
                    ],
                    description: vec![RecipeDescriptionFieldEnum::Text("Oven roasted potatoes are an easy side dish that goes well with almost any meal!".into())],
                    keywords: once("dinner").map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.into())).collect(),
                    is_based_on: to_is_based_on("Spend With Pennies"),
                    name: vec!["Oven Roasted Potatoes".into()],
                    nutrition: vec![NutritionInformation {
                        calories:vec![Energy::new("147 kcal")],
                        serving_size:vec!["1 serving".into()],
                        carbohydrate_content: vec![Mass::new("24 g")],
                        context: at_context(),
                        fat_content: vec![Mass::new("4 g")],
                        fiber_content: vec![Mass::new("2 g")],
                        protein_content: vec![Mass::new("2 g")],
                        sodium_content: vec![Mass::new("27 mg")],
                        sugar_content: vec![Mass::new("1 g")],
                        r#type: AtType::NutritionInformation.to_opt(),
                        ..Default::default()
                    }],
                    prep_time: vec![DurationOrText::Text("5 Minutes".into())],
                    total_time: vec![DurationOrText::Text("35 Minutes".into())],
                    recipe_category: vec!["meat".into()],
                    image: vec![RecipeImageFieldEnum::URL("https://chefbook-prod.s3.us-west-2.amazonaws.com/1784425991105-a0799e041061cd".into())],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text(
                            "2 pounds red potatoes (or yellow or Yukon gold potatoes )".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "2 tablespoons olive oil".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 teaspoon garlic powder".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "3 tablespoons chopped fresh herbs (any combination of rosemary, parsley, thyme, or basil, or 2 teaspoons dried herbs)".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "½ teaspoon paprika".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "kosher salt (to taste)".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "black pepper (to taste)".into(),
                        ),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Preheat the oven to 425°F.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Scrub the potatoes and cut them into 1-inch cubes.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "If time allows, soak potatoes in cold water for 20 minutes or up to 1 hour. This is optional, but it removes starch and makes for a fluffier potato. Drain and dry them well.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Toss the potatoes with olive oil, garlic powder, herbs, paprika, salt, and pepper.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Spread them in a single layer on a baking sheet and roast for 30 to 35 minutes or until browned and tender.".into(),
                        ),
                    ],
                    recipe_yield: to_yield(6),
                    url: vec!["https://www.spendwithpennies.com/simple-herb-oven-roasted-potatoes/".into()],
                    ..Default::default()
                },
            ]
        }
    }
}
