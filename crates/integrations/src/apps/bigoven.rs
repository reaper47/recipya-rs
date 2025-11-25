use std::fmt::Display;
use std::io::{Read, Seek};

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::line_ending;
use nom::combinator::{map, map_res, opt};
use nom::multi::many1;
use nom::sequence::{delimited, preceded, terminated};
use nom::{IResult, Parser};
use url::Url;

use schema_org::field::{
    AggregateRatingRatingValueFieldEnum, ItemListItemListElementFieldEnum, RecipeKeywordsFieldEnum,
    RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
};
use schema_org::{AggregateRating, AtType, ItemList, Recipe};

use super::helpers::read_file;
use crate::Result;
use crate::helpers::{seconds_to_duration, to_yield};

#[allow(dead_code)]
struct BigOvenRecipe {
    title: String,
    servings: f32,
    source: Option<String>,
    taste_rating: u8,
    effort_rating: u8,
    appearance_rating: u8,
    affordability_rating: u8,
    ingredients: Vec<RecipeRecipeIngredientFieldEnum>,
    instructions: Vec<String>,
    active_minutes: u16,
    total_minutes: u16,
    category: Option<String>,
    keywords: Vec<String>,
}

struct RecipeComponents<'a> {
    title: &'a str,
    servings: f32,
    source: Option<&'a str>,
    taste_rating: u8,
    effort_rating: u8,
    appearance_rating: u8,
    affordability_rating: u8,
    ingredients: Vec<IngredientType<'a>>,
    instructions: Vec<&'a str>,
    active_minutes: u16,
    total_minutes: u16,
    categories: Vec<&'a str>,
}

#[allow(dead_code)]
enum IngredientType<'a> {
    Line(Ingredient<'a>),
    Section(Ingredient<'a>),
}

#[allow(dead_code)]
struct Ingredient<'a> {
    heading: u16,
    quantity: Option<&'a str>,
    measure: Option<&'a str>,
    name: &'a str,
    prep_notes: Option<&'a str>,
    double_qty: f32,
    line_order: u16,
    nutrient_info: u16,
    base_recipe_gmwt: f32,
}

impl Display for Ingredient<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!(
            "{} {} {}",
            self.quantity.map(|i| i.to_string()).unwrap_or_default(),
            self.measure.unwrap_or_default(),
            self.name
        )
        .replace("  ", " ");

        write!(f, "{s}")
    }
}

impl From<RecipeComponents<'_>> for BigOvenRecipe {
    fn from(r: RecipeComponents<'_>) -> Self {
        let items = r.categories.split_first();

        Self {
            title: r.title.into(),
            servings: r.servings,
            source: r.source.map(String::from),
            taste_rating: r.taste_rating,
            effort_rating: r.effort_rating,
            appearance_rating: r.appearance_rating,
            affordability_rating: r.affordability_rating,
            ingredients: transform_ingredient_types(&r.ingredients),
            instructions: r.instructions.into_iter().map(String::from).collect(),
            active_minutes: r.active_minutes,
            total_minutes: r.total_minutes,
            category: items.map(|(a, _b)| a.to_string()),
            keywords: items
                .map(|(_a, b)| b.iter().map(|s| s.to_string()).collect())
                .unwrap_or_default(),
        }
    }
}

fn transform_ingredient_types<'a>(
    input: &'a [IngredientType<'a>],
) -> Vec<RecipeRecipeIngredientFieldEnum> {
    if input
        .iter()
        .any(|v| matches!(v, IngredientType::Section(_)))
    {
        let mut result = Vec::new();
        let mut current_section = None;
        let mut items = Vec::new();

        for item in input {
            match item {
                IngredientType::Line(ing) => {
                    let s = format!(
                        "{} {} {}",
                        ing.quantity.map(|i| i.to_string()).unwrap_or_default(),
                        ing.measure.unwrap_or_default(),
                        ing.name
                    )
                    .replace("  ", " ");

                    if current_section.is_some() {
                        items.push(s)
                    }
                }
                IngredientType::Section(ing) => {
                    if let Some(section) = current_section {
                        result.push((section, items.clone()));
                        items.clear();
                    }
                    current_section = Some(ing.name.to_string());
                }
            }
        }

        if let Some(section) = current_section {
            result.push((section, items))
        }

        result
            .into_iter()
            .map(|(section, items)| {
                let num_items = items.len();

                RecipeRecipeIngredientFieldEnum::ItemList(ItemList {
                    item_list_element: items
                        .into_iter()
                        .map(ItemListItemListElementFieldEnum::Text)
                        .collect(),
                    name: vec![section.trim_matches('-').trim().to_string()],
                    number_of_items: vec![num_items as i32],
                    ..Default::default()
                })
            })
            .collect()
    } else {
        let flat: Vec<&Ingredient> = input
            .iter()
            .filter_map(|v| match v {
                IngredientType::Line(ing) => Some(ing),
                _ => None,
            })
            .collect();

        flat.iter()
            .map(|ing| {
                let s = format!(
                    "{} {} {}",
                    ing.quantity.map(|i| i.to_string()).unwrap_or_default(),
                    ing.measure.unwrap_or_default(),
                    ing.name
                )
                .replace("  ", " ");

                RecipeRecipeIngredientFieldEnum::Text(s)
            })
            .collect()
    }
}

