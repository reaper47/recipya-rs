use std::io::Read;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while_m_n, take_while1};
use nom::bytes::take_while;
use nom::character::complete::{char, space0, space1};
use nom::combinator::{map, map_res, opt, recognize, verify};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{delimited, preceded, terminated};
use nom::{IResult, Parser};
use tracing::error;

use crate::core::integrations::error::{Error, Result};
use crate::core::model::recipe::Sections;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MealMasterRecipe {
    title: String,
    category: Option<String>,
    keywords: Vec<String>,
    yield_: i16,
    ingredients: Sections,
    instructions: Sections,
}

impl MealMasterRecipe {
    /// Parses a MealMaster recipe from the file's content.
    pub fn parse<R>(mut r: R) -> Result<Vec<Self>>
    where
        R: Read,
    {
        let mut content = String::new();
        r.read_to_string(&mut content)?;

        let recipes = parse_meal_master_recipe(&content)?;
        Ok(recipes)
    }
}

#[derive(Debug)]
struct RecipeComponents<'a> {
    header: &'a str,
    title: &'a str,
    categories: Vec<&'a str>,
    servings: i16,
    ingredients: Vec<Ingredient<'a>>,
    instructions: Vec<Instruction<'a>>,
    footer: &'a str,
}

#[derive(Debug, PartialEq)]
enum Instruction<'a> {
    Line(&'a str),
    Section(&'a str),
}

#[derive(Clone, Debug, PartialEq)]
enum Ingredient<'a> {
    Line(&'a str),
    Section(&'a str),
}

fn parse_meal_master_recipe(input: &str) -> Result<Vec<MealMasterRecipe>> {
    let (_res, recipes) = match many0(alt((
        map(
            preceded(take_while_m_n(1, 10, is_vchar_or_space), eol),
            |_| None,
        ),
        map(recipe, |r| {
            let items = r.categories.split_first();

            Some(MealMasterRecipe {
                title: r.title.to_string(),
                category: items.map(|(a, _b)| a.to_string()),
                keywords: items
                    .map(|(_a, b)| b.iter().map(|s| s.to_string()).collect())
                    .unwrap_or_default(),
                yield_: r.servings,
                ingredients: r
                    .ingredients
                    .into_iter()
                    .fold(Sections::new(), |mut acc, ing| {
                        match ing {
                            Ingredient::Line(name) => {
                                let name = name.trim().to_string();

                                if let Some((_, lines)) = acc.last_mut() {
                                    lines.push(name);
                                } else {
                                    acc.push(("".into(), vec![name]));
                                }
                            }
                            Ingredient::Section(section) => {
                                acc.push((section.to_string(), Vec::new()));
                            }
                        }
                        acc
                    })
                    .into_iter()
                    .map(|(section, lines)| {
                        let merged = (0..lines.len())
                            .filter_map(|i| {
                                let line = &lines[i];

                                if line.ends_with(';') && i + 2 < lines.len() {
                                    Some((i, format!("{} {}", line, lines[i + 2])))
                                } else if i > 1 && lines[i - 2].ends_with(';') {
                                    None
                                } else {
                                    Some((i, line.clone()))
                                }
                            })
                            .map(|(_, line)| line)
                            .collect();

                        (section, merged)
                    })
                    .collect(),
                instructions: r
                    .instructions
                    .into_iter()
                    .fold(Sections::new(), |mut acc, ins| {
                        match ins {
                            Instruction::Section(section) => {
                                acc.push((section.into(), Vec::new()));
                            }
                            Instruction::Line(line) => {
                                let line = line.trim();

                                if !line.is_empty() {
                                    if acc.is_empty() {
                                        acc.push(("".into(), Vec::new()));
                                    }

                                    if let Some((_, lines)) = acc.last_mut() {
                                        if let Some(last_line) = lines.last_mut() {
                                            let merged =
                                                format!("{last_line} {line}").trim().into();
                                            *last_line = merged;
                                        } else {
                                            lines.push(line.into());
                                        }
                                    }
                                } else if let Some((_, lines)) = acc.last_mut() {
                                    lines.push(line.into());
                                }
                            }
                        }
                        acc
                    })
                    .into_iter()
                    .map(|(section, mut lines)| {
                        if let Some(l) = lines.last() {
                            if l.is_empty() {
                                lines.pop();
                            }
                        }
                        (section, lines)
                    })
                    .collect(),
            })
        }),
    )))
    .parse(input)
    {
        Ok((i, recipes)) => (i, recipes),
        Err(err) => {
            error!("MealMaster parsing error: {err}");
            return Err(Error::Parse);
        }
    };

    Ok(recipes.into_iter().flatten().collect())
}

fn recipe(input: &str) -> IResult<&str, RecipeComponents> {
    map(
        (
            header,
            title,
            categories,
            servings,
            ingredients,
            instructions,
            footer,
        ),
        |(header, title, categories, servings, ingredients, instructions, footer)| {
            RecipeComponents {
                header,
                title,
                categories,
                servings,
                ingredients,
                instructions,
                footer,
            }
        },
    )
    .parse(input)
}

fn header(input: &str) -> IResult<&str, &str> {
    let meal_master = "Meal-Master";

    recognize((
        separator,
        take_until(meal_master),
        tag(meal_master),
        take_while(is_vchar_or_space),
        many1(eol),
    ))
    .parse(input)
}

fn title(input: &str) -> IResult<&str, &str> {
    map(
        (
            opt(char(' ')),
            tag("     Title: "),
            take_while_m_n(0, 60, is_vchar_or_space),
            eol,
        ),
        |(_, _, s, _)| s,
    )
    .parse(input)
}

fn categories(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        (opt(char(' ')), tag("Categories: ")),
        terminated(categlist, many1(eol)),
    )
    .parse(input)
}

fn categlist(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(
        char(','),
        preceded(
            space0,
            take_while_m_n(1, 11, |c: char| is_vchar_or_space(c) && c != ','),
        ),
    )
    .parse(input)
}

fn servings(input: &str) -> IResult<&str, i16> {
    map_res(
        (
            space0,
            alt((tag("Servings: "), tag("Yield: "))),
            take_while_m_n(1, 4, |c: char| c.is_ascii_digit()),
            opt((char(' '), opt(take_while_m_n(1, 10, is_vchar_or_space)))),
            many1(eol),
        ),
        |(_, _, digits, _, _)| digits.parse::<i16>(),
    )
    .parse(input)
}

fn ingredients(input: &str) -> IResult<&str, Vec<Ingredient>> {
    alt((twocolumn, onecolumn)).parse(input)
}

fn onecolumn(input: &str) -> IResult<&str, Vec<Ingredient>> {
    many0(alt((
        map(section, Ingredient::Section),
        terminated(ingredone, eol),
        map((space0, eol), |_| Ingredient::Line("")),
    )))
    .parse(input)
}

fn twocolumn(input: &str) -> IResult<&str, Vec<Ingredient>> {
    many0(alt((
        map(section, |s| vec![Ingredient::Section(s)]),
        map(
            (ingredtwo, char(' '), ingredone, many0(eol)),
            |(ing1, _, ing2, _)| vec![ing1, ing2],
        ),
        map(terminated(ingredone, many0(eol)), |ing| vec![ing]),
    )))
    .parse(input)
    .map(|(rest, nested)| (rest, nested.into_iter().flatten().collect()))
}

fn ingredone(input: &str) -> IResult<&str, Ingredient> {
    map(
        recognize((
            amount,
            char(' '),
            unit,
            space1,
            take_while_m_n(1, 28, is_vchar_or_space),
        )),
        Ingredient::Line,
    )
    .parse(input)
}

fn ingredtwo(input: &str) -> IResult<&str, Ingredient> {
    map(
        recognize((
            amount,
            char(' '),
            unit,
            char(' '),
            take_while_m_n(29, 29, is_vchar_or_space),
        )),
        Ingredient::Line,
    )
    .parse(input)
}

fn amount(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 7, |c: char| is_vchar_or_space(c) || c == '.' || c == '/').parse(input)
}

