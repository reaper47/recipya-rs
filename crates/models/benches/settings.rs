mod common;

use std::sync::LazyLock;

use divan::{Bencher, black_box};
use time_tz::timezones;

use models::settings::UserSettingDetails;
use test_db::default_config;
use test_fixtures::insert_user;
use test_harness::create_app_state;

use crate::common::{RUNTIME, Seeded};

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
        let settings = UserSettingDetails::get(&s.state.mm, s.user.id)
            .await
            .unwrap();
        assert_eq!(settings.user_id, s.user.id);
        assert!(!settings.paper_sizes.is_empty());
    });

    divan::main();
}

#[divan::bench]
fn get(bencher: Bencher) {
    let s = &*SEEDED;

    bencher.bench_local(|| RUNTIME.block_on(UserSettingDetails::get(&s.state.mm, s.user.id)));
}

#[divan::bench]
fn get_missing_user(bencher: Bencher) {
    let s = &*SEEDED;
    let id = uuid::Uuid::new_v4();

    bencher.bench_local(|| RUNTIME.block_on(UserSettingDetails::get(&s.state.mm, id)));
}

#[divan::bench_group]
mod tz_name {
    use super::{Bencher, UserSettingDetails, black_box, timezones};

    #[divan::bench(args = ["UTC", "Etc/GMT+5", "America/Toronto"])]
    fn tz_name(bencher: Bencher, name: &str) {
        let settings = UserSettingDetails {
            timezone: timezones::get_by_name(name).unwrap(),
            ..UserSettingDetails::default()
        };

        bencher.bench_local(|| black_box(&settings).tz_name());
    }
}