impl From<BigOvenRecipe> for Recipe {
    fn from(r: BigOvenRecipe) -> Self {
        let url = Url::parse(&r.source.clone().unwrap_or_default()).ok();

        Recipe {
            r#type: Some(AtType::Recipe.to_string()),
            aggregate_rating: if r.taste_rating > 0 {
                vec![AggregateRating {
                    r#type: Some(AtType::AggregateRating.to_string()),
                    rating_value: vec![AggregateRatingRatingValueFieldEnum::Number(
                        r.taste_rating as f32,
                    )],
                    ..Default::default()
                }]
            } else {
                vec![]
            },
            cook_time: seconds_to_duration((r.total_minutes - r.active_minutes) as i32),
            keywords: r
                .keywords
                .into_iter()
                .map(RecipeKeywordsFieldEnum::TextOrURL)
                .collect(),
            name: Some(r.title)
                .filter(|s| !s.is_empty())
                .into_iter()
                .collect(),
            prep_time: seconds_to_duration(r.active_minutes as i32),
            recipe_category: r.category.into_iter().collect(),
            recipe_ingredient: r.ingredients,
            recipe_instructions: r
                .instructions
                .into_iter()
                .map(RecipeRecipeInstructionsFieldEnum::Text)
                .collect(),
            recipe_yield: to_yield(r.servings.round() as i64),
            url: url.map(String::from).into_iter().collect(),
            ..Default::default()
        }
    }
}

/// Parses a BigOven text file recipe.
pub fn parse<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;
    Ok(vec![parse_text_file(&content)?])
}

fn parse_text_file(input: &str) -> Result<Recipe> {
    Ok(map(recipe, BigOvenRecipe::from)
        .parse(input)
        .map(|(_, r)| r.into())?)
}

fn recipe(input: &str) -> IResult<&str, RecipeComponents<'_>> {
    map(
        (
            header,
            title,
            servings,
            instructions,
            source,
            taste_rating,
            effort_rating,
            appearance_rating,
            affordability_rating,
            ingredients,
            active_minutes,
            total_minutes,
            categories,
        ),
        |(
            _,
            title,
            servings,
            instructions,
            source,
            taste_rating,
            effort_rating,
            appearance_rating,
            affordability_rating,
            ingredients,
            active_minutes,
            total_minutes,
            categories,
        )| {
            RecipeComponents {
                title,
                servings,
                source,
                taste_rating,
                effort_rating,
                appearance_rating,
                affordability_rating,
                ingredients,
                instructions,
                active_minutes,
                total_minutes,
                categories,
            }
        },
    )
    .parse(input)
}

fn header(input: &str) -> IResult<&str, &str> {
    take_until("<RECIPE>").parse(input)
}

fn title(input: &str) -> IResult<&str, &str> {
    parse_str("<TITLE>", "</TITLE>", "<TITLE/>").parse(input)
}

fn servings(input: &str) -> IResult<&str, f32> {
    parse_f32("<YIELDQTY>", "</YIELDQTY>", "<YIELDQTY/").parse(input)
}

fn instructions(input: &str) -> IResult<&str, Vec<&str>> {
    map(
        parse_str("<INSTRUCTIONS>", "</INSTRUCTIONS>", "<INSTRUCTIONS/>"),
        |s: &str| {
            s.lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
        },
    )
    .parse(input)
}

