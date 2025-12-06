use std::borrow::Cow;
use std::io::{Read, Seek};

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while, take_while_m_n};
use nom::character::complete::{char, line_ending, space0, space1};
use nom::character::satisfy;
use nom::combinator::{map, opt, recognize, verify};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{delimited, preceded, terminated};
use nom::{IResult, Parser};

use schema_org::field::{
    RecipeAuthorFieldEnum, RecipeIsBasedOnFieldEnum, RecipeKeywordsFieldEnum,
    RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
};
use schema_org::{AtType, Recipe};

use super::helpers::{Ingredient, Instruction, ToSections, is_vchar_or_space, read_file};
use crate::Result;

struct KalorioTextRecipe {
    title: String,
    author: String,
    keywords: Vec<String>,
    instructions: Vec<RecipeRecipeInstructionsFieldEnum>,
    ingredients: Vec<RecipeRecipeIngredientFieldEnum>,
    kalorio_version: Option<String>,
}

struct RecipeComponents<'a> {
    title: &'a str,
    author: Option<&'a str>,
    keywords: Vec<&'a str>,
    ingredients: Vec<Ingredient<'a>>,
    instructions: Vec<Instruction<'a>>,
    version: Option<&'a str>,
}

impl From<RecipeComponents<'_>> for KalorioTextRecipe {
    fn from(r: RecipeComponents<'_>) -> Self {
        let mut ingredients = r.ingredients;
        fix_ingredients(&mut ingredients);

        Self {
            title: r.title.into(),
            author: r.author.unwrap_or_default().into(),
            keywords: r.keywords.into_iter().map(String::from).collect(),
            instructions: r.instructions.to_sections(),
            ingredients: ingredients.to_sections(),
            kalorio_version: r.version.map(String::from),
        }
    }
}

pub fn fix_ingredients(ingredients: &mut Vec<Ingredient>) {
    let mut i = 0;
    while i < ingredients.len() {
        if i >= 2 {
            let current = match ingredients.get(i) {
                Some(Ingredient::Line(current)) => current,
                _ => {
                    i += 1;
                    continue;
                }
            };

            let starts_with_lowercase = current
                .chars()
                .next()
                .map(char::is_lowercase)
                .unwrap_or(false);

            if starts_with_lowercase {
                let idx = match ingredients.get(i - 2) {
                    Some(Ingredient::Line(prev)) if prev.ends_with(']') => 1,
                    _ => 2,
                };

                if let Some(Ingredient::Line(prev)) = ingredients.get(i - idx).cloned() {
                    let combined = format!("{} [{}]", prev, current);
                    ingredients[i - idx] = Ingredient::Line(Cow::Owned(combined));
                    ingredients.remove(i);
                    continue;
                }
            }
        }
        i += 1;
    }
}

impl From<KalorioTextRecipe> for Recipe {
    fn from(r: KalorioTextRecipe) -> Self {
        let (category, keywords) = match r.keywords.as_slice() {
            [first, rest @ ..] => (Some(first.to_string()), rest.to_vec()),
            [] => (None, Vec::new()),
        };

        Self {
            r#type: AtType::Recipe.to_opt(),
            author: vec![RecipeAuthorFieldEnum::new_person(&r.author)],
            is_based_on: r
                .kalorio_version
                .map(|s| vec![RecipeIsBasedOnFieldEnum::new_creative_work_text(&s)])
                .unwrap_or_default(),
            keywords: keywords
                .into_iter()
                .map(RecipeKeywordsFieldEnum::TextOrURL)
                .collect(),
            name: vec![r.title],
            recipe_category: category.map(|c| vec![c]).unwrap_or_default(),
            recipe_ingredient: r.ingredients,
            recipe_instructions: r.instructions,
            recipe_yield: Default::default(),
            ..Default::default()
        }
    }
}

