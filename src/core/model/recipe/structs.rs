use std::path::Path;
use std::sync::Arc;

use diesel::data_types::PgInterval;
use diesel::internal::derives::multiconnection::chrono;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use tracing::warn;
use uuid::Uuid;

use crate::core::model::recipe::RecipeForm;
use crate::core::model::user::User;
use crate::core::repository::schema;
use crate::core::scraper::schema::{
    CreativeWorkOrItemListOrText, HowToToolOrText, NutritionInformationSchema, RecipeSchema,
};
use crate::core::support::fs::FsSupport;
use crate::core::support::strings::extract_number;
use crate::name_entity_with_relations;

/// Represents a collection of sections, where each section has a title and a list of associated items.
pub type Sections = Vec<(String, Vec<String>)>;

impl From<CreativeWorkOrItemListOrText> for Sections {
    fn from(value: CreativeWorkOrItemListOrText) -> Self {
        match value {
            CreativeWorkOrItemListOrText::CreativeWork(work) => {
                let section_name = work.name.unwrap_or_default();
                let item = if let Some(description) = work.description {
                    String::from(description)
                } else {
                    String::new()
                };
                Sections::from([(section_name, vec![item])])
            }
            CreativeWorkOrItemListOrText::ItemList(items) => {
                let mut section_names = items
                    .iter()
                    .map(|item| item.name.clone().unwrap_or_default())
                    .collect::<Vec<_>>();
                section_names.dedup();

                let mut sections = Sections::from(
                    section_names
                        .iter()
                        .map(|name| (name.clone(), Vec::<String>::new()))
                        .collect::<Vec<_>>(),
                );

                for item in items {
                    let name = item.name.clone().unwrap_or_default();

                    if let Some((_name, texts)) = sections.iter_mut().find(|(n, _)| *n == name) {
                        texts.push(item.text);
                    }
                }

                sections
            }
            CreativeWorkOrItemListOrText::Text(s) => Self::from([("".into(), vec![s])]),
        }
    }
}

/// Represents a recipe entity stored in the database.
#[derive(
    AsChangeset, Associations, Clone, Debug, Queryable, Identifiable, PartialEq, Selectable,
)]
#[diesel(belongs_to(User))]
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
    pub yield_: i16,
    /// The language in which the recipe is written.
    pub language: String,
    /// An optional reference to the origin or inspiration of the recipe.
    pub source: Option<String>,
    /// The timestamp when the nutrition entry was created.
    pub created_at: chrono::NaiveDateTime,
    /// The timestamp when the nutrition entry was last updated.
    pub updated_at: chrono::NaiveDateTime,
    /// The foreign key linking the recipe to its creator.
    pub user_id: i64,
}

/// Represents the data required to create a new recipe.
#[derive(Default)]
pub struct RecipeForCreate {
    // For the recipe table
    pub name: String,
    pub description: Option<String>,
    pub images: Vec<Uuid>,
    pub yield_: Option<i16>,
    pub source: Option<String>,
    pub videos: Vec<VideoForCreate>,

    // For association tables
    pub category: Option<String>,
    pub cuisine: Option<String>,
    pub ingredients: Sections,
    pub instructions: Sections,
    pub keywords: Vec<String>,
    pub nutrition: Option<NutritionForCreate>,
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
}

impl From<RecipeForm> for RecipeForCreate {
    fn from(form: RecipeForm) -> Self {
        Self {
            name: form.title,
            description: form.description,
            images: vec![],
            yield_: form.yield_,
            source: form.source,
            videos: vec![],
            category: form.category.or(Some("uncategorized".into())),
            cuisine: form.cuisine,
            ingredients: Sections::from([("".into(), form.ingredients)]),
            instructions: Sections::from([("".into(), form.instructions)]),
            keywords: form.keywords,
            nutrition: form.nutrition,
            times: form.times,
            tools: form.tools,
        }
    }
}

