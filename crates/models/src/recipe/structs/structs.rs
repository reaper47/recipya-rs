use std::path::Path;
use std::sync::Arc;

use diesel::data_types::PgInterval;
use diesel::internal::derives::multiconnection::chrono;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

use repository::schema;
use schema_org::field::{RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum};
use support::fs::FsSupport;

use crate::recipe::{RecipeForCreate, Sections};

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
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(super) struct InstructionForInsert {
    pub name: String,
    pub duration_seconds: Option<i32>,
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

#[cfg(feature = "test-utils")]
pub mod test_utils {
    use super::*;
    use crate::recipe::{
        Nutrition, NutritionForCreate, RecipeForCreate, Times, TimesForCreate, ToolForCreate,
        ToolRecipe, VideoForCreate,
    };
    use crate::{Recipe, RecipeDetails};
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    /// Constructs a `RecipeDetails` based on a complete `RecipeForCreate` instance.
    pub fn a_complete_recipe() -> RecipeDetails {
        let recipe_c = a_complete_recipe_for_create();

        let images = recipe_c.images;
        let additional_images = images.last().iter().cloned().cloned().collect::<Vec<_>>();

        let created_date = NaiveDate::from_ymd_opt(2012, 12, 31).expect("end of the world");
        let updated_date = NaiveDate::from_ymd_opt(2022, 2, 24).expect("russia invaded Ukraine");
        let time = NaiveTime::from_hms_opt(0, 0, 0).expect("invalid time");

        RecipeDetails {
            recipe: Recipe {
                id: 1,
                name: recipe_c.name,
                description: recipe_c.description,
                image: images.first().cloned().or(None),
                yield_: recipe_c.r#yield.ok_or(4).expect("a yield found"),
                language: "en".into(),
                measurement_system_id: 2,
                notes: Some("# Notes\n\nHere are some notes".into()),
                source: recipe_c.source,
                is_favourite: false,
                rating: None,
                created_at: NaiveDateTime::new(created_date, time),
                updated_at: NaiveDateTime::new(updated_date, time),
                user_id: 1,
            },
            additional_images,
            category: recipe_c.category.expect("a category"),
            cuisine: recipe_c.cuisine,
            ingredients: recipe_c.ingredients,
            instructions: recipe_c.instructions,
            keywords: recipe_c.keywords,
            nutrition: recipe_c.nutrition.map(|n| Nutrition {
                id: 1,
                recipe_id: 1,
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
                serving_size: n.serving_size,
            }),
            times: Times {
                id: 1,
                recipe_id: 1,
                prep_seconds: 3600,
                cook_seconds: 900,
                total_seconds: 4500,
            },
            tools: recipe_c
                .tools
                .iter()
                .enumerate()
                .map(|(i, t)| ToolRecipe {
                    name: t.name.clone(),
                    quantity: t.quantity,
                    tool_order: i as i16,
                })
                .collect(),
            videos: vec![],
        }
    }

    /// Constructs a `RecipeForCreate` instance with all components populated, preparing
    /// it for database insertion.
    pub fn a_complete_recipe_for_create() -> RecipeForCreate {
        let main_image = Uuid::nil();
        let secondary_image = Uuid::nil();
        let video = Uuid::nil();

        RecipeForCreate {
            name: "Best Chinese Kale".into(),
            description: Some("This is the most delicious recipe!".into()),
            images: vec![main_image, secondary_image],
            measurement_system_id: 2,
            r#yield: Some(4),
            source: "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/".into(),
            is_favourite: false,
            rating: Some(4),
            videos: vec![VideoForCreate {
                video,
                duration: Some(chrono::Duration::minutes(7)),
                content_url: Some("https://example.com/best-food.mp4".into()),
                embed_url: Some("https://example.com/embed/j43yfe3.mp4".into()),
            }],
            category: Some("dinner".into()),
            cuisine: Some("thai".into()),
            ingredients: vec![
                RecipeRecipeIngredientFieldEnum::new_section("Sauce", vec![
                    "1 cup blue spinach",
                    "1/2 tbsp cinnamon",
                ]),
                RecipeRecipeIngredientFieldEnum::new_section("Main", vec![
                    "4 pounds top quality chicken filet".into(),
                    "1/8 cup lemon juice".into(),
                ]),

            ],
            instructions: vec![
                RecipeRecipeInstructionsFieldEnum::new_section("Sauce", vec![
                    "Mix all these ingredients",
                ]),
                RecipeRecipeInstructionsFieldEnum::new_section("Chicken", vec![
                    "Mix all these ingredients",
                ]),
            ], Sections::from([
                (
                    .into(),
                    vec![SectionItem::new()],
                ),
                (
                    "".into(),
                    vec![
                        SectionItem::new("Turn the oven at 300 F"),
                        SectionItem::new("Soak the chicken in the lemon juice"),
                        SectionItem {
                            text: "Bake for 35 minutes".into(),
                            duration_seconds: Some(2100),
                        },
                    ],
                ),
            ]),
            keywords: Vec::<String>::from(["vegetarian".into(), "tofu".into()]),
            notes: Some("# My Recipe Notes\n\nThis dish should be served medium-cold".into()),
            nutrition: Some(NutritionForCreate {
                calories_kcal: Some(300),
                total_carbohydrates: Some(55),
                sugars_g: Some(43),
                protein_g: Some(7),
                total_fat_g: Some(6),
                saturated_fat_g: Some(1),
                unsaturated_fat_g: Some(2),
                cholesterol_mg: Some(5),
                sodium_mg: Some(12),
                fiber_g: Some(10),
                trans_fat_g: Some(3),
                serving_size: Some("100g".into()),
            }),
            times: Some(TimesForCreate {
                prep_seconds: 120,
                cook_seconds: 3600,
            }),
            tools: vec![
                ToolForCreate {
                    name: "wok".into(),
                    quantity: 1,
                },
                ToolForCreate {
                    name: "frying pan".into(),
                    quantity: 1,
                },
            ],
        }
    }
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

            let got_str = got.iter().map(|v| v.to_string()).collect::<Vec<_>>();
            pretty_assertions::assert_eq!(got_str, Vec::<String>::new());
        }

        #[test]
        fn test_all_images_only_main_image() {
            let mut recipe = a_recipe();
            let an_image = Uuid::new_v4();
            recipe.recipe.image = Some(an_image);

            let got = recipe.all_images();

            let got_str = got.iter().map(|v| v.to_string()).collect::<Vec<_>>();
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

        #[test]
        fn test_detect_language_en() {
            let recipe_c = RecipeForCreate {
                name: "The best hamburger ever".into(),
                description: Some("This is the best hamburger ever".into()),
                ingredients: Sections::from([(
                    "".into(),
                    vec![
                        SectionItem::new("1 cup of flour"),
                        SectionItem::new("1 cup of water"),
                    ],
                )]),
                instructions: Sections::from([(
                    "".into(),
                    vec![
                        SectionItem::new("Mix all ingredients"),
                        SectionItem::new("Bake for 30 minutes"),
                    ],
                )]),
                ..Default::default()
            };

            let got = recipe_c.detect_language();

            pretty_assertions::assert_eq!(got, Lang::Eng);
        }
    }
}