/// Parses a Kalorio text file to extract the recipes from the file's content.
pub fn parse<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;

    let mut recipes = parse_txt(&content)?
        .into_iter()
        .map(Recipe::from)
        .collect::<Vec<_>>();

    let is_based_on = recipes.last().map(|r| r.is_based_on.clone());

    if let Some(based_on) = is_based_on {
        recipes
            .iter_mut()
            .for_each(|r| r.is_based_on = based_on.clone());
    }

    Ok(recipes)
}

fn parse_txt(input: &str) -> Result<Vec<KalorioTextRecipe>> {
    Ok(many0(alt((
        map(
            preceded(take_while_m_n(1, 10, is_vchar_or_space), line_ending),
            |_| None,
        ),
        map(recipe, |r| Some(r.into())),
    )))
    .parse(input)
    .map(|(_, recipes)| recipes.into_iter().flatten().collect())?)
}

fn recipe(input: &str) -> IResult<&str, RecipeComponents<'_>> {
    map(
        (
            line_ending,
            title,
            ingredients,
            instructions,
            opt(fingerprint),
            opt(keywords),
            opt(author),
            opt(registered_on),
            footer,
            opt(version),
        ),
        |(_, title, ingredients, instructions, _, keywords, author, _, _, version)| {
            RecipeComponents {
                title,
                author,
                keywords: keywords.unwrap_or_default(),
                ingredients,
                instructions,
                version: version.map(|s| s.trim()),
            }
        },
    )
    .parse(input)
}

fn title(input: &str) -> IResult<&str, &str> {
    preceded(space1, terminated(take_until("\n"), many1(line_ending))).parse(input)
}

fn ingredients(input: &str) -> IResult<&str, Vec<Ingredient<'_>>> {
    alt((twocolumn, onecolumn)).parse(input)
}

fn twocolumn(input: &str) -> IResult<&str, Vec<Ingredient<'_>>> {
    many1(alt((
        map(section, |s| vec![Ingredient::Section(Cow::Borrowed(s))]),
        map(
            (tag("    "), ingredtwo, tag("    "), ingredone, line_ending),
            |(_, ing1, _, ing2, _)| vec![ing1, ing2],
        ),
        map(
            (tag("    "), ingredone, many1(line_ending)),
            |(_, ing, _)| vec![ing],
        ),
    )))
    .parse(input)
    .map(|(rest, nested)| (rest, nested.into_iter().flatten().collect()))
}

fn ingredtwo(input: &str) -> IResult<&str, Ingredient<'_>> {
    map(
        recognize((
            amount,
            char(' '),
            unit,
            char(' '),
            take_while_m_n(27, 27, is_vchar_or_space),
        )),
        |s| Ingredient::Line(Cow::Borrowed(s.trim())),
    )
    .parse(input)
}

fn onecolumn(input: &str) -> IResult<&str, Vec<Ingredient<'_>>> {
    many0(alt((
        map(section, |s| Ingredient::Section(Cow::Borrowed(s))),
        terminated(ingredone, line_ending),
        map((space0, line_ending), |_| {
            Ingredient::Line(Cow::Borrowed(""))
        }),
    )))
    .parse(input)
}

fn ingredone(input: &str) -> IResult<&str, Ingredient<'_>> {
    map(
        recognize((
            amount,
            char(' '),
            unit,
            char(' '),
            take_while_m_n(1, 27, is_vchar_or_space),
        )),
        |s| Ingredient::Line(Cow::Borrowed(s.trim())),
    )
    .parse(input)
}

fn section(input: &str) -> IResult<&str, &str> {
    map(
        (
            (opt(line_ending), char(' ')),
            recognize((
                satisfy(|c| c.is_alphabetic()),
                take_while(is_vchar_or_space),
            )),
            line_ending,
        ),
        |(_, s, _)| s,
    )
    .parse(input)
}

fn amount(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 3, |c: char| is_vchar_or_space(c) || c == '.' || c == '/').parse(input)
}

fn unit(input: &str) -> IResult<&str, &str> {
    take_while_m_n(2, 2, is_vchar_or_space).parse(input)
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction<'_>>> {
    many1(map(instruction, |s| Instruction::Line(Cow::Borrowed(s)))).parse(input)
}

