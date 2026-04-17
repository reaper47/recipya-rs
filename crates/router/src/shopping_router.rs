use axum::{Router, middleware::from_fn_with_state, routing::get};

use app::state::AppState;

use crate::{handlers::shopping::shopping_lists_handler, middleware::mw_auth::mw_refresh_token};

/// Defines the shopping routes for the application.
#[allow(clippy::literal_string_with_formatting_args)]
pub fn shopping_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/lists", get(shopping_lists_handler))
        .layer(from_fn_with_state(state.clone(), mw_refresh_token))
}

#[cfg(test)]
mod tests {
    use reqwest::Method;

    use test_db::TestDb;
    use test_fixtures::assert_html;
    use test_utils::{assert_must_be_logged_in, build_server_logged_in};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_lists {
        use super::*;

        const BASE_URI: &str = "/shopping/lists";

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, BASE_URI).await
        }

        #[tokio::test]
        async fn test_user_has_no_lists_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_html(
                &res,
                vec![
                    r##"<div id="shopping-list-container"><div class="grid grid-flow-col gap-2 place-items-center"><p class="text-center font-semibold text-lg underline">Shopping Lists</p><button class="btn btn-xs btn-square btn-ghost" hx-post="/shopping/list" hx-prompt="Name of the new shopping list:" hx-target="#shopping-list-menu" hx-swap="beforeend"><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" width="24px" height="24px" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="8" x2="12" y2="16"></line><line x1="8" y1="12" x2="16" y2="12"></line></svg></button></div>"##,
                    r#"<ul id="shopping-list-menu" class="menu block bg-base-100 w-full overflow-y-auto max-h-[40vh] md:max-h-[89vh] pb-16 md:pb-0 h-full"></ul></div></aside>"#,
                    r#"<div class="order-1 divider my-0 md:order-2 md:divider-horizontal md:mx-0"></div>"#,
                    r#"<div class="order-0 flex-1 overflow-y-auto min-h-0 md:order-3 max-h-[94vh]"><div id="shopping-list-view-pane"><p class="p-4">← Create your first shopping list to get started.</p></div>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_user_has_lists_ok() -> Result<()> {
            todo!()
        }
    }
}
