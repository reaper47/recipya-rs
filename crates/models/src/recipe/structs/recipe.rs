use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;
use whatlang::Lang;

use math::cooking::units;
use repository::schema;
use schema_org::field::{RecipeDescriptionFieldEnum, RecipeKeywordsFieldEnum, RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum};
use support::name_entity_with_relations;

use crate::recipe::{Nutrition, NutritionForCreate, RecipeForm, Sections, Times, TimesForCreate, ToolForCreate, ToolRecipe, Video, VideoForCreate};
use crate::user::User;

/// Represents a recipe entity stored in the database.
#[derive(
    Clone, Debug, Default, PartialEq, AsChangeset, Associations, Queryable, Identifiable, Selectable,
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
    pub yield_: i16,
    /// The language in which the recipe is written.
    pub language: String,
    /// The original measurement system the recipe is in.
    pub measurement_system_id: i16,
    /// Optional notes of the recipe.
    pub notes: Option<String>,
    /// An optional reference to the origin or inspiration of the recipe.
    pub source: String,
    /// Specifies whether the recipe has been marked as favourite.
    pub is_favourite: bool,
    /// Optional 1-5 rating. None is used for no rating.
    pub rating: Option<i16>,
    /// The timestamp when the nutrition entry was created.
    pub created_at: chrono::NaiveDateTime,
    /// The timestamp when the nutrition entry was last updated.
    pub updated_at: chrono::NaiveDateTime,
    /// The foreign key linking the recipe to its creator.
    pub user_id: i64,
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
    pub notes: Option<String>,
    pub source: String,
    pub is_favourite: bool,
    pub rating: Option<i16>,
    pub user_id: i64,
}

/// Represents the full details of a recipe, including its metadata and related entities.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct RecipeDetails {
    pub recipe: Recipe,
    pub additional_images: Vec<Uuid>,
    pub category: String,
    pub cuisine: Option<String>,
    pub ingredients: Vec<RecipeRecipeIngredientFieldEnum>,
    pub instructions: Vec<RecipeRecipeInstructionsFieldEnum>,
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

impl From<RecipeForCreate> for RecipeDetails {
    fn from(recipe_c: RecipeForCreate) -> Self {
        Self {
            recipe: Recipe {
                name: recipe_c.name.clone(),
                description: recipe_c.description.clone(),
                yield_: recipe_c.r#yield.unwrap_or(4),
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
            nutrition: recipe_c.nutrition.map(|n| Nutrition::from(&n)),
            times: Times::from(recipe_c.times.unwrap_or_default()),
            tools: recipe_c
                .tools
                .into_iter()
                .enumerate()
                .map(|(idx, t)| {
                    let mut tool = ToolRecipe::from(&t);
                    tool.tool_order = idx as i16;
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
    pub source: String,
    pub rating: Option<i16>,
    pub videos: Vec<VideoForCreate>,
    pub r#yield: Option<i16>,

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

    /// Determines the recipe's language.
    pub fn detect_language(&self) -> Lang {
        whatlang::detect_lang(
            &[
                self.name.as_str(),
                self.description.as_deref().unwrap_or(""),
                &itertools::Itertools::intersperse(
                    self.ingredients.items_as_text().iter().chain(&self.instructions.items_as_text()).map(|s| s.to_string()),
                    " ".into(),
                )
                    .collect::<String>(),
            ]
                .join(" "),
        )
            .unwrap_or(Lang::Eng)
    }
}

impl From<&RecipeForm> for RecipeForCreate {
    fn from(form: &RecipeForm) -> Self {
        let ingredients = &form.ingredients;
        let measurement_system_id = units::MeasurementSystem::from(ingredients.clone()).id();

        Self {
            name: form.title.clone(),
            description: form.description.clone(),
            images: vec![],
            r#yield: form.yield_,
            source: form.source.clone().unwrap_or_default(),
            is_favourite: false,
            rating: form.rating,
            videos: vec![],
            category: form.category.clone().or(Some("uncategorized".into())),
            cuisine: form.cuisine.clone(),
            ingredients: Sections::new(ingredients.clone()),
            instructions: Sections::new(form.instructions.clone()),
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
        let ingredients = form.ingredients;
        let measurement_system_id = units::MeasurementSystem::from(ingredients.clone()).id();

        Self {
            name: form.title,
            description: form.description,
            images: vec![],
            r#yield: form.yield_,
            source: form.source.unwrap_or_default(),
            is_favourite: false,
            rating: form.rating,
            videos: vec![],
            category: form.category.or(Some("uncategorized".into())),
            cuisine: form.cuisine,
            ingredients: Sections::new(ingredients),
            instructions: Sections::new(form.instructions),
            keywords: form.keywords,
            measurement_system_id,
            nutrition: form.nutrition,
            times: form.times,
            tools: form.tools,
            notes: form.notes,
        }
    }
}

impl From<&schema_org::Recipe> for RecipeForCreate {
    fn from(schema: &schema_org::Recipe) -> Self {
        let original_ingredients = schema.recipe_ingredient.clone();
        let ingredients = Sections::from(original_ingredients);
        let measurement_system_id = units::MeasurementSystem::from(ingredients.items_as_text()).id();

        Self {
            name: schema.name.first().cloned().unwrap_or_default(),
            description: schema.description.first().map(|v| match v {
                RecipeDescriptionFieldEnum::Text(s) => s.clone(),
                RecipeDescriptionFieldEnum::TextObject(obj) => obj.text.first().cloned().unwrap_or_default(),
            }),
            images: vec![],
            r#yield: schema.recipe_yield.first().map(|v| v.to_i16()).unwrap_or_default(),
            source: schema.url.first().cloned().unwrap_or_default(),
            is_favourite: false,
            rating: None, // TODO: Look into it because the Recipe schema doesn't have such dedicated field
            videos: vec![],
            category: schema.recipe_category.get(0).cloned(),
            cuisine: schema.recipe_cuisine.get(0).cloned(),
            ingredients,
            instructions: Sections::from(schema.recipe_instructions.clone()),
            keywords: schema.keywords.iter().map(|k| match k {
                RecipeKeywordsFieldEnum::DefinedTerm(term) => term.name.first().cloned().unwrap_or_default(),
                RecipeKeywordsFieldEnum::TextOrURL(s) => s.clone(),
            }).collect(),
            measurement_system_id,
            notes: None, // TODO: Look into it because the Recipe schema doesn't have such dedicated field
            nutrition: schema.nutrition.first().map(NutritionForCreate::from),
            times: Some(TimesForCreate::from_components(
                schema.prep_time.first().map(|d| d.to_iso8601()).unwrap_or_default(),
                schema.cook_time.first().map(|d| d.to_iso8601()).unwrap_or_default(),
            )),
            tools: schema
                .tool
                .iter()
                .map(ToolForCreate::from)
                .collect(),
        }
    }
}

/// Represents a measurement system entity stored in the database.
#[derive(Debug, Queryable, Identifiable, PartialEq, Selectable)]
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