impl From<&RecipeSchema> for RecipeForCreate {
    fn from(schema: &RecipeSchema) -> Self {
        Self {
            name: schema.name.clone().unwrap_or_default(),
            description: schema.description.clone().map(String::from),
            images: vec![],
            yield_: i16::try_from(schema.recipe_yield.clone()).ok(),
            source: schema.url.clone().map(|s| s.into()),
            videos: vec![],
            category: String::try_from(schema.recipe_category.clone()).ok(),
            cuisine: schema.recipe_cuisine.clone().map(String::from),
            ingredients: Sections::from([(
                "".into(),
                schema.recipe_ingredient.clone().unwrap_or_default(),
            )]),
            instructions: Sections::from(schema.recipe_instructions.clone().unwrap_or_default()),
            keywords: schema
                .keywords
                .clone()
                .into_iter()
                .map(String::from)
                .collect(),
            nutrition: schema.nutrition.clone().map(NutritionForCreate::from),
            times: Some(TimesForCreate::from_components(
                schema.prep_time,
                schema.cook_time,
            )),
            tools: schema
                .tool
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(ToolForCreate::from)
                .collect(),
        }
    }
}

/// Represents the data required to insert a new recipe into the database.
#[derive(Associations, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = schema::recipes)]
pub(super) struct RecipeForInsert {
    pub name: String,
    pub description: Option<String>,
    pub image: Option<Uuid>,
    pub yield_: Option<i16>,
    pub language: String,
    pub source: Option<String>,
    pub user_id: i64,
}

/// Represents the full details of a recipe, including its metadata and related entities.
#[derive(Debug, Clone, PartialEq)]
pub struct RecipeDetails {
    pub recipe: Recipe,
    pub additional_images: Vec<Uuid>,
    pub category: String,
    pub cuisine: Option<String>,
    pub ingredients: Sections,
    pub instructions: Sections,
    pub keywords: Vec<String>,
    pub nutrition: Option<Nutrition>,
    pub times: Times,
    pub tools: Vec<ToolRecipe>,
    pub videos: Vec<Video>,
}

impl RecipeDetails {
    /// Returns the number of images the recipe has.
    pub fn num_images(&self) -> usize {
        match self.recipe.image {
            Some(_) => 1 + self.additional_images.len(),
            None => 0,
        }
    }

    /// Returns the number of images and videos the recipe has.
    pub fn num_media(&self) -> usize {
        self.num_images() + self.num_videos()
    }

    /// Returns the number of videos the recipe has.
    pub fn num_videos(&self) -> usize {
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
}

/// Represents a nutrition entity stored in the database.
#[derive(
    AsChangeset, Associations, Clone, Debug, Default, Queryable, Identifiable, PartialEq, Selectable,
)]
#[diesel(belongs_to(Recipe))]
#[diesel(table_name = schema::nutrition)]
#[diesel(treat_none_as_null = true)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Nutrition {
    /// Unique identifier for the nutrition entry.
    pub id: i64,
    /// Identifier of the associated recipe.
    pub recipe_id: i64,
    /// Total calories in kilocalories (kcal) per serving.
    pub calories_kcal: Option<i16>,
    /// Total carbohydrates in grams (g) per serving.
    pub total_carbohydrates: Option<i16>,
    /// Total sugar content in grams (g) per serving.
    pub sugars_g: Option<i16>,
    /// Total protein content in grams (g) per serving.
    pub protein_g: Option<i16>,
    /// Total fat content in grams (g) per serving.
    pub total_fat_g: Option<i16>,
    /// Saturated fat content in grams (g) per serving.
    pub saturated_fat_g: Option<i16>,
    /// Unsaturated fat content in grams (g) per serving.
    pub unsaturated_fat_g: Option<i16>,
    /// Cholesterol content in milligrams (mg) per serving.
    pub cholesterol_mg: Option<i16>,
    /// Sodium content in milligrams (mg) per serving.
    pub sodium_mg: Option<i16>,
    /// Dietary fiber content in grams (g) per serving.
    pub fiber_g: Option<i16>,
    /// Trans fat content in grams (g) per serving.
    pub trans_fat_g: Option<i16>,
    /// Description of the serving size (e.g., "100g", "1 cup").
    pub serving_size: Option<String>,
}

