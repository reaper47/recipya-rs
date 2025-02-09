use diesel::data_types::PgInterval;
use diesel::internal::derives::multiconnection::chrono;
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::core::model::user::User;
use crate::core::repository::schema;
use crate::name_entity_with_relations;

/// Represents a collection of sections, where each section has a title and a list of associated items.
pub type Sections = Vec<(String, Vec<String>)>;

/// Represents a recipe entity stored in the database.
#[derive(Associations, Debug, Queryable, Identifiable, PartialEq, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = schema::recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    /// The unique identifier for the recipe.
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
pub struct RecipeForCreate {
    // For the recipe table
    pub name: String,
    pub description: Option<String>,
    pub images: Option<Vec<Uuid>>,
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
#[derive(Debug, PartialEq)]
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

/// Represents a nutrition entity stored in the database.
#[derive(Associations, Debug, Queryable, Identifiable, PartialEq, Selectable)]
#[diesel(belongs_to(Recipe))]
#[diesel(table_name = schema::nutrition)]
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

/// Represents the nutritional information provided when creating a new recipe.
#[derive(Clone)]
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
#[derive(Associations, Debug, Queryable, Identifiable, PartialEq, Selectable)]
#[diesel(belongs_to(Recipe))]
#[diesel(table_name = schema::times)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Times {
    pub(super) id: i64,
    pub(super) recipe_id: i64,
    pub prep_seconds: i32,
    pub cook_seconds: i32,
    pub total_seconds: i32,
}

/// Represents the preparation and cooking times for a recipe during creation.
#[derive(Clone)]
pub struct TimesForCreate {
    pub prep_seconds: i32,
    pub cook_seconds: i32,
}

/// Represents the preparation and cooking times for a recipe stored in the database.
#[derive(Associations, Insertable)]
#[diesel(table_name = schema::times)]
#[diesel(belongs_to(Recipe))]
pub(super) struct TimesForInsert {
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
pub struct ToolForCreate {
    pub name: String,
    pub quantity: i16,
}

/// Represents a tool being inserted into the `tools` table.
#[derive(Insertable)]
#[diesel(table_name = schema::tools)]
pub(super) struct ToolForInsert {
    pub name: String,
}

/// Represents the details of a tool used in a recipe.
#[derive(Debug, PartialEq)]
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
pub(super) struct SectionForInsert {
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
#[derive(Debug, PartialEq)]
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
