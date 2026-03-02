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
    API,
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

impl PrimaryReportType<Website> {
    /// Creates a new Website primary report type.
    pub const fn new() -> Self {
        Self { _p: PhantomData }
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
            inner: SecondaryInner::API,
            _p: PhantomData,
        }
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
            SecondaryInner::API => 3,
            SecondaryInner::Raw => 4,
            SecondaryInner::Software => 5,
        }
    }
}

/// Tertiary types are generic over their allowed primary and secondary types.
pub struct TertiaryReportType<Primary, Secondary> {
    inner: TertiaryInner,
    _p: PhantomData<(Primary, Secondary)>,
}

impl TertiaryReportType<Import, API> {
    fn new(inner: TertiaryInner) -> Self {
        Self {
            inner,
            _p: PhantomData,
        }
    }

    /// Creates a new tertiary report type for the Mealie API.
    pub fn mealie() -> Self {
        Self::new(TertiaryInner::Mealie)
    }

    /// Creates a new tertiary report type for the Nextcloud API.
    pub fn nextcloud() -> Self {
        Self::new(TertiaryInner::Nextcloud)
    }

    /// Creates a new tertiary report type for the Tandoor API.
    pub fn tandoor() -> Self {
        Self::new(TertiaryInner::Tandoor)
    }
}

impl TertiaryReportType<Import, Raw> {
    fn new(inner: TertiaryInner) -> Self {
        Self {
            inner,
            _p: PhantomData,
        }
    }

    /// Creates a new tertiary report type for the JSON file format.
    pub fn json() -> Self {
        Self::new(TertiaryInner::Json)
    }
}

impl TertiaryReportType<Import, Software> {
    fn new(inner: TertiaryInner) -> Self {
        Self {
            inner,
            _p: PhantomData,
        }
    }

    /// Creates a new tertiary report type for the AccuChef software.
    pub fn accuchef() -> Self {
        Self::new(TertiaryInner::AccuChef)
    }

    /// Creates a new tertiary report type for the BigOven software.
    pub fn bigoven() -> Self {
        Self::new(TertiaryInner::BigOven)
    }

    /// Creates a new tertiary report type for the ChefTap software.
    pub fn cheftap() -> Self {
        Self::new(TertiaryInner::ChefTap)
    }

    /// Creates a new tertiary report type for the Cooklang software.
    pub fn cooklang() -> Self {
        Self::new(TertiaryInner::Cooklang)
    }

    /// Creates a new tertiary report type for the Cookmate software.
    pub fn cookmate() -> Self {
        Self::new(TertiaryInner::CookMate)
    }

    /// Creates a new tertiary report type for the Crouton software.
    pub fn crouton() -> Self {
        Self::new(TertiaryInner::Crouton)
    }

    /// Creates a new tertiary report type for the Kalorio software.
    pub fn kalorio() -> Self {
        Self::new(TertiaryInner::Kalorio)
    }

    /// Creates a new tertiary report type for the MasterCook software.
    pub fn mastercook() -> Self {
        Self::new(TertiaryInner::MasterCook)
    }

    /// Creates a new tertiary report type for the MealMaster software.
    pub fn mealmaster() -> Self {
        Self::new(TertiaryInner::MealMaster)
    }

    /// Creates a new tertiary report type for the Parpika software.
    pub fn paprika() -> Self {
        Self::new(TertiaryInner::Paprika)
    }

    /// Creates a new tertiary report type for the RecipeMD software.
    pub fn recipe_md() -> Self {
        Self::new(TertiaryInner::RecipeMD)
    }

    /// Creates a new tertiary report type for the RecipeSage software.
    pub fn recipe_sage() -> Self {
        Self::new(TertiaryInner::RecipeSage)
    }

    /// Creates a new tertiary report type for the Rezkonv software.
    pub fn rezkonv() -> Self {
        Self::new(TertiaryInner::Rezkonv)
    }

    /// Creates a new tertiary report type for the Saffron software.
    pub fn saffron() -> Self {
        Self::new(TertiaryInner::Saffron)
    }
}

impl<P, S> ReportTypeId for TertiaryReportType<P, S> {
    fn to_id(&self) -> i16 {
        match self.inner {
            TertiaryInner::Mealie => 6,
            TertiaryInner::Nextcloud => 7,
            TertiaryInner::Tandoor => 8,
            TertiaryInner::Json => 9,
            TertiaryInner::AccuChef => 10,
            TertiaryInner::BigOven => 11,
            TertiaryInner::ChefTap => 12,
            TertiaryInner::Cooklang => 13,
            TertiaryInner::CookMate => 14,
            TertiaryInner::Crouton => 15,
            TertiaryInner::Kalorio => 16,
            TertiaryInner::MasterCook => 17,
            TertiaryInner::MealMaster => 18,
            TertiaryInner::Paprika => 19,
            TertiaryInner::RecipeMD => 20,
            TertiaryInner::RecipeSage => 21,
            TertiaryInner::Rezkonv => 22,
            TertiaryInner::Saffron => 23,
        }
    }
}

/// Represents a full report type.
pub struct ReportTypeFull<Primary, Secondary> {
    pub primary: PrimaryReportType<Primary>,
    pub secondary: Option<SecondaryReportType<Primary, Secondary>>,
    pub tertiary: Option<TertiaryReportType<Primary, Secondary>>,
}

impl ReportTypeFull<Website, ()> {
    /// Creates a new website report type.
    pub fn website() -> Self {
        Self {
            primary: PrimaryReportType::<Website>::new(),
            secondary: None,
            tertiary: None,
        }
    }
}

/// Represents a type of primary report.
#[derive(Debug, Eq, PartialEq, Identifiable, Queryable, Selectable)]
#[diesel(table_name = schema::report_types_primary)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReportTypePrimary {
    pub id: i16,
    pub name: String,
}

/// Represents a type of secondary report.
#[derive(Debug, Eq, PartialEq, Identifiable, Queryable, Selectable)]
#[diesel(table_name = schema::report_types_secondary)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReportTypeSecondary {
    pub id: i16,
    pub name: String,
}

/// Represents a type of tertiary report.
#[derive(Debug, Eq, PartialEq, Identifiable, Queryable, Selectable)]
#[diesel(table_name = schema::report_types_tertiary)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReportTypeTertiary {
    pub id: i16,
    pub name: String,
}
