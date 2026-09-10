use std::ops::Not;

use diesel::prelude::*;

use nutrition::NutritionComponents;
use repository::schema;
use schema_org::{Energy, Mass, NutritionInformation};

/// Holds details about nutrition information for a recipe.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NutritionDetails {
    pub per_100g: Option<Nutrition>,
    pub per_serving: Option<NutritionPerServingDetails>,
}

impl NutritionDetails {
    /// Formats the nutrition as a sentence.
    pub fn to_line(&self) -> (Option<String>, Option<String>) {
        (
            self.per_100g
                .as_ref()
                .map(|n| {
                    n.to_line()
                        .map(|line| format!("Nutrition Facts\nPer 100g: {line}"))
                })
                .unwrap_or_default(),
            self.per_serving.as_ref().map(|n| {
                n.nutrition
                    .to_line()
                    .map(|line| {
                        format!("Nutrition Facts\nPer serving ({}): {line}", n.serving_size)
                    })
                    .unwrap_or_default()
            }),
        )
    }

    /// Verifies whether the nutrition has been precalculated by the source.
    pub fn is_precalculated(&self) -> bool {
        self.per_100g
            .as_ref()
            .is_some_and(|n| n.is_precalculated_by_source)
            || self
                .per_serving
                .as_ref()
                .is_some_and(|n| n.nutrition.is_precalculated_by_source)
    }
}

impl From<&NutritionDetailsForCreate> for NutritionDetails {
    fn from(nutrition_c: &NutritionDetailsForCreate) -> Self {
        Self {
            per_100g: nutrition_c.per_100g.as_ref().map(Nutrition::from),
            per_serving: nutrition_c
                .per_serving
                .as_ref()
                .map(NutritionPerServingDetails::from),
        }
    }
}

impl From<Nutrition> for NutritionForCreate {
    fn from(value: Nutrition) -> Self {
        Self {
            calories_kcal: value.calories_kcal,
            total_carbohydrates: value.total_carbohydrates,
            sugars_g: value.sugars_g,
            protein_g: value.protein_g,
            total_fat_g: value.total_fat_g,
            saturated_fat_g: value.saturated_fat_g,
            unsaturated_fat_g: value.unsaturated_fat_g,
            cholesterol_mg: value.cholesterol_mg,
            sodium_mg: value.sodium_mg,
            fiber_g: value.fiber_g,
            trans_fat_g: value.trans_fat_g,
        }
    }
}

impl From<&NutritionInformation> for NutritionDetailsForCreate {
    fn from(schema: &NutritionInformation) -> Self {
        let nutrition_c = NutritionForCreate {
            calories_kcal: schema.calories.first().map(Energy::to_number),
            total_carbohydrates: schema
                .carbohydrate_content
                .first()
                .map(Mass::to_number::<f64>),
            sugars_g: schema.sugar_content.first().map(Mass::to_number::<f64>),
            protein_g: schema.protein_content.first().map(Mass::to_number::<f64>),
            total_fat_g: schema.fat_content.first().map(Mass::to_number::<f64>),
            saturated_fat_g: schema
                .saturated_fat_content
                .first()
                .map(Mass::to_number::<f64>),
            unsaturated_fat_g: schema
                .unsaturated_fat_content
                .first()
                .map(Mass::to_number::<f64>),
            cholesterol_mg: schema
                .cholesterol_content
                .first()
                .map(Mass::to_number::<f64>),
            sodium_mg: schema.sodium_content.first().map(Mass::to_number::<f64>),
            fiber_g: schema.fiber_content.first().map(Mass::to_number::<f64>),
            trans_fat_g: schema.trans_fat_content.first().map(Mass::to_number::<f64>),
        };

        if schema.is_per_100g() {
            Self {
                per_100g: Some(nutrition_c),
                per_serving: None,
            }
        } else {
            Self {
                per_100g: None,
                per_serving: Some(NutritionPerServingDetailsForCreate {
                    nutrition: nutrition_c,
                    serving_size: schema.serving_size.first().cloned().unwrap_or_default(),
                }),
            }
        }
    }
}

