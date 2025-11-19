//! Parses the REZCONV file format.
//!
//! The following versions are supported:
//!     - 'Kalorio V4.03' nach REZKONV
//!     - Recipe via Cookmate [REZKONV Export Format]

use std::borrow::Cow;
use std::io::{Read, Seek};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::streaming::take_until;
use nom::bytes::{take_till, take_while};
use nom::character::complete::{char, digit1, line_ending, not_line_ending, space0, space1};
use nom::combinator::{complete, map, map_res, not, opt, peek, recognize, rest};
use nom::multi::{many0, many1, separated_list0};
use nom::sequence::{delimited, preceded, terminated};
use nom::{IResult, Parser};

use schema_org::Recipe;
use schema_org::field::{RecipeAuthorFieldEnum, RecipeKeywordsFieldEnum};

use crate::Result;
use crate::apps::helpers::{Ingredient, Instruction, ToSections, read_file};
use crate::helpers::{to_is_based_on, to_yield};

struct RecipeComponents<'a> {
    software_version: &'a str,
    title: &'a str,
    category: Vec<&'a str>,
    r#yield: i16,
    ingredients: Vec<Ingredient<'a>>,
    instructions: Vec<Instruction<'a>>,
    keywords: Vec<&'a str>,
    author: Option<&'a str>,
    #[allow(unused)]
    erfasst: Option<&'a str>,
}

impl From<RecipeComponents<'_>> for Recipe {
    fn from(r: RecipeComponents<'_>) -> Self {
        let (category, keywords) = match r
            .category
            .into_iter()
            .filter(|&s| !s.trim().is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .as_slice()
        {
            [first, rest @ ..] => (
                first.to_string(),
                rest.iter().map(|s| s.trim().to_string()).collect(),
            ),
            [] => match r.keywords.as_slice() {
                [first, rest @ ..] => (
                    first.to_string(),
                    rest.iter().copied().map(str::to_string).collect::<Vec<_>>(),
                ),
                [] => (String::new(), Vec::new()),
            },
        };

        Self {
            author: r
                .author
                .map(|s| vec![RecipeAuthorFieldEnum::new_person(s)])
                .unwrap_or_default(),
            is_based_on: to_is_based_on(
                r.software_version
                    .replace("(unreg.) ", "")
                    .trim_start_matches("Recipe via")
                    .trim_end_matches("=====")
                    .trim()
                    .into(),
            ),
            keywords: keywords
                .into_iter()
                .map(RecipeKeywordsFieldEnum::TextOrURL)
                .collect(),
            name: vec![r.title.into()],
            recipe_category: vec![category],
            recipe_ingredient: r
                .ingredients
                .into_iter()
                .filter(|i| match i {
                    Ingredient::Line(s) => !s.is_empty(),
                    Ingredient::Section(s) => !s.is_empty(),
                })
                .fold(Vec::new(), |mut acc, item| {
                    match item {
                        Ingredient::Line(s) if s.starts_with('-') => match acc.last_mut() {
                            Some(Ingredient::Line(prev)) => {
                                *prev =
                                    Cow::Owned(format!("{}, {}", prev, s.replace("-", "").trim()));
                            }
                            _ => acc.push(Ingredient::Line(s)),
                        },
                        _ => acc.push(item),
                    }
                    acc
                })
                .to_sections(),
            recipe_instructions: r.instructions.to_sections(),
            recipe_yield: to_yield(r.r#yield as i64),
            ..Default::default()
        }
    }
}

/// Parses the recipes in a REZKONV file.
pub fn parse<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;

    Ok(parse_recipes(&content)?
        .into_iter()
        .map(Recipe::from)
        .collect())
}

fn parse_recipes(input: &str) -> Result<Vec<RecipeComponents<'_>>> {
    Ok(many1(recipe)
        .parse(input)
        .map(|(_, recipes)| recipes.into_iter().collect())?)
}

