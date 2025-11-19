use std::borrow::Cow;
use std::io::{Read, Seek};

use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{digit1, line_ending, space0, space1};
use nom::combinator::{map, opt, recognize, rest};
use nom::multi::{many1, separated_list0};
use nom::sequence::{delimited, preceded, terminated};

use schema_org::field::{
    AggregateRatingRatingValueFieldEnum, ImageObjectImageFieldEnum, RecipeAuthorFieldEnum,
    RecipeDescriptionFieldEnum, RecipeImageFieldEnum, RecipeIsBasedOnFieldEnum,
    RecipeKeywordsFieldEnum, RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
};
use schema_org::{
    AggregateRating, AtType, Energy, ImageObject, Mass, NutritionInformation, Recipe,
};
use serde::Deserialize;

use crate::apps::helpers::{
    Ingredient, Instruction, extract_archive_contents, read_file, update_recipe_image_paths,
};
use crate::helpers::{seconds_to_duration, to_yield};
use crate::{Error, Result};

#[derive(Default)]
struct RecipeComponents<'a> {
    author: &'a str,
    categories: Vec<&'a str>,
    description: &'a str,
    ingredients: Vec<Ingredient<'a>>,
    instructions: Vec<Instruction<'a>>,
    nutrition: Vec<&'a str>,
    prep_time: &'a str,
    rating: Option<&'a str>,
    source: Option<&'a str>,
    title: &'a str,
    total_time: Option<&'a str>,
    r#yield: i64,
}

impl From<RecipeComponents<'_>> for Recipe {
    fn from(r: RecipeComponents) -> Self {
        let (category, keywords) = match r.categories.as_slice() {
            [first, rest @ ..] => (first.trim().to_string(), rest.to_vec()),
            [] => ("".into(), Vec::new()),
        };

        let rating = r
            .rating
            .and_then(|r| r.trim().split_once(' '))
            .map(|(numerator, denominator)| {
                let numerator: f32 = numerator.parse().unwrap_or_default();
                let denominator: f32 = denominator.parse().unwrap_or_default();
                numerator / denominator
            })
            .unwrap_or_default();

        let prep_secs = parse_time(r.prep_time);
        let total_secs = r.total_time.map_or(0, parse_time);
        let cook_secs = total_secs.saturating_sub(prep_secs);

        let source = r
            .source
            .map(|s| {
                let mut s = s.trim_end_matches("\"").to_string();
                if !s.starts_with("Exported from") {
                    s.push_str(" [Exported from MasterCook]");
                }
                s
            })
            .unwrap_or_default();

        let mut instructions = r
            .instructions
            .into_iter()
            .map(|s| match s {
                Instruction::Line(s) => s.to_string(),
                Instruction::Section(s) => s.to_string(),
            })
            .collect::<Vec<_>>();

        if source == "Exported from  MasterCook II" && instructions.len() == 1 {
            instructions = instructions
                .remove(0)
                .split("\n")
                .map(String::from)
                .collect::<Vec<_>>();
        }

        Self {
            r#type: Some(AtType::Recipe.to_string()),
            aggregate_rating: (rating > 0.0)
                .then(|| {
                    vec![AggregateRating {
                        r#type: Some(AtType::AggregateRating.to_string()),
                        rating_value: vec![AggregateRatingRatingValueFieldEnum::Number(rating)],
                        ..Default::default()
                    }]
                })
                .unwrap_or_default(),
            author: vec![RecipeAuthorFieldEnum::new_person(r.author.trim())],
            cook_time: seconds_to_duration(cook_secs),
            description: vec![RecipeDescriptionFieldEnum::Text(
                r.description.trim_end_matches("\"").into(),
            )],
            is_based_on: vec![RecipeIsBasedOnFieldEnum::new_creative_work_text(&source)],
            keywords: keywords
                .into_iter()
                .map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.to_string()))
                .collect(),
            name: vec![r.title.into()],
            nutrition: parse_nutrition_schema(r.nutrition),
            prep_time: seconds_to_duration(prep_secs),
            recipe_category: if category.is_empty() {
                vec![]
            } else {
                vec![category]
            },
            recipe_ingredient: r
                .ingredients
                .into_iter()
                .map(|s| match s {
                    Ingredient::Line(s) => s.to_string(),
                    Ingredient::Section(s) => s.to_string(),
                })
                .map(|s| {
                    RecipeRecipeIngredientFieldEnum::Text(
                        s.split_whitespace()
                            .collect::<Vec<_>>()
                            .join(" ")
                            .replace(" --", ","),
                    )
                })
                .collect(),
            recipe_instructions: instructions
                .into_iter()
                .map(RecipeRecipeInstructionsFieldEnum::Text)
                .collect(),
            recipe_yield: to_yield(r.r#yield),
            total_time: seconds_to_duration(total_secs),
            ..Default::default()
        }
    }
}

#[derive(Deserialize)]
struct Mx2 {
    #[serde(rename = "@source")]
    source: String,
    #[serde(rename = "@date")]
    date: String,
    #[serde(rename = "Summ")]
    summary: Summary,
    #[serde(rename = "RcpE")]
    recipes: Vec<MastercookRecipe>,
}

#[derive(Deserialize)]
struct Summary {
    #[serde(rename = "Nam")]
    name: Vec<String>,
}