/// Holds details about nutrition information per serving for a recipe.
#[derive(Clone, Debug, Default, PartialEq, Queryable)]
pub struct NutritionPerServingDetails {
    pub nutrition: Nutrition,
    pub serving_size: String,
}

impl From<&NutritionPerServingDetailsForCreate> for NutritionPerServingDetails {
    fn from(nutrition_c: &NutritionPerServingDetailsForCreate) -> Self {
        Self {
            nutrition: Nutrition::from(&nutrition_c.nutrition),
            serving_size: nutrition_c.serving_size.clone(),
        }
    }
}

/// Holds the nutritional information for creating a new recipe.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NutritionDetailsForCreate {
    pub per_100g: Option<NutritionForCreate>,
    pub per_serving: Option<NutritionPerServingDetailsForCreate>,
}

impl NutritionDetailsForCreate {
    /// Creates a new instance of `NutritionDetailsForCreate`.
    pub fn new(
        per_100g: NutritionForCreate,
        per_serving: NutritionPerServingDetailsForCreate,
    ) -> Self {
        Self {
            per_100g: per_100g.is_empty().not().then_some(per_100g),
            per_serving: per_serving
                .nutrition
                .is_empty()
                .not()
                .then_some(per_serving),
        }
    }

    /// Checks whether the nutrition details is empty.
    pub const fn is_empty(&self) -> bool {
        match (&self.per_100g, &self.per_serving) {
            (None, None) => true,
            (Some(per_100g), Some(per_serving)) => {
                per_100g.is_empty() && per_serving.nutrition.is_empty()
            }
            (Some(per_100g), None) => per_100g.is_empty(),
            (None, Some(per_serving)) => per_serving.nutrition.is_empty(),
        }
    }
}

impl From<NutritionDetails> for NutritionDetailsForCreate {
    fn from(n: NutritionDetails) -> Self {
        Self {
            per_100g: n.per_100g.map(NutritionForCreate::from),
            per_serving: n.per_serving.map(NutritionPerServingDetailsForCreate::from),
        }
    }
}

/// Holds the nutritional information for creating a new nutrition per serving record.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NutritionPerServingDetailsForCreate {
    pub nutrition: NutritionForCreate,
    pub serving_size: String,
}

impl From<NutritionPerServingDetails> for NutritionPerServingDetailsForCreate {
    fn from(n: NutritionPerServingDetails) -> Self {
        Self {
            nutrition: NutritionForCreate::from(n.nutrition),
            serving_size: n.serving_size,
        }
    }
}

/// Represents the nutritional information provided when creating a new recipe.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NutritionForCreate {
    pub calories_kcal: Option<i16>,
    pub total_carbohydrates: Option<f64>,
    pub sugars_g: Option<f64>,
    pub protein_g: Option<f64>,
    pub total_fat_g: Option<f64>,
    pub saturated_fat_g: Option<f64>,
    pub unsaturated_fat_g: Option<f64>,
    pub cholesterol_mg: Option<f64>,
    pub sodium_mg: Option<f64>,
    pub fiber_g: Option<f64>,
    pub trans_fat_g: Option<f64>,
}

impl NutritionForCreate {
    /// Verifies whether all fields are blank.
    pub const fn is_empty(&self) -> bool {
        self.calories_kcal.is_none()
            && self.total_carbohydrates.is_none()
            && self.sugars_g.is_none()
            && self.protein_g.is_none()
            && self.total_fat_g.is_none()
            && self.saturated_fat_g.is_none()
            && self.unsaturated_fat_g.is_none()
            && self.cholesterol_mg.is_none()
            && self.sodium_mg.is_none()
            && self.fiber_g.is_none()
            && self.trans_fat_g.is_none()
    }
}

