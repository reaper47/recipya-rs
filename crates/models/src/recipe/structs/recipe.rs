use bitflags::bitflags;
use diesel::prelude::*;
use schema_org::ToIso8601;
use tracing::error;
use uuid::Uuid;
use whatlang::Lang;

use math::cooking::units::system;
use repository::schema;
use schema_org::field::{FieldEnum20, RecipeDescriptionFieldEnum, RecipeKeywordsFieldEnum};
use support::name_entity_with_relations;
use support::strings::extract_number;

use crate::recipe::RecipeForm;
use crate::recipe::structs::media::{Video, VideoForCreate};
use crate::recipe::structs::nutrition::{NutritionDetails, NutritionDetailsForCreate};
use crate::recipe::structs::section::{Section, SectionComponents};
use crate::recipe::structs::time::{Times, TimesForCreate};
use crate::recipe::structs::tool::{ToolForCreate, ToolRecipe};
use crate::recipe::structs::types::Source;
use crate::user::User;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RecipeField: u32 {
        const NAME         = 1 << 0;
        const DESCRIPTION  = 1 << 1;
        const MEDIA       = 1 << 2;
        const NOTES        = 1 << 3;
        const SOURCE       = 1 << 4;
        const RATING       = 1 << 5;
        const YIELD        = 1 << 6;
        const CATEGORY     = 1 << 7;
        const CUISINE      = 1 << 8;
        const INGREDIENTS  = 1 << 9;
        const INSTRUCTIONS = 1 << 10;
        const KEYWORDS     = 1 << 11;
        const NUTRITION    = 1 << 12;
        const TIMES        = 1 << 13;
        const TOOLS        = 1 << 14;
    }
}

/// Represents a recipe entity stored in the database.
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    AsChangeset,
    Associations,
    Queryable,
    Identifiable,
    Selectable,
)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(MeasurementSystem))]
#[diesel(treat_none_as_null = true)]
#[diesel(table_name = schema::recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    /// The unique identifier of the recipe.
    pub id: i64,
    /// The name of the recipe.
    pub name: String,
    /// An optional description of the recipe.
    pub description: Option<String>,
    /// An optional UUID referencing an image associated with the recipe.
    pub image: Option<Uuid>,
    /// The quantity of servings or portions the recipe produces.
    pub r#yield: i16,
    /// The language in which the recipe is written.
    pub language: String,
    /// The original measurement system the recipe is in.
    pub measurement_system_id: i16,
    /// Optional notes of the recipe.
    pub notes: Option<String>,
    /// An optional reference to the origin or inspiration of the recipe.
    pub source: Source,
    /// Specifies whether the recipe has been marked as favourite.
    pub is_favourite: bool,
    /// Optional 1-5 rating. None is used for no rating.
    pub rating: Option<i16>,
    /// The timestamp when the nutrition entry was created.
    pub created_at: chrono::NaiveDateTime,
    /// The timestamp when the nutrition entry was last updated.
    pub updated_at: chrono::NaiveDateTime,
    /// The foreign key linking the recipe to its creator.
    pub user_id: Uuid,
}

/// Represents the data required to insert a new recipe into the database.
#[derive(Associations, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = schema::recipes)]
pub(crate) struct RecipeForInsert {
    pub name: String,
    pub description: Option<String>,
    pub image: Option<Uuid>,
    pub r#yield: Option<i16>,
    pub language: String,
    pub notes: Option<String>,
    pub source: Source,
    pub is_favourite: bool,
    pub rating: Option<i16>,
    pub user_id: Uuid,
}

/// Represents the full details of a recipe, including its metadata and related entities.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct RecipeDetails {
    pub recipe: Recipe,
    pub additional_images: Vec<Uuid>,
    pub category: String,
    pub cuisine: Option<String>,
    pub ingredients: SectionComponents,
    pub instructions: SectionComponents,
    pub keywords: Vec<String>,
    pub nutrition: NutritionDetails,
    pub times: Times,
    pub tools: Vec<ToolRecipe>,
    pub videos: Vec<Video>,
}