fn unit(input: &str) -> IResult<&str, &str> {
    alt((units1, units2, units3)).parse(input)
}

fn units1(input: &str) -> IResult<&str, &str> {
    alt((
        tag("x "),
        tag("sm"),
        tag("md"),
        tag("lg"),
        tag("cn"),
        tag("pk"),
        tag("pn"),
        tag("dr"),
        tag("ds"),
        tag("ct"),
        tag("bn"),
        tag("sl"),
    ))
    .parse(input)
}

fn units2(input: &str) -> IResult<&str, &str> {
    alt((
        tag("ea"),
        tag("t "),
        tag("ts"),
        tag("T "),
        tag("tb"),
        tag("fl"),
        tag("c "),
        tag("pt"),
        tag("qt"),
        tag("ga"),
        tag("oz"),
        tag("lb"),
    ))
    .parse(input)
}

fn units3(input: &str) -> IResult<&str, &str> {
    alt((
        tag("ml"),
        tag("cb"),
        tag("cl"),
        tag("dl"),
        tag("l "),
        tag("mg"),
        tag("cg"),
        tag("dg"),
        tag("g "),
        tag("kg"),
        tag("  "),
    ))
    .parse(input)
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many0(alt((
        map(section, Instruction::Section),
        map(
            terminated(
                verify(take_while_m_n(0, 255, is_vchar_or_space), |line: &str| {
                    !line.trim_start().starts_with("MMMMM")
                        && !line.trim_start().starts_with("-----")
                }),
                eol,
            ),
            Instruction::Line,
        ),
    )))
    .parse(input)
}

