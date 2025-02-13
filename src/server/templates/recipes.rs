use maud::{html, Markup};
use crate::server::templates::data::{Data, ViewRecipe};
use crate::server::templates::layouts;

/// Renders the details of a recipe.
pub fn view_recipe(data: Data) -> Markup {
    if data.is_hx_request {
        html!(
            title hx-swap-oob="true" {
                 (data.view.recipe_details.recipe.name) " | Recipya"
            }
            (view_recipe_helper(data))
        )
    } else {
        html!()
    }
}

fn view_recipe_helper(data: Data) -> Markup {
    layouts::main("", data)
}