impl RecipeDetails {
    /// Returns the number of images the recipe has.
    pub const fn num_images(&self) -> usize {
        match self.recipe.image {
            Some(_) => 1 + self.additional_images.len(),
            None => 0,
        }
    }

    /// Returns the number of images and videos the recipe has.
    pub const fn num_media(&self) -> usize {
        self.num_images() + self.num_videos()
    }

    /// Returns the number of videos the recipe has.
    pub const fn num_videos(&self) -> usize {
        self.videos.len()
    }

    /// Returns all images the recipe has.
    pub fn all_images(&self) -> Vec<Uuid> {
        self.recipe
            .image
            .iter()
            .chain(self.additional_images.iter())
            .copied()
            .collect()
    }

    /// Returns whether the recipe is right-to-left.
    pub fn is_rtl(&self) -> bool {
        matches!(
            Lang::from_code(&self.recipe.language).unwrap_or(Lang::Eng),
            Lang::Ara | Lang::Heb | Lang::Yid | Lang::Urd | Lang::Pes
        )
    }
}

impl From<RecipeForCreate> for RecipeDetails {
    fn from(recipe_c: RecipeForCreate) -> Self {
        Self {
            recipe: Recipe {
                name: recipe_c.name.clone(),
                description: recipe_c.description.clone(),
                r#yield: recipe_c.r#yield.unwrap_or(4),
                language: recipe_c.detect_language().code().to_string(),
                measurement_system_id: 1,
                source: recipe_c.source,
                ..Default::default()
            },
            category: recipe_c.category.unwrap_or_default(),
            cuisine: recipe_c.cuisine,
            ingredients: recipe_c.ingredients,
            instructions: recipe_c.instructions,
            keywords: recipe_c.keywords,
            nutrition: NutritionDetails::from(&recipe_c.nutrition),
            times: Times::from(recipe_c.times.unwrap_or_default()),
            tools: recipe_c
                .tools
                .into_iter()
                .enumerate()
                .map(|(idx, t)| {
                    let mut tool = ToolRecipe::from(&t);
                    tool.tool_order = i16::try_from(idx)
                        .inspect_err(|err| error!("Failed to convert tool index to i16: {err}"))
                        .unwrap_or_default();
                    tool
                })
                .collect(),
            ..Default::default()
        }
    }
}

/// Represents the data required to create a new recipe.
#[derive(Default)]
pub struct RecipeForCreate {
    // For the recipe table
    pub name: String,
    pub description: Option<String>,
    pub images: Vec<Uuid>,
    pub is_favourite: bool,
    pub measurement_system_id: i16,
    pub notes: Option<String>,
    pub source: Source,
    pub rating: Option<i16>,
    pub videos: Vec<VideoForCreate>,
    pub r#yield: Option<i16>,

    // For association tables
    pub category: Option<String>,
    pub cuisine: Option<String>,
    pub ingredients: SectionComponents,
    pub instructions: SectionComponents,
    pub keywords: Vec<String>,
    pub nutrition: NutritionDetailsForCreate,
    pub times: Option<TimesForCreate>,
    pub tools: Vec<ToolForCreate>,
}

impl RecipeForCreate {
    /// Returns the first image UUID if available and a vector of the remaining image UUIDs.
    pub fn first_and_rest_images(&self) -> (Option<Uuid>, Vec<Uuid>) {
        match self.images.as_slice() {
            [first, rest @ ..] => (Some(*first), rest.to_vec()),
            [] => (None, Vec::new()),
        }
    }