impl Nutrition {
    /// Formats the nutrition as a sentence.
    pub fn to_line(&self) -> String {
        if self == &Nutrition::default() {
            return "No nutrition available.".into();
        }

        let mut result: String = if self.serving_size.as_deref() == Some("100g") {
            "Per 100g: "
        } else {
            "Per serving: "
        }
        .into();

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
        result
    }
}

/// Represents the nutritional information provided when creating a new recipe.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NutritionForCreate {
    pub calories_kcal: Option<i16>,
    pub total_carbohydrates: Option<i16>,
    pub sugars_g: Option<i16>,
    pub protein_g: Option<i16>,
    pub total_fat_g: Option<i16>,
    pub saturated_fat_g: Option<i16>,
    pub unsaturated_fat_g: Option<i16>,
    pub cholesterol_mg: Option<i16>,
    pub sodium_mg: Option<i16>,
    pub fiber_g: Option<i16>,
    pub trans_fat_g: Option<i16>,
    pub serving_size: Option<String>,
}

impl NutritionForCreate {
    /// Verifies whether all fields are blank.
    pub fn is_empty(&self) -> bool {
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
            && self.serving_size.is_none()
    }
}

impl From<NutritionInformationSchema> for NutritionForCreate {
    fn from(schema: NutritionInformationSchema) -> Self {
        Self {
            calories_kcal: schema.calories.map(i16::try_from).and_then(|res| res.ok()),
            total_carbohydrates: schema
                .carbohydrate_content
                .map(i16::try_from)
                .and_then(|res| res.ok()),
            sugars_g: schema
                .sugar_content
                .map(i16::try_from)
                .and_then(|res| res.ok()),
            protein_g: schema
                .protein_content
                .map(i16::try_from)
                .and_then(|res| res.ok()),
            total_fat_g: schema
                .fat_content
                .map(i16::try_from)
                .and_then(|res| res.ok()),
            saturated_fat_g: schema
                .saturated_fat_content
                .map(i16::try_from)
                .and_then(|res| res.ok()),
            unsaturated_fat_g: schema
                .unsaturated_fat_content
                .map(i16::try_from)
                .and_then(|res| res.ok()),
            cholesterol_mg: schema
                .cholesterol_content
                .map(i16::try_from)
                .and_then(|res| res.ok()),
            sodium_mg: schema
                .sodium_content
                .map(i16::try_from)
                .and_then(|res| res.ok()),
            fiber_g: schema
                .fiber_content
                .map(i16::try_from)
                .and_then(|res| res.ok()),
            trans_fat_g: schema
                .trans_fat_content
                .map(i16::try_from)
                .and_then(|res| res.ok()),
            serving_size: schema.serving_size,
        }
    }
}

/// Represents the nutritional information associated with a recipe in the database.
#[derive(Associations, Insertable)]
#[diesel(belongs_to(Recipe))]
#[diesel(table_name = schema::nutrition)]
pub(super) struct NutritionForInsert {
    pub recipe_id: i64,
    pub calories_kcal: Option<i16>,
    pub total_carbohydrates: Option<i16>,
    pub sugars_g: Option<i16>,
    pub protein_g: Option<i16>,
    pub total_fat_g: Option<i16>,
    pub saturated_fat_g: Option<i16>,
    pub unsaturated_fat_g: Option<i16>,
    pub cholesterol_mg: Option<i16>,
    pub sodium_mg: Option<i16>,
    pub fiber_g: Option<i16>,
    pub trans_fat_g: Option<i16>,
    pub serving_size: Option<String>,
}

/// Represents a time components of a recipe.
#[derive(Associations, Clone, Debug, Queryable, Identifiable, PartialEq, Selectable)]
#[diesel(belongs_to(Recipe))]
#[diesel(table_name = schema::times)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Times {
    pub id: i64,
    pub recipe_id: i64,
    pub prep_seconds: i32,
    pub cook_seconds: i32,
    pub total_seconds: i32,
}

