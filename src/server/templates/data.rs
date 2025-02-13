use crate::core::model::RecipeDetails;

/// Data holds data to pass on to the templates.
pub struct Data {
    pub is_authenticated: bool,
    pub is_hx_request: bool,

    pub view: ViewRecipe,
}

/// ViewRecipeData holds template data related to viewing a recipe.
pub struct ViewRecipe {
    pub recipe_details: RecipeDetails,
}
