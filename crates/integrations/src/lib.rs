mod app;
mod apps;
mod common;
mod error;
mod fileformat;
mod helpers;

pub use app::{App, all_apps};
pub use error::{Error, Result};
pub use fileformat::FileFormat;

use std::io::{Read, Seek};

use schema_org::Recipe;

use crate::apps::cooklang::CookLang;
use crate::apps::{
    accuchef, cheftap, cookmate, cookml, crouton, kalorio, mastercook, mealmaster, paprika,
    recipemd, recipesage, rezkonv, saffron,
};

/// Parses a recipe from the given input source and returns a vector of `IntegrationRecipe` objects.
pub fn parse_recipe<R>(
    r: &mut R,
    app: &App,
    file_name: &str,
    file_format: &FileFormat,
) -> Result<Vec<Recipe>>
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
            FileFormat::Txt => kalorio::parse(r),
            FileFormat::Xml => cookml::parse(r),
            _ => Err(Error::UnsupportedFileFormat),
        },
        App::MasterCook => match file_format {
            FileFormat::MX2 => mastercook::parse_mx2(r),
            FileFormat::MXP => mastercook::parse_mxp(r),
            FileFormat::MZ2 => mastercook::parse_mz2(r),
            FileFormat::Txt => mastercook::parse_txt(r),
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
        App::Unknown => Err(Error::UnsupportedApp),
    }
}