fn instruction(input: &str) -> IResult<&str, &str> {
    terminated(
        verify(take_until("\n\n"), |text: &str| {
            !text.starts_with(":") && !text.trim_start().starts_with("-----")
        }),
        (line_ending, line_ending),
    )
    .parse(input)
}

fn fingerprint(input: &str) -> IResult<&str, &str> {
    delimited(
        tag(":Fingerprint: "),
        take_until("\n"),
        (line_ending, opt(line_ending)),
    )
    .parse(input)
}

fn keywords(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        tag(":Stichworte: "),
        terminated(
            separated_list1(
                char(','),
                preceded(
                    space0,
                    take_while_m_n(1, 18, |c: char| is_vchar_or_space(c) && c != ','),
                ),
            ),
            line_ending,
        ),
    )
    .parse(input)
}

fn author(input: &str) -> IResult<&str, &str> {
    preceded(
        tag(":Erfasser/Name: "),
        terminated(
            take_while(is_vchar_or_space),
            (line_ending, opt(line_ending)),
        ),
    )
    .parse(input)
}

fn registered_on(input: &str) -> IResult<&str, &str> {
    preceded(
        tag(":Erfasst am: "),
        terminated(take_while(is_vchar_or_space), (line_ending, line_ending)),
    )
    .parse(input)
}

fn footer(input: &str) -> IResult<&str, &str> {
    delimited(
        opt(line_ending),
        tag("----------------------------------------------------------------------------"),
        line_ending,
    )
    .parse(input)
}