/// Represents a nutrition entity stored in the database.
#[derive(AsChangeset, Clone, Debug, Default, Queryable, Identifiable, PartialEq, Selectable)]
#[diesel(table_name = schema::nutrition)]
#[diesel(treat_none_as_null = true)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Nutrition {
    /// Unique identifier for the nutrition entry.
    pub id: i64,
    /// Whether the nutrition values are pre-calculated by the source.
    pub is_precalculated_by_source: bool,
    /// Total calories in kilocalories (kcal) per serving.
    pub calories_kcal: Option<i16>,
    /// Total carbohydrates in grams (g) per serving.
    pub total_carbohydrates: Option<f64>,
    /// Total sugar content in grams (g) per serving.
    pub sugars_g: Option<f64>,
    /// Total protein content in grams (g) per serving.
    pub protein_g: Option<f64>,
    /// Total fat content in grams (g) per serving.
    pub total_fat_g: Option<f64>,
    /// Saturated fat content in grams (g) per serving.
    pub saturated_fat_g: Option<f64>,
    /// Unsaturated fat content in grams (g) per serving.
    pub unsaturated_fat_g: Option<f64>,
    /// Cholesterol content in milligrams (mg) per serving.
    pub cholesterol_mg: Option<f64>,
    /// Sodium content in milligrams (mg) per serving.
    pub sodium_mg: Option<f64>,
    /// Dietary fiber content in grams (g) per serving.
    pub fiber_g: Option<f64>,
    /// Trans fat content in grams (g) per serving.
    pub trans_fat_g: Option<f64>,
}

impl Nutrition {
    /// Formats the nutrition as a sentence.
    pub fn to_line(&self) -> Option<String> {
        if self == &Self::default() {
            return None;
        }

        let mut result: String = String::new();

        let parts: Vec<String> = [
            self.calories_kcal
                .as_ref()
                .map(|v| format!("calories {v} kcal")),
            self.total_carbohydrates
                .as_ref()
                .map(|v| format!("total carbohydrates {v}g")),
            self.sugars_g.as_ref().map(|v| format!("sugar {v}g")),
            self.protein_g.as_ref().map(|v| format!("protein {v}g")),
            self.total_fat_g.as_ref().map(|v| format!("total fat {v}g")),
            self.saturated_fat_g
                .as_ref()
                .map(|v| format!("saturated fat {v}g")),
            self.unsaturated_fat_g
                .as_ref()
                .map(|v| format!("unsaturated fat {v}g")),
            self.trans_fat_g.as_ref().map(|v| format!("trans fat {v}g")),
            self.cholesterol_mg
                .as_ref()
                .map(|v| format!("cholesterol {v}mg")),
            self.sodium_mg.as_ref().map(|v| format!("sodium {v}mg")),
            self.fiber_g.as_ref().map(|v| format!("fiber {v}g")),
        ]
        .into_iter()
        .flatten()
        .collect();

        result.push_str(&parts.join("; "));
        Some(result)
    }

    pub fn sanitize(mut self) -> Option<Self> {
        self.total_carbohydrates = Self::clean_float(self.total_carbohydrates);
        self.sugars_g = Self::clean_float(self.sugars_g);
        self.protein_g = Self::clean_float(self.protein_g);
        self.total_fat_g = Self::clean_float(self.total_fat_g);
        self.saturated_fat_g = Self::clean_float(self.saturated_fat_g);
        self.unsaturated_fat_g = Self::clean_float(self.unsaturated_fat_g);
        self.cholesterol_mg = Self::clean_float(self.cholesterol_mg);
        self.sodium_mg = Self::clean_float(self.sodium_mg);
        self.fiber_g = Self::clean_float(self.fiber_g);
        self.trans_fat_g = Self::clean_float(self.trans_fat_g);

        // Check if all values are None/zero
        if self.has_no_data() { None } else { Some(self) }
    }

    fn clean_float(value: Option<f64>) -> Option<f64> {
        value.filter(|&v| !v.is_nan() && v != 0.0)
    }

    fn has_no_data(&self) -> bool {
        self.calories_kcal.unwrap_or(0) == 0
            && self.total_carbohydrates.is_none()
            && self.sugars_g.is_none()
            && self.protein_g.is_none()
            && self.total_fat_g.is_none()
            && self.saturated_fat_g.is_none()
            && self.unsaturated_fat_g.is_none()
            && self.cholesterol_mg.is_none()
            && self.sodium_mg.is_none()
            && self.fiber_g.is_none()
            && self.trans_fat_g.is_none()
    }
}

