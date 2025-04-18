mod apps;
mod error;

pub use error::{Error, Result};

use std::io::Read;

use iso8601::DateTime;

use crate::core::integrations::apps::cooklang::CookLang;
use crate::core::integrations::apps::{accuchef, cheftap, cookmate, crouton, mealmaster, recipemd, recipesage, saffron};
use crate::core::model::recipe::{NutritionForCreate, Sections, TimesForCreate, ToolForCreate};
use crate::core::scraper::schema::RecipeSchema;

/// Represents a collection of recipe management applications.
/// Each variant corresponds to a specific recipe or cooking-related app.
pub enum App {
    AccuChef,
    BigOven,
    ChefTap,
    Cooklang,
    CookMate,
    Crouton,
    MealMaster,
    RecipeMD,
    RecipeSage,
    Saffron,
}

/// Represents the supported file formats for some applications.
pub enum FileFormat {
    Json,
    Txt,
    Xml,
}

/// Represents a recipe along with its metadata and various attributes. This structure is typically used
/// for storing detailed information about a recipe and its associated properties.
#[derive(Debug, Default, PartialEq)]
struct IntegrationRecipe {
    affordability_rating: Option<u8>,
    appearance_rating: Option<u8>,
    author: Option<String>,
    category: Option<String>,
    comments: Vec<String>,
    cookbook: Option<String>,
    created_at: Option<DateTime>,
    cuisine: Option<String>,
    description: Option<String>,
    diet: Vec<String>,
    difficulty: Option<String>,
    effort_rating: Option<u8>,
    images: Vec<String>,
    ingredients: Sections,
    instructions: Sections,
    keywords: Vec<String>,
    nutrition: Option<NutritionForCreate>,
    rating: Option<u8>,
    schema: Option<RecipeSchema>,
    source: Option<String>,
    taste_rating: Option<u8>,
    times: Option<TimesForCreate>,
    title: String,
    tools: Vec<ToolForCreate>,
    yield_: Option<i16>,
    updated_at: Option<DateTime>,
    videos: Vec<String>,
}

/// Parses a recipe from the given input source and returns a vector of `IntegrationRecipe` objects.
pub fn parse_recipe<R>(
    r: R,
    app: App,
    file_name: &str,
    file_format: FileFormat,
) -> Result<Vec<IntegrationRecipe>>
where
    R: Read,
{
    match app {
        App::AccuChef => accuchef::parse(r),
        App::BigOven => accuchef::parse(r),
        App::ChefTap => cheftap::parse(r),
        App::Cooklang => CookLang::default().parse(r, file_name),
        App::CookMate => match file_format {
            FileFormat::Xml => cookmate::parse(r),
            _ => Err(Error::UnsupportedFileFormat),
        },
        App::Crouton => crouton::parse(r),
        App::MealMaster => mealmaster::parse(r),
        App::RecipeMD => recipemd::parse(r),
        App::RecipeSage => match file_format {
            FileFormat::Json => recipesage::parse_json(r),
            FileFormat::Txt => recipesage::parse_txt(r),
            FileFormat::Xml => recipesage::parse_xml(r),
        },
        App::Saffron => saffron::parse(r),
    }
}
