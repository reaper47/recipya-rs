mod common;

use std::sync::LazyLock;

use divan::Bencher;
use strum::IntoEnumIterator;

use models::settings::UserSettingDetails;
use models::theme::{Theme, ThemeModel};
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
        let all = ThemeModel::all(&s.state.mm).await.unwrap();
        assert_eq!(all.len(), Theme::iter().len());
        assert_eq!(Theme::Halloween.get_id(&s.state.mm).await.unwrap(), 24);
    });

    divan::main();
}

#[divan::bench_group]
mod reads {
    use super::{Bencher, RUNTIME, SEEDED, Theme, ThemeModel, UserSettingDetails};

    #[divan::bench]
    fn all(bencher: Bencher) {
        let mm = &SEEDED.state.mm;

        bencher.bench_local(|| RUNTIME.block_on(ThemeModel::all(mm)));
    }

    #[divan::bench(args = [Theme::Halloween, Theme::Autumn])]
    fn get_id(bencher: Bencher, theme: &Theme) {
        let mm = &SEEDED.state.mm;

        bencher.bench_local(|| RUNTIME.block_on(theme.get_id(mm)));
    }

    #[divan::bench]
    fn user_settings_get(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| RUNTIME.block_on(UserSettingDetails::get(&s.state.mm, s.user.id)));
    }
}

#[divan::bench_group]
mod updates {
    use super::{Bencher, RUNTIME, SEEDED, Theme};

    #[divan::bench(args = [Theme::Halloween, Theme::Autumn])]
    fn save_default(bencher: Bencher, theme: &Theme) {
        let s = &*SEEDED;

        bencher.bench_local(|| RUNTIME.block_on(theme.save_default(&s.state.mm, s.user.id)));
    }

    #[divan::bench(args = [Theme::Halloween, Theme::Autumn])]
    fn save_selected(bencher: Bencher, theme: &Theme) {
        let s = &*SEEDED;

        bencher.bench_local(|| RUNTIME.block_on(theme.save_selected(&s.state.mm, s.user.id)));
    }
}