fn is_vchar_or_space(c: char) -> bool {
    c.is_ascii_graphic() || c == ' '
}

fn section(input: &str) -> IResult<&str, &str> {
    preceded(
        separator,
        terminated(delimited(dashes, not_dash, dashes), eol),
    )
    .parse(input)
}

fn dashes(input: &str) -> IResult<&str, &str> {
    recognize(many1(char('-'))).parse(input)
}

fn not_dash(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c != '-').parse(input) // Take until we hit a dash again
}
fn footer(input: &str) -> IResult<&str, &str> {
    terminated(separator, eol).parse(input)
}

fn separator(input: &str) -> IResult<&str, &str> {
    recognize(alt((tag("MMMMM"), tag("-----")))).parse(input)
}

fn eol(input: &str) -> IResult<&str, &str> {
    recognize(alt((tag("\r\n"), tag("\n")))).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_recipes {
        use super::*;

        #[test]
        fn test_recipe1_ok() -> Result<()> {
            let file = r##"---------- Recipe via Meal-Master (tm) v8.01

      Title: Cannoli
 Categories: Italian, Desserts
      Yield: 16 servings

----------------------------------FILLING----------------------------------
  1 1/2 c  Whole-milk ricotta cheese;      1 1/2 c  Milk chocolate;
           - well drained                           - coarsely chopped
      3 tb Sugar                             1/4 c  Pistachio nuts;
  1 1/2 ts Cinnamon                                 - coarsely chopped

-----------------------------------DOUGH-----------------------------------
      1 c  All-purpose flour                        - or dry white wine
      1 tb Sugar                               2 c  Vegetable oil
      1 tb Butter or lard                           Colored sprinkles
      4 tb To 5 Tbl sweet Marsala wine

  In a bowl, combine all the filling ingredients and mix well. Refreigerate,
  covered, until ready to fill the cannoli shells.

  To make the dough, place the flour in a bowl or food processor. Add the
  butter or lard and sugar and mix with a fork, or pulse, until the mixture
  resembles coarse meal. Slowly add the 1/4 cup of wine and shape the mixture
  into a ball; add a little more wine if the dough appears too dry. It should
  be soft but not sticky. Knead the dough on a floured surface until smooth,
  about 10 minutes.  Wrap the dough and refrigerate for 45 minutes.

  Place the chilled dough on a floured work surface. Divide the dough in
  half.  Work with 1 piece of dough at a time; keep the remaining dough
  refrigerated. Roll the dough out to a very thin long rectangle about 14
  inches long and 3 inches wide, either by hand or using a pasta machine set
  to the finest setting.  Cut the dough into 3-inch squares. Place a cannoli
  form diagnoally across 1 square. Roll the dough up around the form so the
  points meet in the center.  Seal the points with a little water. Continue
  making cylinders until all the dough is used.

  In an electric skillet, heat the vegetable oil to 375F. Fry the cannoli 3
  or 4 at a time, turning them as they brown and blister, until golden brown
  on all sides. Drain them on brown paper. When they are cool enough to
  handle, carefully slide the cannoli off the forms.

  To serve, use a long iced tea spoon or a pastry bag without a tip to fill
  the cannoli with the ricotta cheese mixture. Dip the ends into colored
  sprinkles, arrange them on a tray, and sprinkle confectioner's sugar over
  fill the cannoli just before serving - any sooner will make the shells
  soggy.
  the tops.  Serve at once.

  NOTE:  If you prefer, you can fry the cannoli in a deep fryer. Be sure to

  This recipe from CIAO ITALIA by Mary Ann Esposito

-----
"##;
            let buf = Cursor::new(file);

            let got = MealMasterRecipe::parse(buf)?;

            pretty_assertions::assert_eq!(
                got,
                vec![MealMasterRecipe {
                    title: "Cannoli".into(),
                    category: Some("Italian".into()),
                    keywords: vec!["Desserts".into()],
                    yield_: 16,
                    ingredients: Sections::from([
                        (
                            "FILLING".into(),
                            vec![
                                "1 1/2 c  Whole-milk ricotta cheese; - well drained".into(),
                                "1 1/2 c  Milk chocolate; - coarsely chopped".into(),
                                "3 tb Sugar".into(),
                                "1/4 c  Pistachio nuts; - coarsely chopped".into(),
                                "1 1/2 ts Cinnamon".into(),
                            ],
                        ),
                        (
                            "DOUGH".into(),
                            vec![
                                "1 c  All-purpose flour".into(),
                                "- or dry white wine".into(),
                                "1 tb Sugar".into(),
                                "2 c  Vegetable oil".into(),
                                "1 tb Butter or lard".into(),
                                "Colored sprinkles".into(),
                                "4 tb To 5 Tbl sweet Marsala wine".into(),
                            ],
                        ),
                    ],),
                    instructions: Sections::from([
                         (
                             "".into(),
                             vec![
                                 "In a bowl, combine all the filling ingredients and mix well. Refreigerate, covered, until ready to fill the cannoli shells.".into(),
                                 "To make the dough, place the flour in a bowl or food processor. Add the butter or lard and sugar and mix with a fork, or pulse, until the mixture resembles coarse meal. Slowly add the 1/4 cup of wine and shape the mixture into a ball; add a little more wine if the dough appears too dry. It should be soft but not sticky. Knead the dough on a floured surface until smooth, about 10 minutes.  Wrap the dough and refrigerate for 45 minutes.".into(),
                                 "Place the chilled dough on a floured work surface. Divide the dough in half.  Work with 1 piece of dough at a time; keep the remaining dough refrigerated. Roll the dough out to a very thin long rectangle about 14 inches long and 3 inches wide, either by hand or using a pasta machine set to the finest setting.  Cut the dough into 3-inch squares. Place a cannoli form diagnoally across 1 square. Roll the dough up around the form so the points meet in the center.  Seal the points with a little water. Continue making cylinders until all the dough is used.".into(),
                                 "In an electric skillet, heat the vegetable oil to 375F. Fry the cannoli 3 or 4 at a time, turning them as they brown and blister, until golden brown on all sides. Drain them on brown paper. When they are cool enough to handle, carefully slide the cannoli off the forms.".into(),
                                 "To serve, use a long iced tea spoon or a pastry bag without a tip to fill the cannoli with the ricotta cheese mixture. Dip the ends into colored sprinkles, arrange them on a tray, and sprinkle confectioner's sugar over fill the cannoli just before serving - any sooner will make the shells soggy. the tops.  Serve at once.".into(),
                                 "NOTE:  If you prefer, you can fry the cannoli in a deep fryer. Be sure to".into(),
                                 "This recipe from CIAO ITALIA by Mary Ann Esposito".into(),
                            ],
                         ),
                    ]),
                }]
            );
            Ok(())
        }
    }
}
