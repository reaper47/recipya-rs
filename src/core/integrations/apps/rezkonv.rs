//! Parses the REZCONV file format.
//!
//! The following versions are supported:
//!     - 'Kalorio V4.03' nach REZKONV

use std::borrow::Cow;
use std::io::Read;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::streaming::take_until;
use nom::bytes::{take_till, take_while};
use nom::character::complete::{char, digit1, line_ending, space0, space1};
use nom::combinator::{map, map_res, not, opt, recognize};
use nom::multi::{many0, many1, separated_list0};
use nom::sequence::{delimited, preceded, terminated};
use nom::{IResult, Parser};

use crate::core::integrations::apps::helpers::{Ingredient, Instruction, ToSections};
use crate::core::integrations::helpers::{
    sections_to_itemlist, sections_to_vec, to_defined_text, to_is_based_on, to_organization_type,
    to_yield,
};
use crate::core::integrations::{Error, Result};
use crate::core::scraper::schema::{AtType, RecipeCategory, RecipeSchema};

struct RecipeComponents<'a> {
    software_version: &'a str,
    title: &'a str,
    category: Vec<&'a str>,
    r#yield: i16,
    ingredients: Vec<Ingredient<'a>>,
    instructions: Vec<Instruction<'a>>,
    keywords: Vec<&'a str>,
    author: Option<&'a str>,
    erfasst: Option<&'a str>,
}

impl From<RecipeComponents<'_>> for RecipeSchema {
    fn from(r: RecipeComponents<'_>) -> Self {
        let (category, _) = match r
            .category
            .into_iter()
            .filter(|&s| !s.trim().is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .as_slice()
        {
            [first, rest @ ..] => (Some(first).cloned(), rest.to_vec()),
            [] => (None, Vec::new()),
        };

        let (category, keywords) = match category {
            None => match r.keywords.as_slice() {
                [first, rest @ ..] => (first.to_string(), rest.to_vec()),
                [] => (String::new(), Vec::new()),
            },
            Some(c) => (c, r.keywords),
        };

        Self {
            at_context: Default::default(),
            at_type: Some(AtType::Recipe),
            author: to_organization_type(r.author.unwrap_or_default().into()),
            is_based_on: to_is_based_on(r.software_version.replace("(unreg.) ", "")),
            keywords: to_defined_text(keywords.join(",")),
            name: Some(r.title.into()),
            recipe_category: RecipeCategory::Text(category),
            recipe_ingredient: sections_to_vec(
                r.ingredients
                    .into_iter()
                    .fold(Vec::new(), |mut acc, item| {
                        match item {
                            Ingredient::Line(s) if s.starts_with('-') => match acc.last_mut() {
                                Some(Ingredient::Line(prev)) => {
                                    *prev = Cow::Owned(format!(
                                        "{} [{}]",
                                        prev,
                                        s.replace("-", "").trim()
                                    ));
                                }
                                _ => acc.push(Ingredient::Line(s)),
                            },
                            _ => acc.push(item),
                        }
                        acc
                    })
                    .to_sections(),
            ),
            recipe_instructions: sections_to_itemlist(r.instructions.to_sections()),
            recipe_yield: to_yield(r.r#yield as i64),
            ..Default::default()
        }
    }
}

/// Parses the recipes in a REZKONV file.
pub fn parse<R>(mut r: R) -> Result<Vec<RecipeSchema>>
where
    R: Read,
{
    let mut content = String::new();
    r.read_to_string(&mut content)?;
    Ok(parse_recipes(&content)?
        .into_iter()
        .map(RecipeSchema::from)
        .collect())
}

fn parse_recipes(input: &str) -> Result<Vec<RecipeComponents>> {
    many1(recipe)
        .parse(input)
        .map(|(_, recipes)| recipes.into_iter().collect())
        .map_err(|err| Error::Parse(err.to_string()))
}

fn recipe(input: &str) -> IResult<&str, RecipeComponents> {
    map(
        (
            terminated(header, line_ending),
            title,
            opt(category),
            servings,
            many1(line_ending),
            ingredients,
            opt(quelle),
            instructions,
            keywords,
            opt(author),
            opt(erfasst),
            footer,
        ),
        |(
            header,
            title,
            category,
            servings,
            _,
            ingredients,
            _,
            instructions,
            keywords,
            author,
            erfasst,
            _,
        )| {
            RecipeComponents {
                software_version: header,
                title,
                category: category.unwrap_or_default(),
                r#yield: servings,
                ingredients,
                instructions,
                keywords,
                author,
                erfasst,
            }
        },
    )
    .parse(input)
}

fn header(input: &str) -> IResult<&str, &str> {
    terminated(
        preceded(tag("========== "), take_till(|c| c == '\n')),
        line_ending,
    )
    .parse(input)
}

fn title(input: &str) -> IResult<&str, &str> {
    delimited(
        (space0, tag("Titel: ")),
        take_till(|c| c == '\n'),
        line_ending,
    )
    .parse(input)
}

fn category(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        (space0, tag("Kategorien: ")),
        separated_list0(
            tag(","),
            nom::bytes::complete::take_till(|c| c == ',' || c == '\n'),
        ),
        line_ending,
    )
    .parse(input)
}

