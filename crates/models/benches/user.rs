mod common;

use std::sync::LazyLock;

use divan::{Bencher, black_box};

use models::{Recipe, settings::UserSettingDetails, user::User};
use test_db::default_config;
use test_fixtures::insert_user;
use test_fixtures::test_user_email;
use test_harness::create_app_state;

use crate::common::RUNTIME;
use crate::common::Seeded;
use crate::common::a_complete_recipe_for_create;

static SEEDED: LazyLock<Seeded> = LazyLock::new(|| {
    RUNTIME.block_on(async {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await.unwrap();
        let settings = UserSettingDetails::get(&state.mm, user.id).await.unwrap();

        for (i, is_favourite) in [(1, false), (2, true), (3, true)] {
            let (mut recipe, _) = a_complete_recipe_for_create();
            recipe.name = format!("Recipe {i}");
            recipe.is_favourite = is_favourite;
            Recipe::create(&state.mm, user.id, &recipe, &settings)
                .await
                .unwrap();
        }

        Seeded { state, user }
    })
});

fn main() {
    let s = &*SEEDED;
    RUNTIME.block_on(async {
        assert_eq!(
            User::favourite_recipes(&s.state.mm, s.user.id)
                .await
                .unwrap()
                .len(),
            2
        );
        assert!(
            User::get_user_by_id(&s.state.mm, s.user.id)
                .await
                .unwrap()
                .is_some()
        );
    });
    divan::main();
}

#[divan::bench_group]
mod reads {
    use uuid::Uuid;

    use super::{Bencher, RUNTIME, SEEDED, User, test_user_email};

    #[divan::bench]
    fn all(bencher: Bencher) {
        let s = &*SEEDED;
        bencher.bench_local(|| RUNTIME.block_on(User::all(&s.state.mm)));
    }

    #[divan::bench]
    fn num_users(bencher: Bencher) {
        let s = &*SEEDED;
        bencher.bench_local(|| RUNTIME.block_on(User::num_users(&s.state.mm)));
    }

    #[divan::bench]
    fn get_user_by_id(bencher: Bencher) {
        let s = &*SEEDED;
        bencher.bench_local(|| RUNTIME.block_on(User::get_user_by_id(&s.state.mm, s.user.id)));
    }

    #[divan::bench]
    fn get_user_by_id_missing(bencher: Bencher) {
        let s = &*SEEDED;
        bencher.bench_local(|| RUNTIME.block_on(User::get_user_by_id(&s.state.mm, Uuid::new_v4())));
    }

    #[divan::bench]
    fn get_user_by_email(bencher: Bencher) {
        let s = &*SEEDED;
        let email = test_user_email();
        bencher.bench_local(|| RUNTIME.block_on(User::get_user_by_email(&s.state.mm, &email)));
    }

    #[divan::bench]
    fn get_user_auth_by_email(bencher: Bencher) {
        let s = &*SEEDED;
        let email = test_user_email();
        bencher.bench_local(|| RUNTIME.block_on(User::get_user_auth_by_email(&s.state.mm, &email)));
    }

    #[divan::bench]
    fn categories(bencher: Bencher) {
        let s = &*SEEDED;
        bencher.bench_local(|| RUNTIME.block_on(User::categories(&s.state.mm, s.user.id)));
    }

    #[divan::bench]
    fn keywords(bencher: Bencher) {
        let s = &*SEEDED;
        bencher.bench_local(|| RUNTIME.block_on(User::keywords(&s.state.mm, s.user.id)));
    }

    #[divan::bench]
    fn favourite_recipes(bencher: Bencher) {
        let s = &*SEEDED;
        bencher.bench_local(|| RUNTIME.block_on(User::favourite_recipes(&s.state.mm, s.user.id)));
    }
}

#[divan::bench_group]
mod updates {
    use auth::types::Password;

    use super::{Bencher, RUNTIME, SEEDED, User, black_box};

    #[divan::bench]
    fn update_paper_size(bencher: Bencher) {
        let s = &*SEEDED;
        bencher
            .bench_local(|| RUNTIME.block_on(s.user.update_paper_size(&s.state.mm, black_box(2))));
    }

    #[divan::bench]
    fn update_remember_me(bencher: Bencher) {
        let s = &*SEEDED;
        bencher.bench_local(|| {
            RUNTIME.block_on(User::update_remember_me(&s.state.mm, s.user.id, true))
        });
    }

    #[divan::bench]
    fn update_password(bencher: Bencher) {
        let s = &*SEEDED;
        bencher
            .with_inputs(|| Password::try_from("subway yay").unwrap())
            .bench_values(|password| {
                RUNTIME.block_on(s.user.update_password(&s.state.mm, password))
            });
    }

    #[divan::bench]
    fn update_password_by_user_id(bencher: Bencher) {
        let s = &*SEEDED;
        bencher
            .with_inputs(|| Password::try_from("subway yay").unwrap())
            .bench_values(|password| {
                RUNTIME.block_on(User::update_password_by_user_id(
                    &s.state.mm,
                    s.user.id,
                    password,
                ))
            });
    }
}

#[divan::bench_group]
mod deletes {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use test_fixtures::insert_other_user;

    use super::{Bencher, RUNTIME, SEEDED, User};

    #[divan::bench]
    fn delete_user(bencher: Bencher) {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let s = &*SEEDED;

        bencher
            .with_inputs(|| {
                let n = NEXT.fetch_add(1, Ordering::Relaxed);
                let email = format!("john{n}@deer.test");
                RUNTIME
                    .block_on(insert_other_user(&s.state.mm, &email))
                    .unwrap()
                    .id
            })
            .bench_values(|id| RUNTIME.block_on(User::delete(&s.state.mm, id)));
    }
}
