mod common;

use std::{fs, path::Path};

use divan::{Bencher, black_box, counter::ItemsCount};
use tempfile::TempDir;

use models::{
    Recipe, RecipeDetails,
    export::{ExportData, ExportOptions, ExportType},
    recipe::structs::{
        media::Video,
        nutrition::{Nutrition, NutritionDetails, NutritionPerServingDetails},
        section::{Item, SectionComponents, SectionItem},
        time::Times,
        tool::ToolRecipe,
    },
    types::{DurationSeconds, Rating, RecipeYield, Source},
};
use test_fixtures::RecipeImages;
use time::Duration;

use crate::common::RUNTIME;

fn main() {
    let dir = TempDir::new().unwrap();
    let path = RUNTIME
        .block_on(
            ExportData::new(
                ExportType::Json,
                recipes(3),
                dir.path(),
                ExportOptions::default(),
            )
            .export(),
        )
        .unwrap();
    assert!(path.exists());

    let archive = zip::ZipArchive::new(fs::File::open(&path).unwrap()).unwrap();
    assert!(archive.file_names().any(|n| n.unwrap() == "manifest.json"));
    assert!(archive.file_names().any(|n| n.unwrap() == "1/recipe.json"));

    divan::main();
}

fn recipes(n: usize) -> Vec<RecipeDetails> {
    (0..n)
        .map(|i| {
            let mut r = a_complete_recipe_details();
            r.recipe.id = i64::try_from(i + 1).unwrap();
            r.recipe.name = format!("Recipe {i}");
            r
        })
        .collect()
}

fn write_images(dir: &Path, recipes: &[RecipeDetails], bytes: usize) {
    for r in recipes {
        for img in r.all_images() {
            fs::write(dir.join(format!("{img}.webp")), vec![0u8; bytes]).unwrap();
        }
    }
}

fn run(bencher: Bencher, r#type: &ExportType, n: usize) {
    let images = TempDir::new().unwrap();
    let base = recipes(n);

    bencher
        .counter(ItemsCount::new(n))
        .with_inputs(|| {
            ExportData::new(
                r#type.clone(),
                base.clone(),
                images.path(),
                ExportOptions::default(),
            )
        })
        .bench_values(|data| RUNTIME.block_on(data.export()));
}

#[divan::bench_group]
mod export {
    use super::{Bencher, ExportType, run};

    const SIZES: [usize; 3] = [1, 10, 100];

    #[divan::bench(args = SIZES)]
    fn json(bencher: Bencher, n: usize) {
        run(bencher, &ExportType::Json, n);
    }

    #[divan::bench(args = SIZES)]
    fn markdown(bencher: Bencher, n: usize) {
        run(bencher, &ExportType::Markdown, n);
    }

    #[divan::bench(args = SIZES)]
    fn text(bencher: Bencher, n: usize) {
        run(bencher, &ExportType::Text, n);
    }
}

#[divan::bench(args = [0, 64 * 1024, 1024 * 1024])]
fn export_with_images(bencher: Bencher, image_bytes: usize) {
    let images = TempDir::new().unwrap();
    let base = recipes(10);
    if image_bytes > 0 {
        write_images(images.path(), &base, image_bytes);
    }

    bencher
        .counter(ItemsCount::new(base.len()))
        .with_inputs(|| {
            ExportData::new(
                ExportType::Json,
                base.clone(),
                images.path(),
                ExportOptions::default(),
            )
        })
        .bench_values(|data| RUNTIME.block_on(data.export()));
}

#[divan::bench]
fn export_empty(bencher: Bencher) {
    let images = TempDir::new().unwrap();

    bencher
        .with_inputs(|| {
            ExportData::new(
                ExportType::Json,
                Vec::new(),
                images.path(),
                ExportOptions::default(),
            )
        })
        .bench_values(|data| RUNTIME.block_on(data.export()));
}

#[divan::bench(args = [ExportType::Json, ExportType::Markdown, ExportType::Pdf, ExportType::Text])]
fn extension(bencher: Bencher, r#type: &ExportType) {
    bencher.bench_local(|| black_box(r#type).extension());
}

#[allow(clippy::too_many_lines)]
fn a_complete_recipe_details() -> RecipeDetails {
    let images = RecipeImages::default();

    RecipeDetails {
        recipe: Recipe {
            id: 1,
            name: "Best Chinese Kale".into(),
            description: Some("This is the most delicious recipe!".into()),
            image: Some(images.main),
            r#yield: RecipeYield::default(),
            language: "en".into(),
            measurement_system_id: 2,
            notes: Some("# My Recipe Notes\n\nThis dish should be served medium-cold".into()),
            source: Source::new(
                "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/",
            ),
            is_favourite: false,
            rating: Rating::from(4),
            created_at: time::macros::datetime!(2023-01-01 00:00),
            updated_at: time::macros::datetime!(2023-01-01 00:00),
            user_id: uuid::Uuid::nil(),
        },
        additional_images: vec![images.additional],
        category: "dinner".into(),
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
        keywords: vec!["vegetarian".into(), "tofu".into()],
        nutrition: NutritionDetails {
            per_100g: Some(Nutrition {
                id: 1,
                is_precalculated_by_source: false,
                calories_kcal: Some(300),
                total_carbohydrates: Some(55.),
                sugars_g: Some(43.),
                protein_g: Some(7.),
                total_fat_g: Some(6.),
                saturated_fat_g: Some(1.),
                unsaturated_fat_g: Some(2.),
                cholesterol_mg: Some(5.),
                sodium_mg: Some(12.),
                fiber_g: Some(10.), // Note: fibre_g in Create, fiber_g in Nutrition
                trans_fat_g: Some(3.),
            }),
            per_serving: Some(NutritionPerServingDetails {
                nutrition: Nutrition {
                    id: 2,
                    is_precalculated_by_source: false,
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
        times: Times {
            id: 1,
            recipe_id: 1,
            prep_seconds: DurationSeconds::new(120),
            cook_seconds: DurationSeconds::new(3600),
            total_seconds: DurationSeconds::new(3720),
        },
        tools: vec![
            ToolRecipe {
                name: "wok".into(),
                quantity: 1,
                tool_order: 0,
            },
            ToolRecipe {
                name: "frying pan".into(),
                quantity: 1,
                tool_order: 1,
            },
        ],
        videos: vec![Video {
            video: images.video,
            duration: Some(Duration::minutes(7)),
            content_url: Some("https://example.com/best-food.mp4".into()),
            embed_url: Some("https://example.com/embed/j43yfe3.mp4".into()),
            created_at: time::macros::datetime!(2023-01-01 00:00),
        }],
    }
}
