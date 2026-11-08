mod common;

use std::sync::LazyLock;

use divan::{Bencher, counter::ItemsCount};
use time::{PrimitiveDateTime, macros::format_description};
use uuid::Uuid;

use models::{
    Recipe,
    recipe::timeline::{RecipeTimeline, RecipeTimelineForCreate},
    settings::UserSettingDetails,
    types::Rating,
    user::User,
};
use test_db::default_config;
use test_fixtures::insert_user;
use test_harness::{AppState, create_app_state};

use crate::common::{RUNTIME, a_complete_recipe_for_create, unique_name};

const SEEDED_ENTRIES: usize = 20;

struct Seeded {
    state: AppState,
    user: User,
    recipe_id: i64,
    timeline_id: i64,
}

static SEEDED: LazyLock<Seeded> = LazyLock::new(|| {
    RUNTIME.block_on(async {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await.unwrap();
        let recipe_id = insert_recipe(&state, user.id).await;

        let mut timeline_id = 0;
        for i in 0..SEEDED_ENTRIES {
            let entry = if i % 2 == 0 {
                RecipeTimelineForCreate::default()
            } else {
                full_entry()
            };
            let id = RecipeTimeline::create(&state.mm, recipe_id, user.id, &entry)
                .await
                .unwrap();
            if i == 0 {
                timeline_id = id;
            }
        }

        Seeded {
            state,
            user,
            recipe_id,
            timeline_id,
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

fn full_entry() -> RecipeTimelineForCreate {
    RecipeTimelineForCreate {
        title: "A title".into(),
        comment: Some("A comment".into()),
        rating: Rating::from(5),
        image: Some(Uuid::new_v4()),
        created_at: Some(a_date("2026-12-25 01:12:28")),
    }
}

fn a_date(s: &str) -> PrimitiveDateTime {
    PrimitiveDateTime::parse(
        s,
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    )
    .unwrap()
}

fn main() {
    let s = &*SEEDED;

    RUNTIME.block_on(async {
        let all = RecipeTimeline::all(&s.state.mm, s.recipe_id, s.user.id)
            .await
            .unwrap();
        assert_eq!(all.len(), SEEDED_ENTRIES);
    });

    divan::main();
}

#[divan::bench_group]
mod all {
    use super::{
        Bencher, ItemsCount, RUNTIME, RecipeTimeline, RecipeTimelineForCreate, SEEDED,
        SEEDED_ENTRIES, insert_recipe,
    };

    #[divan::bench]
    fn seeded(bencher: Bencher) {
        let s = &*SEEDED;

        bencher
            .counter(ItemsCount::new(SEEDED_ENTRIES))
            .bench_local(|| {
                RUNTIME.block_on(RecipeTimeline::all(&s.state.mm, s.recipe_id, s.user.id))
            });
    }

    #[divan::bench(args = [0, 10, 100])]
    fn by_size(bencher: Bencher, n: usize) {
        let s = &*SEEDED;
        let recipe_id = RUNTIME.block_on(async {
            let id = insert_recipe(&s.state, s.user.id).await;
            for _ in 0..n {
                RecipeTimeline::create(
                    &s.state.mm,
                    id,
                    s.user.id,
                    &RecipeTimelineForCreate::default(),
                )
                .await
                .unwrap();
            }
            id
        });

        bencher.counter(ItemsCount::new(n)).bench_local(|| {
            RUNTIME.block_on(RecipeTimeline::all(&s.state.mm, recipe_id, s.user.id))
        });
    }
}

#[divan::bench_group]
mod create {
    use super::{
        Bencher, RUNTIME, RecipeTimeline, RecipeTimelineForCreate, SEEDED, full_entry,
        insert_recipe,
    };

    fn fresh_recipe() -> i64 {
        let s = &*SEEDED;

        RUNTIME.block_on(insert_recipe(&s.state, s.user.id))
    }

    #[divan::bench]
    fn default_entry(bencher: Bencher) {
        let s = &*SEEDED;
        let entry = RecipeTimelineForCreate::default();

        bencher.with_inputs(fresh_recipe).bench_values(|recipe_id| {
            RUNTIME.block_on(RecipeTimeline::create(
                &s.state.mm,
                recipe_id,
                s.user.id,
                &entry,
            ))
        });
    }

    #[divan::bench]
    fn full_entry_bench(bencher: Bencher) {
        let s = &*SEEDED;
        let entry = full_entry();

        bencher.with_inputs(fresh_recipe).bench_values(|recipe_id| {
            RUNTIME.block_on(RecipeTimeline::create(
                &s.state.mm,
                recipe_id,
                s.user.id,
                &entry,
            ))
        });
    }
}

#[divan::bench_group]
mod edit {
    use super::{Bencher, RUNTIME, Rating, RecipeTimeline, SEEDED, Seeded, Uuid, a_date};

    fn timeline(s: &Seeded, title: &str, image: Option<Uuid>) -> RecipeTimeline {
        RecipeTimeline {
            id: s.timeline_id,
            recipe_id: s.recipe_id,
            user_id: s.user.id,
            title: title.into(),
            comment: Some("hello".into()),
            rating: Rating::from(4),
            image,
            created_at: a_date("2026-01-25 02:44:55"),
        }
    }

    #[divan::bench]
    fn no_changes(bencher: Bencher) {
        let s = &*SEEDED;
        let same = timeline(s, "A title", None);

        bencher
            .bench_local(|| RUNTIME.block_on(RecipeTimeline::edit(&s.state.mm, s.user.id, &same)));
    }

    #[divan::bench]
    fn with_changes(bencher: Bencher) {
        let s = &*SEEDED;

        bencher
            .with_inputs(|| timeline(s, "A title 2", Some(Uuid::new_v4())))
            .bench_refs(|t| RUNTIME.block_on(RecipeTimeline::edit(&s.state.mm, s.user.id, t)));
    }
}
