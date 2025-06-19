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
use recipe_schema::{AtType, RecipeCategory, RecipeSchema, Sections};

use super::helpers::{Ingredient, Instruction, ToSections, is_vchar_or_space, read_file};
use crate::helpers::{
    sections_to_itemlist, sections_to_vec, to_defined_text, to_is_based_on, to_organization_type,
};
use crate::{Error, Result};

struct KalorioTextRecipe {
    title: String,
    author: String,
    keywords: Vec<String>,
    instructions: Sections,
    ingredients: Sections,
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

impl From<KalorioTextRecipe> for RecipeSchema {
    fn from(r: KalorioTextRecipe) -> Self {
        let (category, keywords) = match r.keywords.as_slice() {
            [first, rest @ ..] => (Some(first.to_string()), rest.to_vec()),
            [] => (None, Vec::new()),
        };

        Self {
            at_context: Default::default(),
            at_type: Some(AtType::Recipe),
            author: to_organization_type(r.author),
            is_based_on: to_is_based_on(r.kalorio_version.unwrap_or_default()),
            keywords: to_defined_text(keywords.join(",")),
            name: Some(r.title),
            recipe_category: RecipeCategory::Text(category.unwrap_or_default()),
            recipe_ingredient: sections_to_vec(r.ingredients),
            recipe_instructions: sections_to_itemlist(r.instructions),
            recipe_yield: Default::default(),
            ..Default::default()
        }
    }
}

/// Parses a Kalorio text file to extract the recipes from the file's content.
pub fn parse<R>(r: R) -> Result<Vec<RecipeSchema>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;

    let mut recipes = parse_txt(&content)?
        .into_iter()
        .map(RecipeSchema::from)
        .collect::<Vec<_>>();

    if let Some(last_recipe) = recipes.last() {
        let is_based_on = last_recipe.is_based_on.clone();

        recipes
            .iter_mut()
            .for_each(|r| r.is_based_on = is_based_on.clone())
    }

    Ok(recipes)
}

fn parse_txt(input: &str) -> Result<Vec<KalorioTextRecipe>> {
    many0(alt((
        map(
            preceded(take_while_m_n(1, 10, is_vchar_or_space), line_ending),
            |_| None,
        ),
        map(recipe, |r| Some(r.into())),
    )))
    .parse(input)
    .map(|(_, recipes)| recipes.into_iter().flatten().collect())
    .map_err(|err| Error::Parse(err.to_string()))
}

