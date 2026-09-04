#[cfg(test)]
mod tests {
    use axum::http::Method;

    use models::{Recipe, settings::UserSettingDetails, user::User};
    use test_db::default_config;
    use test_fixtures::{assert_html, assert_ws_message};
    use test_models::a_complete_recipe_for_create;
    use test_utils::{assert_must_be_logged_in, build_server_logged_in, build_server_ws};

    use crate::recipes_router::params::FavouriteParams;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    fn base_uri(recipe_id: i64) -> String {
        format!("/recipes/{recipe_id}/favourite")
    }

    fn form() -> FavouriteParams {
        FavouriteParams {
            is_view_recipe: Some(false),
        }
    }

    #[tokio::test]
    async fn test_must_be_logged_in_ok() -> Result<()> {
        assert_must_be_logged_in(Method::POST, &base_uri(1)).await
    }

    #[tokio::test]
    async fn test_recipe_does_not_exist_ok() -> Result<()> {
        let (server, mut ws_server, _) = build_server_ws(default_config()).await?;

        let res = server.post(&base_uri(20)).form(&form()).await;

        res.assert_status_internal_server_error();
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Error toggling favourite.","status":"alert-error","title":"Operation Failed"}}"#).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_mark_as_favourite_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

        let res = server.post(&base_uri(recipe_id)).form(&form()).await;

        assert_html(
            &res,
            &[&format!(
                r##"<button id="favourite-{recipe_id}" class="btn btn-square btn-sm rounded-md absolute top-2 right-2 cursor-default hover:text-secondary" title="Add to favourites" hx-post="/recipes/{recipe_id}/favourite" hx-target="#favourite-{recipe_id}" hx-swap="outerHTML" hx-push-url="false" hx-vals="{{&quot;view-recipe&quot;:false}}" aria-label="Add to favorites" aria-pressed="true" _="on mousedown halt the event"><svg xmlns="http://www.w3.org/2000/svg" fill="currentColor""##
            )],
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_mark_as_favourite_already_favourite_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (mut recipe, _) = a_complete_recipe_for_create();
        recipe.is_favourite = true;
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

        let res = server.post(&base_uri(recipe_id)).form(&form()).await;

        assert_html(
            &res,
            &[&format!(
                r##"<button id="favourite-{recipe_id}" class="btn btn-square btn-sm rounded-md absolute top-2 right-2 cursor-default hover:text-secondary" title="Add to favourites" hx-post="/recipes/{recipe_id}/favourite" hx-target="#favourite-{recipe_id}" hx-swap="outerHTML" hx-push-url="false" hx-vals="{{&quot;view-recipe&quot;:false}}" aria-label="Add to favorites" aria-pressed="false" _="on mousedown halt the event"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor""##
            )],
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_mark_twice_in_a_row_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

        let _ = server.post(&base_uri(recipe_id)).form(&form()).await;
        let res = server.post(&base_uri(recipe_id)).form(&form()).await;

        assert_html(
            &res,
            &[&format!(
                r##"<button id="favourite-{recipe_id}" class="btn btn-square btn-sm rounded-md absolute top-2 right-2 cursor-default hover:text-secondary" title="Add to favourites" hx-post="/recipes/{recipe_id}/favourite" hx-target="#favourite-{recipe_id}" hx-swap="outerHTML" hx-push-url="false" hx-vals="{{&quot;view-recipe&quot;:false}}" aria-label="Add to favorites" aria-pressed="false" _="on mousedown halt the event"><svg xmlns="http://www.w3.org/2000/svg" fill="none""##
            )],
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_mark_as_favourite_view_recipe_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

        let res = server
            .post(&base_uri(recipe_id))
            .form(&FavouriteParams {
                is_view_recipe: Some(true),
            })
            .await;

        assert_html(
            &res,
            &[&format!(
                r##"<button id="favourite-{recipe_id}" class="mr-2 hidden sm:block hover:text-secondary" title="Add to favourites" hx-post="/recipes/{recipe_id}/favourite" hx-target="#favourite-{recipe_id}" hx-swap="outerHTML" hx-push-url="false" hx-vals="{{&quot;view-recipe&quot;:true}}" aria-label="Add to favorites" aria-pressed="true" _="on mousedown halt the event"><svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor""##
            )],
        );
        Ok(())
    }
}
