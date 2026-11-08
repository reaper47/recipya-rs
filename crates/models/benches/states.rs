mod common;

use std::sync::LazyLock;

use divan::Bencher;

use models::states::UserStates;
use test_db::default_config;
use test_fixtures::insert_other_user;
use test_fixtures::insert_user;
use test_harness::create_app_state;

use crate::common::RUNTIME;
use crate::common::Seeded;
use crate::common::unique_name;

static SEEDED: LazyLock<Seeded> = LazyLock::new(|| {
    RUNTIME.block_on(async {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await.unwrap();
        Seeded { state, user }
    })
});

fn main() {
    let s = &*SEEDED;

    RUNTIME.block_on(async {
        let states = UserStates::new(&s.state.mm, s.user.id);
        assert!(!states.is_importing_app_or_api().await.unwrap());
    });

    divan::main();
}

#[divan::bench_group]
mod is_importing_app_or_api {
    use super::{Bencher, RUNTIME, SEEDED, UserStates, insert_other_user, unique_name};

    #[divan::bench]
    fn not_importing(bencher: Bencher) {
        let s = &*SEEDED;
        let states = UserStates::new(&s.state.mm, s.user.id);

        bencher.bench_local(|| RUNTIME.block_on(states.is_importing_app_or_api()));
    }

    #[divan::bench]
    fn importing_app(bencher: Bencher) {
        let s = &*SEEDED;
        let user = RUNTIME
            .block_on(insert_other_user(&s.state.mm, &unique_name()))
            .unwrap();
        let states = UserStates::new(&s.state.mm, user.id);
        RUNTIME.block_on(states.set_is_importing_app()).unwrap();
        assert!(RUNTIME.block_on(states.is_importing_app_or_api()).unwrap());

        bencher.bench_local(|| RUNTIME.block_on(states.is_importing_app_or_api()));
    }

    #[divan::bench]
    fn importing_api(bencher: Bencher) {
        let s = &*SEEDED;
        let user = RUNTIME
            .block_on(insert_other_user(&s.state.mm, &unique_name()))
            .unwrap();
        let states = UserStates::new(&s.state.mm, user.id);
        RUNTIME.block_on(states.set_is_importing_api()).unwrap();
        assert!(RUNTIME.block_on(states.is_importing_app_or_api()).unwrap());

        bencher.bench_local(|| RUNTIME.block_on(states.is_importing_app_or_api()));
    }
}

#[divan::bench_group]
mod setters {
    use super::{Bencher, RUNTIME, SEEDED, UserStates, insert_other_user, unique_name};

    #[divan::bench]
    fn set_is_importing_app(bencher: Bencher) {
        let s = &*SEEDED;
        let user = RUNTIME
            .block_on(insert_other_user(&s.state.mm, &unique_name()))
            .unwrap();
        let states = UserStates::new(&s.state.mm, user.id);

        bencher.bench_local(|| RUNTIME.block_on(states.set_is_importing_app()));
    }

    #[divan::bench]
    fn set_is_importing_api(bencher: Bencher) {
        let s = &*SEEDED;
        let user = RUNTIME
            .block_on(insert_other_user(&s.state.mm, &unique_name()))
            .unwrap();
        let states = UserStates::new(&s.state.mm, user.id);

        bencher.bench_local(|| RUNTIME.block_on(states.set_is_importing_api()));
    }
}
