mod common;

use std::sync::LazyLock;

use divan::Bencher;
use time::{Duration, OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

use app::state::AppState;
use models::{Recipe, settings::UserSettingDetails, share::ShareRecipe, user::User};
use test_db::default_config;
use test_fixtures::insert_user;
use test_harness::create_app_state;

use crate::common::{RUNTIME, a_complete_recipe_for_create, unique_name};

struct Seeded {
    state: AppState,
    user: User,
    shared_recipe_id: i64,
    link: Uuid,
}

static SEEDED: LazyLock<Seeded> = LazyLock::new(|| {
    RUNTIME.block_on(async {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await.unwrap();
        let shared_recipe_id = insert_recipe(&state, user.id).await;
        let link = ShareRecipe::new(&state.mm, shared_recipe_id, user.id, None)
            .await
            .unwrap()
            .link;

        Seeded {
            state,
            user,
            shared_recipe_id,
            link,
        }
    })
});

async fn insert_recipe(state: &AppState, user_id: Uuid) -> i64 {
    let (mut recipe, _) = a_complete_recipe_for_create();
    recipe.name = unique_name();
    let settings = UserSettingDetails::get(&state.mm, user_id).await.unwrap();
    Recipe::create(&state.mm, user_id, &recipe, &settings)
        .await
        .unwrap()
}

fn main() {
    let s = &*SEEDED;

    RUNTIME.block_on(async {
        let (got, _) = ShareRecipe::get_by_link(&s.state.mm, s.link).await.unwrap();
        assert_eq!(got.recipe_id, s.shared_recipe_id);
        assert!(
            ShareRecipe::get_by_link(&s.state.mm, Uuid::new_v4())
                .await
                .is_err()
        );
    });

    divan::main();
}

fn fresh_recipe() -> i64 {
    let s = &*SEEDED;

    RUNTIME.block_on(insert_recipe(&s.state, s.user.id))
}

#[divan::bench_group]
mod new {
    use super::{
        Bencher, Duration, OffsetDateTime, PrimitiveDateTime, RUNTIME, SEEDED, ShareRecipe,
        fresh_recipe,
    };

    #[divan::bench]
    fn default_expiration(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.with_inputs(fresh_recipe).bench_values(|recipe_id| {
            RUNTIME.block_on(ShareRecipe::new(&s.state.mm, recipe_id, s.user.id, None))
        });
    }

    #[divan::bench]
    fn custom_expiration(bencher: Bencher) {
        let s = &*SEEDED;

        bencher
            .with_inputs(|| {
                let dt = OffsetDateTime::now_utc() + Duration::weeks(2);
                (fresh_recipe(), PrimitiveDateTime::new(dt.date(), dt.time()))
            })
            .bench_values(|(recipe_id, expires_at)| {
                RUNTIME.block_on(ShareRecipe::new(
                    &s.state.mm,
                    recipe_id,
                    s.user.id,
                    Some(expires_at),
                ))
            });
    }

    #[divan::bench]
    fn already_shared(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| {
            RUNTIME.block_on(ShareRecipe::new(
                &s.state.mm,
                s.shared_recipe_id,
                s.user.id,
                None,
            ))
        });
    }
}

#[divan::bench(args = ["hit", "miss"])]
fn get_by_link(bencher: Bencher, which: &str) {
    let s = &*SEEDED;
    let link = if which == "hit" {
        s.link
    } else {
        Uuid::new_v4()
    };

    bencher.bench_local(|| RUNTIME.block_on(ShareRecipe::get_by_link(&s.state.mm, link)));
}
