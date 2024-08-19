fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/";

    let recipe = lib_scraper::scrape(url)?;
    let name = match recipe.name {
        Some(name) => name,
        _ => "".to_string(),
    };

    Ok(())
}