    /// Determines the recipe's language.
    pub fn detect_language(&self) -> Lang {
        whatlang::detect_lang(
            &[
                self.name.as_str(),
                self.description.as_deref().unwrap_or(""),
                &itertools::Itertools::intersperse(
                    self.ingredients
                        .items_as_text()
                        .iter()
                        .chain(&self.instructions.items_as_text())
                        .map(ToString::to_string),
                    " ".into(),
                )
                .collect::<String>(),
            ]
            .join(" "),
        )
        .unwrap_or(Lang::Eng)
    }

    /// Diff gets the changes between the current recipe and another recipe.
    pub fn diff(&self, other: &Self, is_nutrition_calculated_by_source: bool) -> RecipeField {
        let mut changes = RecipeField::empty();

        changes.set(RecipeField::NAME, self.name != other.name);
        changes.set(
            RecipeField::DESCRIPTION,
            self.description != other.description,
        );
        changes.set(
            RecipeField::MEDIA,
            self.images.len() != other.images.len() || self.videos != other.videos,
        );
        changes.set(RecipeField::NOTES, self.notes != other.notes);
        changes.set(RecipeField::RATING, self.rating != other.rating);
        changes.set(RecipeField::YIELD, self.r#yield != other.r#yield);
        changes.set(RecipeField::CATEGORY, self.category != other.category);
        changes.set(RecipeField::CUISINE, self.cuisine != other.cuisine);
        changes.set(
            RecipeField::INGREDIENTS,
            self.ingredients.items_as_text() != other.ingredients.items_as_text(),
        );
        changes.set(
            RecipeField::INSTRUCTIONS,
            self.instructions.items_as_text() != other.instructions.items_as_text(),
        );
        changes.set(RecipeField::KEYWORDS, self.keywords != other.keywords);
        changes.set(
            RecipeField::NUTRITION,
            is_nutrition_calculated_by_source && self.nutrition != other.nutrition,
        );
        changes.set(RecipeField::TIMES, self.times != other.times);
        changes.set(RecipeField::TOOLS, self.tools != other.tools);

        changes
    }
}

impl From<&RecipeForm> for RecipeForCreate {
    fn from(form: &RecipeForm) -> Self {
        let ingredients = &form.ingredients;
        let measurement_system_id = system::MeasurementSystem::from(
            ingredients.iter().map(String::as_str).collect::<Vec<_>>(),
        )
        .id();

        Self {
            name: form.title.clone(),
            description: form.description.clone(),
            images: Vec::new(),
            r#yield: form.yield_,
            source: Source::from(form.source.clone()),
            is_favourite: false,
            rating: form.rating,
            videos: Vec::new(),
            category: form
                .category
                .clone()
                .or_else(|| Some("uncategorized".into())),
            cuisine: form.cuisine.clone(),
            ingredients: SectionComponents::new(ingredients.clone()),
            instructions: SectionComponents::new(form.instructions.clone()),
            keywords: form.keywords.clone(),
            measurement_system_id,
            nutrition: form.nutrition.clone(),
            times: form.times.clone(),
            tools: form.tools.clone(),
            notes: form.notes.clone(),
        }
    }
}

impl From<RecipeForm> for RecipeForCreate {
    fn from(form: RecipeForm) -> Self {
        Self::from(&form)
    }
}

impl From<RecipeDetails> for RecipeForCreate {
    fn from(r: RecipeDetails) -> Self {
        let recipe = r.recipe.clone();

        Self {
            name: recipe.name,
            description: recipe.description,
            images: r.all_images(),
            is_favourite: recipe.is_favourite,
            measurement_system_id: recipe.measurement_system_id,
            notes: recipe.notes,
            source: recipe.source,
            rating: recipe.rating,
            videos: r.videos.into_iter().map(VideoForCreate::from).collect(),
            r#yield: Some(recipe.r#yield),
            category: Some(r.category),
            cuisine: r.cuisine,
            ingredients: r.ingredients,
            instructions: r.instructions,
            keywords: r.keywords,
            nutrition: NutritionDetailsForCreate::from(r.nutrition),
            times: Some(TimesForCreate::from(r.times)),
            tools: r.tools.into_iter().map(ToolForCreate::from).collect(),
        }
    }
}

impl From<&schema_org::Recipe> for RecipeForCreate {
    fn from(schema: &schema_org::Recipe) -> Self {
        let original_ingredients = schema.recipe_ingredient.clone();
        let ingredients = SectionComponents::try_from(original_ingredients)
            .inspect_err(|err| {
                error!("Failed to parse ingredients: {err}");
            })
            .unwrap_or_default();
        let measurement_system_id =
            system::MeasurementSystem::from(ingredients.items_as_text()).id();
        let nutrition = schema
            .nutrition
            .first()
            .map(NutritionDetailsForCreate::from)
            .unwrap_or_default();

        Self {
            name: schema.name.first().cloned().unwrap_or_default(),
            description: schema.description.first().map(|v| match v {
                RecipeDescriptionFieldEnum::Text(s) => s.clone(),
                RecipeDescriptionFieldEnum::TextObject(obj) => {
                    obj.text.first().cloned().unwrap_or_default()
                }
            }),
            images: vec![],
            r#yield: schema.recipe_yield.first().map_or_else(
                || {
                    nutrition
                        .per_serving
                        .as_ref()
                        .map(|v| extract_number(v.serving_size.as_ref()).ok())
                        .unwrap_or_default()
                },
                FieldEnum20::to_i16,
            ),
            source: Source::from(schema.url.first().cloned()),
            is_favourite: false,
            rating: None, // TODO: Look into it because the Recipe schema doesn't have such dedicated field
            videos: vec![],
            category: schema.recipe_category.first().map_or_else(
                || Some("uncategorized".into()),
                |category| Some(category.clone()),
            ),
            cuisine: schema.recipe_cuisine.first().cloned(),
            ingredients,
            instructions: SectionComponents::from(schema.recipe_instructions.clone()),
            keywords: schema
                .keywords
                .iter()
                .map(|k| match k {
                    RecipeKeywordsFieldEnum::DefinedTerm(term) => {
                        term.name.first().cloned().unwrap_or_default()
                    }
                    RecipeKeywordsFieldEnum::TextOrURL(s) => s.clone(),
                })
                .collect(),
            measurement_system_id,
            notes: None, // TODO: Look into it because the Recipe schema doesn't have such dedicated field
            nutrition,
            times: Some(TimesForCreate::from_components(
                schema.prep_time.to_is8601_duration(),
                schema.cook_time.to_is8601_duration(),
                schema.total_time.to_is8601_duration(),
            )),
            tools: schema.tool.iter().map(ToolForCreate::from).collect(),
        }
    }
}

