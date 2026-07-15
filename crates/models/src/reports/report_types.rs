use std::marker::PhantomData;

use diesel::{
    Selectable,
    prelude::{Identifiable, Queryable},
};
use repository::schema;

pub trait ReportTypeId {
    /// Returns the database ID for this report type.
    fn to_id(&self) -> i16;
}

/// Marker type for import primary report types.
pub struct Import;
/// Marker type for website primary report types.
pub struct Website;
/// Marker type for API secondary report types.
pub struct API;
/// Marker type for raw secondary report types.
pub struct Raw;
/// Marker type for software secondary report types.
pub struct Software;

enum SecondaryInner {
    Api,
    Raw,
    Software,
}

enum TertiaryInner {
    // API
    Mealie,
    Nextcloud,
    Tandoor,
    // Raw
    Json,
    // Software
    AccuChef,
    BigOven,
    ChefTap,
    ComputerCuisineDeluxe,
    CookBook,
    Cooklang,
    CookMate,
    Cookn,
    CopyMeThat,
    Crouton,
    HomeCookin,
    Kalorio,
    LeCollectionneurDeRecettes,
    MasterCook,
    MealMaster,
    MrCook,
    MyRecipeBox,
    Paprika,
    Pepperplate,
    RecipeKeeper,
    RecipeMD,
    RecipeSage,
    Recipya,
    Rezkonv,
    Saffron,
    Umami,
}

/// A primary report type of primary report type `P`.
pub struct PrimaryReportType<P> {
    _p: PhantomData<P>,
}

impl PrimaryReportType<Import> {
    /// Creates a new Import primary report type.
    pub const fn new() -> Self {
        Self { _p: PhantomData }
    }
}

impl Default for PrimaryReportType<Import> {
    fn default() -> Self {
        Self::new()
    }
}

impl PrimaryReportType<Website> {
    /// Creates a new Website primary report type.
    pub const fn new() -> Self {
        Self { _p: PhantomData }
    }
}

impl Default for PrimaryReportType<Website> {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportTypeId for PrimaryReportType<Import> {
    fn to_id(&self) -> i16 {
        1
    }
}

impl ReportTypeId for PrimaryReportType<Website> {
    fn to_id(&self) -> i16 {
        2
    }
}

/// Secondary types are generic over their allowed primary types.
pub struct SecondaryReportType<Primary, Secondary> {
    inner: SecondaryInner,
    _p: PhantomData<(Primary, Secondary)>,
}

impl SecondaryReportType<Import, API> {
    /// Creates a new secondary report type for an API.
    pub const fn new() -> Self {
        Self {
            inner: SecondaryInner::Api,
            _p: PhantomData,
        }
    }
}

impl Default for SecondaryReportType<Import, API> {
    fn default() -> Self {
        Self::new()
    }
}

impl SecondaryReportType<Import, Raw> {
    /// Creates a new secondary report type for raw data.
    pub const fn new() -> Self {
        Self {
            inner: SecondaryInner::Raw,
            _p: PhantomData,
        }
    }
}

impl SecondaryReportType<Import, Software> {
    /// Creates a new secondary report type for software.
    pub const fn new() -> Self {
        Self {
            inner: SecondaryInner::Software,
            _p: PhantomData,
        }
    }
}

impl<P, S> ReportTypeId for SecondaryReportType<P, S> {
    fn to_id(&self) -> i16 {
        match self.inner {
            SecondaryInner::Api => 1,
            SecondaryInner::Raw => 2,
            SecondaryInner::Software => 3,
        }
    }
}

impl Default for SecondaryReportType<Import, Raw> {
    fn default() -> Self {
        Self::new()
    }
}

/// Tertiary types are generic over their allowed primary and secondary types.
pub struct TertiaryReportType<Primary, Secondary> {
    inner: TertiaryInner,
    _p: PhantomData<(Primary, Secondary)>,
}

impl TertiaryReportType<Import, API> {
    const fn new(inner: TertiaryInner) -> Self {
        Self {
            inner,
            _p: PhantomData,
        }
    }

    /// Creates a new tertiary report type for the Mealie API.
    pub const fn mealie() -> Self {
        Self::new(TertiaryInner::Mealie)
    }

    /// Creates a new tertiary report type for the Nextcloud API.
    pub const fn nextcloud() -> Self {
        Self::new(TertiaryInner::Nextcloud)
    }

    /// Creates a new tertiary report type for the Tandoor API.
    pub const fn tandoor() -> Self {
        Self::new(TertiaryInner::Tandoor)
    }
}

impl TertiaryReportType<Import, Raw> {
    const fn new(inner: TertiaryInner) -> Self {
        Self {
            inner,
            _p: PhantomData,
        }
    }

