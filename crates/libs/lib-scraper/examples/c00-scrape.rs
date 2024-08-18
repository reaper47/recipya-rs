fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/";
    let _recipe = lib_scraper::scrape(url)?;
    Ok(())
}