/// Represents a measurement system entity stored in the database.
#[derive(Debug, Queryable, Identifiable, Eq, PartialEq, Selectable)]
#[diesel(table_name = schema::measurement_systems)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MeasurementSystem {
    /// Unique identifier of the measurement system entry.
    pub id: i16,
    /// The name of the measurement system.
    pub name: String,
}

name_entity_with_relations!(Category, categories, categories_recipes);
name_entity_with_relations!(Cuisine, cuisines, cuisines_recipes);
name_entity_with_relations!(Keyword, keywords, keywords_recipes);

/// Represents an ingredient in the `ingredients` table.
#[derive(Queryable, Identifiable, Selectable)]
#[diesel(table_name = schema::ingredients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Ingredient {
    pub id: i64,
    pub name: String,
}

/// Represents the insertion of an ingredient-recipe association into the database.
#[derive(Insertable)]
#[diesel(table_name = schema::ingredients)]
pub(crate) struct IngredientForInsert {
    pub name: String,
}

/// Represents the association between an ingredient and a recipe in the
/// `ingredients_recipes` table.
#[derive(Identifiable, Selectable, Queryable, Associations)]
#[diesel(table_name = schema::ingredients_recipes)]
#[diesel(belongs_to(Ingredient), belongs_to(Recipe), belongs_to(Section))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IngredientRecipe {
    pub id: i64,
    pub ingredient_id: i64,
    pub recipe_id: i64,
    pub section_id: i64,
    pub item_order: i16,
}

