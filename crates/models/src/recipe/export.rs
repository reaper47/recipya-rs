use std::fmt::Write;

use crate::recipe::structs::recipe::RecipeDetails;
use crate::recipe::structs::section::SectionComponents;
use crate::time::FormattedTimes;
use crate::{Error, Result};

impl RecipeDetails {
    /// Converts the `Recipe` instance into a Markdown-formatted `String`.
    #[allow(clippy::too_many_lines)]
    pub fn to_markdown(&self, base_url: &str) -> Result<String> {
        let mut md = String::new();

        writeln!(&mut md, "# {}", self.recipe.name)?;
        writeln!(&mut md)?;

        if let Some(main_image) = self.recipe.image {
            writeln!(
                &mut md,
                "![Image of the recipe]({base_url}/data/images/{main_image}.webp)",
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

            writeln!(&mut md, "{description}")?;
            writeln!(&mut md)?;
        }

        if !self.tools.is_empty() {
            writeln!(&mut md, "## Tools")?;
            writeln!(&mut md)?;

            for tool in &self.tools {
                writeln!(&mut md, "* {} {}", tool.quantity, tool.name)?;
            }
            writeln!(&mut md)?;
        }

        if !self.ingredients.is_empty() {
            writeln!(&mut md, "## Ingredients")?;
            writeln!(&mut md)?;

            match &self.ingredients {
                SectionComponents::Grouped(section_items) => {
                    for section in section_items {
                        writeln!(&mut md, "### {}", section.title)?;
                        writeln!(&mut md)?;

                        for item in &section.items {
                            writeln!(&mut md, "* {}", item.text)?;
                        }
                        writeln!(&mut md)?;
                    }
                }
                SectionComponents::Flat(items) => {
                    for item in items {
                        writeln!(&mut md, "* {}", item.text)?;
                    }
                    writeln!(&mut md)?;
                }
            }
        }

        if !self.instructions.is_empty() {
            writeln!(&mut md, "## Instructions")?;
            writeln!(&mut md)?;

            match &self.instructions {
                SectionComponents::Grouped(section_items) => {
                    for section in section_items {
                        writeln!(&mut md, "### {}", section.title)?;
                        writeln!(&mut md)?;

                        for (idx, item) in section.items.iter().enumerate() {
                            writeln!(&mut md, "{}. {}", idx + 1, item.text)?;
                        }
                        writeln!(&mut md)?;
                    }
                }
                SectionComponents::Flat(items) => {
                    for (idx, item) in items.iter().enumerate() {
                        writeln!(&mut md, "{}. {}", idx + 1, item.text)?;
                    }
                    writeln!(&mut md)?;
                }
            }
        }

        if let Some(nutrition) = &self.nutrition.per_100g {
            write!(&mut md, "## Nutrition Facts")?;
            writeln!(&mut md, " (per 100g)")?;
            writeln!(&mut md)?;

            writeln!(&mut md, "|                        |          |")?;
            writeln!(&mut md, "|------------------------|----------|")?;

            if let Some(calories) = nutrition.calories_kcal {
                writeln!(&mut md, "| **Calories**           | {calories} kcal |")?;
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

        let src = self.recipe.source.as_str();
        if !src.is_empty() {
            writeln!(&mut md, "Source: {src}")?;
        }

        Ok(md)
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::recipe::structs::test_utils::a_complete_recipe;

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