#[derive(Deserialize)]
struct MastercookRecipe {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@author")]
    author: String,
    #[serde(rename = "@img")]
    img: String,
    #[serde(rename = "Serv")]
    serving: Serving,
    #[serde(rename = "PrpT")]
    prep_time: Option<PrepTime>,
    #[serde(rename = "CatS")]
    categories: Option<Categories>,
    #[serde(rename = "IngR")]
    ingredients: Vec<Ing>,
    #[serde(rename = "DirS")]
    directions: Directions,
    #[serde(rename = "Desc")]
    description: String,
    #[serde(rename = "Srce")]
    source: String,
    #[serde(rename = "Yield")]
    yield_info: Option<Yield>,
    #[serde(rename = "TTim")]
    total_time: Option<TotalTime>,
    #[serde(rename = "RatS")]
    ratings: Option<Ratings>,
    #[serde(rename = "Wine")]
    wine: Option<String>,
    #[serde(rename = "Note")]
    note: Option<String>,
    #[serde(rename = "Nutr")]
    nutrition: String,
}

#[derive(Deserialize)]
struct Serving {
    #[serde(rename = "@qty")]
    qty: String,
}

#[derive(Default, Deserialize)]
pub struct PrepTime {
    #[serde(rename = "@elapsed")]
    pub elapsed: String,
}

#[derive(Default, Deserialize)]
pub struct Categories {
    #[serde(rename = "CatT")]
    pub categories: Vec<String>,
}

#[derive(Deserialize)]
struct Ing {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@unit")]
    unit: Option<String>,
    #[serde(rename = "@qty")]
    qty: String,
    #[serde(rename = "IPrp")]
    preparation: Option<String>,
}

#[derive(Deserialize)]
struct Directions {
    #[serde(rename = "DirT")]
    directions: Vec<Direction>,
}

#[derive(Deserialize)]
struct Direction {
    #[serde(rename = "@img")]
    img: String,
    #[serde(rename = "#text")]
    text: String,
}

#[derive(Deserialize)]
struct Yield {
    #[serde(rename = "@unit")]
    unit: String,
    #[serde(rename = "@qty")]
    qty: String,
}

#[derive(Deserialize)]
struct TotalTime {
    #[serde(rename = "@elapsed")]
    elapsed: String,
}

#[derive(Deserialize)]
struct Ratings {
    #[serde(rename = "RatE")]
    rating: Rating,
}

#[derive(Deserialize)]
struct Rating {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@value")]
    pub value: String,
}

impl From<MastercookRecipe> for Recipe {
    fn from(r: MastercookRecipe) -> Self {
        let (category, keywords) = match r.categories.unwrap_or_default().categories.as_slice() {
            [first, rest @ ..] => (first.trim().to_string(), rest.to_vec()),
            [] => ("".into(), Vec::new()),
        };

        let prep_secs = parse_time(&r.prep_time.unwrap_or_default().elapsed);
        let total_secs = r.total_time.map_or(0, |t| parse_time(&t.elapsed));
        let cook_secs = total_secs.saturating_sub(prep_secs);

        let source = if r.source.is_empty() {
            "Exported from MasterCook".into()
        } else {
            let mut s = r.source;
            s.push_str(" [Exported from MasterCook]");
            s
        };

        let nutrition = r
            .nutrition
            .trim_start_matches("Per Serving (excluding unknown items): ");

        Self {
            r#type: Some(AtType::Recipe.to_string()),
            aggregate_rating: r
                .ratings
                .map(|r| {
                    let numerator: f32 = r.rating.name.parse().unwrap_or_default();
                    let denominator: f32 = r.rating.value.parse().unwrap_or_default();

                    vec![AggregateRating {
                        r#type: Some(AtType::AggregateRating.to_string()),
                        rating_value: Some(AggregateRatingRatingValueFieldEnum::Number(
                            numerator / denominator,
                        ))
                        .map(|v| vec![v])
                        .unwrap_or_default(),
                        ..Default::default()
                    }]
                })
                .unwrap_or_default(),
            author: vec![RecipeAuthorFieldEnum::new_person(&r.author)],
            cook_time: seconds_to_duration(cook_secs),
            description: vec![RecipeDescriptionFieldEnum::Text(r.description)],
            is_based_on: vec![RecipeIsBasedOnFieldEnum::new_creative_work_text(&source)],
            image: (!r.img.is_empty())
                .then_some(vec![RecipeImageFieldEnum::ImageObject(Box::new(
                    ImageObject {
                        r#type: Some(AtType::ImageObject.to_string()),
                        image: vec![ImageObjectImageFieldEnum::URL(r.img)],
                        ..Default::default()
                    },
                ))])
                .unwrap_or_default(),
            keywords: keywords
                .into_iter()
                .map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.to_string()))
                .collect(),
            name: vec![r.name],
            nutrition: parse_nutrition_schema(nutrition.split(";").collect()),
            prep_time: seconds_to_duration(prep_secs),
            recipe_category: if category.is_empty() {
                vec![]
            } else {
                vec![category]
            },
            recipe_ingredient: r
                .ingredients
                .into_iter()
                .map(|ing| {
                    RecipeRecipeIngredientFieldEnum::Text(format!(
                        "{}{}{}{}",
                        ing.qty,
                        match ing.unit {
                            None => String::new(),
                            Some(s) => format!(" {}", s),
                        },
                        if ing.name.is_empty() {
                            String::new()
                        } else {
                            format!(" {}", ing.name)
                        },
                        match ing.preparation {
                            None => String::new(),
                            Some(s) => format!(", {s}"),
                        }
                    ))
                })
                .collect(),
            recipe_instructions: r
                .directions
                .directions
                .into_iter()
                .map(|d| RecipeRecipeInstructionsFieldEnum::Text(d.text.to_string()))
                .collect(),
            recipe_yield: to_yield(r.serving.qty.parse().unwrap_or_default()),
            total_time: seconds_to_duration(total_secs),
            ..Default::default()
        }
    }
}