fn servings(input: &str) -> IResult<&str, i16> {
    map_res(
        delimited(
            (space0, tag("Menge: ")),
            digit1,
            (take_till(|c| c == '\n'), line_ending),
        ),
        |digits: &str| digits.parse::<i16>(),
    )
    .parse(input)
}

fn ingredients(input: &str) -> IResult<&str, Vec<Ingredient>> {
    terminated(alt((ingredients_list, ingredient_sections)), line_ending).parse(input)
}

fn ingredients_list(input: &str) -> IResult<&str, Vec<Ingredient>> {
    many1(ingredient).parse(input)
}

fn ingredient_sections(input: &str) -> IResult<&str, Vec<Ingredient>> {
    many1(preceded(not(stopping_section), alt((section, ingredient)))).parse(input)
}

fn stopping_section(input: &str) -> IResult<&str, &str> {
    recognize((
        opt(line_ending),
        take_while(|c| c == '='),
        tag(" QUELLE "),
        take_while(|c| c == '='),
        line_ending,
    ))
    .parse(input)
}

fn section(input: &str) -> IResult<&str, Ingredient> {
    map(
        delimited(
            (
                opt(line_ending),
                tag("="),
                take_while(|c| c == '='),
                char(' '),
            ),
            take_until(" ="),
            (tag(" ="), take_while(|c| c == '='), line_ending),
        ),
        |s| Ingredient::Section(Cow::Borrowed(s)),
    )
    .parse(input)
}

fn ingredient(input: &str) -> IResult<&str, Ingredient> {
    map(
        delimited((tag("    "), space0), take_till(|c| c == '\n'), line_ending),
        |s: &str| {
            Ingredient::Line(Cow::Owned(
                s.split_whitespace().collect::<Vec<_>>().join(" "),
            ))
        },
    )
    .parse(input)
}

fn quelle(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        stopping_section,
        terminated(many1(tabbed_line), line_ending),
    )
    .parse(input)
}

fn tabbed_line(input: &str) -> IResult<&str, &str> {
    delimited(space1, take_till(|c: char| c == '\n'), line_ending).parse(input)
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    preceded(
        not(metadata_stop),
        map(paragraphs, |s| {
            s.split("\n\n")
                .map(|s| Instruction::Line(Cow::Borrowed(s)))
                .collect()
        }),
    )
    .parse(input)
}

fn paragraphs(input: &str) -> IResult<&str, &str> {
    terminated(
        alt((take_until("\n\n:"), take_until("\r\n\r\n:"))),
        double_line_ending,
    )
    .parse(input)
}

fn metadata_stop(input: &str) -> IResult<&str, &str> {
    recognize((tag(":"), take_while(|c| c != ':'), tag(": "))).parse(input)
}

fn double_line_ending(input: &str) -> IResult<&str, &str> {
    recognize((line_ending, line_ending)).parse(input)
}

fn keywords(input: &str) -> IResult<&str, Vec<&str>> {
    many0(keyword).parse(input)
}