impl From<&NutritionForCreate> for Nutrition {
    fn from(n: &NutritionForCreate) -> Self {
        Self {
            calories_kcal: n.calories_kcal,
            total_carbohydrates: n.total_carbohydrates,
            sugars_g: n.sugars_g,
            protein_g: n.protein_g,
            total_fat_g: n.total_fat_g,
            saturated_fat_g: n.saturated_fat_g,
            unsaturated_fat_g: n.unsaturated_fat_g,
            cholesterol_mg: n.cholesterol_mg,
            sodium_mg: n.sodium_mg,
            fiber_g: n.fiber_g,
            trans_fat_g: n.trans_fat_g,
            ..Default::default()
        }
    }
}

/// Represents the nutritional information associated with a recipe in the database.
#[derive(Debug, Insertable, PartialEq)]
#[diesel(table_name = schema::nutrition)]
pub(crate) struct NutritionForInsert {
    pub is_precalculated_by_source: bool,
    pub calories_kcal: Option<i16>,
    pub total_carbohydrates: Option<f64>,
    pub sugars_g: Option<f64>,
    pub protein_g: Option<f64>,
    pub total_fat_g: Option<f64>,
    pub saturated_fat_g: Option<f64>,
    pub unsaturated_fat_g: Option<f64>,
    pub cholesterol_mg: Option<f64>,
    pub sodium_mg: Option<f64>,
    pub fiber_g: Option<f64>,
    pub trans_fat_g: Option<f64>,
}

impl From<&NutritionComponents> for NutritionForInsert {
    fn from(n: &NutritionComponents) -> Self {
        Self {
            is_precalculated_by_source: false,
            calories_kcal: n.calories_kcal.into(),
            total_carbohydrates: n.total_carbohydrates.into(),
            sugars_g: n.sugars_g.into(),
            protein_g: n.protein_g.into(),
            total_fat_g: n.total_fat_g.into(),
            saturated_fat_g: n.saturated_fat_g.into(),
            unsaturated_fat_g: n.unsaturated_fat_g.into(),
            cholesterol_mg: n.cholesterol_mg.into(),
            sodium_mg: n.sodium_mg.into(),
            fiber_g: n.fiber_g.into(),
            trans_fat_g: n.trans_fat_g.into(),
        }
    }
}

impl From<NutritionComponents> for NutritionForInsert {
    fn from(n: NutritionComponents) -> Self {
        Self::from(&n)
    }
}

/// Represents a nutrition per 100g record.
#[derive(
    AsChangeset,
    Associations,
    Clone,
    Debug,
    Default,
    Queryable,
    Identifiable,
    Eq,
    PartialEq,
    Selectable,
)]
#[diesel(belongs_to(Nutrition))]
#[diesel(table_name = schema::nutrition_per_100g)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NutritionPer100g {
    pub id: i64,
    pub recipe_id: i64,
    pub nutrition_id: i64,
}

/// Represents the nutritional information per 100g associated with a nutrition.
#[derive(Associations, Insertable)]
#[diesel(belongs_to(Nutrition))]
#[diesel(table_name = schema::nutrition_per_100g)]
pub struct NutritionPer100gForInsert {
    pub recipe_id: i64,
    pub nutrition_id: i64,
}

/// Represents a nutrition per serving record.
#[derive(
    AsChangeset,
    Associations,
    Clone,
    Debug,
    Default,
    Queryable,
    Identifiable,
    Eq,
    PartialEq,
    Selectable,
)]
#[diesel(belongs_to(Nutrition))]
#[diesel(table_name = schema::nutrition_per_serving)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NutritionPerServing {
    pub id: i64,
    pub recipe_id: i64,
    pub nutrition_id: i64,
    pub serving_size: String,
}