/// Represents the data required to insert a new ingredient-recipe association into
/// the `ingredients_recipes` table.
#[derive(Associations, Insertable)]
#[diesel(table_name = schema::ingredients_recipes)]
#[diesel(belongs_to(Recipe))]
pub(crate) struct IngredientRecipeForInsert {
    pub ingredient_id: i64,
    pub recipe_id: i64,
    pub section_id: i64,
    pub item_order: i16,
}

/// Represents the data required to insert an instruction into the `instructions` table.
#[derive(Insertable)]
#[diesel(table_name = schema::instructions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct InstructionForInsert {
    pub name: String,
    pub duration_seconds: Option<i32>,
}

/// Represents the data required to insert an instruction-recipe association into the
/// `instructions_recipes` table.
#[derive(Associations, Insertable)]
#[diesel(table_name = schema::instructions_recipes)]
#[diesel(belongs_to(Recipe))]
pub(crate) struct InstructionRecipeForInsert {
    pub instruction_id: i64,
    pub recipe_id: i64,
    pub section_id: i64,
    pub item_order: i16,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_recipe_details {
        use super::*;

        fn a_recipe() -> RecipeDetails {
            RecipeDetails {
                recipe: Recipe {
                    measurement_system_id: 2,
                    ..Default::default()
                },
                ..Default::default()
            }
        }

        #[test]
        fn test_num_images_no_images() {
            let recipe = a_recipe();

            let got = recipe.num_images();

            pretty_assertions::assert_eq!(got, 0);
        }

        #[test]
        fn test_num_images_only_main_image() {
            let mut recipe = a_recipe();
            recipe.recipe.image = Some(Uuid::new_v4());

            let got = recipe.num_images();

            pretty_assertions::assert_eq!(got, 1);
        }

        #[test]
        fn test_num_images_additional_images() {
            let mut recipe = a_recipe();
            recipe.recipe.image = Some(Uuid::new_v4());
            recipe.additional_images = vec![Uuid::new_v4(), Uuid::new_v4()];

            let got = recipe.num_images();

            pretty_assertions::assert_eq!(got, 3);
        }

        #[test]
        fn test_num_media_no_videos() {
            let mut recipe = a_recipe();
            recipe.recipe.image = Some(Uuid::new_v4());

            let got = recipe.num_media();

            pretty_assertions::assert_eq!(got, 1);
        }

        #[test]
        fn test_num_media_no_images() {
            let mut recipe = a_recipe();
            recipe.videos = vec![Video::default(), Video::default()];

            let got = recipe.num_videos();

            pretty_assertions::assert_eq!(got, 2);
        }

        #[test]
        fn test_num_media() {
            let mut recipe = a_recipe();
            recipe.recipe.image = Some(Uuid::new_v4());
            recipe.additional_images = vec![Uuid::new_v4(), Uuid::new_v4()];
            recipe.videos = vec![Video::default(), Video::default()];

            let got = recipe.num_media();

            pretty_assertions::assert_eq!(got, 5);
        }

        #[test]
        fn test_num_videos() {
            let mut recipe = a_recipe();
            recipe.videos = vec![Video::default(), Video::default(), Video::default()];

            let got = recipe.num_videos();

            pretty_assertions::assert_eq!(got, 3);
        }

        #[test]
        fn test_all_images_no_images() {
            let recipe = a_recipe();

            let got = recipe.all_images();

            let got_str = got.iter().map(ToString::to_string).collect::<Vec<_>>();
            pretty_assertions::assert_eq!(got_str, Vec::<String>::new());
        }

        #[test]
        fn test_all_images_only_main_image() {
            let mut recipe = a_recipe();
            let an_image = Uuid::new_v4();
            recipe.recipe.image = Some(an_image);

            let got = recipe.all_images();

            let got_str = got.iter().map(ToString::to_string).collect::<Vec<_>>();
            pretty_assertions::assert_eq!(got_str, vec![an_image.to_string()]);
        }

        #[test]
        fn test_all_images() {
            let mut recipe = a_recipe();
            let an_image = Uuid::new_v4();
            let an_image2 = Uuid::new_v4();
            recipe.recipe.image = Some(an_image);
            recipe.additional_images = vec![an_image2];

            let got = recipe.all_images();

            let got_str = got.iter().map(ToString::to_string).collect::<Vec<_>>();
            pretty_assertions::assert_eq!(
                got_str,
                vec![an_image.to_string(), an_image2.to_string()]
            );
        }
    }

