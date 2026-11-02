use axum_htmx::HX_LOCATION;

use models::{Recipe, settings::UserSettingDetails, user::User};
use test_db::default_config;
use test_harness::build_server_logged_in;
use test_models::a_complete_recipe_for_create;

use crate::Result;

const BASE_URL: &str = "/recipes/bulk/categorise";
const SNACK_CATEGORY: &str = "snacks";

#[tokio::test]
async fn test_bulk_update_no_ids_err() -> Result<()> {
    let (server, _) = build_server_logged_in(default_config()).await?;

    let res = server
        .post(BASE_URL)
        .form(&[("name", SNACK_CATEGORY.to_string())])
        .await;

    res.assert_status_bad_request();
    Ok(())
}

#[tokio::test]
async fn test_bulk_update_empty_category_ok() -> Result<()> {
    let (server, state) = build_server_logged_in(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let settings = UserSettingDetails::get(&state.mm, user_id).await?;
    let (recipe1, _) = a_complete_recipe_for_create();
    let id1 = Recipe::create(&state.mm, user_id, &recipe1, &settings).await?;

    let res = server
        .post(BASE_URL)
        .form(&[("ids", id1.to_string()), ("name", String::new())])
        .await;

    res.assert_status_no_content();
    res.assert_header(HX_LOCATION, r##"{"path":"/recipes","target":"#content"}"##);
    let got1 = Recipe::get(&state.mm, user_id, id1).await?.category;
    assert_eq!(&got1, "uncategorized");
    Ok(())
}

#[tokio::test]
async fn test_bulk_update_new_category_ok() -> Result<()> {
    let (server, state) = build_server_logged_in(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let settings = UserSettingDetails::get(&state.mm, user_id).await?;
    let (recipe1, _) = a_complete_recipe_for_create();
    let (mut recipe2, _) = a_complete_recipe_for_create();
    recipe2.name = "Pumpkin spice latté".into();
    recipe2.category = Some(SNACK_CATEGORY.into());
    let id1 = Recipe::create(&state.mm, user_id, &recipe1, &settings).await?;
    let id2 = Recipe::create(&state.mm, user_id, &recipe2, &settings).await?;

    let res = server
        .post(BASE_URL)
        .form(&[
            ("ids", id1.to_string()),
            ("ids", id2.to_string()),
            ("name", SNACK_CATEGORY.to_string()),
        ])
        .await;

    res.assert_status_no_content();
    res.assert_header(HX_LOCATION, r##"{"path":"/recipes","target":"#content"}"##);
    let got1 = Recipe::get(&state.mm, user_id, id1).await?.category;
    let got2 = Recipe::get(&state.mm, user_id, id2).await?.category;
    assert_eq!([&got1, &got2], [SNACK_CATEGORY, SNACK_CATEGORY]);
    Ok(())
}