/// Represents the nutritional information per serving associated with a nutrition.
#[derive(Associations, Insertable)]
#[diesel(belongs_to(Nutrition))]
#[diesel(table_name = schema::nutrition_per_serving)]
pub struct NutritionPerServingForInsert {
    pub recipe_id: i64,
    pub nutrition_id: i64,
    pub serving_size: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_nutrition_details() -> NutritionDetails {
        NutritionDetails {
            per_100g: Some(Nutrition {
                id: 0,
                is_precalculated_by_source: false,
                calories_kcal: Some(1),
                total_carbohydrates: Some(2.),
                sugars_g: Some(3.),
                protein_g: Some(4.),
                total_fat_g: Some(5.),
                saturated_fat_g: Some(6.),
                unsaturated_fat_g: Some(7.),
                cholesterol_mg: Some(8.),
                sodium_mg: Some(9.),
                fiber_g: Some(10.),
                trans_fat_g: Some(11.),
            }),
            per_serving: Some(NutritionPerServingDetails {
                nutrition: Nutrition {
                    id: 0,
                    is_precalculated_by_source: false,
                    calories_kcal: Some(12),
                    total_carbohydrates: Some(13.),
                    sugars_g: Some(14.),
                    protein_g: Some(15.),
                    total_fat_g: Some(16.),
                    saturated_fat_g: Some(17.),
                    unsaturated_fat_g: Some(18.),
                    cholesterol_mg: Some(19.),
                    sodium_mg: Some(20.),
                    fiber_g: Some(21.),
                    trans_fat_g: Some(22.),
                },
                serving_size: "2 meatballs".into(),
            }),
        }
    }

    #[test]
    fn test_nutrition_components_to_nutrition_for_insert() {
        let components = NutritionComponents {
            calories_kcal: 1,
            total_carbohydrates: 2.,
            sugars_g: 3.,
            protein_g: 4.,
            total_fat_g: 5.,
            saturated_fat_g: 6.,
            unsaturated_fat_g: 7.,
            cholesterol_mg: 8.,
            sodium_mg: 9.,
            fiber_g: 10.,
            trans_fat_g: 11.,
        };

        let got = NutritionForInsert::from(components);

        pretty_assertions::assert_eq!(
            got,
            NutritionForInsert {
                is_precalculated_by_source: false,
                calories_kcal: Some(1),
                total_carbohydrates: Some(2.),
                sugars_g: Some(3.),
                protein_g: Some(4.),
                total_fat_g: Some(5.),
                saturated_fat_g: Some(6.),
                unsaturated_fat_g: Some(7.),
                cholesterol_mg: Some(8.),
                sodium_mg: Some(9.),
                fiber_g: Some(10.),
                trans_fat_g: Some(11.),
            }
        );
    }

    #[test]
    fn test_nutrition_details_for_create_to_nutrition_details() {
        let details = NutritionDetailsForCreate {
            per_100g: Some(NutritionForCreate {
                calories_kcal: Some(1),
                total_carbohydrates: Some(2.),
                sugars_g: Some(3.),
                protein_g: Some(4.),
                total_fat_g: Some(5.),
                saturated_fat_g: Some(6.),
                unsaturated_fat_g: Some(7.),
                cholesterol_mg: Some(8.),
                sodium_mg: Some(9.),
                fiber_g: Some(10.),
                trans_fat_g: Some(11.),
            }),
            per_serving: Some(NutritionPerServingDetailsForCreate {
                nutrition: NutritionForCreate {
                    calories_kcal: Some(12),
                    total_carbohydrates: Some(13.),
                    sugars_g: Some(14.),
                    protein_g: Some(15.),
                    total_fat_g: Some(16.),
                    saturated_fat_g: Some(17.),
                    unsaturated_fat_g: Some(18.),
                    cholesterol_mg: Some(19.),
                    sodium_mg: Some(20.),
                    fiber_g: Some(21.),
                    trans_fat_g: Some(22.),
                },
                serving_size: "2 meatballs".into(),
            }),
        };

        let got = NutritionDetails::from(&details);

        pretty_assertions::assert_eq!(got, a_nutrition_details());
    }