    mod tests_nutrition {
        use crate::recipe::structs::nutrition::Nutrition;

        #[test]
        fn test_empty_nutrition() {
            let nutrition = Nutrition::default();

            let got = nutrition.to_line();

            assert!(got.is_none());
        }

        #[test]
        fn test_single_nutrition_value() {
            let nutrition = Nutrition {
                calories_kcal: Some(200),
                ..Default::default()
            };

            let got = nutrition.to_line().unwrap();

            assert_eq!(got, "calories 200 kcal");
        }

        #[test]
        fn test_multiple_nutrition_values() {
            let nutrition = Nutrition {
                calories_kcal: Some(200),
                total_carbohydrates: Some(50.),
                sugars_g: Some(20.),
                protein_g: Some(10.),
                total_fat_g: Some(5.),
                ..Default::default()
            };

            let got = nutrition.to_line().unwrap();

            assert_eq!(
                got,
                "calories 200 kcal; total carbohydrates 50g; sugar 20g; protein 10g; total fat 5g"
            );
        }

        #[test]
        fn test_nutrition_with_some_empty_values() {
            let nutrition = Nutrition {
                calories_kcal: Some(250),
                sodium_mg: Some(300.),
                ..Default::default()
            };

            let got = nutrition.to_line().unwrap();

            assert_eq!(got, "calories 250 kcal; sodium 300mg");
        }

        #[test]
        fn test_nutrition_with_all_values() {
            let nutrition = Nutrition {
                calories_kcal: Some(200),
                total_carbohydrates: Some(50.),
                sugars_g: Some(20.),
                protein_g: Some(10.),
                total_fat_g: Some(5.),
                saturated_fat_g: Some(2.),
                unsaturated_fat_g: Some(1.),
                trans_fat_g: Some(0.),
                cholesterol_mg: Some(30.),
                sodium_mg: Some(300.),
                fiber_g: Some(5.),
                ..Default::default()
            };

            let got = nutrition.to_line().unwrap();

            assert_eq!(
                got,
                "calories 200 kcal; total carbohydrates 50g; sugar 20g; protein 10g; total fat 5g; saturated fat 2g; unsaturated fat 1g; trans fat 0g; cholesterol 30mg; sodium 300mg; fiber 5g"
            );
        }
    }

    mod tests_recipe_for_create {
        use schema_org::field::{
            RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
            RecipeRecipeYieldFieldEnum,
        };

        use crate::recipe::structs::section::Item;

        use super::*;

        #[test]
        fn test_first_and_rest_with_multiple_images() {
            let uuid1 = Uuid::new_v4();
            let uuid2 = Uuid::new_v4();
            let uuid3 = Uuid::new_v4();
            let recipe_c = RecipeForCreate {
                images: vec![uuid1, uuid2, uuid3],
                ..Default::default()
            };

            let (first, rest) = recipe_c.first_and_rest_images();

            pretty_assertions::assert_eq!(first, Some(uuid1));
            pretty_assertions::assert_eq!(rest, vec![uuid2, uuid3]);
        }

