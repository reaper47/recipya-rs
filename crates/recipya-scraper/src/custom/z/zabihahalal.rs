use schema_org::{Recipe, enums::RestrictedDietEnum, field::RecipeRecipeIngredientFieldEnum};
use scraper::Html;

use crate::custom::common::required_text_list;

pub fn add_info(doc: &Html, mut recipe: Recipe) -> Recipe {
    recipe.recipe_ingredient = required_text_list(&doc.root_element(), ".ingredients p")
        .unwrap()
        .into_iter()
        .skip(1)
        .map(RecipeRecipeIngredientFieldEnum::Text)
        .collect();
    recipe.suitable_for_diet = vec![RestrictedDietEnum::HalalDiet];
    recipe
}
