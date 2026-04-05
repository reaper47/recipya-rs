use schema_org::{Recipe, field::RecipeRecipeInstructionsFieldEnum};
use scraper::Html;

pub fn add_info(_doc: &Html, mut recipe: Recipe) -> Recipe {
    if recipe.recipe_instructions.len() == 1
        && let RecipeRecipeInstructionsFieldEnum::Text(s) = recipe.recipe_instructions[0].clone()
    {
        recipe.recipe_instructions = s
            .lines()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|s| RecipeRecipeInstructionsFieldEnum::Text(s.to_string()))
            .collect();
    }

    recipe
}
