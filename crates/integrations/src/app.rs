use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

/// Represents a collection of recipe management applications.
/// Each variant corresponds to a specific recipe or cooking-related app.
#[derive(Clone, Debug, Default, strum_macros::Display, EnumIter, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum App {
    AccuChef,
    BigOven,
    ChefTap,
    Cooklang,
    CookMate,
    Crouton,
    Kalorio,
    MasterCook,
    MealMaster,
    Paprika,
    RecipeMD,
    RecipeSage,
    Rezkonv,
    Saffron,
    #[default]
    Unknown,
}

/// Returns a list of all recipe management applications Recipya can import.
pub fn all_apps() -> Vec<App> {
    App::iter().filter(|a| !matches!(a, App::Unknown)).collect()
}