fn source(input: &str) -> IResult<&str, Option<&str>> {
    opt(parse_str("<SUBHEAD>", "</SUBHEAD>", "<SUBHEAD/>")).parse(input)
}

fn effort_rating(input: &str) -> IResult<&str, u8> {
    parse_u8("<EFFORTRATING>", "</EFFORTRATING>", "EFFORTRATING/>").parse(input)
}

fn appearance_rating(input: &str) -> IResult<&str, u8> {
    parse_u8(
        "<APPEARANCERATING>",
        "</APPEARANCERATING>",
        "<APPEARANCERATING/>",
    )
    .parse(input)
}

fn taste_rating(input: &str) -> IResult<&str, u8> {
    parse_u8("<TASTERATING>", "</TASTERATING>", "<TASTERATING/>").parse(input)
}

fn affordability_rating(input: &str) -> IResult<&str, u8> {
    parse_u8(
        "<AFFORDABLERATING>",
        "</AFFORDABLERATING>",
        "<AFFORDABLERATING/>",
    )
    .parse(input)
}

fn ingredients(input: &str) -> IResult<&str, Vec<IngredientType<'_>>> {
    delimited(
        (tag("<INGREDIENTLIST>"), line_ending),
        many1(map(
            (
                (tag("<INGREDIENTLINE>"), line_ending),
                parse_u16("<BHEADING>", "</BHEADING>", "<BHEADING/>"),
                opt(parse_str("<TEXTQTY>", "</TEXTQTY>", "<TEXTQTY/>")),
                opt(parse_str("<MEASURE>", "</MEASURE>", "<MEASURE/>")),
                parse_str("<NAME>", "</NAME>", "<NAME/>"),
                opt(parse_str("<PREPNOTES>", "</PREPNOTES>", "<PREPNOTES/>")),
                parse_f32("<DBLQTY>", "</DBLQTY>", "<DBLQTY/>"),
                parse_u16("<LINEORDER>", "</LINEORDER>", "<LINEORDER/>"),
                parse_u16("<NUTRIENTID>", "</NUTRIENTID>", "<NUTRIENTID/>"),
                parse_f32(
                    "<DBBASERECIPEGMWT>",
                    "</DBBASERECIPEGMWT>",
                    "<DBBASERECIPEGMWT/>",
                ),
                (tag("</INGREDIENTLINE>"), line_ending),
            ),
            |(
                _,
                heading,
                quantity,
                measure,
                name,
                prep_notes,
                dbl_quantity,
                line_order,
                nutrient_info,
                gmnt,
                _,
            )| {
                let ingredient = Ingredient {
                    heading,
                    quantity,
                    measure,
                    name,
                    prep_notes,
                    double_qty: dbl_quantity,
                    line_order,
                    nutrient_info,
                    base_recipe_gmwt: gmnt,
                };

                if name.starts_with("--") {
                    IngredientType::Section(ingredient)
                } else {
                    IngredientType::Line(ingredient)
                }
            },
        )),
        (tag("</INGREDIENTLIST>"), line_ending),
    )
    .parse(input)
}

fn active_minutes(input: &str) -> IResult<&str, u16> {
    parse_u16("<ACTIVEMINUTES>", "</ACTIVEMINUTES>", "<ACTIVEMINUTES/>").parse(input)
}

fn total_minutes(input: &str) -> IResult<&str, u16> {
    parse_u16("<TOTALMINUTES>", "</TOTALMINUTES>", "<TOTALMINUTES/>").parse(input)
}

fn categories(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        (tag("<CATEGORIES>"), line_ending),
        many1(parse_str(
            "<CATEGORYNAME>",
            "</CATEGORYNAME>",
            "<CATEGORYNAME/>",
        )),
        (tag("</CATEGORIES>"), line_ending),
    )
    .parse(input)
}

fn parse_str(
    start_tag: &str,
    closing_tag: &str,
    empty_tag: &str,
) -> impl FnMut(&str) -> IResult<&str, &str> {
    move |input| parse_tag(start_tag, closing_tag, empty_tag).parse(input)
}

fn parse_f32(
    start_tag: &str,
    closing_tag: &str,
    empty_tag: &str,
) -> impl FnMut(&str) -> IResult<&str, f32> {
    move |input| {
        map_res(parse_tag(start_tag, closing_tag, empty_tag), |s| {
            s.parse::<f32>()
        })
        .parse(input)
    }
}

