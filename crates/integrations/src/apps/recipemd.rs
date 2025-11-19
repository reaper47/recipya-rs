use std::io::{Read, Seek};

use recipemd::{Factor, Ingredient, Recipe};
use schema_org::field::{
    RecipeDescriptionFieldEnum, RecipeKeywordsFieldEnum, RecipeRecipeIngredientFieldEnum,
    RecipeRecipeInstructionsFieldEnum,
};

use crate::apps::helpers::read_file;
use crate::error::Result;
use crate::helpers::to_yield;

/// Parses a RecipeMD recipe from the file's content.
pub fn parse<R>(r: R) -> Result<Vec<schema_org::Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;
    let recipe = Recipe::parse(content.as_str())?;

    let ingredients = ingredients_to_string(recipe.ingredients)
        .into_iter()
        .map(RecipeRecipeIngredientFieldEnum::Text)
        .collect::<Vec<_>>();
    let ingredients_with_groups = recipe
        .ingredient_groups
        .into_iter()
        .map(|group| {
            RecipeRecipeIngredientFieldEnum::new_section(
                &group.title,
                ingredients_to_string(group.ingredients)
                    .iter()
                    .map(|s| s.as_str())
                    .collect(),
            )
        })
        .collect::<Vec<_>>();

    Ok(vec![schema_org::Recipe {
        description: vec![RecipeDescriptionFieldEnum::Text(
            recipe.description.unwrap_or_default(),
        )],
        keywords: recipe
            .tags
            .into_iter()
            .map(RecipeKeywordsFieldEnum::TextOrURL)
            .collect(),
        name: vec![recipe.title],
        recipe_ingredient: if ingredients_with_groups.is_empty() {
            ingredients
        } else {
            ingredients_with_groups
        },
        recipe_instructions: recipe
            .instructions
            .unwrap_or_default()
            .replace("\r\n", "\n\n")
            .split("\n\n")
            .map(|s| RecipeRecipeInstructionsFieldEnum::Text(s.replace("\n", " ")))
            .collect(),
        recipe_yield: to_yield(
            recipe
                .yields
                .first()
                .map(|amount| match amount.factor {
                    Factor::Integer(n) => n as i16,
                    Factor::Fraction(numerator, denominator) => (numerator / denominator) as i16,
                    Factor::Float(n) => n as i16,
                })
                .unwrap_or_default() as i64,
        ),
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

        pretty_assertions::assert_eq!(
            got,
            vec![schema_org::Recipe {
                description: vec![RecipeDescriptionFieldEnum::TextOrURL("Some people call it guac.".into())],
                keywords: vec!["sauce", "vegan"].map(DefinedText::new).collect(),
                name: vec!["Guacamole".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("1 avocado".into()),
                    RecipeRecipeIngredientFieldEnum::Text("0.5 teaspoon salt".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1.5 pinches red pepper flakes".into()),
                    RecipeRecipeIngredientFieldEnum::Text("lemon juice".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text(
                        "Remove flesh from avocado and roughly mash with fork. Season to taste with salt, pepper and lemon juice.".into()
                    ),
                    RecipeRecipeInstructionsFieldEnum::Text("Eat, mix and sleep!".into()),
                ],
                recipe_yield: to_yield(4),
                ..Default::default()
            }]
        );
        Ok(())
    }
}
