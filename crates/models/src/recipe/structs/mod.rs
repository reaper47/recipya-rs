pub mod media;
pub mod nutrition;
pub mod recipe;
pub mod section;
pub mod time;
pub mod tool;

#[cfg(feature = "test-utils")]
pub mod test_utils {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use uuid::Uuid;

    use crate::{
        Recipe, RecipeDetails,
        recipe::structs::{
            media::VideoForCreate,
            nutrition::{Nutrition, NutritionForCreate},
            recipe::RecipeForCreate,
            section::{Item, SectionComponents, SectionItem},
            time::{Times, TimesForCreate},
            tool::{ToolForCreate, ToolRecipe},
        },
    };

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
            ingredients: SectionComponents::Grouped(vec![
                SectionItem::new(
                    "Sauce",
                    vec![
                        Item::new("1 cup blue spinach"),
                        Item::new("1/2 tbsp cinnamon"),
                    ],
                ),
                SectionItem::new(
                    "Main",
                    vec![
                        Item::new("4 pounds top quality chicken filet"),
                        Item::new("1/8 cup lemon juice"),
                    ],
                ),
            ]),
            instructions: SectionComponents::Grouped(vec![
                SectionItem::new("Sauce", vec![Item::new("Mix all these ingredients")]),
                SectionItem::new(
                    "Chicken",
                    vec![
                        Item::new("Turn the oven at 300 F"),
                        Item::new("Soak the chicken in the lemon juice"),
                        Item {
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
