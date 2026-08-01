use time::Duration;

use models::recipe::structs::{
    media::VideoForCreate,
    nutrition::{
        NutritionDetailsForCreate, NutritionForCreate, NutritionPerServingDetailsForCreate,
    },
    recipe::RecipeForCreate,
    section::{Item, SectionComponents, SectionItem},
    time::TimesForCreate,
    tool::ToolForCreate,
    types::Source,
};
use test_fixtures::RecipeImages;

/// Constructs a `RecipeForCreate` instance with all components populated, preparing
/// it for database insertion.
#[allow(clippy::too_many_lines)]
pub fn a_complete_recipe_for_create() -> (RecipeForCreate, RecipeImages) {
    let images = RecipeImages::default();

    (
        RecipeForCreate {
            name: "Best Chinese Kale".into(),
            description: Some("This is the most delicious recipe!".into()),
            images: vec![images.main, images.additional],
            measurement_system_id: 2,
            r#yield: Some(4),
            source: Source::new(
                "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/",
            ),
            is_favourite: false,
            rating: Some(4),
            videos: vec![VideoForCreate {
                video: images.video,
                duration: Some(Duration::minutes(7)),
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
                        Item::new("Bake for 35 minutes").with_duration(2100),
                    ],
                ),
            ]),
            keywords: Vec::<String>::from(["vegetarian".into(), "tofu".into()]),
            notes: Some("# My Recipe Notes\n\nThis dish should be served medium-cold".into()),
            nutrition: NutritionDetailsForCreate {
                per_100g: Some(NutritionForCreate {
                    calories_kcal: Some(300),
                    total_carbohydrates: Some(55.),
                    sugars_g: Some(43.),
                    protein_g: Some(7.),
                    total_fat_g: Some(6.),
                    saturated_fat_g: Some(1.),
                    unsaturated_fat_g: Some(2.),
                    cholesterol_mg: Some(5.),
                    sodium_mg: Some(12.),
                    fiber_g: Some(10.),
                    trans_fat_g: Some(3.),
                }),
                per_serving: Some(NutritionPerServingDetailsForCreate {
                    nutrition: NutritionForCreate {
                        calories_kcal: Some(240),
                        total_carbohydrates: Some(30.),
                        sugars_g: Some(24.),
                        protein_g: Some(4.),
                        total_fat_g: Some(6.),
                        saturated_fat_g: Some(2.),
                        unsaturated_fat_g: Some(7.),
                        cholesterol_mg: Some(12.),
                        sodium_mg: Some(100.),
                        fiber_g: Some(18.),
                        trans_fat_g: Some(2.),
                    },
                    serving_size: "2 buns".into(),
                }),
            },
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
        },
        images,
    )
}