fn recipe(input: &str) -> IResult<&str, RecipeComponents> {
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

fn ingredients(input: &str) -> IResult<&str, Vec<Ingredient>> {
    alt((twocolumn, onecolumn)).parse(input)
}

fn twocolumn(input: &str) -> IResult<&str, Vec<Ingredient>> {
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

fn ingredtwo(input: &str) -> IResult<&str, Ingredient> {
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

fn onecolumn(input: &str) -> IResult<&str, Vec<Ingredient>> {
    many0(alt((
        map(section, |s| Ingredient::Section(Cow::Borrowed(s))),
        terminated(ingredone, line_ending),
        map((space0, line_ending), |_| {
            Ingredient::Line(Cow::Borrowed(""))
        }),
    )))
    .parse(input)
}

fn ingredone(input: &str) -> IResult<&str, Ingredient> {
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

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
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
        use super::*;

        pub fn all_recipes_txt() -> Vec<RecipeSchema> {
            vec![RecipeSchema {
                at_context: Default::default(),
                at_type: Some(AtType::Recipe),
                author: to_organization_type("Petra Holzapfel".into()),
                is_based_on: to_is_based_on("Kalorio! V4.04".into()),
                keywords: to_defined_text("Käse,Kuchen".into()),
                name: Some("Ananas-Käsekuchen".into()),
                recipe_category: RecipeCategory::Text("Ananas".into()),
                recipe_ingredient: sections_to_vec(Sections::from([
                    ("".into(), vec![
                        "150 g Kokoszwieback".into(),
                        "1 kl Ananas [a 1 kg]".into(),
                        "60 g Butter".into(),
                        "600 g Doppelrahmfrischkäse".into(),
                        "2 EL Rum".into(),
                        "150 g Saure Sahne".into(),
                        "200 ml Ananassaft".into(),
                        "120 g Zucker".into(),
                        "3 EL Vanille-Puddingpulver".into(),
                        "4 Eier".into(),
                        "1 Sp Zitrone".into(),
                        "1 Limette".into(),
                        "20 g Kokosraspel".into(),
                        "1 EL Mehl".into(),                       
                    ])
                ])),
                recipe_instructions: sections_to_itemlist(Sections::from([
                    ("".into(), vec![
                        "Zwieback in der Küchenmaschine gron zerkleinern. Butter schmelzen und unter die Brösel mischen. Den Boden einer Springform (24 cm Durchmesser) mit Backpapier auslegen. Die Brösel als Boden darauf verteilen und gut andrücken. Kalt stellen.".into(),
                        "Frischkäse, saure Sahne und 100 g Zucker mit den Quirlen des Handrührers glattrühren. Nacheinander die Eier dazugeben. Die Limettenschale fein abreiben, die Limette auspressen. Limettensaft und Mehl unter die Käsemasse heben. Die Masse auf dem Boden in der Springform verteilen. Im heissen Backofen auf der 2. Einschubleiste von unten bei 160GradC 15 Minuten backen. Dann die Temperatur auf 150GradC reduzieren und weitere 35 Minuten backen. Den Kuchen in der Form auf einem Gitter auskühlen lassen und dann mindestens 2 Stunden kalt stellen.".into(),
                        "3. Die Ananas grosszügig schälen, vierteln und den Strunk entfernen. Jedes Viertel längs halbieren und quer in 4 mm dicke Scheiben schneiden. Restlichen Zucker in einem Topf mit dem Rum und Ananassaft zum Kochen bringen. Ananasscheiben dazugeben und aufkochen. Puddingpulver mit etwas Rum, Wasser und einem Zitronenspritzer glattrühren und das Kompott damitbinden. Das Kompott auskühlen lassen und die Hälfte auf dem Käsekuchen verteilen. Den Kuchen mit Kokosraspeln garnieren.".into(),
                        "Das restliche Ananaskompott extra zum Kuchen servieren.".into(),
                    ])
                ])),
                recipe_yield: Default::default(),
                ..Default::default()
            }, RecipeSchema {
                at_context: Default::default(),
                at_type: Some(AtType::Recipe),
                author: to_organization_type("Jochen 'Nunz' Herz".into()),
                is_based_on: to_is_based_on("Kalorio! V4.04".into()),
                keywords: to_defined_text("Raclette,Fondue".into()),
                name: Some("Kräuter-Focaccia".into()),
                recipe_category: RecipeCategory::Text("Kräuter".into()),
                recipe_ingredient: sections_to_vec(Sections::from([
                    ("".into(), vec![
                        "500 g Mehl".into(), 
                        "1 Zweig Rosmarin".into(),
                        "1 Päckchen Trockenhefe".into(),  
                        "1 Zweig Salbei".into(),
                        "Salz".into(),           
                        "Eigelb".into(),
                        "9 EL Öl".into(),         
                        "1 EL grobes Meersalz".into(),
                        "1 EL Honig".into(),   
                        "Backpapier".into(),
                        "1 Zweig Thymian".into(),             
                    ]),
                ])),
                recipe_instructions: sections_to_itemlist(Sections::from([
                    ("".into(), vec![
                        "1. Mehl, Hefe und l TL Salz vermengen; mit 5 EL Öl, Honig und 1/41 warmem Wasser zu einem Teig verkneten. An einem warmen Ort zugedeckt 30 Min. gehen lassen. Krauter waschen, trocken schütteln. Blätter hacken, mit restlichem Öl verrühren.".into(),
                        "2. Den Teig halbieren. Jede Hälfte rund (ca. 30 cm 0) ausrollen. Die eine mit der Kräutermasse bestreichen, die andere darüber legen. Den Teig an den Rändern gut zusammendrücken und mit den Fingern einige Mulden hineindrücken. Das Brot auf ein mit Backpapier belegtes Blech legen und an einem warmen Ort zugedeckt weitere 30 Min. gehen lassen.".into(), 
                        "3. Den Ofen auf 200° (Umluft 180°) vorheizen. Das Blech mit der Focaccia in den heißen Ofen (Mitte) schieben. 150 ml Wasser in die Fettpfanne des Ofens gießen. Das Brot ca. 15 Min. backen. Das Eigelb mit 2 EL Wasser verquirlen, den Teig damit bestreichen und mit grobem Meersalz bestreuen. Die Temperatur auf 180° (Umluft 160°) reduzieren und die Focaccia in weiteren 15 Min. fertig backen.".into(), 
                        "Passt hervorragend zum Raclette oder Fondue".into(),
                    ])
                ])),
                recipe_yield: Default::default(),
                ..Default::default()
            }, RecipeSchema {
                at_context: Default::default(),
                at_type: Some(AtType::Recipe),
                author: to_organization_type("Jochen 'Nunz' Herz".into()),
                is_based_on: to_is_based_on("Kalorio! V4.04".into()),
                keywords: to_defined_text("Zwiebeln".into()),
                name: Some("Zwiebelkuchen".into()),
                recipe_category: RecipeCategory::Text("Backen".into()),
                recipe_ingredient: sections_to_vec(Sections::from([
                    ("TEIG".into(), vec![
                        "250 g Mehl".into(),
                        "1 Pr Salz".into(),
                        "125 g Margarine".into(),
                        "1 Eier".into(),  
                    ]),
                    ("FÜLLUNG".into(), vec![
                        "6 gr Zwiebeln".into(),          
                        "Kümmel [gemahlen]".into(),
                        "75 g Butter".into(),         
                        "1 Be Joghurt".into(),     
                        "Pfeffer".into(),
                        "1 Be Saure Sahne [oder Schmand]".into(),
                        "Paprika [edelsüss]".into(),     
                        "3 Eier".into(),          
                        "1 TL Speisestärke".into(),
                    ]),
                ])),
                recipe_instructions: sections_to_itemlist(Sections::from([
                    ("".into(), vec![
                        "Mehl, Margarine und Salz verrühren, 1 Ei schnell unterkneten und Teig 1-2 Stunden kalt stellen. Teig in 28er Kuchenform auswellen und hohen Rand kneten.".into(),
                        "Zwiebel schälen und in Streifen schneiden und in 75 g Butter und eventuell etwas Fondor eine halbe Stunde dünsten. Kalt stellen.".into(), 
                        "Joghurt, Sahne, Eier, Speisestärke, Paprika, Salz und Pfeffer verrühren. und die Zwiebeln untermischen. Die Füllung auf den Teigboden und den rand herunterdrücken damit er nicht übersteht.".into(), 
                        "Oben mit Kümmel oder Käse nach Belieben verfeindern und im vorgeheizten Backofen bei 220 Grad 30 Minuten backen.".into(),
                    ])
                ])),
                recipe_yield: Default::default(),
                ..Default::default()
            }]
        }
    }
}