    /// Creates a new tertiary report type for the JSON file format.
    pub const fn json() -> Self {
        Self::new(TertiaryInner::Json)
    }
}

impl TertiaryReportType<Import, Software> {
    const fn new(inner: TertiaryInner) -> Self {
        Self {
            inner,
            _p: PhantomData,
        }
    }

    /// Creates a new tertiary report type for the `AccuChef` software.
    pub const fn accuchef() -> Self {
        Self::new(TertiaryInner::AccuChef)
    }

    /// Creates a new tertiary report type for the `BigOven` software.
    pub const fn bigoven() -> Self {
        Self::new(TertiaryInner::BigOven)
    }

    /// Creates a new tertiary report type for the `ChefTap` software.
    pub const fn cheftap() -> Self {
        Self::new(TertiaryInner::ChefTap)
    }

    /// Creates a new tertiary report type for the `ComputerCuisineDeluxe` software.
    pub const fn computer_cuisine_deluxe() -> Self {
        Self::new(TertiaryInner::ComputerCuisineDeluxe)
    }

    /// Creates a new tertiary report type for the `CookBook` software.
    pub const fn cookbook() -> Self {
        Self::new(TertiaryInner::CookBook)
    }

    /// Creates a new tertiary report type for the `Cooklang` software.
    pub const fn cooklang() -> Self {
        Self::new(TertiaryInner::Cooklang)
    }

    /// Creates a new tertiary report type for the `Cookmate` software.
    pub const fn cookmate() -> Self {
        Self::new(TertiaryInner::CookMate)
    }

    /// Creates a new tertiary report type for the `Cook'n` software.
    pub const fn cookn() -> Self {
        Self::new(TertiaryInner::Cookn)
    }

    /// Creates a new tertiary report type for the `Copy Me That` software.
    pub const fn copymethat() -> Self {
        Self::new(TertiaryInner::CopyMeThat)
    }

    /// Creates a new tertiary report type for the `Crouton` software.
    pub const fn crouton() -> Self {
        Self::new(TertiaryInner::Crouton)
    }

    /// Creates a new tertiary report type for the `Home Cookin` software.
    pub const fn home_cookin() -> Self {
        Self::new(TertiaryInner::HomeCookin)
    }

    /// Creates a new tertiary report type for the `Kalorio` software.
    pub const fn kalorio() -> Self {
        Self::new(TertiaryInner::Kalorio)
    }

    /// Creates a new tertiary report type for the `Le Collectionneur de Recettes` software.
    pub const fn lecollectionneurderecettes() -> Self {
        Self::new(TertiaryInner::LeCollectionneurDeRecettes)
    }

    /// Creates a new tertiary report type for the `MasterCook` software.
    pub const fn mastercook() -> Self {
        Self::new(TertiaryInner::MasterCook)
    }

    /// Creates a new tertiary report type for the `MealMaster` software.
    pub const fn mealmaster() -> Self {
        Self::new(TertiaryInner::MealMaster)
    }

    /// Creates a new tertiary report type for the `Mr. Cook` software.
    pub const fn mrcook() -> Self {
        Self::new(TertiaryInner::MrCook)
    }

    /// Creates a new tertiary report type for the `My Recipe Box` software.
    pub const fn myrecipebox() -> Self {
        Self::new(TertiaryInner::MyRecipeBox)
    }

    /// Creates a new tertiary report type for the `Parpika` software.
    pub const fn paprika() -> Self {
        Self::new(TertiaryInner::Paprika)
    }

    /// Creates a new tertiary report type for the `Pepperplate` software.
    pub const fn pepperplate() -> Self {
        Self::new(TertiaryInner::Pepperplate)
    }

    /// Creates a new tertiary report type for the `Recipe Keeper` software.
    pub const fn recipekeeper() -> Self {
        Self::new(TertiaryInner::RecipeKeeper)
    }

    /// Creates a new tertiary report type for the `RecipeMD` software.
    pub const fn recipe_md() -> Self {
        Self::new(TertiaryInner::RecipeMD)
    }

    /// Creates a new tertiary report type for the `RecipeSage` software.
    pub const fn recipe_sage() -> Self {
        Self::new(TertiaryInner::RecipeSage)
    }

    /// Creates a new tertiary report type for the `Recipya` software.
    pub const fn recipya() -> Self {
        Self::new(TertiaryInner::Recipya)
    }

