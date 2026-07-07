use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

/// Represents a collection of recipe management applications.
/// Each variant corresponds to a specific recipe or cooking-related app.
#[derive(Clone, Debug, Default, strum_macros::Display, EnumIter, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum App {
    AccuChef,
    BigOven,
    ChefTap,
    #[strum(to_string = "Computer Cuisine Deluxe")]
    ComputerCuisineDeluxe,
    CookBook,
    Cooklang,
    #[strum(to_string = "COOKmate")]
    CookMate,
    #[strum(to_string = "Cook'n")]
    Cookn,
    #[strum(to_string = "Copy Me That")]
    CopyMeThat,
    Crouton,
    #[strum(to_string = "Home Cookin")]
    HomeCookin,
    Kalorio,
    #[strum(to_string = "Le Collectionneur de Recettes")]
    LeCollectionneurDeRecettes,
    MasterCook,
    MealMaster,
    Paprika,
    #[strum(to_string = "Recipe Keeper")]
    RecipeKeeper,
    RecipeMD,
    RecipeSage,
    Recipya,
    Rezkonv,
    Saffron,
    #[default]
    Unknown,
}

/// Returns a list of all recipe management applications Recipya can import.
pub fn all_apps() -> Vec<App> {
    App::iter().filter(|a| !matches!(a, App::Unknown)).collect()
}
