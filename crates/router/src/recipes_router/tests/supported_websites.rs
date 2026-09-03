#[cfg(test)]
mod tests {
    use axum::http::Method;
    use axum_test::{TestResponse, http::header::CONTENT_TYPE};

    use recipya_scraper::Website;
    use test_db::default_config;
    use test_utils::{assert_must_be_logged_in, build_server_logged_in};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    const BASE_URI: &str = "/recipes/supported-websites";

    #[tokio::test]
    async fn test_must_be_logged_in_ok() -> Result<()> {
        assert_must_be_logged_in(Method::GET, BASE_URI).await
    }

    #[tokio::test]
    async fn test_fetch_websites_ok() -> Result<()> {
        let (server, _) = build_server_logged_in(default_config()).await?;

        let res: TestResponse = server.get(BASE_URI).await;

        res.assert_status_ok();
        res.assert_header(CONTENT_TYPE, "text/html; charset=utf-8");
        let num_rows = res.text().split("<tr ").count();
        assert_eq!(num_rows, Website::all().len() + 1);
        Ok(())
    }
}