    #[test]
    fn test_nutrition_for_create_is_empty_missing_fields() {
        let nutrition_c = NutritionForCreate {
            calories_kcal: None,
            total_carbohydrates: None,
            sugars_g: None,
            protein_g: None,
            total_fat_g: None,
            saturated_fat_g: None,
            unsaturated_fat_g: None,
            cholesterol_mg: None,
            sodium_mg: None,
            fiber_g: None,
            trans_fat_g: None,
        };

        assert!(nutrition_c.is_empty());
    }

    #[test]
    fn test_nutrition_for_create_is_empty_no_missing_fields() {
        let nutrition_c = NutritionForCreate {
            calories_kcal: Some(100),
            total_carbohydrates: Some(20.),
            sugars_g: Some(5.),
            protein_g: Some(10.),
            total_fat_g: Some(5.),
            saturated_fat_g: Some(2.),
            unsaturated_fat_g: Some(3.),
            cholesterol_mg: Some(100.),
            sodium_mg: Some(500.),
            fiber_g: Some(2.),
            trans_fat_g: Some(1.),
        };

        assert!(!nutrition_c.is_empty());
    }

    mod tests_to_line {
        use super::*;

        fn per_100g_line() -> String {
            "Nutrition Facts\nPer 100g: calories 1 kcal; total carbohydrates 2g; sugar 3g; protein 4g; total fat 5g; saturated fat 6g; unsaturated fat 7g; trans fat 11g; cholesterol 8mg; sodium 9mg; fiber 10g".to_string()
        }

        fn per_serving_line() -> String {
            "Nutrition Facts\nPer serving (2 meatballs): calories 12 kcal; total carbohydrates 13g; sugar 14g; protein 15g; total fat 16g; saturated fat 17g; unsaturated fat 18g; trans fat 22g; cholesterol 19mg; sodium 20mg; fiber 21g".to_string()
        }

        #[test]
        fn test_per_100g_only() {
            let mut details = a_nutrition_details();
            details.per_serving = None;

            let (got_per_100g, got_per_serving) = details.to_line();

            pretty_assertions::assert_eq!(got_per_100g, Some(per_100g_line()));
            assert_eq!(got_per_serving, None);
        }

        #[test]
        fn test_per_serving_only() {
            let mut details = a_nutrition_details();
            details.per_100g = None;

            let (got_per_100g, got_per_serving) = details.to_line();

            pretty_assertions::assert_eq!(got_per_serving, Some(per_serving_line()));
            assert_eq!(got_per_100g, None);
        }

        #[test]
        fn test_per_100g_and_serving() {
            let details = a_nutrition_details();

            let (got_per_100g, got_per_serving) = details.to_line();

            pretty_assertions::assert_eq!(got_per_100g, Some(per_100g_line()));
            pretty_assertions::assert_eq!(got_per_serving, Some(per_serving_line()));
        }

        #[test]
        fn test_no_nutrition() {
            let details = NutritionDetails {
                per_100g: None,
                per_serving: None,
            };

            let (got_per_100g, got_per_serving) = details.to_line();

            assert_eq!(got_per_100g, None);
            assert_eq!(got_per_serving, None);
        }
    }

    mod tests_nutrition_details_is_empty {
        use super::*;

        fn non_empty_nutrition() -> NutritionForCreate {
            NutritionForCreate {
                calories_kcal: Some(100),
                total_carbohydrates: Some(10.),
                sugars_g: Some(5.),
                protein_g: Some(20.),
                total_fat_g: Some(8.),
                saturated_fat_g: Some(2.),
                unsaturated_fat_g: Some(6.),
                cholesterol_mg: Some(50.),
                sodium_mg: Some(200.),
                fiber_g: Some(3.),
                trans_fat_g: Some(0.),
            }
        }

        fn non_empty_per_serving() -> NutritionPerServingDetailsForCreate {
            NutritionPerServingDetailsForCreate {
                nutrition: non_empty_nutrition(),
                serving_size: "1 cup".to_string(),
            }
        }

        fn empty_per_serving() -> NutritionPerServingDetailsForCreate {
            NutritionPerServingDetailsForCreate {
                nutrition: NutritionForCreate::default(),
                serving_size: "1 cup".to_string(),
            }
        }

