use axum_htmx::HX_LOCATION;
use reqwest::Method;

use models::{Recipe, settings::UserSettingDetails, user::User};
use test_db::default_config;
use test_fixtures::insert_other_user;
use test_harness::{assert_must_be_logged_in, build_server_logged_in};
use test_models::a_complete_recipe_for_create;

use crate::Result;

const BASE_URL: &str = "/recipes/bulk";

#[tokio::test]
async fn test_bulk_must_be_logged_in_ok() -> Result<()> {
    assert_must_be_logged_in(Method::DELETE, BASE_URL).await
}

#[tokio::test]
async fn test_bulk_recipes_recipe_ids_belong_to_user_ok() -> Result<()> {
    let (server, state) = build_server_logged_in(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let settings = UserSettingDetails::get(&state.mm, user_id).await?;
    let (recipe1, _) = a_complete_recipe_for_create();
    let (mut recipe2, _) = a_complete_recipe_for_create();
    recipe2.name = "Pumpkin spice latté".into();
    let id1 = Recipe::create(&state.mm, user_id, &recipe1, &settings).await?;
    let id2 = Recipe::create(&state.mm, user_id, &recipe2, &settings).await?;

    let res = server
        .post(BASE_URL)
        .form(&[("ids", id1), ("ids", id2)])
        .await;

    res.assert_status_no_content();
    res.assert_header(HX_LOCATION, r##"{"path":"/recipes","target":"#content"}"##);
    let got = Recipe::all(&state.mm, user_id).await?;
    assert_eq!(got.len(), 0);
    Ok(())
}

#[tokio::test]
async fn test_bulk_recipe_ids_do_not_belong_to_user_ok() -> Result<()> {
    let (server, state) = build_server_logged_in(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let user2 = insert_other_user(&state.mm, "slavaukraina@ua.com").await?;
    let settings = UserSettingDetails::get(&state.mm, user_id).await?;
    let (recipe1, _) = a_complete_recipe_for_create();
    let (mut recipe2, _) = a_complete_recipe_for_create();
    recipe2.name = "Pumpkin spice latté".into();
    let id1 = Recipe::create(&state.mm, user_id, &recipe1, &settings).await?;
    let id2 = Recipe::create(&state.mm, user2.id, &recipe1, &settings).await?;
    let id3 = Recipe::create(&state.mm, user2.id, &recipe2, &settings).await?;

    let res = server
        .post(BASE_URL)
        .form(&[("ids", id1), ("ids", id2), ("id3", id3)])
        .await;

    res.assert_status_no_content();
    res.assert_header(HX_LOCATION, r##"{"path":"/recipes","target":"#content"}"##);
    let got = Recipe::all(&state.mm, user_id).await?;
    let got2 = Recipe::all(&state.mm, user2.id).await?;
    assert_eq!(got.len(), 0);
    assert_eq!(got2.len(), 2);
    Ok(())
}
