use std::path::PathBuf;

use uuid::Uuid;

use models::{
    download::{Download, DownloadForCreate},
    user::User,
};
use test_db::default_config;
use test_harness::build_server_anonymous;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

fn test_file_path() -> std::path::PathBuf {
    std::env::temp_dir().join("recipes.zip")
}

fn a_download_for_create(user_id: Uuid) -> DownloadForCreate {
    DownloadForCreate::new(user_id, Uuid::new_v4(), &test_file_path())
}

#[tokio::test]
async fn test_create_download_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;

    let dl_c = a_download_for_create(user_id);
    let result = Download::create(&state.mm, dl_c).await;

    assert!(result.is_ok());
    Ok(())
}

#[tokio::test]
async fn test_find_by_token_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let token = Uuid::new_v4();
    let dl_c = DownloadForCreate::new(user_id, token, &test_file_path());
    Download::create(&state.mm, dl_c).await?;

    let dl = Download::find_by_token(&state.mm, token).await?;

    assert!(dl.is_some());
    let dl = dl.unwrap();
    pretty_assertions::assert_eq!(dl.token, token);
    pretty_assertions::assert_eq!(dl.user_id, user_id);
    pretty_assertions::assert_eq!(dl.file_path, test_file_path());
    Ok(())
}

#[tokio::test]
async fn test_find_by_token_not_found() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;

    let dl = Download::find_by_token(&state.mm, Uuid::new_v4()).await?;

    assert!(dl.is_none());
    Ok(())
}

#[tokio::test]
async fn test_delete_by_token_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let file_path = test_file_path();
    tokio::fs::write(&file_path, b"dummy content").await?;
    let token = Uuid::new_v4();
    let dl_c = DownloadForCreate::new(user_id, token, &file_path);
    Download::create(&state.mm, dl_c).await?;

    Download::delete_by_token(&state.mm, token, file_path.to_string_lossy().into_owned()).await?;

    assert!(Download::find_by_token(&state.mm, token).await?.is_none());
    assert!(!tokio::fs::try_exists(&file_path).await?);
    Ok(())
}

#[tokio::test]
async fn test_delete_by_token_missing_file_err() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let token = Uuid::new_v4();
    let dl_c = DownloadForCreate::new(user_id, token, &PathBuf::from("/tmp/nonexistent.zip"));
    Download::create(&state.mm, dl_c).await?;

    let res = Download::delete_by_token(&state.mm, token, "/tmp/nonexistent.zip".to_string()).await;

    assert!(res.is_err());
    Ok(())
}