fn keyword(input: &str) -> IResult<&str, &str> {
    delimited(tag(":Stichworte: "), take_till(|c| c == '\n'), line_ending).parse(input)
}

fn author(input: &str) -> IResult<&str, &str> {
    delimited(
        tag(":Erfasser/Name: "),
        take_till(|c| c == '\n'),
        line_ending,
    )
    .parse(input)
}

fn erfasst(input: &str) -> IResult<&str, &str> {
    delimited(tag(":Erfasst am: "), take_till(|c| c == '\n'), line_ending).parse(input)
}

fn footer(input: &str) -> IResult<&str, &str> {
    recognize((many1(line_ending), tag("====="), line_ending)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_recipes {
        use super::*;

        use std::io::Cursor;

        type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

        #[test]
        fn test_kalorio_v4_03() -> Result<()> {
            let file = files::kalorio_v4_03();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, results::kalorio_v4_03());
            Ok(())
        }
    }

    mod files {
        pub fn kalorio_v4_03<'a>() -> &'a str {
            r##"========== 'Kalorio V4.03' (unreg.) nach REZKONV

     Titel: Ananas-Käsekuchen
Kategorien: , , 
     Menge: 1 Kuchen

    150     Gramm  Kokoszwieback
     60     Gramm  Butter
    600     Gramm  Doppelrahmfrischkäse
    150     Gramm  Saure Sahne
    120     Gramm  Zucker
      4            Eier
      1            Limette
      1      Essl  Mehl
      1     klein. Ananas
                   - a 1 kg
      2      Essl. Rum
    200        ml  Ananassaft
      3      Essl. Vanille-Puddingpulver
      1  Spritzer  Zitrone
     20     Gramm  Kokosraspel

============================= QUELLE ================================
           essen & trinken 1/2002
           Gepostet von: Petra Holzapfel
           Überarbeitet für Kalorio
           - Erfasst am 9.2.2 von
           - Petra Holzapfel

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

:Stichworte: Ananas
:Stichworte: Käse
:Stichworte: Kuchen
:Erfasser/Name: Petra Holzapfel
:Erfasst am: 9.2.2


=====
========== 'Kalorio V4.03' (unreg.) nach REZKONV

     Titel: Flammenkuchen von Marc
Kategorien: 
     Menge: 2 Backbleche


========================== Für den Brotteig: ========================
      6      Essl. Mehl
    1/2      Teel. ; Salz
      1     Prise  ; Pfeffer
    1/2            Würfel Hefe
    1/4      Litr. ; Wasser
                   - (lauwarm)

============================ Für den Belag ==========================
    250     Gramm  Quark
                   - 40% Fett
      1        Be  Saure Sahne
    1/2      Teel. ; Salz
      1     Prise  ; Pfeffer
      1            Zitrone

========================== Für die Garnitur =========================
      5      groß. Zwiebel
    125     Gramm  Durchwachsener Speck
    125     Gramm  Reibkäse
                   - (optional)

============================= QUELLE ================================
           Von meinem Kollegen Marc Hurstel
           - Erfasst am 8.3.1999 von
           - Jochen Herz

Zuerst den Teig vorbereiten: Mehl,Salz und Pfeffer in eine Schüssel
geben. Die Hefe im Wasser auflösen und anschließend über das Mehl
gießen, dabei mit einem Handmixer oder gleich mit der Hand kneten. Den
Teig rühren bis er nicht mehr klebt (eventuell noch etwas mehr Mehl
beimischen). 
Den Teig aus der Schüssel herrausnehmen und mit den Händen auf 
der Arbeitsplatte weiter verarbeiten, bis er schön geschmeidig wird.
Dabei immer ein bißchen Mehl auf die Platte streuen damit der Teig
nicht kleben bleibt ! Den Teig während der Zubereitung
der Sauce und Garnitur ruhen lassen.

Für die Sauce (Belag), den Quark, die saure Sahne, Salz und Pfeffer in
einer Schüssel mischen. Damit diese Sauce einen noch sauerlicheren
Geschmack bekommt, wird der Saft einer Zitrone untergerührt.

Die Zwiebeln schälen und halbieren, dann in dünne Scheiben (1 bis 2 mm
dick) schneiden. Den Speck in kleine Würfel schneiden.
Den Teigballen in 2 gleichgroße Hälften teilen. Jede Hälfte so dünn
wie möglich ausrollen (je dünner, umso schmackhafter !). Die
Teigplatten auf Backbleche mit Backpapier legen. Die Sauce dünn
auftragen. Die Zwiebeln gleichmäßig darauf verteilen. Ebenso die
Speckwürfel. Eventuell auch Käse dazu.
(Traditioneller Flammenkuchen ist ohne Käse !)

Backzeit: 20 Minuten im vorgeheizten Backofen bei 200°C (oder bis der
Teig an den Rändern goldbraun wird)
Dazu ein Kerner Spätlese halbtrocken aus der Pfalz oder ganz einfach
ein schönes elsäßisches Bier (Fischer Ambré).
Und jetzt: "A güata !" wie man bei uns sagt

:Stichworte: Frankreich
:Stichworte: Elsass
:Stichworte: Hefe
:Erfasser/Name: Jochen Herz
:Erfasst am: 8.3.1999


=====
========== 'Kalorio V4.03' (unreg.) nach REZKONV

     Titel: Möhren-Mandel-Muffins
Kategorien: , , , 
     Menge: 12 Muffins

      4            Eiweiß
      4            Eigelb
     50     Gramm  Zucker
    200     Gramm  geraspelte Möhren
    200     Gramm  gemahlene Mandeln
      1      Teel. Kirschwasser
    1/2      Teel. Zimt
    1/2            Zitrone (Schale davon)
     25     Gramm  Stärkemehl
    150     Gramm  Puderzucker
                   Zitronensaft

============================= QUELLE ================================
           - Erfasst am 18.8.2006 von
           - Jochen 'Nunz' Herz

Die Eiweiß zu festem Schnee schlagen, Eigelb und Zucker schaumig
rühren. Dann die geraspelten Möhren in die Eiermasse rühren, die
Mandeln nach und nach dazugeben. Kirschwasser, Zimt, Zitronenschale
und Stärkemehl unter die Teigmasse rühren, zum Schluss den Eischnee
vorsichtig unter die Teigmasse heben.

Muffinform mit Papierförmchen auslegen (evtl. leicht fetten) und die
Teigmasse gleichmäßig auf die zwölf Förmchen verteilen. 20 Minuten auf
mittlerer Schiene im Ofen bei 175°C abbacken (Umluft 160°C).
Puderzucker und Zitronensaft zu einem glattem Guss verrühren und die
fertigen Muffins damit bestreichen.

:Stichworte: Muffins
:Stichworte: Karotten
:Erfasser/Name: Jochen 'Nunz' Herz
:Erfasst am: 18.8.2006


=====
========== 'Kalorio V4.03' (unreg.) nach REZKONV

     Titel: Zwiebelkuchen
Kategorien: , 
     Menge: 4 Portionen


================================ Teig ===============================
    250     Gramm  Mehl
    125     Gramm  Margarine
      1     Prise  Salz
      1            Eier

=============================== Füllung =============================
      6      groß. Zwiebeln
     75     Gramm  Butter
      1        Be  Joghurt
      1        Be  Saure Sahne
                   - oder Schmand
      3            Eier
                   Kümmel
                   - gemahlen
                   Pfeffer
                   Paprika
                   - edelsüss
      1      Teel. Speisestärke

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

:Stichworte: Backen
:Stichworte: Zwiebeln
:Erfasser/Name: Jochen 'Nunz' Herz


=====
"##
        }
    }

    mod results {
        use super::*;

        use crate::core::model::recipe::Sections;

        pub fn kalorio_v4_03() -> Vec<RecipeSchema> {
            vec![RecipeSchema {
                at_context: Default::default(),
                at_type: Some(AtType::Recipe),
                author: to_organization_type("Petra Holzapfel".into()), 
                is_based_on: to_is_based_on("'Kalorio V4.03' nach REZKONV".into()), 
                keywords: to_defined_text(["Käse", "Kuchen"].join(",")), 
                name: Some("Ananas-Käsekuchen".into()), 
                recipe_category: RecipeCategory::Text("Ananas".into()), 
                recipe_ingredient: sections_to_vec(Sections::from([
                    ("".into(), vec![
                        "150 Gramm Kokoszwieback".into(),
                        "60 Gramm Butter".into(),
                        "600 Gramm Doppelrahmfrischkäse".into(),
                        "150 Gramm Saure Sahne".into(),
                        "120 Gramm Zucker".into(),
                        "4 Eier".into(),
                        "1 Limette".into(),
                        "1 Essl Mehl".into(),
                        "1 klein. Ananas [a 1 kg]".into(),
                        "2 Essl. Rum".into(),
                        "200 ml Ananassaft".into(),
                        "3 Essl. Vanille-Puddingpulver".into(),
                        "1 Spritzer Zitrone".into(),
                        "20 Gramm Kokosraspel".into(),
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
                recipe_yield: to_yield(1),
                ..Default::default()
            }, RecipeSchema {
                at_context: Default::default(),
                at_type: Some(AtType::Recipe),
                author: to_organization_type("Jochen Herz".into()), 
                is_based_on: to_is_based_on("'Kalorio V4.03' nach REZKONV".into()), 
                keywords: to_defined_text(["Elsass", "Hefe"].join(",")), 
                name: Some("Flammenkuchen von Marc".into()), 
                recipe_category: RecipeCategory::Text("Frankreich".into()), 
                recipe_ingredient: sections_to_vec(Sections::from([
                    ("Für den Brotteig:".into(), vec![
                        "6 Essl. Mehl".into(),
                        "1/2 Teel. ; Salz".into(),
                        "1 Prise ; Pfeffer".into(),
                        "1/2 Würfel Hefe".into(),
                        "1/4 Litr. ; Wasser [(lauwarm)]".into()
                    ]),
                    ("Für den Belag".into(), vec![
                        "250 Gramm Quark [40% Fett]".into(),
                        "1 Be Saure Sahne".into(),
                        "1/2 Teel. ; Salz".into(),
                        "1 Prise ; Pfeffer".into(),
                        "1 Zitrone".into(),
                    ]),
                    ("Für die Garnitur".into(), vec![
                        "5 groß. Zwiebel".into(),
                        "125 Gramm Durchwachsener Speck".into(),
                        "125 Gramm Reibkäse [(optional)]".into(),
                    ])
                ])),
                recipe_instructions: sections_to_itemlist(Sections::from([
                    ("".into(), vec![
                        "Zuerst den Teig vorbereiten: Mehl,Salz und Pfeffer in eine Schüssel geben. Die Hefe im Wasser auflösen und anschließend über das Mehl gießen, dabei mit einem Handmixer oder gleich mit der Hand kneten. Den Teig rühren bis er nicht mehr klebt (eventuell noch etwas mehr Mehl beimischen). Den Teig aus der Schüssel herrausnehmen und mit den Händen auf der Arbeitsplatte weiter verarbeiten, bis er schön geschmeidig wird. Dabei immer ein bißchen Mehl auf die Platte streuen damit der Teig nicht kleben bleibt ! Den Teig während der Zubereitung der Sauce und Garnitur ruhen lassen.".into(),
                        "Für die Sauce (Belag), den Quark, die saure Sahne, Salz und Pfeffer in einer Schüssel mischen. Damit diese Sauce einen noch sauerlicheren Geschmack bekommt, wird der Saft einer Zitrone untergerührt.".into(), 
                        "Die Zwiebeln schälen und halbieren, dann in dünne Scheiben (1 bis 2 mm dick) schneiden. Den Speck in kleine Würfel schneiden. Den Teigballen in 2 gleichgroße Hälften teilen. Jede Hälfte so dünn wie möglich ausrollen (je dünner, umso schmackhafter !). Die Teigplatten auf Backbleche mit Backpapier legen. Die Sauce dünn auftragen. Die Zwiebeln gleichmäßig darauf verteilen. Ebenso die Speckwürfel. Eventuell auch Käse dazu. (Traditioneller Flammenkuchen ist ohne Käse !)".into(), 
                        r#"Backzeit: 20 Minuten im vorgeheizten Backofen bei 200°C (oder bis der Teig an den Rändern goldbraun wird) Dazu ein Kerner Spätlese halbtrocken aus der Pfalz oder ganz einfach ein schönes elsäßisches Bier (Fischer Ambré). Und jetzt: "A güata !" wie man bei uns sagt"#.into(),
                    ])
                ])),
                recipe_yield: to_yield(2),
                ..Default::default()
            }, RecipeSchema {
                at_context: Default::default(),
                at_type: Some(AtType::Recipe),
                author: to_organization_type("Jochen 'Nunz' Herz".into()), 
                is_based_on: to_is_based_on("'Kalorio V4.03' nach REZKONV".into()), 
                keywords: to_defined_text("Karotten".into()), 
                name: Some("Möhren-Mandel-Muffins".into()), 
                recipe_category: RecipeCategory::Text("Muffins".into()), 
                recipe_ingredient: sections_to_vec(Sections::from([
                    ("".into(), vec![
                        "4 Eiweiß".into(),
                        "4 Eigelb".into(),
                        "50 Gramm Zucker".into(),
                        "200 Gramm geraspelte Möhren".into(),
                        "200 Gramm gemahlene Mandeln".into(),
                        "1 Teel. Kirschwasser".into(),
                        "1/2 Teel. Zimt".into(),
                        "1/2 Zitrone (Schale davon)".into(),
                        "25 Gramm Stärkemehl".into(),
                        "150 Gramm Puderzucker".into(),
                        "Zitronensaft".into(),
                    ]),
                ])),
                recipe_instructions: sections_to_itemlist(Sections::from([
                    ("".into(), vec![
                        "Die Eiweiß zu festem Schnee schlagen, Eigelb und Zucker schaumig rühren. Dann die geraspelten Möhren in die Eiermasse rühren, die Mandeln nach und nach dazugeben. Kirschwasser, Zimt, Zitronenschale und Stärkemehl unter die Teigmasse rühren, zum Schluss den Eischnee vorsichtig unter die Teigmasse heben.".into(),
                        "Muffinform mit Papierförmchen auslegen (evtl. leicht fetten) und die Teigmasse gleichmäßig auf die zwölf Förmchen verteilen. 20 Minuten auf mittlerer Schiene im Ofen bei 175°C abbacken (Umluft 160°C). Puderzucker und Zitronensaft zu einem glattem Guss verrühren und die fertigen Muffins damit bestreichen.".into(),
                    ])
                ])),
                recipe_yield: to_yield(12),
                ..Default::default()
            }, RecipeSchema {
                at_context: Default::default(),
                at_type: Some(AtType::Recipe),
                author: to_organization_type("Jochen 'Nunz' Herz".into()), 
                is_based_on: to_is_based_on("'Kalorio V4.03' nach REZKONV".into()), 
                keywords: to_defined_text("Zwiebeln".into()), 
                name: Some("Zwiebelkuchen".into()), 
                recipe_category: RecipeCategory::Text("Backen".into()), 
                recipe_ingredient: sections_to_vec(Sections::from([
                    ("Teig".into(), vec![
                        "250 Gramm Mehl".into(),
                        "125 Gramm Margarine".into(),
                        "1 Prise Salz".into(),
                        "1 Eier".into(),
                    ]),
                    ("Füllung".into(), vec![
                        "6 groß. Zwiebeln".into(),
                        "75 Gramm Butter".into(),
                        "1 Be Joghurt".into(),
                        "1 Be Saure Sahne [oder Schmand]".into(),
                        "3 Eier".into(),
                        "Kümmel [gemahlen]".into(),
                        "Pfeffer".into(),
                        "Paprika [edelsüss]".into(),
                        "1 Teel. Speisestärke".into(),
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
                recipe_yield: to_yield(4),
                ..Default::default()
            }]
        }
    }
}