        #[test]
        fn test_first_and_rest_with_one_image() {
            let uuid1 = Uuid::new_v4();
            let recipe_c = RecipeForCreate {
                images: vec![uuid1],
                ..Default::default()
            };

            let (first, rest) = recipe_c.first_and_rest_images();

            pretty_assertions::assert_eq!(first, Some(uuid1));
            assert!(rest.is_empty());
        }

        #[test]
        fn test_first_and_rest_with_no_images() {
            let recipe_c = RecipeForCreate {
                images: vec![],
                ..Default::default()
            };

            let (first, rest) = recipe_c.first_and_rest_images();

            pretty_assertions::assert_eq!(first, None);
            assert!(rest.is_empty());
        }

        #[test]
        fn test_first_and_rest_with_empty_vec() {
            let recipe_c = RecipeForCreate {
                images: vec![],
                ..Default::default()
            };

            let (first, rest) = recipe_c.first_and_rest_images();

            pretty_assertions::assert_eq!(first, None);
            assert!(rest.is_empty());
        }

        #[test]
        fn test_detect_language_en() {
            let recipe_c = RecipeForCreate {
                name: "The best hamburger ever".into(),
                description: Some("This is the best hamburger ever".into()),
                ingredients: SectionComponents::Flat(vec![
                    Item::new("1 cup of flour"),
                    Item::new("1 cup of water"),
                ]),
                instructions: SectionComponents::Flat(vec![
                    Item::new("Mix all ingredients"),
                    Item::new("Bake for 30 minutes"),
                ]),
                ..Default::default()
            };

            let got = recipe_c.detect_language();

            pretty_assertions::assert_eq!(got, Lang::Eng);
        }

        mod tests_from_recipe_schema {
            use super::*;

            #[test]
            fn test_no_yield_specified() {
                let mut schema = a_minimal_schema_recipe();
                schema.recipe_yield = Vec::new();
                schema.nutrition = Vec::new();

                let recipe = RecipeForCreate::from(&schema);

                assert!(recipe.r#yield.is_none());
            }

            #[test]
            fn test_nutrition_yield_specified() {
                let mut schema = a_minimal_schema_recipe();
                schema.recipe_yield = Vec::new();
                schema.nutrition = vec![schema_org::NutritionInformation {
                    serving_size: vec!["4 bunches".into()],
                    ..Default::default()
                }];

                let recipe = RecipeForCreate::from(&schema);

                assert_eq!(recipe.r#yield, Some(4));
            }

            #[test]
            fn test_recipe_yield_specified() {
                let mut schema = a_minimal_schema_recipe();
                schema.recipe_yield = vec![RecipeRecipeYieldFieldEnum::Text("8 cup cakes".into())];
                schema.nutrition = Vec::new();

                let recipe = RecipeForCreate::from(&schema);

                assert_eq!(recipe.r#yield, Some(8));
            }

            #[test]
            fn test_recipe_yield_and_nutrition_specified() {
                let mut schema = a_minimal_schema_recipe();
                schema.recipe_yield = vec![RecipeRecipeYieldFieldEnum::Text("12 cup cakes".into())];
                schema.nutrition = vec![schema_org::NutritionInformation {
                    serving_size: vec!["24 bunches".into()],
                    ..Default::default()
                }];

                let recipe = RecipeForCreate::from(&schema);

                assert_eq!(recipe.r#yield, Some(12));
            }
        }

        fn a_minimal_schema_recipe() -> schema_org::Recipe {
            schema_org::Recipe {
                name: vec!["The best hamburger ever".into()],
                description: vec![RecipeDescriptionFieldEnum::Text(
                    "This is the best hamburger ever".into(),
                )],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("1 cup of flour".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 cup of water".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text("Mix all ingredients".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Bake for 30 minutes".into()),
                ],
                ..Default::default()
            }
        }
    }
}
