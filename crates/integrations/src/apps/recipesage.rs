use std::io::{Read, Seek};
use std::str::FromStr;

use iso8601::DateTime;
use nom::bytes::complete::{tag, take_till, take_until};
use nom::character::complete::line_ending;
use nom::combinator::{map, opt};
use nom::multi::{many_till, many1, separated_list0};
use nom::sequence::{preceded, terminated};
use nom::{IResult, Parser};
use recipe_schema::{
    AtType, CommentType, CreativeWorkOrItemListOrText, CreativeWorkOrText, DateOrDateTime,
    QuantitativeValueOrText, RecipeCategory, RecipeSchema, Sections, TextOrTextObject,
};
use serde::Deserialize;
use support::strings::extract_number;
use tracing::{error, warn};
use url::Url;

use crate::apps::helpers::read_file;
use crate::error::{Error, Result};
use crate::helpers::{
    seconds_to_duration, sections_to_itemlist, sections_to_vec, to_defined_text, to_is_based_on,
    to_organization_type, to_text, to_yield,
};

struct RecipeSage {
    category: Option<String>,
    description: Option<String>,
    ingredients: Sections,
    instructions: Sections,
    keywords: Vec<String>,
    notes: Option<String>,
    source: Option<String>,
    title: String,
    yield_: i16,
}

struct RecipeComponents<'a> {
    category: Option<&'a str>,
    description: Option<&'a str>,
    image: Option<&'a str>,
    ingredients: Vec<&'a str>,
    instructions: &'a str,
    keywords: Vec<&'a str>,
    notes: Option<&'a str>,
    servings: Option<i16>,
    source: Option<&'a str>,
    title: &'a str,
}

#[derive(Deserialize)]
struct RecipeSageXMLRoot {
    pub data: RecipeSageXMLData,
}

#[derive(Deserialize)]
struct RecipeSageXMLData {
    #[serde(rename = "recipe")]
    pub recipes: Vec<RecipeSageXMLRecipe>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecipeSageXMLRecipe {
    id: String,
    title: String,
    description: String,
    #[serde(rename = "yield")]
    r#yield: String,
    active_time: String,
    total_time: String,
    source: String,
    url: String,
    notes: String,
    ingredients: String,
    instructions: String,
    folder: String,
    created_at: String,
    updated_at: String,
    user_id: String,
    from_user: String,
    labels: Option<Vec<Label>>,
    images: Option<Images>,
}

#[derive(Debug, Deserialize)]
struct Label {
    title: String,
}

#[derive(Debug, Deserialize)]
struct Images {
    id: String,
    location: String,
}

impl From<RecipeComponents<'_>> for RecipeSage {
    fn from(r: RecipeComponents<'_>) -> Self {
        Self {
            category: r.category.map(|s| s.into()),
            description: r.description.map(|s| s.into()),
            ingredients: Sections::from([(
                "".into(),
                r.ingredients.into_iter().map(String::from).collect(),
            )]),
            instructions: Sections::from([(
                "".into(),
                r.instructions
                    .split_terminator("\n\n")
                    .map(|s| s.trim().replace("\n", " ").to_string())
                    .collect(),
            )]),
            keywords: r
                .keywords
                .into_iter()
                .map(str::trim)
                .map(String::from)
                .collect(),
            source: r.source.map(|s| s.into()),
            notes: r.notes.filter(|s| !s.is_empty()).map(|s| s.into()),
            title: r.title.into(),
            yield_: r.servings.unwrap_or_default(),
        }
    }
}