/// Represents the preparation and cooking times for a recipe during creation.
#[derive(Clone, Debug, PartialEq)]
pub struct TimesForCreate {
    pub prep_seconds: i32,
    pub cook_seconds: i32,
}

impl Default for TimesForCreate {
    fn default() -> Self {
        Self {
            prep_seconds: 15 * 60,
            cook_seconds: 30 * 60,
        }
    }
}

impl TimesForCreate {
    /// Creates a TimesForCreate from its individual components.
    pub fn from_components(
        prep: Option<iso8601::Duration>,
        cook: Option<iso8601::Duration>,
    ) -> Self {
        Self {
            prep_seconds: prep
                .map(|d| {
                    let duration: std::time::Duration = d.into();
                    duration.as_secs()
                })
                .unwrap_or_else(|| 15 * 60) as i32,
            cook_seconds: cook
                .map(|d| {
                    let duration: std::time::Duration = d.into();
                    duration.as_secs()
                })
                .unwrap_or_else(|| 30 * 60) as i32,
        }
    }
}

/// Represents the preparation and cooking times for a recipe stored in the database.
#[derive(AsChangeset, Associations, Insertable, PartialEq)]
#[diesel(table_name = schema::times)]
#[diesel(belongs_to(Recipe))]
pub struct TimesForInsert {
    pub recipe_id: i64,
    pub prep_seconds: i32,
    pub cook_seconds: i32,
}

/// Represents a tool in the recipe management system.
#[derive(Debug, Queryable, Identifiable, PartialEq, Selectable)]
#[diesel(table_name = schema::tools)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tool {
    id: i64,
    name: String,
}

/// Represents a tool being created in the recipe management system.
#[derive(Debug, PartialEq)]
pub struct ToolForCreate {
    pub name: String,
    pub quantity: i16,
}

impl From<HowToToolOrText> for ToolForCreate {
    fn from(value: HowToToolOrText) -> Self {
        match value {
            HowToToolOrText::HowToTool(_) => {
                warn!("From<HowToToolOrText> for ToolForCreate to be implemented");

                Self {
                    name: "Undetermined".to_string(),
                    quantity: 1,
                }
            }
            HowToToolOrText::Text(s) => {
                let quantity = extract_number(s.clone()).unwrap_or(1);

                Self {
                    name: s.replace(&quantity.to_string(), "").trim().to_string(),
                    quantity,
                }
            }
        }
    }
}

/// Represents a tool being inserted into the `tools` table.
#[derive(Insertable)]
#[diesel(table_name = schema::tools)]
pub(super) struct ToolForInsert {
    pub name: String,
}

/// Represents the details of a tool used in a recipe.
#[derive(Debug, Clone, PartialEq)]
pub struct ToolRecipe {
    pub name: String,
    pub quantity: i16,
    pub tool_order: i16,
}

/// Represents the insertion of a tool-recipe relationship into the database.
#[derive(Associations, Insertable)]
#[diesel(table_name = schema::tools_recipes)]
#[diesel(belongs_to(Recipe))]
pub(super) struct ToolRecipeForInsert {
    pub tool_id: i64,
    pub recipe_id: i64,
    pub quantity: i16,
    pub tool_order: i16,
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
pub(super) struct IngredientForInsert {
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
pub(super) struct IngredientRecipeForInsert {
    pub ingredient_id: i64,
    pub recipe_id: i64,
    pub section_id: i64,
    pub item_order: i16,
}

/// Represents the data required to insert an instruction into the `instructions` table.
#[derive(Insertable)]
#[diesel(table_name = schema::instructions)]
pub(super) struct InstructionForInsert {
    pub name: String,
}

/// Represents the data required to insert an instruction-recipe association into the
/// `instructions_recipes` table.
#[derive(Associations, Insertable)]
#[diesel(table_name = schema::instructions_recipes)]
#[diesel(belongs_to(Recipe))]
pub(super) struct InstructionRecipeForInsert {
    pub instruction_id: i64,
    pub recipe_id: i64,
    pub section_id: i64,
    pub item_order: i16,
}

/// Represents a section in a recipe, typically used for organizing the recipe's
/// ingredients and instructions.
#[derive(Queryable, Identifiable, Selectable)]
#[diesel(table_name = schema::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(super) struct Section {
    pub id: i64,
    pub name: String,
}

/// Represents the data required to insert a new section into the `sections` table.
#[derive(Insertable)]
#[diesel(table_name = schema::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(in crate::core::model::recipe) struct SectionForInsert {
    pub name: String,
}

