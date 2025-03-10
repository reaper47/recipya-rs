use std::fmt::Write;

use super::RecipeDetails;
use crate::core::model::{Error, Result};
use crate::server::templates::data::FormattedTimes;

impl RecipeDetails {
    /// Converts the `Recipe` instance into a Markdown-formatted `String`.
    pub fn to_markdown(&self, base_url: &str) -> Result<String> {
        let mut md = String::new();

        writeln!(&mut md, "# {}", self.recipe.name)?;
        writeln!(&mut md)?;

        if let Some(main_image) = self.recipe.image {
            writeln!(
                &mut md,
                "![Image of the recipe]({base_url}/data/images/{}.webp)",
                main_image
            )?;
            writeln!(&mut md)?;
        }

        writeln!(&mut md, "## Metadata")?;
        writeln!(&mut md)?;
        let formatted_times = FormattedTimes::from_times(&self.times).map_err(|_| Error::Time)?;
        writeln!(&mut md, "Prep time: {}", formatted_times.prep)?;
        writeln!(&mut md, "Cook time: {}", formatted_times.cook)?;
        writeln!(&mut md, "Total time: **{}**", formatted_times.total)?;
        writeln!(&mut md)?;
        writeln!(&mut md, "Category: {}", self.category)?;
        if let Some(cuisine) = &self.cuisine {
            writeln!(&mut md, "Cuisine: {cuisine}")?;
        }
        if !self.keywords.is_empty() {
            writeln!(&mut md, "Keywords: {}", self.keywords.join(", "))?;
        }
        writeln!(&mut md, "Yield: {}", self.recipe.yield_)?;
        writeln!(&mut md)?;

        if let Some(description) = &self.recipe.description {
            writeln!(&mut md, "## Description")?;
            writeln!(&mut md)?;

            writeln!(&mut md, "{}", description)?;
            writeln!(&mut md)?;
        }

        if !self.tools.is_empty() {
            writeln!(&mut md, "## Tools")?;
            writeln!(&mut md)?;

            for tool in self.tools.iter() {
                writeln!(&mut md, "* {} {}", tool.quantity, tool.name)?;
            }
            writeln!(&mut md)?;
        }

        if !self.ingredients.is_empty() {
            writeln!(&mut md, "## Ingredients")?;
            writeln!(&mut md)?;

            for (section, ingredients) in &self.ingredients {
                writeln!(&mut md, "### {section}")?;
                writeln!(&mut md)?;

                for ingredient in ingredients {
                    writeln!(&mut md, "* {ingredient}")?;
                }
                writeln!(&mut md)?;
            }
        }

        if !self.instructions.is_empty() {
            writeln!(&mut md, "## Instructions")?;
            writeln!(&mut md)?;

            for (section, instructions) in &self.instructions {
                writeln!(&mut md, "### {section}")?;
                writeln!(&mut md)?;

                for (idx, instruction) in instructions.iter().enumerate() {
                    writeln!(&mut md, "{}. {instruction}", idx + 1)?;
                }
                writeln!(&mut md)?;
            }
        }

        if let Some(nutrition) = &self.nutrition {
            write!(&mut md, "## Nutrition Facts")?;
            if let Some(serving_size) = &nutrition.serving_size {
                writeln!(&mut md, " (per {serving_size})")?;
            } else {
                writeln!(&mut md)?;
            }
            writeln!(&mut md)?;

            writeln!(&mut md, "|                        |          |")?;
            writeln!(&mut md, "|------------------------|----------|")?;

            if let Some(calories) = nutrition.calories_kcal {
                writeln!(&mut md, "| **Calories**           | {} kcal |", calories)?;
            }

            if let Some(total_fat) = nutrition.total_fat_g {
                writeln!(&mut md, "| **Total fat**          | {total_fat}g       |")?;

                if let Some(saturated) = nutrition.saturated_fat_g {
                    writeln!(&mut md, "| - Saturated fat        | {saturated}g       |")?;
                }

                if let Some(unsaturated) = nutrition.unsaturated_fat_g {
                    writeln!(&mut md, "| - Unsaturated fat      | {unsaturated}g       |")?;
                }

                if let Some(trans) = nutrition.trans_fat_g {
                    writeln!(&mut md, "| - Saturated fat        | {trans}g       |")?;
                }
            }

            if let Some(cholesterol) = nutrition.cholesterol_mg {
                writeln!(&mut md, "| **Cholesterol**        | {cholesterol}mg      |")?;
            }

            if let Some(sodium) = nutrition.sodium_mg {
                writeln!(&mut md, "| **Sodium**             | {sodium}mg     |")?;
            }

            if let Some(carbs) = nutrition.total_carbohydrates {
                writeln!(&mut md, "| **Total carbohydrate** | {carbs}g      |")?;

                if let Some(fiber) = nutrition.fiber_g {
                    writeln!(&mut md, "| - Dietary fiber        | {fiber}g      |")?;
                }

                if let Some(sugars) = nutrition.sugars_g {
                    writeln!(&mut md, "| - Total sugars         | {sugars}g      |")?;
                }
            }

            if let Some(protein) = nutrition.protein_g {
                writeln!(&mut md, "| **Protein**            | {protein}g      |")?;
            }
            writeln!(&mut md)?;
        }

        if let Some(source) = &self.recipe.source {
            writeln!(&mut md, "Source: {}", source)?;
        }

        Ok(md)
    }
}

#[cfg(test)]
mod tests {
    use crate::server::test_utils::a_complete_recipe;
    use uuid::Uuid;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_to_markdown_complete_recipe_ok() -> Result<()> {
        let mut recipe = a_complete_recipe();
        recipe.recipe.image = Some(Uuid::parse_str("637bcefb-9fa4-4970-b490-b73023ac772f")?);

        let got = recipe.to_markdown("https://example.com/images")?;
        let expected = r####"# Best Chinese Kale

![Image of the recipe](https://example.com/images/data/images/637bcefb-9fa4-4970-b490-b73023ac772f.webp)

## Metadata

Prep time: 1h
Cook time: 15m
Total time: **1h 15m**

Category: dinner
Cuisine: thai
Keywords: vegetarian, tofu
Yield: 4

## Description

This is the most delicious recipe!

## Tools

* 1 wok
* 1 frying pan

## Ingredients

### Sauce

* 1 cup blue spinach
* 1/2 tbsp cinnamon

### Main

* 4 pounds top quality chicken filet
* 1/8 cup lemon juice

## Instructions

### Sauce

1. Mix all these ingredients

### Chicken

1. Turn the oven at 300 F
2. Soak the chicken in the lemon juice
3. Bake for 35 minutes

## Nutrition Facts (per 100g)

|                        |          |
|------------------------|----------|
| **Calories**           | 300 kcal |
| **Total fat**          | 6g       |
| - Saturated fat        | 1g       |
| - Unsaturated fat      | 2g       |
| - Saturated fat        | 3g       |
| **Cholesterol**        | 5mg      |
| **Sodium**             | 12mg     |
| **Total carbohydrate** | 55g      |
| - Dietary fiber        | 10g      |
| - Total sugars         | 43g      |
| **Protein**            | 7g      |

Source: https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/
"####;
        pretty_assertions::assert_eq!(expected, got);
        Ok(())
    }
}