impl From<RecipeSageXMLRecipe> for RecipeSage {
    fn from(r: RecipeSageXMLRecipe) -> Self {
        let labels = r.labels.unwrap_or_default();
        let items = labels.split_first();

        Self {
            category: items.map(|(a, _b)| a.title.clone()),
            description: Some(r.description).filter(|s| !s.is_empty()),
            ingredients: Sections::from([(
                "".into(),
                r.ingredients.lines().map(String::from).collect(),
            )]),
            instructions: Sections::from([(
                "".into(),
                r.instructions
                    .split_terminator("\n\n")
                    .map(|s| s.trim().replace("\n", " ").to_string())
                    .collect(),
            )]),
            keywords: items
                .map(|(_a, b)| b.iter().map(|s| s.title.clone()).collect())
                .unwrap_or_default(),
            source: if r.source.is_empty() {
                Some(r.url).filter(|s| !s.is_empty())
            } else {
                Some(r.source)
            },
            notes: Some(r.notes).filter(|s| !s.is_empty()),
            title: r.title,
            yield_: extract_number(r.r#yield).unwrap_or_default(),
        }
    }
}

impl From<RecipeSchema> for RecipeSage {
    fn from(r: RecipeSchema) -> Self {
        Self {
            category: match r.recipe_category {
                RecipeCategory::Text(s) => Some(s),
            },
            description: r
                .description
                .map(|s| match s {
                    TextOrTextObject::Text(s) => s,
                    TextOrTextObject::TextObject(obj) => {
                        warn!("RecipeSage: TextObject not supported: {:?}", obj);
                        "".into()
                    }
                })
                .filter(|s| !s.is_empty()),
            ingredients: Sections::from([("".into(), r.recipe_ingredient.unwrap_or_default())]),
            instructions: Sections::from([(
                "".into(),
                r.recipe_instructions
                    .clone()
                    .into_iter()
                    .map(|ins| match ins {
                        CreativeWorkOrItemListOrText::CreativeWork(obj) => {
                            vec![obj.name.unwrap_or_default()]
                        }
                        CreativeWorkOrItemListOrText::ItemList(item) => {
                            item.into_iter().map(|obj| obj.text).collect()
                        }
                        CreativeWorkOrItemListOrText::Text(s) => vec![s],
                    })
                    .filter(|s| !s.is_empty())
                    .flatten()
                    .collect(),
            )]),
            keywords: vec![],
            source: r.is_based_on.map(|c| match c {
                CreativeWorkOrText::CreativeWork(c) => c.name.unwrap_or_default(),
                CreativeWorkOrText::Text(s) => s,
            }),
            notes: r
                .comment
                .map(|v| {
                    v.into_iter()
                        .map(|c| c.text)
                        .collect::<Vec<_>>()
                        .join("\n\n")
                })
                .filter(|s| !s.is_empty()),
            title: r.name.unwrap_or_default(),
            yield_: match r.recipe_yield {
                QuantitativeValueOrText::QuantitativeValue(v) => v.value as i16,
                QuantitativeValueOrText::Text(s) => extract_number(s).unwrap_or_default(),
            },
        }
    }
}

impl From<RecipeSage> for RecipeSchema {
    fn from(r: RecipeSage) -> Self {
        Self {
            at_context: Default::default(),
            at_type: Some(AtType::Recipe),
            comment: r
                .notes
                .filter(|s| !s.is_empty())
                .map(|s| CommentType {
                    r#type: AtType::Review,
                    text: s,
                    ..Default::default()
                })
                .map(|c| vec![c]),
            keywords: to_defined_text(r.keywords.join(",")),
            name: Some(r.title),
            recipe_category: RecipeCategory::Text(r.category.unwrap_or_default()),
            recipe_ingredient: sections_to_vec(r.ingredients),
            recipe_instructions: sections_to_itemlist(r.instructions),
            recipe_yield: to_yield(r.yield_ as i64),
            url: Url::parse(&r.source.clone().unwrap_or_default()).ok(),
            description: to_text(r.description.unwrap_or_default()),
            is_based_on: to_is_based_on(r.source.unwrap_or_default()),
            ..Default::default()
        }
    }
}

impl From<RecipeSageXMLRecipe> for RecipeSchema {
    fn from(r: RecipeSageXMLRecipe) -> Self {
        let categories = r.labels.unwrap_or_default();
        let categories = categories.split_first();

        let active_time_secs = match humantime::parse_duration(&r.active_time) {
            Ok(d) => d.as_secs() as i32,
            Err(err) => {
                error!("Failed to parse prep time of a RecipeSage recipe: {err}");
                15 * 60
            }
        };

        let total_time_secs = match humantime::parse_duration(&r.total_time) {
            Ok(d) => d.as_secs() as i32,
            Err(err) => {
                error!("Failed to total time of a RecipeSage recipe: {err}");
                30 * 60
            }
        };

        let url = Url::parse(&r.url).ok();

        Self {
            at_context: Default::default(),
            at_type: Some(AtType::Recipe),
            author: to_organization_type(r.from_user),
            comment: Some(
                vec![r.notes]
                    .into_iter()
                    .filter(|s| !s.is_empty())
                    .map(|c| CommentType {
                        r#type: AtType::Review,
                        text: c,
                        ..Default::default()
                    })
                    .collect(),
            )
            .filter(|v: &Vec<CommentType>| !v.is_empty()),
            cook_time: seconds_to_duration(total_time_secs - active_time_secs),
            date_created: DateTime::from_str(&r.created_at).ok().map(|d| {
                DateOrDateTime::DateTime(DateTime {
                    date: d.date,
                    time: d.time,
                })
            }),
            date_modified: DateTime::from_str(&r.updated_at).ok().map(|d| {
                DateOrDateTime::DateTime(DateTime {
                    date: d.date,
                    time: d.time,
                })
            }),
            description: to_text(r.description),
            is_based_on: if url.is_none() && r.url != "" {
                to_is_based_on(r.url)
            } else {
                to_is_based_on(r.source)
            },
            keywords: to_defined_text(
                categories
                    .map(|(_, b)| b.iter().map(|s| s.title.to_string()).collect::<Vec<_>>())
                    .map(|v| v.join(","))
                    .unwrap_or_default(),
            ),
            name: Some(r.title),
            prep_time: seconds_to_duration(active_time_secs),
            recipe_category: RecipeCategory::Text(
                categories
                    .map(|(a, _b)| a.title.to_string())
                    .unwrap_or_default(),
            ),
            recipe_ingredient: sections_to_vec(Sections::from([(
                "".into(),
                r.ingredients.lines().map(String::from).collect(),
            )])),
            recipe_instructions: sections_to_itemlist(Sections::from([(
                "".into(),
                r.instructions
                    .split_terminator("\n\n")
                    .map(|s| s.trim().replace("\n", " ").to_string())
                    .collect(),
            )])),
            recipe_yield: to_yield(extract_number(r.r#yield).unwrap_or_default()),
            url,
            ..Default::default()
        }
    }
}

/// Parses a RecipeSage recipes text file.
pub fn parse_txt<R>(r: R) -> Result<Vec<RecipeSchema>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;
    Ok(parse_text_file(&content)?
        .into_iter()
        .map(RecipeSchema::from)
        .collect())
}

/// Parses a RecipeSage recipes XML file.
pub fn parse_xml<R>(r: R) -> Result<Vec<RecipeSchema>>
where
    R: Read,
{
    let root: RecipeSageXMLData = serde_xml_rs::from_reader(r).map_err(|err| {
        error!("Failed to read RecipeSage XML file: {err}");
        Error::Parse(err.to_string())
    })?;

    Ok(root.recipes.into_iter().map(RecipeSchema::from).collect())
}

/// Parses a RecipeSage recipes JSON file.
pub fn parse_json<R>(r: R) -> Result<Vec<RecipeSchema>>
where
    R: Read,
{
    let res: Vec<RecipeSchema> = serde_json::from_reader(r).map_err(|err| {
        error!("Failed to read RecipeSage JSON file: {err}");
        Error::Parse(err.to_string())
    })?;

    Ok(res.into_iter().map(RecipeSchema::from).collect())
}

fn parse_text_file(input: &str) -> Result<Vec<RecipeSage>> {
    preceded(
        (tag("==== Recipes ===="), line_ending, line_ending),
        many1(map(recipe, RecipeSage::from)),
    )
    .parse(input)
    .map(|(_, r)| r)
    .map_err(|err| Error::Parse(err.to_string()))
}

fn recipe(input: &str) -> IResult<&str, RecipeComponents> {
    map(
        (
            (id, opt(line_ending)),
            title,
            description,
            servings,
            active_time,
            total_time,
            source,
            url,
            notes,
            ingredients,
            instructions,
            (folder, created_at, updated_at, user_id),
            labels,
            images,
            opt(line_ending),
        ),
        |(
            _,
            title,
            description,
            servings,
            _,
            _,
            source,
            url,
            notes,
            ingredients,
            instructions,
            (_, _, _, _),
            labels,
            images,
            _,
        )| {
            let items = labels.split_first();

            RecipeComponents {
                category: items.map(|(&a, _b)| a),
                description: description.filter(|s| !s.is_empty()),
                image: images,
                ingredients,
                instructions,
                keywords: items
                    .map(|(_a, b)| b.iter().map(|&s| s).collect())
                    .unwrap_or_default(),
                notes,
                servings: servings
                    .filter(|s| !s.is_empty())
                    .map(|s| extract_number(s.to_string()).unwrap_or_default()),
                source: source
                    .filter(|s| !s.is_empty())
                    .or(url.filter(|s| !s.is_empty())),
                title: title.unwrap_or(""),
            }
        },
    )
    .parse(input)
}

fn id(input: &str) -> IResult<&str, Option<&str>> {
    parse_tag("id").parse(input)
}

fn title(input: &str) -> IResult<&str, Option<&str>> {
    parse_tag("title").parse(input)
}

fn description(input: &str) -> IResult<&str, Option<&str>> {
    parse_tag("description").parse(input)
}

fn servings(input: &str) -> IResult<&str, Option<&str>> {
    parse_tag("yield").parse(input)
}

fn active_time(input: &str) -> IResult<&str, Option<&str>> {
    parse_tag("activeTime").parse(input)
}

fn total_time(input: &str) -> IResult<&str, Option<&str>> {
    parse_tag("totalTime").parse(input)
}

fn source(input: &str) -> IResult<&str, Option<&str>> {
    parse_tag("source").parse(input)
}

fn url(input: &str) -> IResult<&str, Option<&str>> {
    parse_tag("url").parse(input)
}

fn notes(input: &str) -> IResult<&str, Option<&str>> {
    parse_tag("notes").parse(input)
}

fn ingredients(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        tag("ingredients: "),
        many_till(
            terminated(take_until("\n"), line_ending),
            tag("instructions:"),
        )
        .map(|(lines, _)| lines),
    )
    .parse(input)
}

fn instructions(input: &str) -> IResult<&str, &str> {
    preceded(tag(" "), take_until("folder:")).parse(input)
}

fn folder(input: &str) -> IResult<&str, Option<&str>> {
    parse_tag("folder").parse(input)
}

fn created_at(input: &str) -> IResult<&str, Option<&str>> {
    parse_tag("createdAt").parse(input)
}

fn updated_at(input: &str) -> IResult<&str, Option<&str>> {
    parse_tag("updatedAt").parse(input)
}

fn user_id(input: &str) -> IResult<&str, Option<&str>> {
    parse_tag("userId").parse(input)
}

fn labels(input: &str) -> IResult<&str, Vec<&str>> {
    map(
        preceded(
            tag("labels: "),
            (
                separated_list0(tag(","), take_till(|c| c == ',' || c == '\n')),
                line_ending,
            ),
        ),
        |(s, _)| s,
    )
    .parse(input)
}

fn images(input: &str) -> IResult<&str, Option<&str>> {
    parse_tag("images").parse(input)
}

fn parse_tag(prefix: &str) -> impl FnMut(&str) -> IResult<&str, Option<&str>> {
    move |input| {
        preceded(
            (tag(prefix), tag(": ")),
            terminated(opt(take_until("\n")), tag("\n")),
        )
        .parse(input)
    }
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
            let file = files::txt();
            let buf = Cursor::new(&file);

            let got = parse_txt(buf)?;

            pretty_assertions::assert_eq!(got, results::all_recipes());
            Ok(())
        }

