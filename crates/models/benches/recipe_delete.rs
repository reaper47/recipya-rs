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
}

static SEEDED: LazyLock<Seeded> = LazyLock::new(|| {
    RUNTIME.block_on(async {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await.unwrap();
        let settings = UserSettingDetails::get(&state.mm, user.id).await.unwrap();
        Seeded {
            state,
            user,
            settings,
        }
    })
});

fn main() {
    let s = &*SEEDED;

    RUNTIME.block_on(async {
        assert!(
            Recipe::delete(&s.state.mm, i64::MAX, s.user.id)
                .await
                .is_err()
        );
    });

    divan::main();
}

fn fresh_recipe(category: Option<&str>) -> i64 {
    let s = &*SEEDED;
    let (mut recipe, _) = a_complete_recipe_for_create();
    recipe.name = unique_name();
    recipe.category = category.map(str::to_string);

    RUNTIME
        .block_on(Recipe::create(&s.state.mm, s.user.id, &recipe, &s.settings))
        .expect("create recipe")
}

#[divan::bench_group]
mod delete {
    use super::{Bencher, RUNTIME, Recipe, SEEDED, fresh_recipe};

    #[divan::bench]
    fn found(bencher: Bencher) {
        let s = &*SEEDED;

        bencher
            .with_inputs(|| fresh_recipe(None))
            .bench_values(|id| RUNTIME.block_on(Recipe::delete(&s.state.mm, id, s.user.id)));
    }

    #[divan::bench]
    fn not_found(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| RUNTIME.block_on(Recipe::delete(&s.state.mm, i64::MAX, s.user.id)));
    }
}

#[divan::bench_group(sample_count = 30)]
mod delete_bulk {
    use super::{
        Bencher, ItemsCount, RUNTIME, Recipe, SEEDED, UserSettingDetails, Uuid,
        a_complete_recipe_for_create, fresh_recipe, unique_name,
    };

    #[divan::bench(args = [1, 5, 20])]
    fn all_owned(bencher: Bencher, n: usize) {
        let s = &*SEEDED;

        bencher
            .counter(ItemsCount::new(n))
            .with_inputs(|| (0..n).map(|_| fresh_recipe(None)).collect::<Vec<_>>())
            .bench_values(|ids| {
                RUNTIME.block_on(Recipe::delete_bulk(&s.state.mm, &ids, s.user.id))
            });
    }

    #[divan::bench]
    fn some_not_owned(bencher: Bencher) {
        let s = &*SEEDED;
        let other = RUNTIME.block_on(async {
            let u = test_fixtures::insert_other_user(
                &s.state.mm,
                &format!("{}@bench.test", Uuid::new_v4()),
            )
            .await
            .unwrap();
            let settings = UserSettingDetails::get(&s.state.mm, u.id).await.unwrap();
            (u, settings)
        });

        bencher
            .with_inputs(|| {
                let mine = fresh_recipe(None);
                let (mut recipe, _) = a_complete_recipe_for_create();
                recipe.name = unique_name();
                let theirs = RUNTIME
                    .block_on(Recipe::create(&s.state.mm, other.0.id, &recipe, &other.1))
                    .expect("create other recipe");
                vec![mine, theirs]
            })
            .bench_values(|ids| {
                RUNTIME.block_on(Recipe::delete_bulk(&s.state.mm, &ids, s.user.id))
            });
    }
}

#[divan::bench_group(sample_count = 30)]
mod delete_recipe_category {
    use super::{Bencher, ItemsCount, RUNTIME, Recipe, SEEDED, Uuid, fresh_recipe};

    #[divan::bench]
    fn not_found(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| {
            RUNTIME.block_on(Recipe::delete_recipe_category(
                &s.state.mm,
                "midnight crunchies",
                s.user.id,
            ))
        });
    }

    #[divan::bench(args = [1, 10])]
    fn found(bencher: Bencher, n: usize) {
        let s = &*SEEDED;

        bencher
            .counter(ItemsCount::new(n))
            .with_inputs(|| {
                let name = format!("category-{}", Uuid::new_v4());
                RUNTIME
                    .block_on(Recipe::add_category(&s.state.mm, &name, s.user.id))
                    .unwrap();
                for _ in 0..n {
                    fresh_recipe(Some(&name));
                }
                name
            })
            .bench_refs(|name| {
                RUNTIME.block_on(Recipe::delete_recipe_category(&s.state.mm, name, s.user.id))
            });
    }
}