fn recipe(input: &str) -> IResult<&str, RecipeComponents<'_>> {
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
            (footer, many0(line_ending)),
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
        alt((
            preceded(
                peek(tag("========== ")),
                preceded(tag("========== "), not_line_ending),
            ),
            preceded(
                peek(tag("===== ")),
                preceded(tag("===== "), not_line_ending),
            ),
        )),
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

fn ingredients(input: &str) -> IResult<&str, Vec<Ingredient<'_>>> {
    terminated(
        alt((ingredients_list, ingredient_sections)),
        many1(line_ending),
    )
    .parse(input)
}

fn ingredients_list(input: &str) -> IResult<&str, Vec<Ingredient<'_>>> {
    many1(ingredient).parse(input)
}

fn ingredient_sections(input: &str) -> IResult<&str, Vec<Ingredient<'_>>> {
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

fn section(input: &str) -> IResult<&str, Ingredient<'_>> {
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

fn ingredient(input: &str) -> IResult<&str, Ingredient<'_>> {
    map(
        delimited(
            alt(((tag("    "), space0), (tag("  "), space0))),
            take_till(|c| c == '\n'),
            line_ending,
        ),
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

fn instructions(input: &str) -> IResult<&str, Vec<Instruction<'_>>> {
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
    alt((
        recognize(terminated(
            alt((
                complete(take_until("\n\n:")),
                complete(take_until("\n\n\n\n\n=====")),
                complete(take_until("\r\n\r\n:")),
                complete(take_until("\r\n\r\n\r\n\r\n\r\n=====")),
            )),
            double_line_ending,
        )),
        recognize(rest),
    ))
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
        fn test_cookmate() -> Result<()> {
            let file = files::cookmate();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, results::cookmate());
            Ok(())
        }

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
        pub fn cookmate<'a>() -> &'a str {
            r##"===== Recipe via Cookmate [REZKONV Export Format] =====

      Titel: Aunt Julia's Paella
Kategorien: Pork/ham, Poultry, Fish/sea, Spanish
      Menge: 6

      1    Chicken, cut up (Or 4
           -thighs 1 3/4 oz Jar

           -and legs) 2 ts Capers,

           Salt and pepper to thaste
           -4 oz Jar pimento-stiffed
           -green
      1 lb Lean pork, cut into 1-inch
           --olives
    1/2 lb Calamari (squid), cleaned
      1    md Onion, minced -and
           -sliced
      2    Toes garlic, minced 5 c
           -Water
           Cut into 1 1/2 inch
           -julliene 4 Chicken

      1 ts Saffron threads
    1/2 lg Bell pepper 2 1/2 c Uncle

      1 lg Carrot -uncooked
      1    Stalk celery 3 Hard boiled

      1    c Frozen green peas 1/2 lb
           -Unpeeled shrimp (heads
           -on)
  1 1/2 lb Peeled shrimp Oil for
           -frying


{ Submitted by Chiqui Collier, Cookery N'Orleans Restaurant }
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





=====

===== Recipe via Cookmate [REZKONV Export Format] =====

      Titel: Ausgezogenes Mehlmus
Kategorien: German
      Menge: 4

     80 g  Flour (3/4 cup) 8 Egg

      1 l  Milk (approx. 1 qt) -peaks
    120 g  Sugar (1/2 cup plus 1/2

      1 pn Salt
      8    Egg yolks, whisked to a 50

           -froth


From Central Swabia.
From grandmother's more thrifty times; rarely encountered today.
Combine the flour and a little milk, and stir until smooth. Gradually add
the remainder of the milk, the sugar and salt. Bring to a boil. Remove the
pot from the heat, add the grated lemon peel. Carefully fold in the egg
yolk froth and beaten egg whites. Pour the mixture into a buttered
casserole dish and bake at medium heat for 20 minutes.
Serves 4.
From: D'SCHWAEBISCH' KUCHE' by Aegidius Kolb and Leonhard Lidel, Allgaeuer
Zeitungsverlag, Kempten. 1976. (Translation/Conversion: Karin Brewer)
Posted by: Karin Brewer, Cooking Echo, 8/92





=====

===== Recipe via Cookmate [REZKONV Export Format] =====

      Titel: Austrian Bread Dumplings
Kategorien: Ethnic, Breads
      Menge: 6

      4 oz Dry bread, diced Salt and
           -pepper
    1/2 oz (1 Tbsp) butter or lard 1

      1    Egg -(parsley, chervil,
    1/2    c Milk -marjoram) -
      3 oz (3/4 cup) flour


Tbsp chopped fresh herbs (parsley, chervil, marjoram) - optional, but
a great improvement
You will need a frying pan, a large and a small bowl, and a saucepan of
water or soup. Fry the diced bread lightly in the fat in a frying pan.
Meanwhile, mix the egg and the milk in a small bowl. Tip the contents of
the frying pan into a large bowl, and pour the egg and milk over all. Stir
in the flour, and season with salt and pepper. Add the herbs, if using.
You may need more milk to make a soft dough. Allow it to stand for 1/2 an
hour.
Dip your hand into cold water and roll the mixture into a dozen small
balls. Put a pot of salted water on to boil, if there isn't a simmering
soup pot waiting. Drop little balls of dough into the boiling salted water
or the soup. Poach them for 10 to 15 minutes, until they are light and
firm and well risen.
Yield: 12 dumplings Time: 1 hour
Notes: You may include chopped fried bacon or cubed pork cracklings in the
mixture. Leaving out flour will result in a lighter dumpling.
From: THE OLD WORLD KITCHEN - THE RICH TRADITION OF EUROPEAN PEASANT
COOKING by Elisabeth Luard, ISBN 0-553-05219-5 Posted by: Karin Brewer,
Cooking Echo, 7/92





=====

===== Recipe via Cookmate [REZKONV Export Format] =====

      Titel: Authentic Italian Bread
Kategorien: Italian, Breads
      Menge: 2

      1 ts Active dry yeast or 1/3

           -small cake (6 grams)
           -fresh 2/3 c Milk at room
           -temperature
      1    c (135 grams) unbleached
      1    Scant tsp. malt syrup



This takes 2 days but is worth the wait. Makes 2 round loaves Starter
Stir the yeast and malt into the water; let stand until foamy, about 10
minutes. Stir in the milk and beat in the flour with a rubber spatula or
wooden spoon about 100 strokes until smooth. Cover with plastic wrap and
let stand until bubbly, at least 4 hours but preferably overnight.
Dough 2 cups water, at room temperature 6 1/4 cups (860 grams) unbleached
all-purpose flour 1 T. salt Cornmeal
Mix the starter and the water in a mixer until the starter is well broken
up. Add the flour and salt and mix for 2 to 3 minutes at low speed. The
dough will be smooth but won't pull away from the side of the bowl. Change
to the dough hook and knead at medium speed, scraping down the side of the
bowl as necessary, until the dough is elastic but slightly sticky, 3 to 4
minutes. Finish kneading by hand on a floured work surface.
First rise Place in a well-oiled bowl, cover tightly with plastic wrap, and
let rise until doubled, about 1 1/2 hours. The dough is ready when it is
very bubbled and blistered.
Shaping and second rise Cut the dough in half on a floured surface and
shape into 2 round loaves. Place on an oiled cookie sheet sprinkled with
cornmeal. Cover and let rise till doubled, about 1 hour.
Baking Preheat oven to 400 degrees F. Bake about 1 hour and cool on racks.
To get a really good crust spray the loaves with water 3 times in the first
minutes of baking.





=====

"##
        }

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

        use recipe_schema::components::{SectionItem, Sections};
        use schema_org::Recipe;
        use schema_org::field::{
            RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
        };

        pub fn cookmate() -> Vec<Recipe> {
            vec![
                Recipe {
                    is_based_on: to_is_based_on("Cookmate [REZKONV Export Format]".into()),
                    keywords: ["Poultry", "Fish/sea", "Spanish"].into_iter().map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.into())).collect(),
                    name: vec!["Aunt Julia's Paella".into()],
                    recipe_category: vec!["Pork/ham".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1 Chicken, cut up (Or 4, thighs 1 3/4 oz Jar, and legs) 2 ts Capers,".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Salt and pepper to thaste, 4 oz Jar pimentostiffed, green".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 lb Lean pork, cut into 1-inch, olives".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 lb Calamari (squid), cleaned".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 md Onion, minced -and, sliced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 Toes garlic, minced 5 c, Water".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Cut into 1 1/2 inch, julliene 4 Chicken".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 ts Saffron threads".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 lg Bell pepper 2 1/2 c Uncle".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 lg Carrot -uncooked".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Stalk celery 3 Hard boiled".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 c Frozen green peas 1/2 lb, Unpeeled shrimp (heads, on)".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 1/2 lb Peeled shrimp Oil for, frying".into()),
                    ],
                    recipe_instructions: vec![RecipeRecipeInstructionsFieldEnum::Text(
                        "{ Submitted by Chiqui Collier, Cookery N'Orleans Restaurant } In a large electric skillet or paella pan, brown the chicken pieces (that have been seasoned with salt and pepper) in a little oil. Remove from the pan. Add the pork cubes to the drippinfs and brown for about 5 minutes. Remove from the pan. To the pan drippings (add a little more oil if necessary) add the onion, garlic, bell pepper, celery and carrot. Stir-fry for 2 minutes. Add the peas, peeled shrimp, pimentos, capers, chicken, calamari and pork. Stir. In a separate pot, bring the 5 cups of water to a boil; stir in the bouillon cubes and saffron. Let it stand for 5 minutes until dissolved. Gently stir the rice into the skillet mixture. Slowly pour in enough of the bouillon mixture to cover the rice and chicken pieces. Cover and cook over low heat for about 20 minutes. Uncover and decoaratively arrange the egg slices and raw unpeeled shrimp on the top. (Add more broth as necessary to keep the rice moist. Cover and steam for another 10 minutes until the shrimp are cooked and the rice is tender. (Paella should be moist but not wet!) Place the pan on a hot pad on the serving table and let everyone help themselves. Serve with a mixed green salad, red ripe tomatoes and some French bread. Also mix up a pitcher of Sangria and enjoy! Serves: 12. [ The Legends of Louisisna Cookbook; Sheila Ainbinder; ISBN 0-671-70817-1 ] Posted by Fred Peters".into(),
                    )],
                    recipe_yield: to_yield(6),
                    ..Default::default()
                },
                Recipe {
                    is_based_on: to_is_based_on("Cookmate [REZKONV Export Format]".into()),
                    name: vec!["Ausgezogenes Mehlmus".into()],
                    recipe_category: vec!["German".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("80 g Flour (3/4 cup) 8 Egg".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 l Milk (approx. 1 qt) -peaks".into()),
                        RecipeRecipeIngredientFieldEnum::Text("120 g Sugar (1/2 cup plus 1/2".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 pn Salt".into()),
                        RecipeRecipeIngredientFieldEnum::Text("8 Egg yolks, whisked to a 50, froth".into()),
                    ],
                    recipe_instructions: vec![RecipeRecipeInstructionsFieldEnum::Text(
                        "From Central Swabia. From grandmother's more thrifty times; rarely encountered today. Combine the flour and a little milk, and stir until smooth. Gradually add the remainder of the milk, the sugar and salt. Bring to a boil. Remove the pot from the heat, add the grated lemon peel. Carefully fold in the egg yolk froth and beaten egg whites. Pour the mixture into a buttered casserole dish and bake at medium heat for 20 minutes. Serves 4. From: D'SCHWAEBISCH' KUCHE' by Aegidius Kolb and Leonhard Lidel, Allgaeuer Zeitungsverlag, Kempten. 1976. (Translation/Conversion: Karin Brewer) Posted by: Karin Brewer, Cooking Echo, 8/92".into(),
                    )],
                    recipe_yield: to_yield(4),
                    ..Default::default()
                },
                Recipe {
                    is_based_on: to_is_based_on("Cookmate [REZKONV Export Format]".into()),
                    keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("Breads".into())],
                    name: vec!["Austrian Bread Dumplings".into()],
                    recipe_category: vec!["Ethnic".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("4 oz Dry bread, diced Salt and, pepper".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 oz (1 Tbsp) butter or lard 1".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Egg -(parsley, chervil,".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 c Milk -marjoram)".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3 oz (3/4 cup) flour".into()),
                    ],
                    recipe_instructions: vec![RecipeRecipeInstructionsFieldEnum::Text(
                        "Tbsp chopped fresh herbs (parsley, chervil, marjoram) - optional, but a great improvement You will need a frying pan, a large and a small bowl, and a saucepan of water or soup. Fry the diced bread lightly in the fat in a frying pan. Meanwhile, mix the egg and the milk in a small bowl. Tip the contents of the frying pan into a large bowl, and pour the egg and milk over all. Stir in the flour, and season with salt and pepper. Add the herbs, if using. You may need more milk to make a soft dough. Allow it to stand for 1/2 an hour. Dip your hand into cold water and roll the mixture into a dozen small balls. Put a pot of salted water on to boil, if there isn't a simmering soup pot waiting. Drop little balls of dough into the boiling salted water or the soup. Poach them for 10 to 15 minutes, until they are light and firm and well risen. Yield: 12 dumplings Time: 1 hour Notes: You may include chopped fried bacon or cubed pork cracklings in the mixture. Leaving out flour will result in a lighter dumpling. From: THE OLD WORLD KITCHEN - THE RICH TRADITION OF EUROPEAN PEASANT COOKING by Elisabeth Luard, ISBN 0-553-05219-5 Posted by: Karin Brewer, Cooking Echo, 7/92".into(),
                    )],
                    recipe_yield: to_yield(6),
                    ..Default::default()
                },
                Recipe {
                    is_based_on: to_is_based_on("Cookmate [REZKONV Export Format]".into()),
                    keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("Breads".into())],
                    name: vec!["Authentic Italian Bread".into()],
                    recipe_category: vec!["Italian".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1 ts Active dry yeast or 1/3, small cake (6 grams), fresh 2/3 c Milk at room, temperature".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 c (135 grams) unbleached".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Scant tsp. malt syrup".into()),
                    ],
                    recipe_instructions: vec![RecipeRecipeInstructionsFieldEnum::Text(
                        "This takes 2 days but is worth the wait. Makes 2 round loaves Starter Stir the yeast and malt into the water; let stand until foamy, about 10 minutes. Stir in the milk and beat in the flour with a rubber spatula or wooden spoon about 100 strokes until smooth. Cover with plastic wrap and let stand until bubbly, at least 4 hours but preferably overnight. Dough 2 cups water, at room temperature 6 1/4 cups (860 grams) unbleached all-purpose flour 1 T. salt Cornmeal Mix the starter and the water in a mixer until the starter is well broken up. Add the flour and salt and mix for 2 to 3 minutes at low speed. The dough will be smooth but won't pull away from the side of the bowl. Change to the dough hook and knead at medium speed, scraping down the side of the bowl as necessary, until the dough is elastic but slightly sticky, 3 to 4 minutes. Finish kneading by hand on a floured work surface. First rise Place in a well-oiled bowl, cover tightly with plastic wrap, and let rise until doubled, about 1 1/2 hours. The dough is ready when it is very bubbled and blistered. Shaping and second rise Cut the dough in half on a floured surface and shape into 2 round loaves. Place on an oiled cookie sheet sprinkled with cornmeal. Cover and let rise till doubled, about 1 hour. Baking Preheat oven to 400 degrees F. Bake about 1 hour and cool on racks. To get a really good crust spray the loaves with water 3 times in the first minutes of baking.".into(),
                    )],
                    recipe_yield: to_yield(2),
                    ..Default::default()
                },
            ]
        }

        pub fn kalorio_v4_03() -> Vec<Recipe> {
            vec![
                Recipe {
                    author: to_organization_type("Petra Holzapfel".into()),
                    is_based_on: to_is_based_on("'Kalorio V4.03' nach REZKONV".into()),
                    keywords: ["Käse", "Kuchen"].into_iter().map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.into())).collect(),
                    name: vec!["Ananas-Käsekuchen".into()],
                    recipe_category: vec!["Ananas".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("150 Gramm Kokoszwieback".into()),
                        RecipeRecipeIngredientFieldEnum::Text("60 Gramm Butter".into()),
                        RecipeRecipeIngredientFieldEnum::Text("600 Gramm Doppelrahmfrischkäse".into()),
                        RecipeRecipeIngredientFieldEnum::Text("150 Gramm Saure Sahne".into()),
                        RecipeRecipeIngredientFieldEnum::Text("120 Gramm Zucker".into()),
                        RecipeRecipeIngredientFieldEnum::Text("4 Eier".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Limette".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Essl Mehl".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 klein. Ananas, a 1 kg".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 Essl. Rum".into()),
                        RecipeRecipeIngredientFieldEnum::Text("200 ml Ananassaft".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3 Essl. Vanille-Puddingpulver".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Spritzer Zitrone".into()),
                        RecipeRecipeIngredientFieldEnum::Text("20 Gramm Kokosraspel".into()),
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
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Das restliche Ananaskompott extra zum Kuchen servieren.".into(),
                        ),
                    ],
                    recipe_yield: to_yield(1),
                    ..Default::default()
                },
                Recipe {
                    author: to_organization_type("Jochen Herz".into()),
                    is_based_on: to_is_based_on("'Kalorio V4.03' nach REZKONV".into()),
                    keywords: ["Elsass", "Hefe"].into_iter().map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.into())).collect(),
                    name: vec!["Flammenkuchen von Marc".into()],
                    recipe_category: vec!["Frankreich".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::new_section("Für den Brotteig:", vec![
                            "6 Essl. Mehl",
                            "1/2 Teel. ; Salz",
                            "1 Prise ; Pfeffer",
                            "1/2 Würfel Hefe",
                            "1/4 Litr. ; Wasser, (lauwarm)",
                        ]),
                        RecipeRecipeIngredientFieldEnum::new_section("Für den Belag:", vec![
                            "250 Gramm Quark, 40% Fett",
                            "1 Be Saure Sahne",
                            "1/2 Teel. ; Salz",
                            "1 Prise ; Pfeffer",
                            "1 Zitrone",
                        ]),
                        RecipeRecipeIngredientFieldEnum::new_section("Für die Garnitur:", vec![
                            "5 groß. Zwiebel",
                            "125 Gramm Durchwachsener Speck",
                            "125 Gramm Reibkäse, (optional)",
                        ]),

                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Zuerst den Teig vorbereiten: Mehl,Salz und Pfeffer in eine Schüssel geben. Die Hefe im Wasser auflösen und anschließend über das Mehl gießen, dabei mit einem Handmixer oder gleich mit der Hand kneten. Den Teig rühren bis er nicht mehr klebt (eventuell noch etwas mehr Mehl beimischen). Den Teig aus der Schüssel herrausnehmen und mit den Händen auf der Arbeitsplatte weiter verarbeiten, bis er schön geschmeidig wird. Dabei immer ein bißchen Mehl auf die Platte streuen damit der Teig nicht kleben bleibt ! Den Teig während der Zubereitung der Sauce und Garnitur ruhen lassen.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Für die Sauce (Belag), den Quark, die saure Sahne, Salz und Pfeffer in einer Schüssel mischen. Damit diese Sauce einen noch sauerlicheren Geschmack bekommt, wird der Saft einer Zitrone untergerührt.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Die Zwiebeln schälen und halbieren, dann in dünne Scheiben (1 bis 2 mm dick) schneiden. Den Speck in kleine Würfel schneiden. Den Teigballen in 2 gleichgroße Hälften teilen. Jede Hälfte so dünn wie möglich ausrollen (je dünner, umso schmackhafter !). Die Teigplatten auf Backbleche mit Backpapier legen. Die Sauce dünn auftragen. Die Zwiebeln gleichmäßig darauf verteilen. Ebenso die Speckwürfel. Eventuell auch Käse dazu. (Traditioneller Flammenkuchen ist ohne Käse !)".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            r#"Backzeit: 20 Minuten im vorgeheizten Backofen bei 200°C (oder bis der Teig an den Rändern goldbraun wird) Dazu ein Kerner Spätlese halbtrocken aus der Pfalz oder ganz einfach ein schönes elsäßisches Bier (Fischer Ambré). Und jetzt: "A güata !" wie man bei uns sagt"#.into(),
                        ),
                    ],
                    recipe_yield: to_yield(2),
                    ..Default::default()
                },
                Recipe {
                    author: to_organization_type("Jochen 'Nunz' Herz".into()),
                    is_based_on: to_is_based_on("'Kalorio V4.03' nach REZKONV".into()),
                    keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("Karotten".into())],
                    name: vec!["Möhren-Mandel-Muffins".into()],
                    recipe_category: vec!["Muffins".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("4 Eiweiß".into()),
                        RecipeRecipeIngredientFieldEnum::Text("4 Eigelb".into()),
                        RecipeRecipeIngredientFieldEnum::Text("50 Gramm Zucker".into()),
                        RecipeRecipeIngredientFieldEnum::Text("200 Gramm geraspelte Möhren".into()),
                        RecipeRecipeIngredientFieldEnum::Text("200 Gramm gemahlene Mandeln".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Teel. Kirschwasser".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 Teel. Zimt".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 Zitrone (Schale davon)".into()),
                        RecipeRecipeIngredientFieldEnum::Text("25 Gramm Stärkemehl".into()),
                        RecipeRecipeIngredientFieldEnum::Text("150 Gramm Puderzucker".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Zitronensaft".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Die Eiweiß zu festem Schnee schlagen, Eigelb und Zucker schaumig rühren. Dann die geraspelten Möhren in die Eiermasse rühren, die Mandeln nach und nach dazugeben. Kirschwasser, Zimt, Zitronenschale und Stärkemehl unter die Teigmasse rühren, zum Schluss den Eischnee vorsichtig unter die Teigmasse heben.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Muffinform mit Papierförmchen auslegen (evtl. leicht fetten) und die Teigmasse gleichmäßig auf die zwölf Förmchen verteilen. 20 Minuten auf mittlerer Schiene im Ofen bei 175°C abbacken (Umluft 160°C). Puderzucker und Zitronensaft zu einem glattem Guss verrühren und die fertigen Muffins damit bestreichen.".into(),
                        ),
                    ],
                    recipe_yield: to_yield(12),
                    ..Default::default()
                },
                Recipe {
                    author: to_organization_type("Jochen 'Nunz' Herz".into()),
                    is_based_on: to_is_based_on("'Kalorio V4.03' nach REZKONV".into()),
                    keywords: to_defined_text("Zwiebeln".into()),
                    name: vec!["Zwiebelkuchen".into()],
                    recipe_category: vec!["Backen".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::new_section("Teig", vec![
                            "250 Gramm Mehl",
                            "125 Gramm Margarine",
                            "1 Prise Salz",
                            "1 Eier",
                        ]),
                        RecipeRecipeIngredientFieldEnum::new_section("Füllung", vec![
                            "6 groß. Zwiebeln",
                            "75 Gramm Butter",
                            "1 Be Joghurt",
                            "1 Be Saure Sahne, oder Schmand",
                            "3 Eier",
                            "Kümmel, gemahlen",
                            "Pfeffer",
                            "Paprika, edelsüss",
                            "1 Teel. Speisestärke",
                        ]),
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
                    recipe_yield: to_yield(4),
                    ..Default::default()
                },
            ]
        }
    }
}
