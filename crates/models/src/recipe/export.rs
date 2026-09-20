use std::fmt::Write;

use crate::recipe::structs::recipe::RecipeDetails;
use crate::recipe::structs::section::SectionComponents;
use crate::time::FormattedTimes;
use crate::{Error, Result};

impl RecipeDetails {
    /// Converts the `Recipe` instance into a Markdown-formatted string.
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
        writeln!(&mut md, "Yield: {}", self.recipe.r#yield)?;
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

    /// Converts the `Recipe` instance into a text-formatted string.
    #[allow(clippy::too_many_lines)]
    pub fn to_text(&self, base_url: &str) -> Result<String> {
        let mut out = String::new();

        let minus = String::from("-");
        let equal = String::from("=");

        writeln!(&mut out, "{}", self.recipe.name)?;
        writeln!(&mut out, "{}", minus.repeat(self.recipe.name.len()))?;
        writeln!(&mut out)?;

        if let Some(main_image) = self.recipe.image {
            writeln!(
                &mut out,
                "Image of the recipe: {base_url}/data/images/{main_image}.webp",
            )?;
            writeln!(&mut out)?;
        }

        writeln!(&mut out, "Metadata")?;
        writeln!(&mut out, "{}", equal.repeat(8))?;
        writeln!(&mut out)?;
        let formatted_times = FormattedTimes::from_times(&self.times).map_err(|_| Error::Time)?;
        writeln!(&mut out, "Prep time: {}", formatted_times.prep)?;
        writeln!(&mut out, "Cook time: {}", formatted_times.cook)?;
        writeln!(&mut out, "Total time: {}", formatted_times.total)?;
        writeln!(&mut out)?;
        writeln!(&mut out, "Category: {}", self.category)?;
        if let Some(cuisine) = &self.cuisine {
            writeln!(&mut out, "Cuisine: {cuisine}")?;
        }
        if !self.keywords.is_empty() {
            writeln!(&mut out, "Keywords: {}", self.keywords.join(", "))?;
        }
        writeln!(&mut out, "Yield: {}", self.recipe.r#yield)?;
        writeln!(&mut out)?;

        if let Some(description) = &self.recipe.description {
            let desc = String::from("Description");
            writeln!(&mut out, "{desc}")?;
            writeln!(&mut out, "{}", equal.repeat(desc.len()))?;
            writeln!(&mut out)?;
            writeln!(&mut out, "{description}")?;
            writeln!(&mut out)?;
        }

        if !self.tools.is_empty() {
            let tools = String::from("Tools");
            writeln!(&mut out, "{tools}")?;
            writeln!(&mut out, "{}", equal.repeat(tools.len()))?;
            writeln!(&mut out)?;

            for tool in &self.tools {
                writeln!(&mut out, "* {} {}", tool.quantity, tool.name)?;
            }
            writeln!(&mut out)?;
        }

        if !self.ingredients.is_empty() {
            let ingredients = String::from("Ingredients");
            writeln!(&mut out, "{ingredients}")?;
            writeln!(&mut out, "{}", equal.repeat(ingredients.len()))?;
            writeln!(&mut out)?;

            match &self.ingredients {
                SectionComponents::Grouped(section_items) => {
                    for section in section_items {
                        writeln!(&mut out, "{}", section.title)?;
                        writeln!(&mut out, "{}", minus.repeat(section.title.len()))?;
                        writeln!(&mut out)?;

                        for item in &section.items {
                            writeln!(&mut out, "* {}", item.text)?;
                        }
                        writeln!(&mut out)?;
                    }
                }
                SectionComponents::Flat(items) => {
                    for item in items {
                        writeln!(&mut out, "* {}", item.text)?;
                    }
                    writeln!(&mut out)?;
                }
            }
        }

        if !self.instructions.is_empty() {
            let instructions = String::from("Instructions");
            writeln!(&mut out, "{instructions}")?;
            writeln!(&mut out, "{}", equal.repeat(instructions.len()))?;
            writeln!(&mut out)?;

            match &self.instructions {
                SectionComponents::Grouped(section_items) => {
                    for section in section_items {
                        writeln!(&mut out, "{}", section.title)?;
                        writeln!(&mut out, "{}", minus.repeat(section.title.len()))?;
                        writeln!(&mut out)?;

                        for (idx, item) in section.items.iter().enumerate() {
                            writeln!(&mut out, "{}. {}", idx + 1, item.text)?;
                        }
                        writeln!(&mut out)?;
                    }
                }
                SectionComponents::Flat(items) => {
                    for (idx, item) in items.iter().enumerate() {
                        writeln!(&mut out, "{}. {}", idx + 1, item.text)?;
                    }
                    writeln!(&mut out)?;
                }
            }
        }

        if let Some(nutrition) = &self.nutrition.per_100g {
            let nut = String::from("Nutrition Facts (per 100g)");
            writeln!(&mut out, "{nut}")?;
            writeln!(&mut out, "{}", equal.repeat(nut.len()))?;
            writeln!(&mut out)?;
            writeln!(&mut out, "|--------------------|----------|")?;

            if let Some(calories) = nutrition.calories_kcal {
                writeln!(&mut out, "| Calories           | {calories} kcal |")?;
            }

            if let Some(total_fat) = nutrition.total_fat_g {
                writeln!(&mut out, "| Total fat          | {total_fat}g       |")?;

                if let Some(saturated) = nutrition.saturated_fat_g {
                    writeln!(&mut out, "| - Saturated fat    | {saturated}g       |")?;
                }

                if let Some(unsaturated) = nutrition.unsaturated_fat_g {
                    writeln!(&mut out, "| - Unsaturated fat  | {unsaturated}g       |")?;
                }

                if let Some(trans) = nutrition.trans_fat_g {
                    writeln!(&mut out, "| - Saturated fat    | {trans}g       |")?;
                }
            }

            if let Some(cholesterol) = nutrition.cholesterol_mg {
                writeln!(&mut out, "| Cholesterol        | {cholesterol}mg      |")?;
            }

            if let Some(sodium) = nutrition.sodium_mg {
                writeln!(&mut out, "| Sodium             | {sodium}mg     |")?;
            }

            if let Some(carbs) = nutrition.total_carbohydrates {
                writeln!(&mut out, "| Total carbohydrate | {carbs}g      |")?;

                if let Some(fiber) = nutrition.fiber_g {
                    writeln!(&mut out, "| - Dietary fiber    | {fiber}g      |")?;
                }

                if let Some(sugars) = nutrition.sugars_g {
                    writeln!(&mut out, "| - Total sugars     | {sugars}g      |")?;
                }
            }

            if let Some(protein) = nutrition.protein_g {
                writeln!(&mut out, "| Protein            | {protein}g      |")?;
            }
            writeln!(&mut out)?;
        }

        let src = self.recipe.source.as_str();
        if !src.is_empty() {
            writeln!(&mut out, "Source: {src}")?;
        }

        Ok(out)
    }
}