/// Represents the data required to insert an additional image associated with a recipe into
/// the `additional_images_recipe` table.
#[derive(Associations, Insertable)]
#[diesel(table_name = schema::additional_images_recipe)]
#[diesel(belongs_to(Recipe))]
pub(super) struct AdditionalImageForInsert {
    pub recipe_id: i64,
    pub image: Uuid,
}

/// Represents a video associated with a recipe.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Video {
    pub video: Uuid,
    pub duration: Option<chrono::Duration>,
    pub content_url: Option<String>,
    pub embed_url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

/// Represents the association between a video and a recipe in the `videos_recipes` table.
#[derive(Debug, PartialEq, Identifiable, Selectable, Queryable, Associations)]
#[diesel(table_name = schema::videos_recipes)]
#[diesel(belongs_to(Recipe))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VideoRecipe {
    pub id: i64,
    pub video: Uuid,
    pub recipe_id: i64,
    pub duration: Option<PgInterval>,
    pub content_url: Option<String>,
    pub embed_url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

/// Represents the data required to create a new video associated with a recipe.
#[derive(Clone)]
pub struct VideoForCreate {
    pub video: Uuid,
    pub duration: Option<chrono::Duration>,
    pub content_url: Option<String>,
    pub embed_url: Option<String>,
}

impl VideoForCreate {
    /// Creates the object from a Path.
    pub async fn from_path(fs_support: Arc<dyn FsSupport + Sync + Send>, path: &Path) -> Self {
        Self {
            video: path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
                .parse::<Uuid>()
                .unwrap_or_default(),
            duration: fs_support
                .calc_video_duration(path.to_str().unwrap_or_default())
                .await
                .ok(),
            content_url: None,
            embed_url: None,
        }
    }
}

