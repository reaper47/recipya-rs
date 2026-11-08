mod common;

use std::sync::LazyLock;

use divan::{Bencher, counter::ItemsCount};
use uuid::Uuid;

use models::{Recipe, settings::UserSettingDetails, user::User};
use test_db::default_config;
use test_fixtures::insert_user;
use test_harness::{AppState, create_app_state};

use crate::common::{RUNTIME, a_complete_recipe_for_create, unique_name};

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
        let recipe_id = insert_recipe(&state, user.id, &settings).await;
        Seeded {
            state,
            user,
            settings,
            recipe_id,
        }
    })
});

async fn insert_recipe(state: &AppState, user_id: Uuid, settings: &UserSettingDetails) -> i64 {
    let (mut recipe, _) = a_complete_recipe_for_create();
    recipe.name = unique_name();
    Recipe::create(&state.mm, user_id, &recipe, settings)
        .await
        .unwrap()
}

fn main() {
    let s = &*SEEDED;

    RUNTIME.block_on(async {
        assert!(
            Recipe::toggle_favourite(&s.state.mm, Uuid::new_v4(), s.recipe_id)
                .await
                .is_err()
        );
        Recipe::toggle_favourite(&s.state.mm, s.user.id, s.recipe_id)
            .await
            .unwrap();
        Recipe::toggle_favourite(&s.state.mm, s.user.id, s.recipe_id)
            .await
            .unwrap();
    });

    divan::main();
}

#[divan::bench_group]
mod toggle_favourite {
    use super::{Bencher, RUNTIME, Recipe, SEEDED, Uuid};

    #[divan::bench]
    fn valid(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| {
            RUNTIME.block_on(Recipe::toggle_favourite(
                &s.state.mm,
                s.user.id,
                s.recipe_id,
            ))
        });
    }

    #[divan::bench]
    fn unknown_user(bencher: Bencher) {
        let s = &*SEEDED;
        let user_id = Uuid::new_v4();

        bencher.bench_local(|| {
            RUNTIME.block_on(Recipe::toggle_favourite(&s.state.mm, user_id, s.recipe_id))
        });
    }
}

#[divan::bench_group]
mod update_category {
    use super::{Bencher, ItemsCount, RUNTIME, Recipe, SEEDED, Uuid, insert_recipe};

    #[divan::bench]
    fn empty_category(bencher: Bencher) {
        let s = &*SEEDED;
        let ids = vec![s.recipe_id].try_into().unwrap();

        bencher.bench_local(|| {
            RUNTIME.block_on(Recipe::update_category(&s.state.mm, "", &ids, s.user.id))
        });
    }

    #[divan::bench]
    fn single_recipe(bencher: Bencher) {
        let s = &*SEEDED;
        let ids = vec![s.recipe_id].try_into().unwrap();

        bencher.bench_local(|| {
            RUNTIME.block_on(Recipe::update_category(
                &s.state.mm,
                "breakfast",
                &ids,
                s.user.id,
            ))
        });
    }

    #[divan::bench(args = [1, 10, 50], sample_count = 20)]
    fn many_recipes(bencher: Bencher, n: usize) {
        let s = &*SEEDED;
        let ids: Vec<i64> = RUNTIME.block_on(async {
            let mut ids = Vec::with_capacity(n);
            for _ in 0..n {
                ids.push(insert_recipe(&s.state, s.user.id, &s.settings).await);
            }
            ids
        });
        let ids = ids.try_into().unwrap();

        bencher.counter(ItemsCount::new(n)).bench_local(|| {
            RUNTIME.block_on(Recipe::update_category(
                &s.state.mm,
                "breakfast",
                &ids,
                s.user.id,
            ))
        });
    }

    #[divan::bench]
    fn new_category(bencher: Bencher) {
        let s = &*SEEDED;
        let ids = vec![s.recipe_id].try_into().unwrap();

        bencher
            .with_inputs(|| format!("category-{}", Uuid::new_v4()))
            .bench_refs(|name| {
                RUNTIME.block_on(Recipe::update_category(&s.state.mm, name, &ids, s.user.id))
            });
    }
}