        #[test]
        fn test_xml_ok() -> Result<()> {
            let file = files::xml();
            let buf = Cursor::new(&file);

            let got = parse_xml(buf)?;

            pretty_assertions::assert_eq!(got, results::all_recipes());
            Ok(())
        }

        #[test]
        fn test_json_ok() -> Result<()> {
            let file = files::json();
            let buf = Cursor::new(&file);

            let got = parse_json(buf)?;

            pretty_assertions::assert_eq!(got.iter().collect::<Vec<_>>().len(), got.len());
            Ok(())
        }
    }

    mod files {
        pub fn txt<'a>() -> &'a str {
            r##"==== Recipes ====

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
"##
        }

        pub fn xml<'a>() -> &'a str {
            r##"<data>
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
</data>"##
        }

        pub fn json<'a>() -> &'a str {
            r##"[{"@context":"http://schema.org","@type":"Recipe","identifier":"5ba3144a-e86e-431e-b5bb-c4109ebb8c04","datePublished":"2025-04-16T00:12:39.551Z","description":"","image":[],"name":"Asparagus Soup (Zuppa Di Asparagi)","prepTime":"","recipeIngredient":["2 tb Extra-virgin olive oil 1 qt Chicken broth","2 Cloves garlic, minced 4 Eggs","2 lb Asparagus, trimmed, peeled 1/2 c Freshly grated Parmesan or","-and cut (1 inch pieces) -pecorino cheese","Salt and pepper 6 sl Italian bread, toasted"],"recipeInstructions":[{"@type":"HowToStep","text":"Heat the oil and garlic in a soup pot until the garlic is golden. Add the"},{"@type":"HowToStep","text":"asparagus and cook until they begin to color. Season with salt and pepper."},{"@type":"HowToStep","text":"Add the broth and bring to a boil; reduce the heat and simmer for 15"},{"@type":"HowToStep","text":"minutes, or until the asparagus is tender."},{"@type":"HowToStep","text":"Beat the eggs and cheese together. When the asparagus is tender, reduce"},{"@type":"HowToStep","text":"the heat so the soup is no longer simmering. Very slowly ladle some of the"},{"@type":"HowToStep","text":"hot soup into the beaten eggs, stirring continuously. After adding about 2"},{"@type":"HowToStep","text":"cups of the hot soup to the eggs, reverse the process and gradually stir"},{"@type":"HowToStep","text":"the eggs mixture into the soup pot. The soup must not boil or the eggs"},{"@type":"HowToStep","text":"will scramble. Heat until thickened."},{"@type":"HowToStep","text":"Put one slice of toasted bread into each soup dish. Ladle the hot soup on"},{"@type":"HowToStep","text":"top and pass additional grated cheese."},{"@type":"HowToStep","text":"Serves 6."},{"@type":"HowToStep","text":"NOTE: To trim asparagus, hold the tip in one hand and the base of the"},{"@type":"HowToStep","text":"stalk in the other. Bend gently. The asparagus will snap, leaving the"},{"@type":"HowToStep","text":"tender part with the tip."},{"@type":"HowToSection","text":"[ \"We Called It Macaroni\"; Nancy Verde Barr; Knopf; ISBN 0-394-55798-0 ]"},{"@type":"HowToStep","text":"Posted by Fred Peters."}],"recipeYield":"6 servings","totalTime":"","recipeCategory":["italian","soups/stews","vegetables"],"creditText":"","isBasedOn":"MMF","comment":[{"@type":"Comment","name":"Author Notes","text":""}]},{"@context":"http://schema.org","@type":"Recipe","identifier":"d646992d-4726-48b4-ae7a-a85260b27811","datePublished":"2025-04-16T00:12:39.551Z","description":"","image":[],"name":"Aubergine and Sesame Pate","prepTime":"","recipeIngredient":["1/2 md Aubergine 1/4 Juice of 1 lemon","1 Crushed garlic cloves 1 tb Olive oil","1 1/2 tb Tahini Seasoning","Toasted Sesame seeds Flatleaf Parsley","Cayenne Pepper","25-30 minutes until tender. Cool slightly , then peel and"],"recipeInstructions":[{"@type":"HowToStep","text":"1> Preheat the oven to 200c/400f/Gas 6. Bake the aubergine for"},{"@type":"HowToStep","text":"puree the flesh in a blender or processor."},{"@type":"HowToStep","text":"Add the garlic, tahini and lemon juice and process until mixed."},{"@type":"HowToStep","text":"With the motor running, drizzle in the oil to make a smooth paste."},{"@type":"HowToStep","text":"Season to taste."},{"@type":"HowToStep","text":"Transfer to a serving dish, garnish and serve cold with pitta bread."}],"recipeYield":"2 servings","totalTime":"","recipeCategory":["appetizers","greek","vegetarian"],"creditText":"","isBasedOn":"MMF","comment":[{"@type":"Comment","name":"Author Notes","text":""}]},{"@context":"http://schema.org","@type":"Recipe","identifier":"adea4c61-fd42-4700-a2be-48338cc44818","datePublished":"2025-04-16T00:12:39.551Z","description":"","image":[],"name":"Aubergines a la Toulousaine (Eggplant A La Toulouse)","prepTime":"","recipeIngredient":["1 md Eggplant 2 tb Snipped parsley","1/4 c Salad oil 1 cl Galic, minced","3 lg Tomatoes, peeled 1 tb Salad oil","2 c Fresh bread cubes 1/4 c Grated Parmesan cheese"],"recipeInstructions":[{"@type":"HowToStep","text":"Cut eggplant into 1/2-inch thick slices: pared. Place slices on paper"},{"@type":"HowToStep","text":"towels; sprinkle each generously with salt. let stand for 30 minutes; then"},{"@type":"HowToStep","text":"blot dry with paper towels. Start heating oven to 400 deg. F. Saute"},{"@type":"HowToStep","text":"eggplant in 1/4 cup salad oil until golden. Add more oils as needed. Cut"},{"@type":"HowToStep","text":"tomatoes into 1/2-inch thick slices; saute in same skillet. In a 10x6x2"},{"@type":"HowToStep","text":"inch baking dish, arrange eggplant and tomatoes in alternate layers, (4 in"},{"@type":"HowToStep","text":"all), sprinkling each layer with 1/4 teaspoon salt and 1/8 teaspoon pepper."},{"@type":"HowToStep","text":"Combine bread cubes with parsley, garlic, 1 tablespoon salad oil and"},{"@type":"HowToStep","text":"cheese. Toss well. Sprinkle over top layer. Bake 20 minutes or until bread"},{"@type":"HowToStep","text":"cubes are golden and eggplant is tender."},{"@type":"HowToStep","text":"SOURCE: Good Houskeeping's Around The World Cookbook. Consolidated Book"},{"@type":"HowToStep","text":"Publishers Chicago 1, Illinois 1958"}],"recipeYield":"4 servings","totalTime":"","recipeCategory":["vegetables","french","casseroles"],"creditText":"","isBasedOn":"MMF","comment":[{"@type":"Comment","name":"Author Notes","text":""}]},{"@context":"http://schema.org","@type":"Recipe","identifier":"03237963-3eca-4731-8586-ddaee265b98b","datePublished":"2025-04-16T00:12:39.551Z","description":"","image":[],"name":"August Goerg's Grilled Steak (Spiessbraten August Goerg)","prepTime":"","recipeIngredient":["1 Shallot or small onion cut 1 pn Mace","-into small pieces 1 lg Steak (just over 1 lb), at","Freshly ground black pepper -least 1 1/4 inches"],"recipeInstructions":[{"@type":"HowToStep","text":"((Note: Per Horst Scharfenberg, this recipe originated in the town of"},{"@type":"HowToStep","text":"Idar-Oberstein in the 19 th century, when gemstone prospectors returning"},{"@type":"HowToStep","text":"from South America created their own version of gaucho-grilled steaks. The"},{"@type":"HowToStep","text":"dish was then further refined by Scharfenberg's mentor August Goerg. K.B.))"},{"@type":"HowToStep","text":"Per person: thick, trimmed"},{"@type":"HowToStep","text":"Mix together the shallot or onion with the pepper and mace. Insert a few"},{"@type":"HowToStep","text":"shallot pieces into the steak using the point of a small knife. Coat the"},{"@type":"HowToStep","text":"steak with the shallot mixture, pressing it in so it will adhere."},{"@type":"HowToStep","text":"Remove the loose shallot pieces and grill the steak (over a fire of oak"},{"@type":"HowToStep","text":"logs, says August Goerg, from which the bark has been removed).* Take the"},{"@type":"HowToStep","text":"steaks off the grill while they are still pink inside. Sprinkle them with"},{"@type":"HowToStep","text":"salt."},{"@type":"HowToStep","text":"*Note: A special grill is used, suspended with 3 chains from an iron"},{"@type":"HowToStep","text":"tripod and constantly swinging through the flames."},{"@type":"HowToStep","text":"From: THE CUISINES OF GERMANY by Horst Scharfenberg, Simon &"},{"@type":"HowToStep","text":"Schuster/Poseidon Press, New York. 1989 Posted by: Karin Brewer, Cooking"},{"@type":"HowToStep","text":"Echo, 8/92"}],"recipeYield":"6 servings","totalTime":"","recipeCategory":["german","beef"],"creditText":"","isBasedOn":"MMF","comment":[{"@type":"Comment","name":"Author Notes","text":""}]},{"@context":"http://schema.org","@type":"Recipe","identifier":"4c23ec92-fc6f-4636-bb69-e1edbfa965c3","datePublished":"2025-04-16T00:12:39.551Z","description":"","image":["https://chefbook-prod.s3.us-west-2.amazonaws.com/1744762360133-537cea00530765"],"name":"Aunt Julia's Paella","prepTime":"","recipeIngredient":["1 Chicken, cut up (Or 4 thighs 1 3/4 oz Jar sliced pimento","-and legs) 2 ts Capers, with juice","Salt and pepper to thaste 4 oz Jar pimento-stiffed green","1 lb Lean pork, cut into 1-inch -olives","-cubes 1/2 lb Calamari (squid), cleaned","1 md Onion, minced -and sliced","2 Toes garlic, minced 5 c Water","Cut into 1 1/2 inch julliene 4 Chicken bouillon cubes","-strips: 1 ts Saffron threads","1/2 lg Bell pepper 2 1/2 c Uncle Ben's (c) rice,","1 lg Carrot -uncooked","1 Stalk celery 3 Hard boiled eggs, sliced","1 c Frozen green peas 1/2 lb Unpeeled shrimp (heads on)","1 1/2 lb Peeled shrimp Oil for frying"],"recipeInstructions":[{"@type":"HowToStep","text":"{ Submitted by Chiqui Collier, Cookery N'Orleans Restaurant }"},{"@type":"HowToStep","text":"In a large electric skillet or paella pan, brown the chicken pieces (that"},{"@type":"HowToStep","text":"have been seasoned with salt and pepper) in a little oil. Remove from the"},{"@type":"HowToStep","text":"pan. Add the pork cubes to the drippinfs and brown for about 5 minutes."},{"@type":"HowToStep","text":"Remove from the pan. To the pan drippings (add a little more oil if"},{"@type":"HowToStep","text":"necessary) add the onion, garlic, bell pepper, celery and carrot. Stir-fry"},{"@type":"HowToStep","text":"for 2 minutes."},{"@type":"HowToStep","text":"Add the peas, peeled shrimp, pimentos, capers, chicken, calamari and pork."},{"@type":"HowToStep","text":"Stir. In a separate pot, bring the 5 cups of water to a boil; stir in the"},{"@type":"HowToStep","text":"bouillon cubes and saffron. Let it stand for 5 minutes until dissolved."},{"@type":"HowToStep","text":"Gently stir the rice into the skillet mixture. Slowly pour in enough of"},{"@type":"HowToStep","text":"the bouillon mixture to cover the rice and chicken pieces. Cover and cook"},{"@type":"HowToStep","text":"over low heat for about 20 minutes. Uncover and decoaratively arrange the"},{"@type":"HowToStep","text":"egg slices and raw unpeeled shrimp on the top. (Add more broth as necessary"},{"@type":"HowToStep","text":"to keep the rice moist."},{"@type":"HowToStep","text":"Cover and steam for another 10 minutes until the shrimp are cooked and the"},{"@type":"HowToStep","text":"rice is tender. (Paella should be moist but not wet!) Place the pan on a"},{"@type":"HowToStep","text":"hot pad on the serving table and let everyone help themselves."},{"@type":"HowToStep","text":"Serve with a mixed green salad, red ripe tomatoes and some French bread."},{"@type":"HowToStep","text":"Also mix up a pitcher of Sangria and enjoy!"},{"@type":"HowToStep","text":"Serves: 12."},{"@type":"HowToSection","text":"[ The Legends of Louisisna Cookbook; Sheila Ainbinder; ISBN 0-671-70817-1 ]"},{"@type":"HowToStep","text":"Posted by Fred Peters"}],"recipeYield":"6 servings","totalTime":"","recipeCategory":["fish/sea","pork/ham","poultry","spanish"],"creditText":"","isBasedOn":"MMF","comment":[{"@type":"Comment","name":"Author Notes","text":""}]}]"##
        }
    }

    mod results {
        use super::*;

        pub(crate) fn all_recipes() -> Vec<RecipeSchema> {
            vec![RecipeSchema {
                at_context: Default::default(),
                at_type: Some(AtType::Recipe),
                keywords: to_defined_text(["soups/stews", "vegetables"].join(",")),
                is_based_on: to_is_based_on("MMF".into()),
                name: Some("Asparagus Soup (Zuppa Di Asparagi)".into()),
                recipe_category: RecipeCategory::Text("italian".into()),
                recipe_ingredient: Some(vec![
                    "<section></section>".into(),
                    "2 tb Extra-virgin olive oil 1 qt Chicken broth".into(),
                    "2 Cloves garlic, minced 4 Eggs".into(),
                    "2 lb Asparagus, trimmed, peeled 1/2 c Freshly grated Parmesan or".into(),
                    "-and cut (1 inch pieces) -pecorino cheese".into(),
                    "Salt and pepper 6 sl Italian bread, toasted".into(),
                ]),
                recipe_instructions: sections_to_itemlist(Sections::from([
                    ("".into(), vec![
                        "<section></section>".into(),
                        "Heat the oil and garlic in a soup pot until the garlic is golden. Add the asparagus and cook until they begin to color. Season with salt and pepper. Add the broth and bring to a boil; reduce the heat and simmer for 15 minutes, or until the asparagus is tender.".into(),
                        "Beat the eggs and cheese together. When the asparagus is tender, reduce the heat so the soup is no longer simmering. Very slowly ladle some of the hot soup into the beaten eggs, stirring continuously. After adding about 2 cups of the hot soup to the eggs, reverse the process and gradually stir the eggs mixture into the soup pot. The soup must not boil or the eggs will scramble. Heat until thickened.".into(),
                        "Put one slice of toasted bread into each soup dish. Ladle the hot soup on top and pass additional grated cheese.".into(),
                        "Serves 6.".into(),
                        "NOTE: To trim asparagus, hold the tip in one hand and the base of the stalk in the other. Bend gently. The asparagus will snap, leaving the tender part with the tip.".into(),
                        "[ \"We Called It Macaroni\"; Nancy Verde Barr; Knopf; ISBN 0-394-55798-0 ]".into(),
                        "Posted by Fred Peters.".into(),
                    ])
                ])),
                recipe_yield: to_yield(6),
                ..Default::default()
            }, RecipeSchema {
                at_context: Default::default(),
                at_type: Some(AtType::Recipe),
                keywords: to_defined_text(["greek", "vegetarian"].join(",")),
                is_based_on: to_is_based_on("MMF".into()),
                name: Some("Aubergine and Sesame Pate".into()),
                recipe_category: RecipeCategory::Text("appetizers".into()),
                recipe_ingredient: Some(vec![
                    "<section></section>".into(),
                    "1/2 md Aubergine 1/4 Juice of 1 lemon".into(),
                    "1 Crushed garlic cloves 1 tb Olive oil".into(),
                    "1 1/2 tb Tahini Seasoning".into(),
                    "Toasted Sesame seeds Flatleaf Parsley".into(),
                    "Cayenne Pepper".into(),
                    "25-30 minutes until tender. Cool slightly , then peel and".into(),
                ]),
                recipe_instructions: sections_to_itemlist(Sections::from([
                    ("".into(), vec![
                        "1> Preheat the oven to 200c/400f/Gas 6. Bake the aubergine for puree the flesh in a blender or processor.".into(),
                        "Add the garlic, tahini and lemon juice and process until mixed. With the motor running, drizzle in the oil to make a smooth paste. Season to taste.".into(),
                        "Transfer to a serving dish, garnish and serve cold with pitta bread.".into(),
                    ])
                ])),
                recipe_yield: to_yield(2),
                ..Default::default()
            }, RecipeSchema {
                at_context: Default::default(),
                at_type: Some(AtType::Recipe),
                keywords: to_defined_text(["french", "casseroles"].join(",")),
                is_based_on: to_is_based_on("MMF".into()),
                name: Some("Aubergines a la Toulousaine (Eggplant A La Toulouse)".into()),
                recipe_category: RecipeCategory::Text("vegetables".into()),
                recipe_ingredient: Some(vec![
                    "<section></section>".into(),
                    "1 md Eggplant 2 tb Snipped parsley".into(),
                    "1/4 c Salad oil 1 cl Galic, minced".into(),
                    "3 lg Tomatoes, peeled 1 tb Salad oil".into(),
                    "2 c Fresh bread cubes 1/4 c Grated Parmesan cheese".into(),
                ]),
                recipe_instructions: sections_to_itemlist(Sections::from([
                    ("".into(), vec![
                        "Cut eggplant into 1/2-inch thick slices: pared. Place slices on paper towels; sprinkle each generously with salt. let stand for 30 minutes; then blot dry with paper towels. Start heating oven to 400 deg. F. Saute eggplant in 1/4 cup salad oil until golden. Add more oils as needed. Cut tomatoes into 1/2-inch thick slices; saute in same skillet. In a 10x6x2 inch baking dish, arrange eggplant and tomatoes in alternate layers, (4 in all), sprinkling each layer with 1/4 teaspoon salt and 1/8 teaspoon pepper. Combine bread cubes with parsley, garlic, 1 tablespoon salad oil and cheese. Toss well. Sprinkle over top layer. Bake 20 minutes or until bread cubes are golden and eggplant is tender.".into(),
                        "SOURCE: Good Houskeeping's Around The World Cookbook. Consolidated Book Publishers Chicago 1, Illinois 1958".into(),
                    ])
                ])),
                recipe_yield: to_yield(4),
                ..Default::default()
            }, RecipeSchema {
                at_context: Default::default(),
                at_type: Some(AtType::Recipe),
                keywords: to_defined_text("beef".into()),
                is_based_on: to_is_based_on("MMF".into()),
                name: Some("August Goerg's Grilled Steak (Spiessbraten August Goerg)".into()),
                recipe_category: RecipeCategory::Text("german".into()),
                recipe_ingredient: Some(vec![
                    "<section></section>".into(),
                    "1 Shallot or small onion cut 1 pn Mace".into(),
                    "-into small pieces 1 lg Steak (just over 1 lb), at".into(),
                    "Freshly ground black pepper -least 1 1/4 inches".into(),
                ]),
                recipe_instructions: sections_to_itemlist(Sections::from([
                    ("".into(), vec![
                        "((Note: Per Horst Scharfenberg, this recipe originated in the town of Idar-Oberstein in the 19 th century, when gemstone prospectors returning from South America created their own version of gaucho-grilled steaks. The dish was then further refined by Scharfenberg's mentor August Goerg. K.B.))".into(),
                        "Per person: thick, trimmed".into(),
                        "Mix together the shallot or onion with the pepper and mace. Insert a few shallot pieces into the steak using the point of a small knife. Coat the steak with the shallot mixture, pressing it in so it will adhere.".into(),
                        "Remove the loose shallot pieces and grill the steak (over a fire of oak logs, says August Goerg, from which the bark has been removed).* Take the steaks off the grill while they are still pink inside. Sprinkle them with salt.".into(),
                        "*Note: A special grill is used, suspended with 3 chains from an iron tripod and constantly swinging through the flames.".into(),
                        "From: THE CUISINES OF GERMANY by Horst Scharfenberg, Simon & Schuster/Poseidon Press, New York. 1989 Posted by: Karin Brewer, Cooking Echo, 8/92".into(),
                    ])
                ])),
                recipe_yield: to_yield(6),
                ..Default::default()
            }, RecipeSchema {
                at_context: Default::default(),
                at_type: Some(AtType::Recipe),
                keywords: to_defined_text("pork/ham,poultry,spanish".into()),
                is_based_on: to_is_based_on("MMF".into()),
                name: Some("Aunt Julia's Paella".into()),
                recipe_category: RecipeCategory::Text("fish/sea".into()),
                recipe_ingredient: Some(vec![
                    "<section></section>".into(),
                   "1 Chicken, cut up (Or 4 thighs 1 3/4 oz Jar sliced pimento".into(),
                   "-and legs) 2 ts Capers, with juice".into(),
                   "Salt and pepper to thaste 4 oz Jar pimento-stiffed green".into(),
                   "1 lb Lean pork, cut into 1-inch -olives".into(),
                   "-cubes 1/2 lb Calamari (squid), cleaned".into(),
                   "1 md Onion, minced -and sliced".into(),
                   "2 Toes garlic, minced 5 c Water".into(),
                   "Cut into 1 1/2 inch julliene 4 Chicken bouillon cubes".into(),
                   "-strips: 1 ts Saffron threads".into(),
                   "1/2 lg Bell pepper 2 1/2 c Uncle Ben's (c) rice,".into(),
                   "1 lg Carrot -uncooked".into(),
                   "1 Stalk celery 3 Hard boiled eggs, sliced".into(),
                   "1 c Frozen green peas 1/2 lb Unpeeled shrimp (heads on)".into(),
                   "1 1/2 lb Peeled shrimp Oil for frying".into(),
                ]),
                recipe_instructions: sections_to_itemlist(Sections::from([
                    ("".into(), vec![
                        "{ Submitted by Chiqui Collier, Cookery N'Orleans Restaurant }".into(),
                        "In a large electric skillet or paella pan, brown the chicken pieces (that have been seasoned with salt and pepper) in a little oil. Remove from the pan. Add the pork cubes to the drippinfs and brown for about 5 minutes. Remove from the pan. To the pan drippings (add a little more oil if necessary) add the onion, garlic, bell pepper, celery and carrot. Stir-fry for 2 minutes.".into(),
                        "Add the peas, peeled shrimp, pimentos, capers, chicken, calamari and pork. Stir. In a separate pot, bring the 5 cups of water to a boil; stir in the bouillon cubes and saffron. Let it stand for 5 minutes until dissolved.".into(),
                        "Gently stir the rice into the skillet mixture. Slowly pour in enough of the bouillon mixture to cover the rice and chicken pieces. Cover and cook over low heat for about 20 minutes. Uncover and decoaratively arrange the egg slices and raw unpeeled shrimp on the top. (Add more broth as necessary to keep the rice moist.".into(),
                        "Cover and steam for another 10 minutes until the shrimp are cooked and the rice is tender. (Paella should be moist but not wet!) Place the pan on a hot pad on the serving table and let everyone help themselves.".into(),
                        "Serve with a mixed green salad, red ripe tomatoes and some French bread. Also mix up a pitcher of Sangria and enjoy!".into(),
                        "Serves: 12.".into(),
                        "[ The Legends of Louisisna Cookbook; Sheila Ainbinder; ISBN 0-671-70817-1 ]".into(),
                        "Posted by Fred Peters".into(),
                    ])
                ])),
                recipe_yield: to_yield(6),
                ..Default::default()
            }]
        }
    }
}
