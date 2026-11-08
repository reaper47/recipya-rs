mod common;

use std::sync::LazyLock;

use divan::{Bencher, counter::ItemsCount};
use uuid::Uuid;

use models::{Recipe, params::SearchParams, settings::UserSettingDetails, user::User};
use test_db::default_config;
use test_fixtures::insert_user;
use test_harness::{AppState, create_app_state};

use crate::common::{RUNTIME, a_complete_recipe_for_create};

const LIBRARY_SIZE: usize = 40;

struct Seeded {
    state: AppState,
    user: User,
    ids: Vec<i64>,
}

static SEEDED: LazyLock<Seeded> = LazyLock::new(|| {
    RUNTIME.block_on(async {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await.unwrap();
        let ids = insert_recipes(&state, &user, LIBRARY_SIZE).await;
        Seeded { state, user, ids }
    })
});

async fn insert_recipes(state: &AppState, user: &User, n: usize) -> Vec<i64> {
    let settings = UserSettingDetails::get(&state.mm, user.id).await.unwrap();
    let mut ids = Vec::with_capacity(n);
    for i in 0..n {
        let (mut recipe, _) = a_complete_recipe_for_create();
        recipe.name.push_str(&i.to_string());
        recipe.category = Some(["dinner", "late snack", "dessert"][i % 3].into());
        recipe.cuisine = Some(["thai", "italian", "mexican"][i % 3].into());
        ids.push(
            Recipe::create(&state.mm, user.id, &recipe, &settings)
                .await
                .unwrap(),
        );
    }
    ids
}

fn main() {
    let s = &*SEEDED;

    RUNTIME.block_on(async {
        assert_eq!(
            Recipe::count(&s.state.mm, s.user.id).await.unwrap(),
            i64::try_from(LIBRARY_SIZE).unwrap()
        );
        let page = Recipe::get_page(
            &s.state.mm,
            s.user.id,
            &SearchParams {
                page: Some(1),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        assert_eq!(page.len(), 15);
    });

    divan::main();
}

#[divan::bench_group]
mod lists {
    use super::{Bencher, ItemsCount, LIBRARY_SIZE, RUNTIME, Recipe, SEEDED, SearchParams, Uuid};

    #[divan::bench]
    fn all(bencher: Bencher) {
        let s = &*SEEDED;

        bencher
            .counter(ItemsCount::new(LIBRARY_SIZE))
            .bench_local(|| RUNTIME.block_on(Recipe::all(&s.state.mm, s.user.id)));
    }

    #[divan::bench]
    fn count(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| RUNTIME.block_on(Recipe::count(&s.state.mm, s.user.id)));
    }

    #[divan::bench]
    fn get_page_no_recipes(bencher: Bencher) {
        let s = &*SEEDED;
        let user_id = Uuid::new_v4();
        let params = SearchParams {
            page: Some(1),
            ..Default::default()
        };

        bencher.bench_local(|| RUNTIME.block_on(Recipe::get_page(&s.state.mm, user_id, &params)));
    }

    #[divan::bench(args = [1, 2, 3])]
    fn get_page(bencher: Bencher, page: u64) {
        let s = &*SEEDED;
        let params = SearchParams {
            page: Some(page),
            ..Default::default()
        };

        bencher
            .counter(ItemsCount::new(15usize))
            .bench_local(|| RUNTIME.block_on(Recipe::get_page(&s.state.mm, s.user.id, &params)));
    }
}

#[divan::bench(args = [1, 2, 10, 40])]
fn get_many(bencher: Bencher, n: usize) {
    let s = &*SEEDED;
    let ids = &s.ids[..n];

    bencher
        .counter(ItemsCount::new(n))
        .bench_local(|| RUNTIME.block_on(Recipe::get_many(&s.state.mm, s.user.id, ids)));
}

#[divan::bench_group]
mod fetch {
    use super::{Bencher, RUNTIME, Recipe, SEEDED};

    #[divan::bench]
    fn categories(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| RUNTIME.block_on(Recipe::fetch_categories(&s.state.mm, s.user.id)));
    }

    #[divan::bench]
    fn cuisines(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| RUNTIME.block_on(Recipe::fetch_cuisines(&s.state.mm, s.user.id)));
    }

    #[divan::bench]
    fn ingredients(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| RUNTIME.block_on(Recipe::fetch_ingredients(&s.state.mm, s.user.id)));
    }

    #[divan::bench]
    fn keywords(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| RUNTIME.block_on(Recipe::fetch_keywords(&s.state.mm, s.user.id)));
    }

    #[divan::bench]
    fn tools(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| RUNTIME.block_on(Recipe::fetch_tools(&s.state.mm, s.user.id)));
    }

    #[divan::bench]
    fn sources(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| RUNTIME.block_on(Recipe::fetch_sources(&s.state.mm, s.user.id)));
    }
}

#[divan::bench(args = [1, 10, 40], sample_count = 30)]
fn ingredients_of_recipes(bencher: Bencher, n: usize) {
    let s = &*SEEDED;
    let ids = &s.ids[..n];

    bencher
        .counter(ItemsCount::new(n * 4))
        .bench_local(|| RUNTIME.block_on(Recipe::ingredients(&s.state.mm, ids)));
}