/// Represents the data required to insert a new video associated with a recipe into
/// the `videos_recipes` table.
#[derive(Associations, Insertable)]
#[diesel(table_name = schema::videos_recipes)]
#[diesel(belongs_to(Recipe))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(super) struct VideoForInsert {
    pub video: Uuid,
    pub recipe_id: i64,
    pub duration: Option<PgInterval>,
    pub content_url: Option<String>,
    pub embed_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_recipe_details {
        use super::*;

        fn a_recipe() -> RecipeDetails {
            RecipeDetails {
                recipe: Recipe {
                    id: 0,
                    name: "".to_string(),
                    description: None,
                    image: None,
                    yield_: 0,
                    language: "".to_string(),
                    source: None,
                    created_at: Default::default(),
                    updated_at: Default::default(),
                    user_id: 0,
                },
                additional_images: vec![],
                category: "".to_string(),
                cuisine: None,
                ingredients: vec![],
                instructions: vec![],
                keywords: vec![],
                nutrition: None,
                times: Times {
                    id: 0,
                    recipe_id: 0,
                    prep_seconds: 0,
                    cook_seconds: 0,
                    total_seconds: 0,
                },
                tools: vec![],
                videos: vec![],
            }
        }

        #[test]
        fn test_num_images_no_images_ok() {
            let recipe = a_recipe();

            let got = recipe.num_images();

            pretty_assertions::assert_eq!(got, 0);
        }

        #[test]
        fn test_num_images_only_main_image_ok() {
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
        fn test_num_media_no_images_ok() {
            let mut recipe = a_recipe();
            recipe.videos = vec![Video::default(), Video::default()];

            let got = recipe.num_videos();

            pretty_assertions::assert_eq!(got, 2);
        }

        #[test]
        fn test_num_media_ok() {
            let mut recipe = a_recipe();
            recipe.recipe.image = Some(Uuid::new_v4());
            recipe.additional_images = vec![Uuid::new_v4(), Uuid::new_v4()];
            recipe.videos = vec![Video::default(), Video::default()];

            let got = recipe.num_media();

            pretty_assertions::assert_eq!(got, 5);
        }

        #[test]
        fn test_num_videos_ok() {
            let mut recipe = a_recipe();
            recipe.videos = vec![Video::default(), Video::default(), Video::default()];

            let got = recipe.num_videos();

            pretty_assertions::assert_eq!(got, 3);
        }

        #[test]
        fn test_all_images_no_images_ok() {
            let recipe = a_recipe();

            let got = recipe.all_images();

            let got_str = got.iter().map(|v| v.to_string()).collect::<Vec<_>>();
            pretty_assertions::assert_eq!(got_str, Vec::<String>::new());
        }

        #[test]
        fn test_all_images_only_main_image_ok() {
            let mut recipe = a_recipe();
            let an_image = Uuid::new_v4();
            recipe.recipe.image = Some(an_image);

            let got = recipe.all_images();

            let got_str = got.iter().map(|v| v.to_string()).collect::<Vec<_>>();
            pretty_assertions::assert_eq!(got_str, vec![an_image.to_string()]);
        }

        #[test]
        fn test_all_images_ok() {
            let mut recipe = a_recipe();
            let an_image = Uuid::new_v4();
            let an_image2 = Uuid::new_v4();
            recipe.recipe.image = Some(an_image);
            recipe.additional_images = vec![an_image2];

            let got = recipe.all_images();

            let got_str = got.iter().map(|v| v.to_string()).collect::<Vec<_>>();
            pretty_assertions::assert_eq!(
                got_str,
                vec![an_image.to_string(), an_image2.to_string()]
            );
        }
    }

    mod tests_nutrition {
        use super::*;

        #[test]
        fn test_empty_nutrition() {
            let nutrition = Nutrition::default();

            let got = nutrition.to_line();

            assert_eq!(got, "No nutrition available.");
        }

        #[test]
        fn test_single_nutrition_value() {
            let nutrition = Nutrition {
                calories_kcal: Some(200),
                ..Default::default()
            };

            let got = nutrition.to_line();

            assert_eq!(got, "Per serving: calories 200 kcal");
        }

        #[test]
        fn test_multiple_nutrition_values() {
            let nutrition = Nutrition {
                calories_kcal: Some(200),
                total_carbohydrates: Some(50),
                sugars_g: Some(20),
                protein_g: Some(10),
                total_fat_g: Some(5),
                serving_size: Some("100g".into()),
                ..Default::default()
            };

            let got = nutrition.to_line();

            assert_eq!(
                got,
                "Per 100g: calories 200 kcal; total carbohydrates 50g; sugar 20g; protein 10g; total fat 5g"
            );
        }

        #[test]
        fn test_nutrition_with_some_empty_values() {
            let nutrition = Nutrition {
                calories_kcal: Some(250),
                sodium_mg: Some(300),
                ..Default::default()
            };

            let got = nutrition.to_line();

            assert_eq!(got, "Per serving: calories 250 kcal; sodium 300mg");
        }

        #[test]
        fn test_nutrition_with_all_values() {
            let nutrition = Nutrition {
                calories_kcal: Some(200),
                total_carbohydrates: Some(50),
                sugars_g: Some(20),
                protein_g: Some(10),
                total_fat_g: Some(5),
                saturated_fat_g: Some(2),
                unsaturated_fat_g: Some(1),
                trans_fat_g: Some(0),
                cholesterol_mg: Some(30),
                sodium_mg: Some(300),
                fiber_g: Some(5),
                serving_size: Some("100g".into()),
                ..Default::default()
            };

            let got = nutrition.to_line();

            assert_eq!(
                got,
                "Per 100g: calories 200 kcal; total carbohydrates 50g; sugar 20g; protein 10g; total fat 5g; saturated fat 2g; unsaturated fat 1g; trans fat 0g; cholesterol 30mg; sodium 300mg; fiber 5g"
            );
        }
    }

    mod tests_recipe_for_create {
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
    }
}
