#[cfg(test)]
mod tests {
    use axum::http::Method;
    use axum_test::http::StatusCode;

    use models::{Recipe, recipe::structs::test_utils::a_complete_recipe_for_create, user::User};
    use testing::utils::{
        TestDb, assert_must_be_logged_in, assert_ws_message, build_server_logged_in,
        build_server_ws, create_app_state,
    };

    use crate::recipes_router::params::RecipeCategoryForm;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    const BASE_URI: &str = "/recipes/categories";

    #[tokio::test]
    async fn test_must_be_logged_in_ok() -> Result<()> {
        assert_must_be_logged_in(Method::POST, BASE_URI).await?;
        assert_must_be_logged_in(Method::DELETE, BASE_URI).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_post_category_cannot_be_empty_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let server = build_server_logged_in(config).await?;

        let res = server
            .post(BASE_URI)
            .form(&RecipeCategoryForm {
                category: String::new(),
            })
            .await;

        res.assert_status_bad_request();
        Ok(())
    }

    #[tokio::test]
    async fn test_post_user_already_has_category_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let (server, mut ws_server) = build_server_ws(config).await?;

        let res = server
            .post(BASE_URI)
            .form(&RecipeCategoryForm {
                category: "uncategorized".into(),
            })
            .await;

        res.assert_status_internal_server_error();
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Failed to add recipe category.","status":"alert-error","title":"Operation Failed"}}"#).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_post_add_category_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let server = build_server_logged_in(config).await?;

        let res = server
            .post(BASE_URI)
            .form(&RecipeCategoryForm {
                category: "fish".into(),
            })
            .await;

        res.assert_status_ok();
        res.assert_text_contains(r#"<div class="badge badge-outline p-3 pr-0"><form class="inline-flex" hx-delete="/recipes/categories" hx-target="closest <div/>" hx-swap="delete"><input type="hidden" name="category" value="fish"><span class="select-none">fish</span><button class="btn btn-xs btn-ghost" type="submit">X</button></form></div><div class="badge badge-outline p-3 pr-0"><form class="inline-flex" hx-post="/recipes/categories" hx-target="closest <div/>" hx-swap="outerHTML"><label class="input"><input required type="text" placeholder="New category" class="input input-ghost input-xs w-[16ch] focus:outline-none" name="category" autocomplete="off"></label><button class="btn btn-xs btn-ghost">&#10003;</button></form></div>"#);
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_category_empty_ok() -> Result<()> {
        send_delete_400(String::new()).await
    }

    #[tokio::test]
    async fn test_delete_cannot_delete_uncategorized_ok() -> Result<()> {
        send_delete_400("uncategorized".into()).await
    }

    #[tokio::test]
    async fn test_delete_nonexistent_category_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let server = build_server_logged_in(config).await?;

        let res = server
            .delete(BASE_URI)
            .form(&RecipeCategoryForm {
                category: "ukraine".into(),
            })
            .await;

        res.assert_status(StatusCode::NO_CONTENT);
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_category_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let server = build_server_logged_in(config.clone()).await?;
        let category = String::from("midnight dinner");
        let state = create_app_state(config.clone()).await;
        let users = User::all(&state.mm).await?;
        let user_id = users[0].id;
        let _ = Recipe::create(&state.mm, user_id, &a_complete_recipe_for_create()).await?;
        Recipe::add_category(&state.mm, &category, user_id).await?;

        let res = server
            .delete(BASE_URI)
            .form(&RecipeCategoryForm { category })
            .await;

        res.assert_status(StatusCode::NO_CONTENT);
        Ok(())
    }

    async fn send_delete_400(category: String) -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let (server, mut ws_server) = build_server_ws(config).await?;

        let res = server
            .delete(BASE_URI)
            .form(&RecipeCategoryForm { category })
            .await;

        res.assert_status_bad_request();
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Category cannot be empty or uncategorized.","status":"alert-error","title":"Operation Failed"}}"#).await;
        Ok(())
    }
}