    /// Creates a new tertiary report type for the `Rezkonv` software.
    pub const fn rezkonv() -> Self {
        Self::new(TertiaryInner::Rezkonv)
    }

    /// Creates a new tertiary report type for the `Saffron` software.
    pub const fn saffron() -> Self {
        Self::new(TertiaryInner::Saffron)
    }

    /// Creates a new tertiary report type for the `Umami` software.
    pub const fn umami() -> Self {
        Self::new(TertiaryInner::Umami)
    }
}

impl<P, S> ReportTypeId for TertiaryReportType<P, S> {
    fn to_id(&self) -> i16 {
        match self.inner {
            TertiaryInner::Mealie => 1,
            TertiaryInner::Nextcloud => 2,
            TertiaryInner::Tandoor => 3,
            TertiaryInner::Json => 4,
            TertiaryInner::AccuChef => 5,
            TertiaryInner::BigOven => 6,
            TertiaryInner::ChefTap => 7,
            TertiaryInner::Cooklang => 8,
            TertiaryInner::CookMate => 9,
            TertiaryInner::Crouton => 10,
            TertiaryInner::Kalorio => 11,
            TertiaryInner::MasterCook => 12,
            TertiaryInner::MealMaster => 13,
            TertiaryInner::Paprika => 14,
            TertiaryInner::RecipeMD => 15,
            TertiaryInner::RecipeSage => 16,
            TertiaryInner::Rezkonv => 17,
            TertiaryInner::Saffron => 18,
            TertiaryInner::Recipya => 19,
            TertiaryInner::ComputerCuisineDeluxe => 20,
            TertiaryInner::Cookn => 21,
            TertiaryInner::CopyMeThat => 22,
            TertiaryInner::CookBook => 23,
            TertiaryInner::HomeCookin => 24,
            TertiaryInner::RecipeKeeper => 25,
            TertiaryInner::LeCollectionneurDeRecettes => 26,
            TertiaryInner::MrCook => 27,
            TertiaryInner::MyRecipeBox => 28,
            TertiaryInner::Pepperplate => 29,
            TertiaryInner::Umami => 30,
        }
    }
}

/// Represents a full report type.
pub struct ReportTypeFull<Primary, Secondary> {
    pub primary: PrimaryReportType<Primary>,
    pub secondary: Option<SecondaryReportType<Primary, Secondary>>,
    pub tertiary: Option<TertiaryReportType<Primary, Secondary>>,
}

impl ReportTypeFull<Import, Raw> {
    /// Creates a new raw import report type.
    pub const fn raw(tertiary: TertiaryReportType<Import, Raw>) -> Self {
        Self {
            primary: PrimaryReportType::<Import>::new(),
            secondary: Some(SecondaryReportType::<Import, Raw>::new()),
            tertiary: Some(tertiary),
        }
    }
}

impl ReportTypeFull<Website, ()> {
    /// Creates a new website report type.
    pub const fn website() -> Self {
        Self {
            primary: PrimaryReportType::<Website>::new(),
            secondary: None,
            tertiary: None,
        }
    }
}

impl ReportTypeFull<Import, Software> {
    /// Creates a new software import report type.
    pub const fn app(tertiary: TertiaryReportType<Import, Software>) -> Self {
        Self {
            primary: PrimaryReportType::<Import>::new(),
            secondary: Some(SecondaryReportType::<Import, Software>::new()),
            tertiary: Some(tertiary),
        }
    }
}

impl ReportTypeFull<Import, API> {
    /// Creates a new API import report type.
    pub const fn api(tertiary: TertiaryReportType<Import, API>) -> Self {
        Self {
            primary: PrimaryReportType::<Import>::new(),
            secondary: Some(SecondaryReportType::<Import, API>::new()),
            tertiary: Some(tertiary),
        }
    }
}

/// Represents a type of primary report.
#[derive(Clone, Debug, Default, Eq, PartialEq, Identifiable, Queryable, Selectable)]
#[diesel(table_name = schema::report_types_primary)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReportTypePrimary {
    pub id: i16,
    pub name: String,
}

/// Represents a type of secondary report.
#[derive(Clone, Debug, Eq, PartialEq, Identifiable, Queryable, Selectable)]
#[diesel(table_name = schema::report_types_secondary)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReportTypeSecondary {
    pub id: i16,
    pub name: String,
}

/// Represents a type of tertiary report.
#[derive(Clone, Debug, Eq, PartialEq, Identifiable, Queryable, Selectable)]
#[diesel(table_name = schema::report_types_tertiary)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReportTypeTertiary {
    pub id: i16,
    pub name: String,
}
