mod common;

use std::sync::LazyLock;

use divan::{Bencher, counter::ItemsCount};

use models::{
    Recipe, RecipeDetails,
    recipe::structs::{
        recipe::RecipeForCreate,
        section::{Item, SectionComponents, SectionItem},
    },
    settings::UserSettingDetails,
    user::User,
};
use test_db::default_config;
use test_fixtures::insert_user;
use test_harness::{AppState, create_app_state};

use crate::common::{RUNTIME, unique_name};

struct Seeded {
    state: AppState,
    user: User,
    settings: UserSettingDetails,
    recipe_id: i64,
}

static SEEDED: LazyLock<Seeded> = LazyLock::new(|| {
    RUNTIME.block_on(async {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await.unwrap();
        let settings = UserSettingDetails::get(&state.mm, user.id).await.unwrap();
        let recipe_id = Recipe::create(&state.mm, user.id, &bare_minimum_recipe(), &settings)
            .await
            .unwrap();
        Seeded {
            state,
            user,
            settings,
            recipe_id,
        }
    })
});

fn bare_minimum_recipe() -> RecipeForCreate {
    RecipeForCreate {
        name: "Best Chinese Kale".into(),
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
        measurement_system_id: 2,
        ..Default::default()
    }
}

fn load_seeded() -> RecipeDetails {
    let s = &*SEEDED;

    RUNTIME
        .block_on(Recipe::get(&s.state.mm, s.user.id, s.recipe_id))
        .unwrap()
}

fn main() {
    let s = &*SEEDED;
    let mut got = load_seeded();
    RUNTIME
        .block_on(got.bold_ingredients_in_instructions(&s.state.mm))
        .unwrap();

    let SectionComponents::Grouped(sections) = &got.instructions else {
        panic!("expected grouped instructions");
    };
    assert!(
        sections[1].items[1].text.contains("<b>chicken</b>"),
        "ingredients were not bolded"
    );

    divan::main();
}

#[divan::bench]
fn bold_ingredients(bencher: Bencher) {
    let s = &*SEEDED;

    bencher.with_inputs(load_seeded).bench_refs(|recipe| {
        RUNTIME.block_on(recipe.bold_ingredients_in_instructions(&s.state.mm))
    });
}

#[divan::bench]
fn bold_after_update(bencher: Bencher) {
    let s = &*SEEDED;

    bencher
        .with_inputs(|| {
            let mut recipe = load_seeded();
            recipe.instructions = SectionComponents::Grouped(vec![SectionItem::new(
                "Chicken",
                vec![
                    Item::new("Wash the blue spinach carefully."),
                    Item::new("Soak the chicken in the lemon juice."),
                    Item::new("Bake for 35 minutes then sprinkle with cinnamon.")
                        .with_duration(2100),
                ],
            )]);
            recipe
        })
        .bench_refs(|recipe| {
            RUNTIME.block_on(recipe.bold_ingredients_in_instructions(&s.state.mm))
        });
}

#[divan::bench(args = [3, 30, 300])]
fn by_instruction_count(bencher: Bencher, n: usize) {
    let s = &*SEEDED;

    bencher
        .counter(ItemsCount::new(n))
        .with_inputs(|| {
            let mut recipe = load_seeded();
            recipe.instructions = SectionComponents::Flat(
                (0..n)
                    .map(|i| {
                        Item::new(format!(
                            "Step {i}: soak the chicken in the lemon juice, add cinnamon and blue spinach."
                        ))
                    })
                    .collect(),
            );
            recipe
        })
        .bench_refs(|recipe| {
            RUNTIME.block_on(recipe.bold_ingredients_in_instructions(&s.state.mm))
        });
}

#[divan::bench(args = [4, 40, 200])]
fn by_ingredient_count(bencher: Bencher, n: usize) {
    let s = &*SEEDED;

    let mut recipe = bare_minimum_recipe();
    recipe.name = unique_name();
    recipe.ingredients = SectionComponents::Flat(
        (0..n)
            .map(|i| Item::new(format!("{i} cup ingredient{i}")))
            .collect(),
    );
    let id = RUNTIME
        .block_on(Recipe::create(&s.state.mm, s.user.id, &recipe, &s.settings))
        .unwrap();

    bencher
        .counter(ItemsCount::new(n))
        .with_inputs(|| {
            let mut details = RUNTIME
                .block_on(Recipe::get(&s.state.mm, s.user.id, id))
                .unwrap();
            details.instructions = SectionComponents::Flat(vec![Item::new(
                "Mix ingredient0, ingredient1 and ingredient3 together.",
            )]);
            details
        })
        .bench_refs(|details| {
            RUNTIME.block_on(details.bold_ingredients_in_instructions(&s.state.mm))
        });
}

#[divan::bench]
fn update_instructions(bencher: Bencher) {
    let s = &*SEEDED;

    bencher
        .with_inputs(|| {
            let mut recipe = bare_minimum_recipe();
            recipe.instructions = SectionComponents::Grouped(vec![SectionItem::new(
                "Chicken",
                vec![
                    Item::new("Wash the blue spinach carefully.").with_id(2),
                    Item::new("Soak the chicken in the lemon juice.").with_id(3),
                    Item::new("Bake for 35 minutes then sprinkle with cinnamon.").with_id(4),
                ],
            )]);
            recipe
        })
        .bench_refs(|recipe| {
            RUNTIME.block_on(Recipe::update(&s.state.mm, s.user.id, s.recipe_id, recipe))
        });
}
