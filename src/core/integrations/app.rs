use strum_macros::{EnumIter, EnumString};

/// Represents a collection of recipe management applications.
/// Each variant corresponds to a specific recipe or cooking-related app.\
#[derive(Debug, Default, strum_macros::Display, EnumIter, EnumString)]
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