fn parse_time(s: &str) -> i32 {
    if s == "0:00" || s.is_empty() {
        return 0;
    }

    let mut s = s.trim_end_matches("\"").replacen(":", "m", 1);
    s.push('s');
    humantime::parse_duration(&s).unwrap_or_default().as_secs() as i32
}

fn parse_nutrition_schema(s: Vec<&str>) -> Vec<NutritionInformation> {
    let mut nutrition = NutritionInformation::default();
    s.iter().for_each(|s| {
        let (value, key) = s.trim().split_once(' ').unwrap_or_default();
        if value == "0g" || value == "0mg" {
            return;
        }

        let energy = Energy::new(value);
        let mass = Mass::new(value);

        if key == "Calories" {
            nutrition.calories = vec![energy]
        } else if key.starts_with("Fat") {
            nutrition.fat_content = vec![mass]
        } else if key == "Protein" {
            nutrition.protein_content = vec![mass]
        } else if key == "Carbohydrate" {
            nutrition.carbohydrate_content = vec![mass]
        } else if key == "Dietary Fiber" {
            nutrition.fiber_content = vec![mass]
        } else if key == "Cholesterol" {
            nutrition.cholesterol_content = vec![mass]
        } else if key == "Sodium" {
            nutrition.sodium_content = vec![mass]
        } else if key == "Total Sugars" {
            nutrition.sugar_content = vec![mass]
        }
    });

    nutrition
        .is_empty()
        .then_some(vec![nutrition])
        .unwrap_or_default()
}

/// Parses a MasterCook MX2 file.
pub fn parse_mx2<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;

    let cleaned = content
        .lines()
        .skip_while(|line| {
            let s = line.trim();
            s.starts_with("<?xml") || s.starts_with("<!DOCTYPE") || s.is_empty()
        })
        .collect::<Vec<_>>()
        .join("\n");

    let root: Mx2 =
        serde_xml_rs::from_str(&cleaned).map_err(|err| Error::Parse(err.to_string()))?;

    Ok(root.recipes.into_iter().map(Recipe::from).collect())
}

/// Parses a MasterCook MXP file.
pub fn parse_mxp<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;

    let recipes = parse_mxp_helper(&content)?
        .into_iter()
        .map(Recipe::from)
        .collect::<Vec<_>>();

    Ok(recipes)
}

/// Parses a MasterCook MZ2 file.
pub fn parse_mz2<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let archive = zip::ZipArchive::new(r)?;
    let (mut recipes, images) = extract_archive_contents(archive)?;
    update_recipe_image_paths(&mut recipes, &images);
    Ok(recipes)
}

fn parse_mxp_helper(input: &str) -> Result<Vec<RecipeComponents<'_>>> {
    Ok(many1(map(recipe_mxp, |r| r)).parse(input).map(|(_, r)| r)?)
}

fn recipe_mxp(input: &str) -> IResult<&str, RecipeComponents<'_>> {
    map(
        (
            header_mxp,
            title,
            author,
            serving_size,
            prep_time,
            categories_mxp,
            line_ending,
            ingredients,
            line_ending,
            instructions_mxp,
            flush_mxp,
        ),
        |(
            _,
            title,
            author,
            serving_size,
            prep_time,
            categories,
            _,
            ingredients,
            _,
            instructions,
            _,
        )| RecipeComponents {
            author,
            categories,
            ingredients,
            instructions,
            prep_time,
            title,
            r#yield: serving_size,
            source: Some("Exported from  MasterCook II"),
            ..Default::default()
        },
    )
    .parse(input)
}

fn header_mxp(input: &str) -> IResult<&str, &str> {
    recognize((
        space0,
        tag("*  Exported from  MasterCook II  *"),
        line_ending,
        line_ending,
    ))
    .parse(input)
}

fn categories_mxp(input: &str) -> IResult<&str, Vec<&str>> {
    map(
        delimited(
            tag("Categories    :"),
            take_until("\n\n  Amount"),
            line_ending,
        ),
        |s: &str| {
            s.split("  ")
                .filter_map(|s| match s.trim() {
                    trimmed if !trimmed.is_empty() => Some(trimmed),
                    _ => None,
                })
                .collect::<Vec<&str>>()
        },
    )
    .parse(input)
}

fn instructions_mxp(input: &str) -> IResult<&str, Vec<Instruction<'_>>> {
    map(
        take_until("- - - - - - - - - - - - - - - - - -"),
        |content: &str| {
            content
                .split("\n\n")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| Instruction::Line(Cow::Borrowed(s)))
                .collect()
        },
    )
    .parse(input)
}

