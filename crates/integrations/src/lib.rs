#![recursion_limit = "256"]

mod app;
mod common;
mod error;
mod fileformat;
mod helpers;

pub mod api;
pub mod apps;

pub use app::{App, all_apps};
pub use error::{Error, Result};
pub use fileformat::FileFormat;

use std::io::{Read, Seek};

use schema_org::Recipe;

use crate::apps::cooklang::CookLang;
use crate::apps::{
    accuchef, bigoven, cheftap, computer_cuisine_deluxe, cookbook, cookmate, cookml, cookn,
    copymethat, crouton, homecookin, kalorio, lecollectionneurderecettes, mastercook, mealmaster,
    mrcook, myrecipebox, paprika, pepperplate, recipekeeper, recipemd, recipeml, recipesage,
    recipya, rezkonv, saffron, umami,
};

/// Parses a recipe from the given input source and returns a vector of `IntegrationRecipe` objects.
#[allow(clippy::too_many_lines)]
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
        App::BigOven => bigoven::parse(r),
        App::ChefTap => match file_format {
            FileFormat::Html => cheftap::parse_html(r),
            FileFormat::Txt => cheftap::parse_txt(r),
            FileFormat::Zip => cheftap::parse_zip(r),
            _ => Err(Error::UnsupportedFileFormat),
        },
        App::ComputerCuisineDeluxe => computer_cuisine_deluxe::parse_csv(r),
        App::CookBook => match file_format {
            FileFormat::Txt => cookbook::parse_txt(r),
            FileFormat::Zip => cookbook::parse_archive(r),
            _ => Err(Error::UnsupportedFileFormat),
        },
        App::Cooklang => CookLang::default().parse(r, file_name),
        App::CookMate => match file_format {
            FileFormat::MCB | FileFormat::Zip => cookmate::parse_backup(r),
            FileFormat::Rezkonv => rezkonv::parse(r),
            FileFormat::MealMaster => mealmaster::parse(r),
            FileFormat::Xml => cookmate::parse_xml(r),
            _ => Err(Error::UnsupportedFileFormat),
        },
        App::Cookn => match file_format {
            FileFormat::Txt => cookn::parse_txt(r),
            FileFormat::Zip => cookn::parse_archive(r),
            _ => Err(Error::UnsupportedFileFormat),
        },
        App::Crouton => crouton::parse(r),
        App::CopyMeThat => match file_format {
            FileFormat::Txt => copymethat::parse_txt(r),
            FileFormat::Yaml => copymethat::parse_yaml(r),
            FileFormat::Zip => copymethat::parse_archive(r),
            _ => Err(Error::UnsupportedFileFormat),
        },
        App::HomeCookin => match file_format {
            FileFormat::Hc => homecookin::parse_hc(r),
            FileFormat::MZ2 => mastercook::parse_mz2(r),
            FileFormat::Txt => homecookin::parse_txt(r),
            FileFormat::Xml => recipeml::parse_xml(r),
            _ => Err(Error::UnsupportedFileFormat),
        },
        App::Kalorio => match file_format {
            FileFormat::Txt => kalorio::parse(r),
            FileFormat::Xml => cookml::parse(r),
            _ => Err(Error::UnsupportedFileFormat),
        },
        App::LeCollectionneurDeRecettes => match file_format {
            FileFormat::Html => lecollectionneurderecettes::parse_html(r),
            FileFormat::Txt => lecollectionneurderecettes::parse_txt(r),
            FileFormat::Zip => lecollectionneurderecettes::parse_archive(r),
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
        App::MrCook => match file_format {
            FileFormat::Csv => mrcook::parse_csv(r),
            FileFormat::Zip => mrcook::parse_archive(r),
            _ => Err(Error::UnsupportedFileFormat),
        },
        App::MyRecipeBox => match file_format {
            FileFormat::Csv => myrecipebox::parse_csv(r),
            FileFormat::Rtk => myrecipebox::parse_rtk(r),
            FileFormat::Zip => myrecipebox::parse_archive(r),
            _ => Err(Error::UnsupportedFileFormat),
        },
        App::Paprika => paprika::parse(r),
        App::Pepperplate => match file_format {
            FileFormat::Txt => pepperplate::parse_txt(r),
            FileFormat::Zip => pepperplate::parse_archive(r),
            _ => Err(Error::UnsupportedFileFormat),
        },
        App::RecipeKeeper => recipekeeper::parse_archive(r),
        App::RecipeMD => recipemd::parse(r),
        App::RecipeSage => match file_format {
            FileFormat::Json => recipesage::parse_json(r),
            FileFormat::Txt => recipesage::parse_txt(r),
            FileFormat::Xml => recipesage::parse_xml(r),
            _ => Err(Error::UnsupportedFileFormat),
        },
        App::Recipya => match file_format {
            FileFormat::Zip => recipya::parse_zip(r),
            _ => Err(Error::UnsupportedFileFormat),
        },
        App::Rezkonv => rezkonv::parse(r),
        App::Saffron => saffron::parse(r),
        App::Umami => match file_format {
            FileFormat::Html => umami::parse_html(r),
            FileFormat::Json => umami::parse_json(r),
            FileFormat::Md => umami::parse_md(r),
            FileFormat::Txt => umami::parse_txt(r),
            FileFormat::Zip => umami::parse_archive(r),
            _ => Err(Error::UnsupportedApp),
        },
        App::Unknown => Err(Error::UnsupportedApp),
    }
}
