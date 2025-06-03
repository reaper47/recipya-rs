mod apps;
mod error;
mod helpers;

pub use error::{Error, Result};

use crate::core::integrations::apps::cooklang::CookLang;
use crate::core::integrations::apps::{
    accuchef, cheftap, cookmate, cookml, crouton, kalorio, mealmaster, paprika, recipemd,
    recipesage, rezkonv, saffron,
};
use crate::core::scraper::schema::RecipeSchema;
use std::io::{Read, Seek};

/// Represents a collection of recipe management applications.
/// Each variant corresponds to a specific recipe or cooking-related app.
pub enum App {
    AccuChef,
    BigOven,
    ChefTap,
    Cooklang,
    CookMate,
    Crouton,
    Kalorio,
    MealMaster,
    Paprika,
    RecipeMD,
    RecipeSage,
    Rezkonv,
    Saffron,
}

/// Represents the supported file formats for some applications.
pub enum FileFormat {
    CookML,
    Json,
    MCB,
    MealMaster,
    Rezkonv,
    Txt,
    Xml,
}

/// Parses a recipe from the given input source and returns a vector of `IntegrationRecipe` objects.
pub fn parse_recipe<R>(
    r: R,
    app: App,
    file_name: &str,
    file_format: FileFormat,
) -> Result<Vec<RecipeSchema>>
where
    R: Read + Seek,
{
    match app {
        App::AccuChef => accuchef::parse(r),
        App::BigOven => accuchef::parse(r),
        App::ChefTap => cheftap::parse(r),
        App::Cooklang => CookLang::default().parse(r, file_name),
        App::CookMate => match file_format {
            FileFormat::MCB => cookmate::parse_backup(r),
            FileFormat::Rezkonv => rezkonv::parse(r),
            FileFormat::MealMaster => mealmaster::parse(r),
            FileFormat::Xml => cookmate::parse(r),
            _ => Err(Error::UnsupportedFileFormat),
        },
        App::Crouton => crouton::parse(r),
        App::Kalorio => match file_format {
            FileFormat::CookML => cookml::parse(r),
            FileFormat::Txt => kalorio::parse(r),
            _ => Err(Error::UnsupportedFileFormat),
        },
        App::MealMaster => mealmaster::parse(r),
        App::Paprika => paprika::parse(r),
        App::RecipeMD => recipemd::parse(r),
        App::RecipeSage => match file_format {
            FileFormat::Json => recipesage::parse_json(r),
            FileFormat::Txt => recipesage::parse_txt(r),
            FileFormat::Xml => recipesage::parse_xml(r),
            _ => Err(Error::UnsupportedFileFormat),
        },
        App::Rezkonv => rezkonv::parse(r),
        App::Saffron => saffron::parse(r),
    }
}