fn flush_mxp(input: &str) -> IResult<&str, &str> {
    alt((take_until("*  Exported from  MasterCook II  *"), rest)).parse(input)
}

/// Parses a MasterCook TXT file.
pub fn parse_txt<R>(mut r: R) -> Result<Vec<Recipe>>
where
    R: Read,
{
    let mut content = String::new();
    r.read_to_string(&mut content)?;

    let recipes = parse_txt_helper(&content)?
        .into_iter()
        .map(Recipe::from)
        .collect::<Vec<_>>();

    Ok(recipes)
}

fn parse_txt_helper(input: &str) -> Result<Vec<RecipeComponents<'_>>> {
    Ok(many1(map(recipe_txt, |r| r)).parse(input).map(|(_, r)| r)?)
}

fn recipe_txt(input: &str) -> IResult<&str, RecipeComponents<'_>> {
    map(
        (
            header,
            title,
            author,
            serving_size,
            prep_time,
            categories,
            line_ending,
            ingredients,
            line_ending,
            instructions,
            description,
            opt(source),
            opt(servings),
            opt(total_time),
            opt(rating),
            metasection,
            nutrition,
            flush,
        ),
        |(
            _,
            title,
            author,
            serving_size,
            prep_time,
            categories,
            _,
            ingredients,
            _,
            instructions,
            description,
            source,
            _,
            total_time,
            rating,
            _,
            nutrition,
            _,
        )| RecipeComponents {
            author,
            categories,
            description,
            ingredients,
            instructions,
            nutrition,
            prep_time,
            rating,
            source,
            title,
            total_time,
            r#yield: serving_size,
        },
    )
    .parse(input)
}

fn header(input: &str) -> IResult<&str, &str> {
    recognize((
        space0,
        line_ending,
        tag("* Exported from MasterCook *"),
        line_ending,
        line_ending,
    ))
    .parse(input)
}

fn title(input: &str) -> IResult<&str, &str> {
    delimited(space1, take_until("\n"), (line_ending, line_ending)).parse(input)
}

fn author(input: &str) -> IResult<&str, &str> {
    delimited(tag("Recipe By     :"), take_until("\n"), line_ending).parse(input)
}

fn serving_size(input: &str) -> IResult<&str, i64> {
    map(
        delimited(tag("Serving Size  : "), digit1, space1),
        |s: &str| s.parse::<i64>().unwrap_or_default(),
    )
    .parse(input)
}

fn prep_time(input: &str) -> IResult<&str, &str> {
    delimited(tag("Preparation Time :"), take_until("\n"), line_ending).parse(input)
}

fn categories(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(
        tag(","),
        delimited(tag("Categories    :"), take_until("\n"), line_ending),
    )
    .parse(input)
}

fn ingredients(input: &str) -> IResult<&str, Vec<Ingredient<'_>>> {
    map(
        (
            (
                tag("  Amount  Measure       Ingredient -- Preparation Method"),
                line_ending,
            ),
            (
                tag("--------  ------------  --------------------------------"),
                line_ending,
            ),
            many1(ingredient),
        ),
        |(_, _, v)| v,
    )
    .parse(input)
}

fn ingredient(input: &str) -> IResult<&str, Ingredient<'_>> {
    map(delimited(space1, take_until("\n"), line_ending), |s| {
        Ingredient::Line(Cow::Borrowed(s))
    })
    .parse(input)
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction<'_>>> {
    map(take_until("Description:"), |content: &str| {
        content
            .split("\n\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| Instruction::Line(Cow::Borrowed(s)))
            .collect()
    })
    .parse(input)
}

fn description(input: &str) -> IResult<&str, &str> {
    delimited(
        (tag("Description:"), line_ending, tag("  \"")),
        take_until("\n"),
        line_ending,
    )
    .parse(input)
}

fn source(input: &str) -> IResult<&str, &str> {
    delimited(
        (tag("Source:"), line_ending, tag("  \"")),
        take_until("\n"),
        line_ending,
    )
    .parse(input)
}

fn servings(input: &str) -> IResult<&str, &str> {
    delimited(
        (tag("Yield:"), line_ending, tag("  \"")),
        take_until("\n"),
        line_ending,
    )
    .parse(input)
}

fn total_time(input: &str) -> IResult<&str, &str> {
    delimited(
        (tag("Start to Finish Time:"), line_ending, tag("  \"")),
        take_until("\n"),
        line_ending,
    )
    .parse(input)
}

fn rating(input: &str) -> IResult<&str, &str> {
    delimited(tag("Ratings       : "), take_until("\n"), line_ending).parse(input)
}

fn metasection(input: &str) -> IResult<&str, &str> {
    delimited(
        (opt(line_ending), space0),
        tag("- - - - - - - - - - - - - - - - - - -"),
        (space0, line_ending, line_ending),
    )
    .parse(input)
}

fn nutrition(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        tag("Per Serving (excluding unknown items): "),
        map(
            terminated(take_until("\n"), line_ending),
            |content: &str| content.split(';').map(|s| s.trim()).collect::<Vec<&str>>(),
        ),
    )
    .parse(input)
}