        #[test]
        fn test_both_none() {
            let details = NutritionDetailsForCreate::default();

            assert!(details.is_empty());
        }

        #[test]
        fn test_both_some_both_empty() {
            let details = NutritionDetailsForCreate {
                per_100g: Some(NutritionForCreate::default()),
                per_serving: Some(empty_per_serving()),
            };

            assert!(details.is_empty());
        }

        #[test]
        fn test_is_empty_both_some_per_100g_not_empty() {
            let details = NutritionDetailsForCreate {
                per_100g: Some(non_empty_nutrition()),
                per_serving: Some(empty_per_serving()),
            };

            assert!(!details.is_empty());
        }

        #[test]
        fn test_both_some_per_serving_not_empty() {
            let details = NutritionDetailsForCreate {
                per_100g: Some(NutritionForCreate::default()),
                per_serving: Some(non_empty_per_serving()),
            };

            assert!(!details.is_empty());
        }

        #[test]
        fn test_both_some_both_not_empty() {
            let details = NutritionDetailsForCreate {
                per_100g: Some(non_empty_nutrition()),
                per_serving: Some(non_empty_per_serving()),
            };

            assert!(!details.is_empty());
        }

        #[test]
        fn test_only_per_100g_empty() {
            let details = NutritionDetailsForCreate {
                per_100g: Some(NutritionForCreate::default()),
                per_serving: None,
            };

            assert!(details.is_empty());
        }

        #[test]
        fn test_only_per_100g_not_empty() {
            let details = NutritionDetailsForCreate {
                per_100g: Some(non_empty_nutrition()),
                per_serving: None,
            };

            assert!(!details.is_empty());
        }

        #[test]
        fn test_only_per_serving_empty() {
            let details = NutritionDetailsForCreate {
                per_100g: None,
                per_serving: Some(empty_per_serving()),
            };

            assert!(details.is_empty());
        }

        #[test]
        fn test_only_per_serving_not_empty() {
            let details = NutritionDetailsForCreate {
                per_100g: None,
                per_serving: Some(non_empty_per_serving()),
            };

            assert!(!details.is_empty());
        }
    }

    mod tests_nutrition_details_is_precalculated_by_source {
        use super::*;

        #[test]
        fn test_both_none() {
            let details = NutritionDetails::default();

            assert!(!details.is_precalculated());
        }

        #[test]
        fn test_per_100g_precalculated() {
            let details = NutritionDetails {
                per_100g: Some(Nutrition {
                    is_precalculated_by_source: true,
                    ..Default::default()
                }),
                per_serving: None,
            };

            assert!(details.is_precalculated());
        }

        #[test]
        fn test_per_100g_not_precalculated() {
            let details = NutritionDetails {
                per_100g: Some(Nutrition::default()),
                per_serving: None,
            };

            assert!(!details.is_precalculated());
        }

        #[test]
        fn test_per_serving_precalculated() {
            let mut nutrition = NutritionPerServingDetails::default();
            nutrition.nutrition.is_precalculated_by_source = true;

            let details = NutritionDetails {
                per_100g: None,
                per_serving: Some(nutrition),
            };

            assert!(details.is_precalculated());
        }

        #[test]
        fn test_per_serving_not_precalculated() {
            let details = NutritionDetails {
                per_100g: None,
                per_serving: Some(NutritionPerServingDetails::default()),
            };

            assert!(!details.is_precalculated());
        }

        #[test]
        fn test_both_precalculated() {
            let mut nutrition2 = NutritionPerServingDetails::default();
            nutrition2.nutrition.is_precalculated_by_source = true;

            let details = NutritionDetails {
                per_100g: Some(Nutrition {
                    is_precalculated_by_source: true,
                    ..Default::default()
                }),
                per_serving: Some(nutrition2),
            };

            assert!(details.is_precalculated());
        }

        #[test]
        fn test_both_not_precalculated() {
            let details = NutritionDetails {
                per_100g: Some(Nutrition::default()),
                per_serving: Some(NutritionPerServingDetails::default()),
            };

            assert!(!details.is_precalculated());
        }
    }
}