fn parse_u8(
    start_tag: &str,
    closing_tag: &str,
    empty_tag: &str,
) -> impl FnMut(&str) -> IResult<&str, u8> {
    move |input| {
        map_res(parse_tag(start_tag, closing_tag, empty_tag), |s| {
            s.parse::<u8>()
        })
        .parse(input)
    }
}

fn parse_u16(
    start_tag: &str,
    closing_tag: &str,
    empty_tag: &str,
) -> impl FnMut(&str) -> IResult<&str, u16> {
    move |input| {
        map_res(parse_tag(start_tag, closing_tag, empty_tag), |s| {
            s.parse::<u16>()
        })
        .parse(input)
    }
}

fn parse_tag(
    start_tag: &str,
    closing_tag: &str,
    empty_tag: &str,
) -> impl FnMut(&str) -> IResult<&str, &str> {
    move |input| {
        alt((
            map((tag(empty_tag), line_ending), |_| ""),
            preceded(
                (take_until(start_tag), tag(start_tag)),
                terminated(take_until(closing_tag), (tag(closing_tag), line_ending)),
            ),
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_recipes {
        use super::*;
        use std::io::Cursor;

        #[test]
        fn test_recipe1_ok() -> Result<()> {
            let file = files::recipe1();
            let buf = Cursor::new(&file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, vec![results::recipe1()]);
            Ok(())
        }
    }

    mod files {
        pub fn recipe1<'a>() -> &'a str {
            r##"<html>

<head>
<meta http-equiv="Content-Language" content="en-us">
<meta name="GENERATOR" content="Microsoft FrontPage 12.0">
<meta name="ProgId" content="FrontPage.Editor.Document">
<meta http-equiv="Content-Type" content="text/html; charset=windows-1252">
<title>Applebee's Oriental Chicken Salad</title>
<style fprolloverstyle>A:hover {color: #FF0000; text-decoration: underline}
</style>
<style>
<!--
a:link       { text-decoration: none; color: #000000 }
a:hover	     { text-decoration: underline;  color: #FF0000 }
a:visited    { text-decoration: none; color: #000000 }
-->
</style>
</head>

<body oncontextmenu='return false' link="#000000" bgproperties="fixed">

  <table border="1" cellpadding="0" cellspacing="0" style="border-collapse: collapse; border-width: 0; valign:top; align:top" bordercolor="#111111" width="100%" background="theme0back.gif" valign="top">
    <tr>
      <td width="100%" style="border-left:medium none #006600; border-right:medium none #006600; border-top-style:none; border-top-width:medium; border-bottom-color:#008000" align="center">
  <div style="border-top: 1px solid #008000">
  <p style="margin-top: 0; margin-bottom: 0; " align="left">
  <font face="Wingdings" size="5">s</font><b><font face="Garamond" size="5"> Applebee's Oriental Chicken Salad </font></b><font face="Wingdings" size="5">
  s </font><IMG SRC='images\star0.gif' BORDER=0>&nbsp;&nbsp; </p>
      </div>
      </td>
    </tr>
  </table>
<table border="1" cellpadding="0" cellspacing="0" style="border-collapse: collapse; border-width: 0" bordercolor="#111111" width="100%" background="theme0back.gif">
  <tr>
    <td width="100%" style="border-style: none; border-width: medium" valign="top">
    <p style="margin-top: 0; margin-bottom: 5">
    <font face="Arial Narrow" size="1"><br>
    </font><i><font size="2">http://www.food.com/recipe/tsr-version-of-applebees-oriental-chicken-salad-by-todd-wilbur-19253?ftab=reviews<br>
    </font>
    </i><font size="2">             Yields: 4 Servings<BR></font><i><br>
    </i>
    <font face="Arial Narrow" size="1"><br>
    <IMG SRC='images\296_Applebee_s Oriental Chicken Salad.jpg'></font><p style="margin-top: 5; margin-bottom: 5">
    <span style="font-variant: small-caps; letter-spacing: 4">
    <font face="Arial Narrow" size="2" color="#808080">Ingredients</font></span><p style="margin-top: 0; margin-bottom: 3">
    <font face="Arial Narrow" size="2">
      -- Salad Dressing -- <BR>
6 tablespoon Honey <BR>
3 tablespoon Rice wine vinegar <BR>
1/2 cup Mayonnaise <BR>
2 teaspoon Grey Poupon Dijon Mustard <BR>
1/4 teaspoon Sesame Oil <BR>
  -- Salad -- <BR>
1 package Breaded Chicken Tenders <BR>
6 cups Romaine lettuce <BR>
2 cup Red cabbage <BR>
1  Carrot <BR>
2  Green Onion <BR>
2 tablespoon Sliced almonds <BR>
1 cup Chow mein noodles <BR>
</FONT><!--@BOBEGIN--<RECIPE>
<TITLE>Applebee's Oriental Chicken Salad</TITLE>
<YIELDQTY>4.00</YIELDQTY>
<INSTRUCTIONS>Below assumes you are making your own chicken tenders, whereas the recipe above uses pre-packaged chicken tenders.
Directions

    Preheat oil in deep fryer or deep pan over medium heat.
    You want the temperature of the oil to be around 350 degrees.
    Blend together all ingredients for dressing in a small bowl with an electric mixer.
    Put dressing in refrigerator to chill while you prepare the salad.
    In a small, shallow bowl beat egg, add milk, and mix well.
    In another bowl, combine flour with corn flake crumbs, salt and pepper.
    Cut chicken breast into 4 or 5 long strips.
    Dip each strip of chicken first into egg mixture then into the flour mixture, coating each piece completely.
    Fry each chicken finger for 5 minutes or until coating has darkened to brown.
    Prepare salad by tossing the chopped romaine with the chopped red cabbage, Napa cabbage, and carrots.
    Sprinkle sliced green onion on top of the lettuce.
    Sprinkle almonds over the salad, then the chow mein noodles.
    Cut the chicken into small bite-size chunks.
    Place the chicken onto the salad forming a pile in the middle.
    Serve with salad dressing drizzled over it or on the side.</INSTRUCTIONS>
<CUISINE>Chinese</CUISINE>
<MAININGREDIENT/>
<YIELDTEXT>4 Servings</YIELDTEXT>
<YIELDUNIT>Servings</YIELDUNIT>
<SUBHEAD>http://www.food.com/recipe/tsr-version-of-applebees-oriental-chicken-salad-by-todd-wilbur-19253?ftab=reviews</SUBHEAD>
<VARIATIONS/>
<TASTERATING>0</TASTERATING>
<EFFORTRATING>0</EFFORTRATING>
<APPEARANCERATING>0</APPEARANCERATING>
<AFFORDABLERATING>0</AFFORDABLERATING>
<INGREDIENTLIST>
<INGREDIENTLINE>
<BHEADING>1</BHEADING>
<TEXTQTY/>
<MEASURE/>
<NAME>-- Salad Dressing --</NAME>
<PREPNOTES/>
<DBLQTY>0.0000</DBLQTY>
<LINEORDER>1</LINEORDER>
<NUTRIENTID>0</NUTRIENTID>
<DBBASERECIPEGMWT>0.000000</DBBASERECIPEGMWT>
</INGREDIENTLINE>
<INGREDIENTLINE>
<BHEADING>0</BHEADING>
<TEXTQTY>6</TEXTQTY>
<MEASURE>tablespoon</MEASURE>
<NAME>Honey</NAME>
<PREPNOTES/>
<DBLQTY>6.0000</DBLQTY>
<LINEORDER>2</LINEORDER>
<NUTRIENTID>19296</NUTRIENTID>
<DBBASERECIPEGMWT>127.125000</DBBASERECIPEGMWT>
</INGREDIENTLINE>
<INGREDIENTLINE>
<BHEADING>0</BHEADING>
<TEXTQTY>3</TEXTQTY>
<MEASURE>tablespoon</MEASURE>
<NAME>Rice wine vinegar</NAME>
<PREPNOTES/>
<DBLQTY>3.0000</DBLQTY>
<LINEORDER>3</LINEORDER>
<NUTRIENTID>0</NUTRIENTID>
<DBBASERECIPEGMWT>0.000000</DBBASERECIPEGMWT>
</INGREDIENTLINE>
<INGREDIENTLINE>
<BHEADING>0</BHEADING>
<TEXTQTY>1/2</TEXTQTY>
<MEASURE>cup</MEASURE>
<NAME>Mayonnaise</NAME>
<PREPNOTES/>
<DBLQTY>0.5000</DBLQTY>
<LINEORDER>4</LINEORDER>
<NUTRIENTID>4018</NUTRIENTID>
<DBBASERECIPEGMWT>117.500000</DBBASERECIPEGMWT>
</INGREDIENTLINE>
<INGREDIENTLINE>
<BHEADING>0</BHEADING>
<TEXTQTY>2</TEXTQTY>
<MEASURE>teaspoon</MEASURE>
<NAME>Grey Poupon Dijon Mustard</NAME>
<PREPNOTES/>
<DBLQTY>2.0000</DBLQTY>
<LINEORDER>5</LINEORDER>
<NUTRIENTID>0</NUTRIENTID>
<DBBASERECIPEGMWT>0.000000</DBBASERECIPEGMWT>
</INGREDIENTLINE>
<INGREDIENTLINE>
<BHEADING>0</BHEADING>
<TEXTQTY>1/4</TEXTQTY>
<MEASURE>teaspoon</MEASURE>
<NAME>Sesame Oil</NAME>
<PREPNOTES/>
<DBLQTY>0.2500</DBLQTY>
<LINEORDER>6</LINEORDER>
<NUTRIENTID>0</NUTRIENTID>
<DBBASERECIPEGMWT>0.000000</DBBASERECIPEGMWT>
</INGREDIENTLINE>
<INGREDIENTLINE>
<BHEADING>1</BHEADING>
<TEXTQTY/>
<MEASURE/>
<NAME>-- Salad --</NAME>
<PREPNOTES/>
<DBLQTY>0.0000</DBLQTY>
<LINEORDER>7</LINEORDER>
<NUTRIENTID>0</NUTRIENTID>
<DBBASERECIPEGMWT>0.000000</DBBASERECIPEGMWT>
</INGREDIENTLINE>
<INGREDIENTLINE>
<BHEADING>0</BHEADING>
<TEXTQTY>1</TEXTQTY>
<MEASURE>package</MEASURE>
<NAME>Breaded Chicken Tenders</NAME>
<PREPNOTES/>
<DBLQTY>1.0000</DBLQTY>
<LINEORDER>8</LINEORDER>
<NUTRIENTID>0</NUTRIENTID>
<DBBASERECIPEGMWT>0.000000</DBBASERECIPEGMWT>
</INGREDIENTLINE>
<INGREDIENTLINE>
<BHEADING>0</BHEADING>
<TEXTQTY>6</TEXTQTY>
<MEASURE>cups</MEASURE>
<NAME>Romaine lettuce</NAME>
<PREPNOTES/>
<DBLQTY>6.0000</DBLQTY>
<LINEORDER>9</LINEORDER>
<NUTRIENTID>0</NUTRIENTID>
<DBBASERECIPEGMWT>0.000000</DBBASERECIPEGMWT>
</INGREDIENTLINE>
<INGREDIENTLINE>
<BHEADING>0</BHEADING>
<TEXTQTY>2</TEXTQTY>
<MEASURE>cup</MEASURE>
<NAME>Red cabbage</NAME>
<PREPNOTES/>
<DBLQTY>2.0000</DBLQTY>
<LINEORDER>10</LINEORDER>
<NUTRIENTID>0</NUTRIENTID>
<DBBASERECIPEGMWT>0.000000</DBBASERECIPEGMWT>
</INGREDIENTLINE>
<INGREDIENTLINE>
<BHEADING>0</BHEADING>
<TEXTQTY>1</TEXTQTY>
<MEASURE/>
<NAME>Carrot</NAME>
<PREPNOTES/>
<DBLQTY>1.0000</DBLQTY>
<LINEORDER>11</LINEORDER>
<NUTRIENTID>0</NUTRIENTID>
<DBBASERECIPEGMWT>0.000000</DBBASERECIPEGMWT>
</INGREDIENTLINE>
<INGREDIENTLINE>
<BHEADING>0</BHEADING>
<TEXTQTY>2</TEXTQTY>
<MEASURE/>
<NAME>Green Onion</NAME>
<PREPNOTES/>
<DBLQTY>2.0000</DBLQTY>
<LINEORDER>12</LINEORDER>
<NUTRIENTID>0</NUTRIENTID>
<DBBASERECIPEGMWT>0.000000</DBBASERECIPEGMWT>
</INGREDIENTLINE>
<INGREDIENTLINE>
<BHEADING>0</BHEADING>
<TEXTQTY>2</TEXTQTY>
<MEASURE>tablespoon</MEASURE>
<NAME>Sliced almonds</NAME>
<PREPNOTES/>
<DBLQTY>2.0000</DBLQTY>
<LINEORDER>13</LINEORDER>
<NUTRIENTID>0</NUTRIENTID>
<DBBASERECIPEGMWT>0.000000</DBBASERECIPEGMWT>
</INGREDIENTLINE>
<INGREDIENTLINE>
<BHEADING>0</BHEADING>
<TEXTQTY>1</TEXTQTY>
<MEASURE>cup</MEASURE>
<NAME>Chow mein noodles</NAME>
<PREPNOTES/>
<DBLQTY>1.0000</DBLQTY>
<LINEORDER>14</LINEORDER>
<NUTRIENTID>0</NUTRIENTID>
<DBBASERECIPEGMWT>0.000000</DBBASERECIPEGMWT>
</INGREDIENTLINE>
</INGREDIENTLIST>
<ACTIVEMINUTES>0</ACTIVEMINUTES>
<TOTALMINUTES>0</TOTALMINUTES>
<CATEGORIES>
<CATEGORYNAME>Low Carb</CATEGORYNAME>
<CATEGORYNAME>Low Fat</CATEGORYNAME>
<CATEGORYNAME>Summer</CATEGORYNAME>
<CATEGORYNAME>Spring</CATEGORYNAME>
<CATEGORYNAME>Vegetables</CATEGORYNAME>
<CATEGORYNAME>Salads</CATEGORYNAME>
<CATEGORYNAME>Main Dish</CATEGORYNAME>
</CATEGORIES>
<UNIQUEID>0</UNIQUEID>
</RECIPE>
@BOEND--></font></td>
    <td width="1" style="border-style: none; border-width: medium" valign="top">
    <font face="Arial Narrow"><br>
    &nbsp;</font></td>
  </tr>
  <tr>
    <td width="100%" style="border-style: none; border-width: medium" valign="top" colspan="2">
<font face="Tahoma">

  <span style="font-variant: small-caps; letter-spacing: 4">
  <font face="Arial Narrow" color="#808080"><font size="2">Instructions</font><br>
  </font></span>
</font>
<font face="Arial Narrow" size="2">

Below assumes you are making your own chicken tenders, whereas the recipe above uses pre-packaged chicken tenders.<BR>Directions<BR><BR>    Preheat oil in deep fryer or deep pan over medium heat.<BR>    You want the temperature of the oil to be around 350 degrees.<BR>    Blend together all ingredients for dressing in a small bowl with an electric mixer.<BR>    Put dressing in refrigerator to chill while you prepare the salad.<BR>    In a small, shallow bowl beat egg, add milk, and mix well.<BR>    In another bowl, combine flour with corn flake crumbs, salt and pepper.<BR>    Cut chicken breast into 4 or 5 long strips.<BR>    Dip each strip of chicken first into egg mixture then into the flour mixture, coating each piece completely.<BR>    Fry each chicken finger for 5 minutes or until coating has darkened to brown.<BR>    Prepare salad by tossing the chopped romaine with the chopped red cabbage, Napa cabbage, and carrots.<BR>    Sprinkle sliced green onion on top of the lettuce.<BR>    Sprinkle almonds over the salad, then the chow mein noodles.<BR>    Cut the chicken into small bite-size chunks.<BR>    Place the chicken onto the salad forming a pile in the middle.<BR>    Serve with salad dressing drizzled over it or on the side.<BR><BR><FONT SIZE=-2>This recipe published with <A href='http://www.bigoven.com'>BigOven</A>, and can be imported instantly by BigOven users.  Download your free trial at <a href='http://www.bigoven.com'><u>www.bigoven.com</u></A>.<BR><a href='http://www.bigoven.com'><img src='images/recipefrombigoven.gif' BORDER=0></A></font></td>
  </tr>
  <tr>
    <td width="100%" style="border-style: none; border-width: medium" valign="top" colspan="2">

<p style="margin-top: 0; margin-bottom: 3">
  <i>
<font face="Arial Narrow" size="2">

  http://www.food.com/recipe/tsr-version-of-applebee</font></i><font face="Tahoma" size="1"></p>

</font>
<font face="Tahoma" size="2">

<table border="1" cellpadding="0" cellspacing="0" style="border-collapse: collapse; border-width: 0" bordercolor="#111111" width="100%">
  <tr>
    <td width="50%" style="border-style: none; border-width: medium">
<p style="margin-top: 4; margin-bottom: 4" align="right">
<font face="Arial Narrow" size="2">
  Cuisine :&nbsp;&nbsp; <b>Chinese&nbsp;&nbsp;&nbsp;&nbsp; </b>

</font>
    </td>
    <td width="100%" style="border-style: none; border-width: medium">
    <p align="left" style="margin-top: 4; margin-bottom: 4">
<font face="Arial Narrow" size="2">
    Main Ingredient :&nbsp;&nbsp; <b></b></font></td>
  </tr>
  <tr>
    <td width="50%" style="border-style: none; border-width: medium">

    <font face="Arial Narrow"><i></i></font></td>
    <td width="100%" style="border-style: none; border-width: medium">

    <font face="Arial Narrow"><font color="#800000"><br>
  </font></font></td>
  </tr>
</table>

</font>

    </td>
  </tr>
  </table>
</body>

</html><!--@BOLOCAL-->"##
        }
    }

    mod results {
        use super::*;
        use schema_org::Recipe;
        use schema_org::field::RecipeKeywordsFieldEnum;

        pub fn recipe1() -> Recipe {
            Recipe {
                r#type: Some(AtType::Recipe.to_string()),
                keywords: vec![
                    RecipeKeywordsFieldEnum::TextOrURL("Low Fat".into()),
                    RecipeKeywordsFieldEnum::TextOrURL("Summer".into()),
                    RecipeKeywordsFieldEnum::TextOrURL("Spring".into()),
                    RecipeKeywordsFieldEnum::TextOrURL("Vegetables".into()),
                    RecipeKeywordsFieldEnum::TextOrURL("Salads".into()),
                    RecipeKeywordsFieldEnum::TextOrURL("Main Dish".into()),
                ],
                name: vec!["Applebee's Oriental Chicken Salad".into()],
                recipe_category: vec!["Low Carb".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::new_section("Salad Dressing", vec![
                        "6 tablespoon Honey",
                        "3 tablespoon Rice wine vinegar",
                        "1/2 cup Mayonnaise",
                        "2 teaspoon Grey Poupon Dijon Mustard",
                        "1/4 teaspoon Sesame Oil",
                    ]),
                    RecipeRecipeIngredientFieldEnum::new_section("Salad", vec![
                        "1 package Breaded Chicken Tenders",
                        "6 cups Romaine lettuce",
                        "2 cup Red cabbage",
                        "1 Carrot",
                        "2 Green Onion",
                        "2 tablespoon Sliced almonds",
                        "1 cup Chow mein noodles",
                    ]),
                ],
                recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("Below assumes you are making your own chicken tenders, whereas the recipe above uses pre-packaged chicken tenders.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Directions".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Preheat oil in deep fryer or deep pan over medium heat.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("You want the temperature of the oil to be around 350 degrees.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Blend together all ingredients for dressing in a small bowl with an electric mixer.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Put dressing in refrigerator to chill while you prepare the salad.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("In a small, shallow bowl beat egg, add milk, and mix well.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("In another bowl, combine flour with corn flake crumbs, salt and pepper.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Cut chicken breast into 4 or 5 long strips.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Dip each strip of chicken first into egg mixture then into the flour mixture, coating each piece completely.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Fry each chicken finger for 5 minutes or until coating has darkened to brown.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Prepare salad by tossing the chopped romaine with the chopped red cabbage, Napa cabbage, and carrots.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Sprinkle sliced green onion on top of the lettuce.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Sprinkle almonds over the salad, then the chow mein noodles.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Cut the chicken into small bite-size chunks.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Place the chicken onto the salad forming a pile in the middle.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Serve with salad dressing drizzled over it or on the side.".into()),
                ],
                recipe_yield: to_yield(4),
                url: vec!["http://www.food.com/recipe/tsr-version-of-applebees-oriental-chicken-salad-by-todd-wilbur-19253?ftab=reviews".into()],
                ..Default::default()
            }
        }
    }
}
