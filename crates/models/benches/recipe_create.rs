mod common;

use std::sync::LazyLock;

use divan::{Bencher, counter::ItemsCount};
use uuid::Uuid;

use models::{
    Recipe,
    recipe::structs::{
        recipe::RecipeForCreate,
        section::{Item, SectionComponents, SectionItem},
        tool::ToolForCreate,
    },
    settings::UserSettingDetails,
    types::{Rating, RecipeYield},
    user::User,
};
use test_db::default_config;
use test_fixtures::insert_user;
use test_harness::{AppState, create_app_state};

use crate::common::{RUNTIME, a_complete_recipe_for_create, unique_name};

struct Seeded {
    state: AppState,
    user: User,
    settings: UserSettingDetails,
    existing_name: String,
}

static SEEDED: LazyLock<Seeded> = LazyLock::new(|| {
    RUNTIME.block_on(async {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await.unwrap();
        let settings = UserSettingDetails::get(&state.mm, user.id).await.unwrap();

        let (recipe, _) = a_complete_recipe_for_create();
        let existing_name = recipe.name.clone();
        Recipe::create(&state.mm, user.id, &recipe, &settings)
            .await
            .unwrap();

        Seeded {
            state,
            user,
            settings,
            existing_name,
        }
    })
});

fn bare_minimum_recipe() -> RecipeForCreate {
    RecipeForCreate {
        name: "Best Chinese Kale".into(),
        r#yield: RecipeYield::default(),
        rating: Rating::from(4),
        category: Some("uncategorized".into()),
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

fn main() {
    let s = &*SEEDED;

    RUNTIME.block_on(async {
        let (recipe, _) = a_complete_recipe_for_create();
        assert_eq!(recipe.name, s.existing_name);
        assert!(
            Recipe::create(&s.state.mm, s.user.id, &recipe, &s.settings)
                .await
                .is_err()
        );
    });

    divan::main();
}

#[divan::bench_group]
mod create {
    use super::{
        Bencher, Item, RUNTIME, Recipe, SEEDED, SectionComponents, ToolForCreate,
        a_complete_recipe_for_create, bare_minimum_recipe, unique_name,
    };

    #[divan::bench]
    fn complete(bencher: Bencher) {
        let s = &*SEEDED;
        bencher
            .with_inputs(|| {
                let (mut recipe, _) = a_complete_recipe_for_create();
                recipe.name = unique_name();
                recipe
            })
            .bench_refs(|recipe| {
                RUNTIME.block_on(Recipe::create(&s.state.mm, s.user.id, recipe, &s.settings))
            });
    }

    #[divan::bench]
    fn bare_minimum(bencher: Bencher) {
        let s = &*SEEDED;

        bencher
            .with_inputs(|| {
                let mut recipe = bare_minimum_recipe();
                recipe.name = unique_name();
                recipe
            })
            .bench_refs(|recipe| {
                RUNTIME.block_on(Recipe::create(&s.state.mm, s.user.id, recipe, &s.settings))
            });
    }

    #[divan::bench]
    fn uppercase_fields(bencher: Bencher) {
        let s = &*SEEDED;

        bencher
            .with_inputs(|| {
                let mut recipe = bare_minimum_recipe();
                recipe.name = unique_name();
                recipe.keywords = vec!["CHICKEN".into(), "MEAT".into()];
                recipe.category = Some("KVELDSMAT".into());
                recipe.cuisine = Some("NORWEGIAN".into());
                recipe.tools = vec![
                    ToolForCreate {
                        name: "FRYING PAN".into(),
                        quantity: 1,
                    },
                    ToolForCreate {
                        name: "WOK".into(),
                        quantity: 1,
                    },
                ];
                recipe
            })
            .bench_refs(|recipe| {
                RUNTIME.block_on(Recipe::create(&s.state.mm, s.user.id, recipe, &s.settings))
            });
    }

    #[divan::bench]
    fn parse_durations(bencher: Bencher) {
        let s = &*SEEDED;

        bencher
            .with_inputs(|| {
                let mut recipe = bare_minimum_recipe();
                recipe.name = unique_name();
                recipe.instructions = SectionComponents::Flat(vec![
                    Item::new("Heat oil on medium heat in a large"),
                    Item::new("When tomatoes have softened and have started to release their juices (about 4-5 min) add basil"),
                    Item::new("Simmer on low for at least 1 hour, or up to 6 hours, stirring occasionally. The longer you simmer, the better."),
                ]);
                recipe
            })
            .bench_refs(|recipe| {
                RUNTIME.block_on(Recipe::create(&s.state.mm, s.user.id, recipe, &s.settings))
            });
    }

    #[divan::bench]
    fn duplicate_name(bencher: Bencher) {
        let s = &*SEEDED;
        let (recipe, _) = a_complete_recipe_for_create();

        bencher.bench_local(|| {
            RUNTIME.block_on(Recipe::create(&s.state.mm, s.user.id, &recipe, &s.settings))
        });
    }
}

#[divan::bench(args = [4, 50, 200])]
fn by_size(bencher: Bencher, n: usize) {
    let s = &*SEEDED;

    bencher
        .counter(ItemsCount::new(n * 2))
        .with_inputs(|| {
            let mut recipe = bare_minimum_recipe();
            recipe.name = unique_name();
            recipe.ingredients = SectionComponents::Flat(
                (0..n)
                    .map(|i| Item::new(format!("{i} cup ingredient number {i}")))
                    .collect(),
            );
            recipe.instructions = SectionComponents::Flat(
                (0..n)
                    .map(|i| Item::new(format!("Step {i}: stir the pot gently.")))
                    .collect(),
            );
            recipe
        })
        .bench_refs(|recipe| {
            RUNTIME.block_on(Recipe::create(&s.state.mm, s.user.id, recipe, &s.settings))
        });
}

#[divan::bench_group]
mod add_category {
    use super::{Bencher, RUNTIME, Recipe, SEEDED, Uuid};

    #[divan::bench]
    fn new(bencher: Bencher) {
        let s = &*SEEDED;

        bencher
            .with_inputs(|| format!("category-{}", Uuid::new_v4()))
            .bench_refs(|name| {
                RUNTIME.block_on(Recipe::add_category(&s.state.mm, name, s.user.id))
            });
    }

    #[divan::bench]
    fn duplicate(bencher: Bencher) {
        let s = &*SEEDED;
        let name = format!("category-{}", Uuid::new_v4());
        RUNTIME
            .block_on(Recipe::add_category(&s.state.mm, &name, s.user.id))
            .unwrap();

        bencher
            .bench_local(|| RUNTIME.block_on(Recipe::add_category(&s.state.mm, &name, s.user.id)));
    }
}