fn flush(input: &str) -> IResult<&str, &str> {
    alt((take_until("\n* Exported from MasterCook *"), rest)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod test_recipes {
        use super::*;
        use std::io::Cursor;

        #[test]
        fn test_mx2() -> Result<()> {
            let file = files::mx2();
            let buf = Cursor::new(file);

            let got = parse_mx2(buf)?;

            pretty_assertions::assert_eq!(got, results::txt());
            Ok(())
        }

        #[test]
        fn test_mxp() -> Result<()> {
            let file = files::mxp();
            let buf = Cursor::new(file);

            let got = parse_mxp(buf)?;

            pretty_assertions::assert_eq!(got, results::mxp());
            Ok(())
        }

        #[test]
        fn test_mz2() -> Result<()> {
            let buf = files::mz2();

            let mut got = parse_mz2(buf)?;

            let want = results::txt();
            got[0].image = want[0].image.clone();
            pretty_assertions::assert_eq!(got, want);
            Ok(())
        }

        #[test]
        fn test_txt1() -> Result<()> {
            let file = files::txt();
            let buf = Cursor::new(file);

            let got = parse_txt(buf)?;

            pretty_assertions::assert_eq!(got, results::txt());
            Ok(())
        }
    }

    mod files {
        use std::io::Cursor;
        use testing::utils::open_test_file;

        pub fn mx2<'a>() -> &'a str {
            r##"<?xml version="1.0" standalone="yes" encoding="ISO-8859-1"?>
<!DOCTYPE mx2 SYSTEM "mx2.dtd">
<mx2 source="MasterCook" date="April 14, 2025">
<Summ>
<Nam>
Best Chicken
</Nam>
<Nam>
Delicious Ramen
</Nam></Summ>
<RcpE name="Best Chicken" author="Macpoule" img="">
<Serv qty="18"/>
<PrpT elapsed="0:30"/>
<CatS>
<CatT>
Dinner
</CatT>
</CatS>
<IngR name="chicken breast" unit="pounds" qty="6">
<IPrp>
for braising meat
</IPrp>
</IngR>
<IngR name="oil" unit="tablespoons" qty="12">
<IPrp>
flaked
</IPrp>
</IngR>
<DirS>
<DirT img="">
Cut the chicken into many pieces
</DirT>
<DirT img="">
Mix it with love
</DirT>
<DirT img="">
Kiss it
</DirT>
<DirT img="">
Eat
</DirT>
</DirS>
<Desc>
The best chicken in the universe!
</Desc>
<Srce>
My mother&apos;s recipe cookbook
</Srce>
<Yield unit="cups" qty="36.000000"/>
<TTim elapsed="1:45"/>
<RatS>
<RatE name="5" value="10"/>
</RatS>
<Wine>
Columbia; Pinot Gris; 1996
</Wine>
<Note>
This recipe is extremely difficult to prepare.
</Note>
<Nutr>
Per Serving (excluding unknown items): 340 Calories; 23g Fat (62.1% calories from fat); 32g Protein; 0g Carbohydrate; 0g Dietary Fiber; 97mg Cholesterol; 95mg Sodium; 0g Total Sugars; 1mcg Vitamin D; 17mg Calcium; 1mg Iron; 333mg Potassium; 263mg Phosphorus.  Exchanges: .
</Nutr>
</RcpE>
<RcpE name="Delicious Ramen" author="Macpoule" img="">
<Serv qty="24"/>
<PrpT elapsed="0:00"/>
<IngR name="egg" qty="4"></IngR>
<IngR name="Algood Preserves, Strawberry" unit="heads" qty="12"></IngR>
<IngR name="salt" unit="teaspoon" qty="1"></IngR>
<DirS>
<DirT img="">
Cook for 12 hours
</DirT>
<DirT img="">
Make sure to boil
</DirT>
<DirT img="">
Mix all ingredients together and eat
</DirT>
</DirS>
<Desc>
Ramen has never been soooo delicious
</Desc>
<Srce>
My mother&apos;s recipe cookbook
</Srce>
<Nutr>
Per Serving (excluding unknown items): 34 Calories; 1g Fat (21.3% calories from fat); 1g Protein; 6g Carbohydrate; 0g Dietary Fiber; 31mg Cholesterol; 108mg Sodium; 4g Total Sugars; trace Vitamin D; 5mg Calcium; trace Iron; 12mg Potassium; 17mg Phosphorus.  Exchanges: .
</Nutr>
</RcpE></mx2>"##
        }

        pub fn mxp<'a>() -> &'a str {
            r##"                    *  Exported from  MasterCook II  *

                                Apple Slaw

Recipe By     :
Serving Size  : 5    Preparation Time :0:00
Categories    : Side Dish

  Amount  Measure       Ingredient -- Preparation Method
--------  ------------  --------------------------------
                        Apples -- thinly sliced
   2      tablespoons   Lemon Juice
   3      cups          Shredded Cabbage
                        Stalk Celery -- chopped
                        Carrot -- grated
                        Med Onion -- thinly sliced
     1/2  cup           Sour Cream
     1/4  cup           Mayonnaise
     3/4  teaspoon      Celery Salt

Sprinkle sliced apples with lemon juice. Mix with cabbage, celery, carrot, and onion.
Combine sour cream, mayonnaise and celery salt. Toss with apple mixture and serve.
Lemon juice keeps apples from discoloring.

                   - - - - - - - - - - - - - - - - - -


                    *  Exported from  MasterCook II  *

                            Apples and Noodles

Recipe By     :
Serving Size  : 4    Preparation Time :0:00
Categories    : Side Dish

  Amount  Measure       Ingredient -- Preparation Method
--------  ------------  --------------------------------
   2      cups          Noodles -- cooked and drained
                        Apples -- peeled and sliced
   1      dash          Cinnamon
   4      tablespoons   Brown Sugar
   4      tablespoons   Butter

Preheat oven to 350 deg F.
Place half of the noodles and apples in a buttered baking dish.
Sprinkle with half the brown sugar and a dash of cinnamon. Dot with half the butter. Repeat. Cover and bake 30 minutes. Stir well before serving (or, leave uncovered toward the end of baking to give the top a nice crunchy texture.)

                   - - - - - - - - - - - - - - - - - -


                    *  Exported from  MasterCook II  *

                           ARTICHOKES AND PEAS

Recipe By     :
Serving Size  : 4    Preparation Time :0:00
Categories    : Vegetarian                       Vegetables
                Side Dish

  Amount  Measure       Ingredient -- Preparation Method
--------  ------------  --------------------------------
   2      lg            Artichokes
   1                    Lemon (juice only)
   2      tablespoons   Virgin olive oil

lg Basil leaves; - sliced in strips 1 lg Onion, white or yellow - sliced 1/4-in thick Salt 1 lb Fresh pod peas; -=OR=- 1 c  Frozen peas Finely chopped parsley Freshly milled pepper 1 tb Sweet butter -=OR=-Extra-Virgin Olive Oil Lemon juice; to taste -=OR=- Champagne Vinegar  SLICE THE UPPER 2/3 of the leaves off the artichokes, then break off the remaining leaves, snapping them off at the base. Trim off the dark green stubs, going around the artichoke with a paring knife. Cut them in quarters, remove the fuzzy choke and slice each quarter into pieces 1/4-to-1/2-inch thick. As you work, rub the cut surfaces with lemon and put them in a bowl with the lemon juice and water to cover. Gently warm the olive oil with half the basil. When it is fairly hot, but not sizzling, add the onions and the sliced artichokes. Salt lightly and give them a stir to coat them with the oil, then add 3/4 cup water and cook over a medium-low flame. As the water cooks off, add more, in 1/2-cup increments until the artichokes are cooked, about 25 minutes. Add the peas and continue cooking until they are done. Let any liquids reduce until they are syrupy. Taste and season with salt. Add the rest of the basil, the parsley and the butter or olive oil. To brighten the flavors, stir in a little lemon juice or champagne vinegar to taste.

 DEBORAH MADISON - PRODIGY GUEST CHEFS COOKBOOK

                   - - - - - - - - - - - - - - - - - -


                    *  Exported from  MasterCook II  *

              Aunt Sadie's Fabulous Chopped Liver "Pineapple

Recipe By     :
Serving Size  : 25   Preparation Time :0:00
Categories    : Side Dish                        Appetizers
                Jewish

  Amount  Measure       Ingredient -- Preparation Method
--------  ------------  --------------------------------
   6      pounds        Fresh chicken livers
  12      each          Eggs -- hard cooked
   1      each          Large apple -- peeled
   3      each          Bermuda onions -- chopped
   1      each          Bunch celery -- (hearts only)
                        Chicken fat
   1      each          Large pineapple
   1      each          Jar of lg pimento green oliv
                        Salt and pepper to taste

I just have to pass this along to you. I don't want you to prepare this bec none of us need the bad cholesterol ingredients. I just wanted you to have pleasure of reading it. Saute liver, one chopped onion, and celery in generous amount of chicken >> fat; cooking until liver is just done, with no pink showing, but while live are still soft. Put everything through a meat grinder, adding salt and pepper to taste. Bl well, adding chicken fat if necessary. Chill well. Cut the top off the pineapple and reserve top. Mold liver on a large servi tray, copying the fresh >>>>>> pineapple shape as closely as possible. Score diamond shapes and press olive slices into each diamond.  Place pineapple top onto top of liver mold. Surround "Pineapple" with curly lettu buffet rye bread and cherry tomatoes.

                   - - - - - - - - - - - - - - - - - -


"##
        }

        pub fn mz2() -> Cursor<Vec<u8>> {
            open_test_file("integrations/mastercook1.mz2")
        }

        pub fn txt<'a>() -> &'a str {
            r##"
* Exported from MasterCook *

                               Best Chicken

Recipe By     :Macpoule
Serving Size  : 18    Preparation Time :0:30
Categories    : Dinner

  Amount  Measure       Ingredient -- Preparation Method
--------  ------------  --------------------------------
  6             pounds  chicken breast -- for braising meat
  12       tablespoons  oil -- flaked

Cut the chicken into many pieces

Mix it with love

Kiss it

Eat

Description:
  "The best chicken in the universe!"
Source:
  "My mother's recipe cookbook"
Yield:
  "36 cups"
Start to Finish Time:
  "1:45"
Ratings       : 5 10
                                    - - - - - - - - - - - - - - - - - - -

Per Serving (excluding unknown items): 340 Calories; 23g Fat (62.1% calories from fat); 32g Protein; 0g Carbohydrate; 0g Dietary Fiber; 97mg Cholesterol; 95mg Sodium; 0g Total Sugars; 1mcg Vitamin D; 17mg Calcium; 1mg Iron; 333mg Potassium; 263mg Phosphorus.  Exchanges: .

Suggested Wine: Columbia; Pinot Gris; 1996

NOTES : This recipe is extremely difficult to prepare.

Nutr. Assoc. : 0 0


* Exported from MasterCook *

                             Delicious Ramen

Recipe By     :Macpoule
Serving Size  : 24    Preparation Time :0:00
Categories    :

  Amount  Measure       Ingredient -- Preparation Method
--------  ------------  --------------------------------
  4                     egg
  12             heads  Algood Preserves, Strawberry
  1           teaspoon  salt

Cook for 12 hours

Make sure to boil

Mix all ingredients together and eat

Description:
  "Ramen has never been soooo delicious"
Source:
  "My mother's recipe cookbook"

                                    - - - - - - - - - - - - - - - - - - -

Per Serving (excluding unknown items): 34 Calories; 1g Fat (21.3% calories from fat); 1g Protein; 6g Carbohydrate; 0g Dietary Fiber; 31mg Cholesterol; 108mg Sodium; 4g Total Sugars; trace Vitamin D; 5mg Calcium; trace Iron; 12mg Potassium; 17mg Phosphorus.  Exchanges: .


Nutr. Assoc. : 0 0 0
"##
        }
    }

    mod results {
        use super::*;

        pub fn mxp() -> Vec<Recipe> {
            vec![
                Recipe {
                    r#type: Some(AtType::Recipe.to_string()),
                    is_based_on: to_is_based_on("Exported from  MasterCook II".into()),
                    name: vec!["Apple Slaw".into()],
                    recipe_category: vec!["Side Dish".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("Apples, thinly sliced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 tablespoons Lemon Juice".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3 cups Shredded Cabbage".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Stalk Celery, chopped".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Carrot, grated".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Med Onion, thinly sliced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 cup Sour Cream".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/4 cup Mayonnaise".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3/4 teaspoon Celery Salt".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Sprinkle sliced apples with lemon juice. Mix with cabbage, celery, carrot, and onion.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Combine sour cream, mayonnaise and celery salt. Toss with apple mixture and serve.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("Lemon juice keeps apples from discoloring.".into()),
                    ],
                    recipe_yield: to_yield(5),
                    ..Default::default()
                },
                Recipe {
                    r#type: Some(AtType::Recipe.to_string()),
                    is_based_on: to_is_based_on("Exported from  MasterCook II".into()),
                    name: vec!["Apples and Noodles".into()],
                    recipe_category: RecipeCategory::Text("Side Dish".into()),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("2 cups Noodles, cooked and drained".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Apples, peeled and sliced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 dash Cinnamon".into()),
                        RecipeRecipeIngredientFieldEnum::Text("4 tablespoons Brown Sugar".into()),
                        RecipeRecipeIngredientFieldEnum::Text("4 tablespoons Butter".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("Preheat oven to 350 deg F.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Place half of the noodles and apples in a buttered baking dish.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Sprinkle with half the brown sugar and a dash of cinnamon. Dot with half the butter. Repeat. Cover and bake 30 minutes. Stir well before serving (or, leave uncovered toward the end of baking to give the top a nice crunchy texture.)".into(),
                        ),
                    ],
                    recipe_yield: to_yield(4),
                    ..Default::default()
                },
                Recipe {
                    r#type: Some(AtType::Recipe.to_string()),
                    is_based_on: to_is_based_on("Exported from  MasterCook II".into()),
                    name: vec!["ARTICHOKES AND PEAS".into()],
                    keywords: to_defined_text(["Vegetables", "Side Dish"].join(",")),
                    recipe_category: RecipeCategory::Text("Vegetarian".into()),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("2 lg Artichokes".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Lemon (juice only)".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 tablespoons Virgin olive oil".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "lg Basil leaves; - sliced in strips 1 lg Onion, white or yellow - sliced 1/4-in thick Salt 1 lb Fresh pod peas; -=OR=- 1 c  Frozen peas Finely chopped parsley Freshly milled pepper 1 tb Sweet butter -=OR=-Extra-Virgin Olive Oil Lemon juice; to taste -=OR=- Champagne Vinegar  SLICE THE UPPER 2/3 of the leaves off the artichokes, then break off the remaining leaves, snapping them off at the base. Trim off the dark green stubs, going around the artichoke with a paring knife. Cut them in quarters, remove the fuzzy choke and slice each quarter into pieces 1/4-to-1/2-inch thick. As you work, rub the cut surfaces with lemon and put them in a bowl with the lemon juice and water to cover. Gently warm the olive oil with half the basil. When it is fairly hot, but not sizzling, add the onions and the sliced artichokes. Salt lightly and give them a stir to coat them with the oil, then add 3/4 cup water and cook over a medium-low flame. As the water cooks off, add more, in 1/2-cup increments until the artichokes are cooked, about 25 minutes. Add the peas and continue cooking until they are done. Let any liquids reduce until they are syrupy. Taste and season with salt. Add the rest of the basil, the parsley and the butter or olive oil. To brighten the flavors, stir in a little lemon juice or champagne vinegar to taste.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("DEBORAH MADISON - PRODIGY GUEST CHEFS COOKBOOK".into()),
                    ],
                    recipe_yield: to_yield(4),
                    ..Default::default()
                },
                Recipe {
                    r#type: Some(AtType::Recipe.to_string()),
                    is_based_on: to_is_based_on("Exported from  MasterCook II".into()),
                    keywords: to_defined_text(["Appetizers", "Jewish"].join(",")),
                    name: vec![r#"Aunt Sadie's Fabulous Chopped Liver "Pineapple"#.into()],
                    recipe_category: RecipeCategory::Text("Side Dish".into()),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("6 pounds Fresh chicken livers".into()),
                        RecipeRecipeIngredientFieldEnum::Text("12 each Eggs, hard cooked".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 each Large apple, peeled".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3 each Bermuda onions, chopped".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 each Bunch celery, (hearts only)".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Chicken fat".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 each Large pineapple".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 each Jar of lg pimento green oliv".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Salt and pepper to taste".into()),
                    ],
                    recipe_instructions: vec![RecipeRecipeInstructionsFieldEnum::Text(
                        r#"I just have to pass this along to you. I don't want you to prepare this bec none of us need the bad cholesterol ingredients. I just wanted you to have pleasure of reading it. Saute liver, one chopped onion, and celery in generous amount of chicken >> fat; cooking until liver is just done, with no pink showing, but while live are still soft. Put everything through a meat grinder, adding salt and pepper to taste. Bl well, adding chicken fat if necessary. Chill well. Cut the top off the pineapple and reserve top. Mold liver on a large servi tray, copying the fresh >>>>>> pineapple shape as closely as possible. Score diamond shapes and press olive slices into each diamond.  Place pineapple top onto top of liver mold. Surround "Pineapple" with curly lettu buffet rye bread and cherry tomatoes."#.into(),
                    )],
                    recipe_yield: to_yield(25),
                    ..Default::default()
                },
            ]
        }

        pub fn txt() -> Vec<Recipe> {
            vec![
                Recipe {
                    r#type: Some(AtType::Recipe.to_string()),
                    author: vec![RecipeAuthorFieldEnum::new_person("Macpoule")],
                    aggregate_rating: vec![AggregateRating {
                        r#type: Some(AtType::AggregateRating.to_string()),
                        rating_value: vec![AggregateRatingRatingValueFieldEnum::Number(0.5)],
                        ..Default::default()
                    }],
                    cook_time: seconds_to_duration(4500),
                    description: vec![RecipeDescriptionFieldEnum::Text(
                        "The best chicken in the universe!".into(),
                    )],
                    is_based_on: to_is_based_on(
                        "My mother's recipe cookbook [Exported from MasterCook]".into(),
                    ),
                    name: vec!["Best Chicken".into()],
                    nutrition: vec![NutritionInformation {
                        r#type: Some(AtType::NutritionInformation.to_string()),
                        calories: vec![Energy::new("340")],
                        cholesterol_content: vec![Mass::new("97mg")],
                        fat_content: vec![Mass::new("23g")],
                        protein_content: vec![Mass::new("32g")],
                        sodium_content: vec![Mass::new("95mg")],
                        ..Default::default()
                    }],
                    prep_time: seconds_to_duration(30 * 60),
                    recipe_category: vec!["Dinner".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text(
                            "6 pounds chicken breast, for braising meat".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text("12 tablespoons oil, flaked".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Cut the chicken into many pieces".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("Mix it with love".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Kiss it".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Eat".into()),
                    ],
                    recipe_yield: to_yield(18),
                    total_time: seconds_to_duration(6300),
                    ..Default::default()
                },
                Recipe {
                    r#type: Some(AtType::Recipe.to_string()),
                    author: vec![RecipeAuthorFieldEnum::new_person("Macpoule".into())],
                    description: vec![RecipeDescriptionFieldEnum::Text(
                        "Ramen has never been soooo delicious".into(),
                    )],
                    is_based_on: to_is_based_on(
                        "My mother's recipe cookbook [Exported from MasterCook]".into(),
                    ),
                    name: vec!["Delicious Ramen".into()],
                    nutrition: vec![NutritionInformation {
                        r#type: Some(AtType::NutritionInformation.to_string()),
                        calories: vec![Energy::new("34")],
                        carbohydrate_content: vec![Mass::new("6g")],
                        cholesterol_content: vec![Mass::new("31mg")],
                        fat_content: vec![Mass::new("1g")],
                        protein_content: vec![Mass::new("1g")],
                        sodium_content: vec![Mass::new("108mg")],
                        sugar_content: vec![Mass::new("4g")],
                        ..Default::default()
                    }],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("4 egg".into()),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "12 heads Algood Preserves, Strawberry".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text("1 teaspoon salt".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("Cook for 12 hours".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Make sure to boil".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Mix all ingredients together and eat".into(),
                        ),
                    ],
                    recipe_yield: to_yield(24),
                    ..Default::default()
                },
            ]
        }
    }
}