fn version(input: &str) -> IResult<&str, &str> {
    preceded(
        char(' '),
        preceded(
            take_until("Kalorio"),
            alt((take_until("["), take_while(is_vchar_or_space))),
        ),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod test_recipes {
        use super::*;
        use std::io::Cursor;

        #[test]
        fn test_txt() -> Result<()> {
            let file = files::txt();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, results::all_recipes_txt());
            Ok(())
        }
    }

    mod files {
        pub fn txt<'a>() -> &'a str {
            r##"
 Ananas-Käsekuchen

    150  g Kokoszwieback                    1 kl Ananas
     60  g Butter                                a 1 kg
    600  g Doppelrahmfrischkäse             2 EL Rum
    150  g Saure Sahne                    200 ml Ananassaft
    120  g Zucker                           3 EL Vanille-Puddingpulver
      4    Eier                             1 Sp Zitrone
      1    Limette                         20  g Kokosraspel
      1 EL Mehl

Zwieback in der Küchenmaschine gron zerkleinern. Butter schmelzen und
unter die Brösel mischen. Den Boden einer Springform (24 cm
Durchmesser) mit Backpapier auslegen. Die Brösel als Boden darauf
verteilen und gut andrücken. Kalt stellen.

Frischkäse, saure Sahne und 100 g Zucker mit den Quirlen des
Handrührers glattrühren. Nacheinander die Eier dazugeben. Die
Limettenschale fein abreiben, die Limette auspressen. Limettensaft
und Mehl unter die Käsemasse heben. Die Masse auf dem Boden in der
Springform verteilen. Im heissen Backofen auf der 2. Einschubleiste
von
unten bei 160GradC 15 Minuten backen. Dann die Temperatur auf 150GradC
reduzieren und weitere 35 Minuten backen. Den Kuchen in der Form auf
einem Gitter auskühlen lassen und dann mindestens 2 Stunden kalt
stellen.

3. Die Ananas grosszügig schälen, vierteln und den Strunk entfernen.
Jedes Viertel längs halbieren und quer in 4 mm dicke Scheiben
schneiden. Restlichen Zucker in einem Topf mit dem Rum und Ananassaft
zum Kochen bringen. Ananasscheiben dazugeben und aufkochen.
Puddingpulver mit etwas Rum, Wasser und einem Zitronenspritzer
glattrühren
und das Kompott damitbinden. Das Kompott auskühlen lassen und die
Hälfte auf dem
Käsekuchen verteilen. Den Kuchen mit Kokosraspeln garnieren.

Das restliche Ananaskompott extra zum Kuchen servieren.

:Stichworte: Ananas, Käse, Kuchen
:Erfasser/Name: Petra Holzapfel
:Erfasst am: 9.2.2

----------------------------------------------------------------------------

 Kräuter-Focaccia

    500  g Mehl                             1    Zweig Rosmarin
      1    Päckchen Trockenhefe             1    Zweig Salbei
           Salz                                  Eigelb
      9 EL Öl                               1 EL grobes Meersalz
      1 EL Honig                                 Backpapier
      1    Zweig Thymian

1. Mehl, Hefe und l TL Salz vermengen; mit 5 EL Öl, Honig und 1/41
warmem Wasser zu einem Teig verkneten. An einem warmen Ort
zugedeckt 30 Min. gehen lassen. Krauter waschen, trocken schütteln.
Blätter hacken, mit restlichem Öl verrühren.

2. Den Teig halbieren. Jede Hälfte rund (ca. 30 cm 0) ausrollen. Die
eine mit der Kräutermasse bestreichen, die andere darüber legen. Den
Teig an den Rändern gut zusammendrücken und mit den Fingern einige
Mulden hineindrücken. Das Brot auf ein mit Backpapier belegtes Blech
legen und an einem warmen Ort zugedeckt weitere 30 Min. gehen lassen.

3. Den Ofen auf 200° (Umluft 180°) vorheizen. Das Blech mit der
Focaccia in den heißen Ofen (Mitte) schieben. 150 ml Wasser in die
Fettpfanne des Ofens gießen. Das Brot ca. 15 Min. backen. Das Eigelb
mit 2 EL Wasser verquirlen, den Teig damit bestreichen und mit grobem
Meersalz bestreuen. Die Temperatur auf 180° (Umluft 160°) reduzieren
und die Focaccia in weiteren 15 Min. fertig backen.

Passt hervorragend zum Raclette oder Fondue

:Stichworte: Kräuter, Raclette, Fondue
:Erfasser/Name: Jochen 'Nunz' Herz
:Erfasst am: 6.1.2008

----------------------------------------------------------------------------

 Zwiebelkuchen

 TEIG
    250  g Mehl                             1 Pr Salz
    125  g Margarine                        1    Eier

 FÜLLUNG
      6 gr Zwiebeln                              Kümmel
     75  g Butter                                gemahlen
      1 Be Joghurt                               Pfeffer
      1 Be Saure Sahne                           Paprika
           oder Schmand                          edelsüss
      3    Eier                             1 TL Speisestärke

Mehl, Margarine und Salz verrühren, 1 Ei schnell unterkneten und Teig
1-2 Stunden kalt stellen.
Teig in 28er Kuchenform auswellen und hohen Rand kneten.

Zwiebel schälen und in Streifen schneiden und in 75 g Butter und
eventuell etwas Fondor eine halbe Stunde dünsten.
Kalt stellen.

Joghurt, Sahne, Eier, Speisestärke, Paprika, Salz und Pfeffer
verrühren. und die Zwiebeln untermischen.
Die Füllung auf den Teigboden und den rand herunterdrücken damit er
nicht übersteht.

Oben mit Kümmel oder Käse nach Belieben verfeindern und im
vorgeheizten Backofen bei 220 Grad 30 Minuten backen.

:Stichworte: Backen, Zwiebeln
:Erfasser/Name: Jochen 'Nunz' Herz

----------------------------------------------------------------------------
 Recipe handled by Kalorio! V4.04 [ unregistered ]"##
        }
    }

    mod results {
        use schema_org::{ItemList, Recipe, field::ItemListItemListElementFieldEnum};

        use super::*;

        pub fn all_recipes_txt() -> Vec<Recipe> {
            vec![
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    author: vec![RecipeAuthorFieldEnum::new_person("Petra Holzapfel")],
                    is_based_on: vec![RecipeIsBasedOnFieldEnum::new_creative_work_text(
                        "Kalorio! V4.04".into(),
                    )],
                    keywords: vec![
                        RecipeKeywordsFieldEnum::TextOrURL("Käse".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("Kuchen".into()),
                    ],
                    name: vec!["Ananas-Käsekuchen".into()],
                    recipe_category: vec!["Ananas".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("150 g Kokoszwieback".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 kl Ananas [a 1 kg]".into()),
                        RecipeRecipeIngredientFieldEnum::Text("60 g Butter".into()),
                        RecipeRecipeIngredientFieldEnum::Text("600 g Doppelrahmfrischkäse".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 EL Rum".into()),
                        RecipeRecipeIngredientFieldEnum::Text("150 g Saure Sahne".into()),
                        RecipeRecipeIngredientFieldEnum::Text("200 ml Ananassaft".into()),
                        RecipeRecipeIngredientFieldEnum::Text("120 g Zucker".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3 EL Vanille-Puddingpulver".into()),
                        RecipeRecipeIngredientFieldEnum::Text("4 Eier".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Sp Zitrone".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Limette".into()),
                        RecipeRecipeIngredientFieldEnum::Text("20 g Kokosraspel".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 EL Mehl".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Zwieback in der Küchenmaschine gron zerkleinern. Butter schmelzen und unter die Brösel mischen. Den Boden einer Springform (24 cm Durchmesser) mit Backpapier auslegen. Die Brösel als Boden darauf verteilen und gut andrücken. Kalt stellen.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Frischkäse, saure Sahne und 100 g Zucker mit den Quirlen des Handrührers glattrühren. Nacheinander die Eier dazugeben. Die Limettenschale fein abreiben, die Limette auspressen. Limettensaft und Mehl unter die Käsemasse heben. Die Masse auf dem Boden in der Springform verteilen. Im heissen Backofen auf der 2. Einschubleiste von unten bei 160GradC 15 Minuten backen. Dann die Temperatur auf 150GradC reduzieren und weitere 35 Minuten backen. Den Kuchen in der Form auf einem Gitter auskühlen lassen und dann mindestens 2 Stunden kalt stellen.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Die Ananas grosszügig schälen, vierteln und den Strunk entfernen. Jedes Viertel längs halbieren und quer in 4 mm dicke Scheiben schneiden. Restlichen Zucker in einem Topf mit dem Rum und Ananassaft zum Kochen bringen. Ananasscheiben dazugeben und aufkochen. Puddingpulver mit etwas Rum, Wasser und einem Zitronenspritzer glattrühren und das Kompott damitbinden. Das Kompott auskühlen lassen und die Hälfte auf dem Käsekuchen verteilen. Den Kuchen mit Kokosraspeln garnieren.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("Das restliche Ananaskompott extra zum Kuchen servieren.".into()),
                    ],
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    author: vec![RecipeAuthorFieldEnum::new_person("Jochen 'Nunz' Herz")],
                    is_based_on: vec![RecipeIsBasedOnFieldEnum::new_creative_work_text(
                        "Kalorio! V4.04".into(),
                    )],
                    keywords: vec![
                        RecipeKeywordsFieldEnum::TextOrURL("Raclette".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("Fondue".into()),
                    ],
                    name: vec!["Kräuter-Focaccia".into()],
                    recipe_category: vec!["Kräuter".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("500 g Mehl".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Zweig Rosmarin".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Päckchen Trockenhefe".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Zweig Salbei".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Salz".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Eigelb".into()),
                        RecipeRecipeIngredientFieldEnum::Text("9 EL Öl".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 EL grobes Meersalz".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 EL Honig".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Backpapier".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Zweig Thymian".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Mehl, Hefe und l TL Salz vermengen; mit 5 EL Öl, Honig und 1/41 warmem Wasser zu einem Teig verkneten. An einem warmen Ort zugedeckt 30 Min. gehen lassen. Krauter waschen, trocken schütteln. Blätter hacken, mit restlichem Öl verrühren.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Den Teig halbieren. Jede Hälfte rund (ca. 30 cm 0) ausrollen. Die eine mit der Kräutermasse bestreichen, die andere darüber legen. Den Teig an den Rändern gut zusammendrücken und mit den Fingern einige Mulden hineindrücken. Das Brot auf ein mit Backpapier belegtes Blech legen und an einem warmen Ort zugedeckt weitere 30 Min. gehen lassen.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Den Ofen auf 200° (Umluft 180°) vorheizen. Das Blech mit der Focaccia in den heißen Ofen (Mitte) schieben. 150 ml Wasser in die Fettpfanne des Ofens gießen. Das Brot ca. 15 Min. backen. Das Eigelb mit 2 EL Wasser verquirlen, den Teig damit bestreichen und mit grobem Meersalz bestreuen. Die Temperatur auf 180° (Umluft 160°) reduzieren und die Focaccia in weiteren 15 Min. fertig backen.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("Passt hervorragend zum Raclette oder Fondue".into()),
                    ],
                ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    author: vec![RecipeAuthorFieldEnum::new_person("Jochen 'Nunz' Herz")],
                    is_based_on: vec![RecipeIsBasedOnFieldEnum::new_creative_work_text(
                        "Kalorio! V4.04".into(),
                    )],
                    keywords: vec![
                        RecipeKeywordsFieldEnum::TextOrURL("Zwiebeln".into()),
                    ],
                    name: vec!["Zwiebelkuchen".into()],
                    recipe_category: vec!["Backen".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::ItemList(ItemList {
                            r#type: AtType::ItemList.to_string(),
                            item_list_element: vec![
                                ItemListItemListElementFieldEnum::Text("250 g Mehl".into()),
                                ItemListItemListElementFieldEnum::Text("1 Pr Salz".into()),
                                ItemListItemListElementFieldEnum::Text("125 g Margarine".into()),
                                ItemListItemListElementFieldEnum::Text("1 Eier".into()),
                            ],
                            number_of_items: vec![4],
                            name: vec!["TEIG".into()],
                            ..Default::default()
                        }),
                        RecipeRecipeIngredientFieldEnum::ItemList(ItemList {
                            r#type: AtType::ItemList.to_string(),
                            item_list_element: vec![
                                ItemListItemListElementFieldEnum::Text("6 gr Zwiebeln".into()),
                                ItemListItemListElementFieldEnum::Text("Kümmel [gemahlen]".into()),
                                ItemListItemListElementFieldEnum::Text("75 g Butter".into()),
                                ItemListItemListElementFieldEnum::Text("1 Be Joghurt".into()),
                                ItemListItemListElementFieldEnum::Text("Pfeffer".into()),
                                ItemListItemListElementFieldEnum::Text("1 Be Saure Sahne [oder Schmand]".into()),
                                ItemListItemListElementFieldEnum::Text("Paprika [edelsüss]".into()),
                                ItemListItemListElementFieldEnum::Text("3 Eier".into()),
                                ItemListItemListElementFieldEnum::Text("1 TL Speisestärke".into()),
                            ],
                            number_of_items: vec![9],
                            name: vec!["FÜLLUNG".into()],
                            ..Default::default()
                        }),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Mehl, Margarine und Salz verrühren, 1 Ei schnell unterkneten und Teig 1-2 Stunden kalt stellen. Teig in 28er Kuchenform auswellen und hohen Rand kneten.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Zwiebel schälen und in Streifen schneiden und in 75 g Butter und eventuell etwas Fondor eine halbe Stunde dünsten. Kalt stellen.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Joghurt, Sahne, Eier, Speisestärke, Paprika, Salz und Pfeffer verrühren. und die Zwiebeln untermischen. Die Füllung auf den Teigboden und den rand herunterdrücken damit er nicht übersteht.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Oben mit Kümmel oder Käse nach Belieben verfeindern und im vorgeheizten Backofen bei 220 Grad 30 Minuten backen.".into(),
                        ),
                    ],
                    ..Default::default()
                },
            ]
        }
    }
}
