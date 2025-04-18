use std::io::Read;

use recipemd::{Factor, Ingredient, Recipe};

use crate::core::integrations::error::Result;
use crate::core::integrations::IntegrationRecipe;
use crate::core::model::recipe::Sections;

/// Parses a RecipeMD recipe from the file's content.
pub fn parse<R>(mut r: R) -> Result<Vec<IntegrationRecipe>>
where
    R: Read,
{
    let mut content = String::new();
    r.read_to_string(&mut content)?;

    let recipe = Recipe::parse(content.as_str())?;

    let ingredients = Sections::from([("".into(), ingredients_to_string(recipe.ingredients))]);
    let ingredients_with_groups = recipe
        .ingredient_groups
        .into_iter()
        .map(|group| (group.title, ingredients_to_string(group.ingredients)))
        .collect::<Vec<_>>();

    Ok(vec![IntegrationRecipe {
        title: recipe.title,
        description: recipe.description,
        keywords: recipe.tags,
        yield_: recipe.yields.first().map(|amount| match amount.factor {
            Factor::Integer(n) => n as i16,
            Factor::Fraction(numerator, denominator) => (numerator / denominator) as i16,
            Factor::Float(n) => n as i16,
        }),
        ingredients: if ingredients_with_groups.is_empty() {
            ingredients
        } else {
            ingredients_with_groups
        },
        instructions: Sections::from([
            ("".into(), recipe
                .instructions
                .unwrap_or_default()
                .replace("\r\n", "\n\n")
                .split("\n\n")
                .map(|s| s.replace("\n", " "))
                .collect())
        ]),
        ..Default::default()
    }])
}

fn ingredients_to_string(ingredients: Vec<Ingredient>) -> Vec<String> {
    ingredients.into_iter().map(ingredient_to_string).collect()
}

fn ingredient_to_string(ingredient: Ingredient) -> String {
    let amount = ingredient
        .amount
        .map(|amount| {
            let factor = match amount.factor {
                Factor::Integer(n) => n as f64,
                Factor::Fraction(numerator, denominator) => numerator as f64 / denominator as f64,
                Factor::Float(n) => n as f64,
            };
            let unit = amount.unit.unwrap_or_default();
            format!("{factor} {unit}").trim().to_string()
        })
        .unwrap_or_default();

    let link = match ingredient.link {
        None => "".into(),
        Some(link) => format!("[{link}]"),
    };

    format!("{amount} {} {link}", ingredient.name).trim().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_basic_recipe_ok() -> Result<()> {
        let data = r##"# Guacamole

Some people call it guac.

*sauce, vegan*

**4 Servings, 200g**

---

- *1* avocado
- *.5 teaspoon* salt
- *1 1/2 pinches* red pepper flakes
- lemon juice

---

Remove flesh from avocado and roughly mash with fork. Season to taste
with salt, pepper and lemon juice.

Eat, mix and sleep!
"##;
        let buf = Cursor::new(data.as_bytes());

        let got = parse(buf)?;

        pretty_assertions::assert_eq!(got, vec![IntegrationRecipe {
            title: "Guacamole".into(),
            description: Some(
                        "Some people call it guac.".into(),
                    ),
            keywords: vec!["sauce".into(),
      "vegan".into(),],
            yield_: Some(4),
            ingredients: Sections::from([("".into(), vec![
                "1 avocado".into(),
                "0.5 teaspoon salt".into(),
                "1.5 pinches red pepper flakes".into(),
                "lemon juice".into(),
            ])]),
            instructions: Sections::from([
                ("".into(), vec![
                    "Remove flesh from avocado and roughly mash with fork. Season to taste with salt, pepper and lemon juice.".into(),
                    "Eat, mix and sleep!".into(),
                ])
            ]),
            ..Default::default()
        }]);
        Ok(())
    }
}
