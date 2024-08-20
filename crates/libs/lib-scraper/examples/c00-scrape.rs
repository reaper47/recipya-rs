fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/";
    let recipe = lib_scraper::scrape(url)?;

    let _name = recipe.name.unwrap_or_default();
    let _ingredients = recipe.recipe_ingredient.unwrap_or_default();
    let _instructions = recipe.recipe_instructions.unwrap_or_default();

    Ok(())
}
