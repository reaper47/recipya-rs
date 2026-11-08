mod common;

use std::{path::PathBuf, sync::LazyLock};

use divan::Bencher;
use uuid::Uuid;

use models::{
    download::{Download, DownloadForCreate},
    user::User,
};
use test_db::default_config;
use test_fixtures::insert_user;
use test_harness::{AppState, create_app_state};

use crate::common::RUNTIME;

struct Seeded {
    state: AppState,
    user: User,
    token: Uuid,
    file_path: PathBuf,
}

fn bench_dir() -> PathBuf {
    std::env::temp_dir().join("recipya-bench-downloads")
}

static SEEDED: LazyLock<Seeded> = LazyLock::new(|| {
    RUNTIME.block_on(async {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await.unwrap();
        tokio::fs::create_dir_all(bench_dir()).await.unwrap();

        let token = Uuid::new_v4();
        let file_path = bench_dir().join("seeded.zip");
        Download::create(
            &state.mm,
            DownloadForCreate::new(user.id, token, &file_path),
        )
        .await
        .unwrap();

        Seeded {
            state,
            user,
            token,
            file_path,
        }
    })
});

fn main() {
    let s = &*SEEDED;

    RUNTIME.block_on(async {
        let dl = Download::find_by_token(&s.state.mm, s.token)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(dl.file_path, s.file_path);
        assert!(
            Download::find_by_token(&s.state.mm, Uuid::new_v4())
                .await
                .unwrap()
                .is_none()
        );
    });

    divan::main();
}

fn fresh_download(with_file: bool) -> (Uuid, String) {
    let s = &*SEEDED;
    let token = Uuid::new_v4();
    let path = bench_dir().join(format!("{token}.zip"));
    RUNTIME.block_on(async {
        if with_file {
            tokio::fs::write(&path, b"dummy content").await.unwrap();
        }
        Download::create(&s.state.mm, DownloadForCreate::new(s.user.id, token, &path))
            .await
            .unwrap();
    });
    (token, path.to_string_lossy().into_owned())
}

#[divan::bench_group]
mod create {
    use super::{Bencher, Download, DownloadForCreate, RUNTIME, SEEDED, Uuid, bench_dir};

    #[divan::bench]
    fn create(bencher: Bencher) {
        let s = &*SEEDED;
        let path = bench_dir().join("create.zip");

        bencher
            .with_inputs(|| DownloadForCreate::new(s.user.id, Uuid::new_v4(), &path))
            .bench_values(|dl| RUNTIME.block_on(Download::create(&s.state.mm, dl)));
    }
}

#[divan::bench(args = ["hit", "miss"])]
fn find_by_token(bencher: Bencher, which: &str) {
    let s = &*SEEDED;
    let token = if which == "hit" {
        s.token
    } else {
        Uuid::new_v4()
    };

    bencher.bench_local(|| RUNTIME.block_on(Download::find_by_token(&s.state.mm, token)));
}

#[divan::bench_group]
mod delete_by_token {
    use super::{Bencher, Download, RUNTIME, SEEDED, fresh_download};

    #[divan::bench]
    fn with_file(bencher: Bencher) {
        let s = &*SEEDED;

        bencher
            .with_inputs(|| fresh_download(true))
            .bench_values(|(token, path)| {
                RUNTIME.block_on(Download::delete_by_token(&s.state.mm, token, path))
            });
    }

    #[divan::bench]
    fn missing_file(bencher: Bencher) {
        let s = &*SEEDED;

        bencher
            .with_inputs(|| fresh_download(false))
            .bench_values(|(token, path)| {
                RUNTIME.block_on(Download::delete_by_token(&s.state.mm, token, path))
            });
    }
}
