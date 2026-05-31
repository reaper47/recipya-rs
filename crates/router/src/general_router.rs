use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};

use app::state::AppState;

use crate::handlers::general::{
    download_handler, fetch_handler, index_handler, paper_sizes_handler,
    search_suggestions_handler, upload_note_image, user_initials_handler, ws_handler,
};
use crate::middleware::mw_auth::mw_refresh_token;

/// Defines the routes for general endpoints of the web application.
pub fn general_routes(state: &AppState) -> Router<AppState> {
    let protected = Router::new()
        .route("/download", get(download_handler))
        .route("/fetch", get(fetch_handler))
        .route("/paper-sizes", get(paper_sizes_handler))
        .route("/search-suggestions", get(search_suggestions_handler))
        .route(
            "/upload/note-image",
            post(upload_note_image).layer(DefaultBodyLimit::max(10 * 1024 * 1024)),
        )
        .route("/user-initials", get(user_initials_handler))
        .route("/ws", get(ws_handler))
        .layer(from_fn_with_state(state.clone(), mw_refresh_token));

    Router::new()
        .route("/", get(index_handler))
        .merge(protected)
}

#[cfg(test)]
mod tests {
    use axum::http::Method;
    use axum_test::multipart::{MultipartForm, Part};
    use serde_json::json;
    use test_db::TestDb;
    use test_utils::{assert_must_be_logged_in, build_server_logged_in, create_app_state};
    use uuid::Uuid;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_fetch {
        use super::*;

        fn url(target: &str) -> String {
            format!("/fetch?url={target}")
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, &url("https://www.example.com")).await
        }

        #[tokio::test]
        async fn test_missing_url_param_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&url("")).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_invalid_url_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&url("not-a-url")).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_non_http_scheme_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&url("ftp://example.com/file.txt")).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_file_scheme_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&url("file:///etc/passwd")).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_loopback_ip_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&url("http://127.0.0.1/admin")).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_private_ip_10_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&url("http://10.0.0.1/internal")).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_private_ip_172_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&url("http://172.16.0.1/internal")).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_private_ip_192_168_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&url("http://192.168.1.1/internal")).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_cloud_metadata_ip_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server
                .get(&url("http://169.254.169.254/latest/meta-data"))
                .await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_unreachable_url_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&url("http://192.0.2.1/")).await;

            res.assert_status_bad_request();
            Ok(())
        }
    }

    mod tests_index {
        use super::*;

        use config::Config;

        const BASE_URI: &str = "/";

        #[tokio::test]
        async fn test_get_index_redirect_to_recipes_when_autologin_ok() -> Result<()> {
            let config = Some(Config {
                is_autologin: true,
                ..Default::default()
            });
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_see_other();
            res.assert_header("Location", "/recipes");
            Ok(())
        }
    }

    mod tests_download {
        use std::path::PathBuf;

        use models::{
            download::{Download, DownloadForCreate},
            user::User,
        };
        use reqwest::header::{CONTENT_DISPOSITION, CONTENT_TYPE};

        use super::*;

        fn url(token: Uuid) -> String {
            format!("/download?token={token}")
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, &url(Uuid::new_v4())).await
        }

        #[tokio::test]
        async fn test_token_not_found_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&url(Uuid::new_v4())).await;

            res.assert_status_not_found();
            Ok(())
        }

        #[tokio::test]
        async fn test_download_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let server = build_server_logged_in(config).await?;
            let user_id = User::all(&state.mm).await?[0].id;
            let file_path = "/tmp/test_export.zip".to_string();
            tokio::fs::write(&file_path, b"some zip bytes").await?;
            let token = Uuid::new_v4();
            Download::create(
                &state.mm,
                DownloadForCreate::new(user_id, token, PathBuf::from(&file_path)),
            )
            .await?;

            let res = server.get(&url(token)).await;

            res.assert_status_ok();
            res.assert_header(CONTENT_TYPE, "application/zip");
            res.assert_header(
                CONTENT_DISPOSITION,
                "attachment; filename=\"test_export.zip\"",
            );
            assert!(Download::find_by_token(&state.mm, token).await?.is_none());
            assert!(!tokio::fs::try_exists(&file_path).await?);
            Ok(())
        }

        #[tokio::test]
        async fn test_download_file_missing_on_disk_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let server = build_server_logged_in(config).await?;
            let user = User::all(&state.mm).await?[0].clone();
            let token = Uuid::new_v4();
            Download::create(
                &state.mm,
                DownloadForCreate {
                    user_id: user.id,
                    token,
                    file_path: PathBuf::from("/tmp/nonexistent_export.zip"),
                },
            )
            .await?;

            let res = server.get(&url(token)).await;

            res.assert_status_internal_server_error();
            Ok(())
        }
    }

    mod tests_paper_sizes {
        use super::*;

        const BASE_URI: &str = "/paper-sizes";

        #[tokio::test]
        async fn test_assert_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, BASE_URI).await
        }

        #[tokio::test]
        async fn test_successful_request_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res: axum_test::TestResponse = server.get(BASE_URI).await;

            res.assert_status_ok();
            res.assert_text_contains(r#"<div class="card bg-base-100 shadow-sm p-2 min-w-[50vw]"><div class="card-body"><h3 class="mb-1 grid grid-flow-col"><label class="input input-sm"><svg class="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g stroke-linejoin="round" stroke-linecap="round" stroke-width="2.5" fill="none" stroke="currentColor"><circle cx="11" cy="11" r="8"></circle><path d="m21 21-4.3-4.3"></path></g></svg><input type="search" placeholder="Search a paper size" _="on input show <tbody>tr/> in next <table/> when its textContent.toLowerCase() contains my value.toLowerCase()"></label></h3><div class="overflow-auto h-[50vh]"><table class="table table-zebra table-sm"><thead><tr class="text-center"><th class="py-1 text-left"></th><th class="py-1">Name</th><th class="py-1 text-left">Category</th><th class="py-1">Size (mm)</th><th class="py-1">Size (inches)</th></tr></thead><tbody id="search-results"><tr><td class="py-1">1</td><td class="py-1">US Government Legal</td><td class="py-1 text-center select-none">Standard US Sizes</td><td class="py-1 text-center">216.00 x 330.00</td><td class="py-1 text-center">8.50 x 12.99</td></tr><tr><td class="py-1">2</td><td class="py-1">US Government Letter</td><td class="py-1 text-center select-none">Standard US Sizes</td><td class="py-1 text-center">203.00 x 267.00</td><td class="py-1 text-center">7.99 x 10.51</td></tr><tr><td class="py-1">3</td><td class="py-1">US Half Letter</td><td class="py-1 text-center select-none">Standard US Sizes</td><td class="py-1 text-center">140.00 x 216.00</td><td class="py-1 text-center">5.51 x 8.50</td></tr><tr><td class="py-1">4</td><td class="py-1">US Junior Legal</td><td class="py-1 text-center select-none">Standard US Sizes</td><td class="py-1 text-center">203.20 x 127.00</td><td class="py-1 text-center">8.00 x 5.00</td></tr><tr><td class="py-1">5</td><td class="py-1">US Executive</td><td class="py-1 text-center select-none">Standard US Sizes</td><td class="py-1 text-center">190.50 x 254.00</td><td class="py-1 text-center">7.50 x 10.00</td></tr><tr><td class="py-1">6</td><td class="py-1">US Tabloid</td><td class="py-1 text-center select-none">Standard US Sizes</td><td class="py-1 text-center">279.00 x 432.00</td><td class="py-1 text-center">10.98 x 17.01</td></tr><tr><td class="py-1">7</td><td class="py-1">US Legal</td><td class="py-1 text-center select-none">Standard US Sizes</td><td class="py-1 text-center">215.90 x 355.60</td><td class="py-1 text-center">8.50 x 14.00</td></tr><tr><td class="py-1">8</td><td class="py-1">US Letter</td><td class="py-1 text-center select-none">Standard US Sizes</td><td class="py-1 text-center">216.00 x 279.00</td><td class="py-1 text-center">8.50 x 10.98</td></tr><tr><td class="py-1">9</td><td class="py-1">US Envelope 13½</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">330.00 x 254.00</td><td class="py-1 text-center">12.99 x 10.00</td></tr><tr><td class="py-1">10</td><td class="py-1">US Envelope 10½</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">305.00 x 229.00</td><td class="py-1 text-center">12.01 x 9.02</td></tr><tr><td class="py-1">11</td><td class="py-1">US Envelope A9 Diplomat</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">222.00 x 146.00</td><td class="py-1 text-center">8.74 x 5.75</td></tr><tr><td class="py-1">12</td><td class="py-1">US Envelope A7 Besselheim</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">184.00 x 133.00</td><td class="py-1 text-center">7.24 x 5.24</td></tr><tr><td class="py-1">13</td><td class="py-1">US Envelope A6 Thompsons</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">165.00 x 121.00</td><td class="py-1 text-center">6.50 x 4.76</td></tr><tr><td class="py-1">14</td><td class="py-1">US Envelope A2 Lady Grey</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">146.00 x 111.00</td><td class="py-1 text-center">5.75 x 4.37</td></tr><tr><td class="py-1">15</td><td class="py-1">US Envelope 14</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">292.00 x 127.00</td><td class="py-1 text-center">11.50 x 5.00</td></tr><tr><td class="py-1">16</td><td class="py-1">US Envelope 11</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">264.00 x 114.00</td><td class="py-1 text-center">10.39 x 4.49</td></tr><tr><td class="py-1">17</td><td class="py-1">US Envelope 10</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">241.00 x 104.00</td><td class="py-1 text-center">9.49 x 4.09</td></tr><tr><td class="py-1">18</td><td class="py-1">US Envelope 9</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">225.00 x 98.00</td><td class="py-1 text-center">8.86 x 3.86</td></tr><tr><td class="py-1">19</td><td class="py-1">US Envelope 7¾ Monarch</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">191.00 x 98.00</td><td class="py-1 text-center">7.52 x 3.86</td></tr><tr><td class="py-1">20</td><td class="py-1">US Envelope 6¾</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">165.00 x 92.00</td><td class="py-1 text-center">6.50 x 3.62</td></tr><tr><td class="py-1">21</td><td class="py-1">ISO Envelope S5</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">185.00 x 255.00</td><td class="py-1 text-center">7.28 x 10.04</td></tr><tr><td class="py-1">22</td><td class="py-1">ISO Envelope S4</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">250.00 x 330.00</td><td class="py-1 text-center">9.84 x 12.99</td></tr><tr><td class="py-1">23</td><td class="py-1">ISO Envelope R7</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">120.00 x 135.00</td><td class="py-1 text-center">4.72 x 5.32</td></tr><tr><td class="py-1">24</td><td class="py-1">ISO Envelope C7</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">81.00 x 114.00</td><td class="py-1 text-center">3.19 x 4.49</td></tr><tr><td class="py-1">25</td><td class="py-1">ISO Envelope C6</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">114.00 x 162.00</td><td class="py-1 text-center">4.49 x 6.38</td></tr><tr><td class="py-1">26</td><td class="py-1">ISO Envelope C5</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">162.00 x 229.00</td><td class="py-1 text-center">6.38 x 9.02</td></tr><tr><td class="py-1">27</td><td class="py-1">ISO Envelope C4M</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">318.00 x 229.00</td><td class="py-1 text-center">12.52 x 9.02</td></tr><tr><td class="py-1">28</td><td class="py-1">ISO Envelope C4</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">229.00 x 324.00</td><td class="py-1 text-center">9.02 x 12.76</td></tr><tr><td class="py-1">29</td><td class="py-1">ISO Envelope C3</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">324.00 x 458.00</td><td class="py-1 text-center">12.76 x 18.03</td></tr><tr><td class="py-1">30</td><td class="py-1">ISO Envelope B6</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">125.00 x 176.00</td><td class="py-1 text-center">4.92 x 6.93</td></tr><tr><td class="py-1">31</td><td class="py-1">ISO Envelope B5</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">176.00 x 250.00</td><td class="py-1 text-center">6.93 x 9.84</td></tr><tr><td class="py-1">32</td><td class="py-1">ISO Envelope B4</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">250.00 x 353.00</td><td class="py-1 text-center">9.84 x 13.90</td></tr><tr><td class="py-1">33</td><td class="py-1">ISO Envelope DL</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">110.00 x 220.00</td><td class="py-1 text-center">4.33 x 8.66</td></tr><tr><td class="py-1">34</td><td class="py-1">ISO A3+</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">329.00 x 483.00</td><td class="py-1 text-center">12.95 x 19.02</td></tr><tr><td class="py-1">35</td><td class="py-1">ISO A1+</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">609.00 x 914.00</td><td class="py-1 text-center">23.98 x 35.98</td></tr><tr><td class="py-1">36</td><td class="py-1">ISO A0+</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">914.00 x 1292.00</td><td class="py-1 text-center">35.98 x 50.87</td></tr><tr><td class="py-1">37</td><td class="py-1">ISO 4A0</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">1682.00 x 2378.00</td><td class="py-1 text-center">66.22 x 93.62</td></tr><tr><td class="py-1">38</td><td class="py-1">ISO 2A0</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">1189.00 x 1682.00</td><td class="py-1 text-center">46.81 x 66.22</td></tr><tr><td class="py-1">39</td><td class="py-1">ISO A13</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">9.00 x 13.00</td><td class="py-1 text-center">0.35 x 0.51</td></tr><tr><td class="py-1">40</td><td class="py-1">ISO A12</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">13.00 x 18.00</td><td class="py-1 text-center">0.51 x 0.71</td></tr><tr><td class="py-1">41</td><td class="py-1">ISO A11</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">18.00 x 24.00</td><td class="py-1 text-center">0.71 x 0.94</td></tr><tr><td class="py-1">42</td><td class="py-1">ISO A10</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">26.00 x 37.00</td><td class="py-1 text-center">1.02 x 1.46</td></tr><tr><td class="py-1">43</td><td class="py-1">ISO A9</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">37.00 x 52.00</td><td class="py-1 text-center">1.46 x 2.05</td></tr><tr><td class="py-1">44</td><td class="py-1">ISO A8</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">52.00 x 74.00</td><td class="py-1 text-center">2.05 x 2.91</td></tr><tr><td class="py-1">45</td><td class="py-1">ISO A7</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">74.00 x 105.00</td><td class="py-1 text-center">2.91 x 4.13</td></tr><tr><td class="py-1">46</td><td class="py-1">ISO A6</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">105.00 x 148.00</td><td class="py-1 text-center">4.13 x 5.83</td></tr><tr><td class="py-1">47</td><td class="py-1">ISO A5</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">148.00 x 210.00</td><td class="py-1 text-center">5.83 x 8.27</td></tr><tr><td class="py-1">48</td><td class="py-1">ISO A4</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">210.00 x 297.00</td><td class="py-1 text-center">8.27 x 11.69</td></tr><tr><td class="py-1">49</td><td class="py-1">ISO A3</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">297.00 x 420.00</td><td class="py-1 text-center">11.69 x 16.54</td></tr><tr><td class="py-1">50</td><td class="py-1">ISO A2</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">420.00 x 594.00</td><td class="py-1 text-center">16.54 x 23.39</td></tr><tr><td class="py-1">51</td><td class="py-1">ISO A1</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">594.00 x 841.00</td><td class="py-1 text-center">23.39 x 33.11</td></tr><tr><td class="py-1">52</td><td class="py-1">ISO A0</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">841.00 x 1189.00</td><td class="py-1 text-center">33.11 x 46.81</td></tr><tr><td class="py-1">53</td><td class="py-1">ISO B2+</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">520.00 x 720.00</td><td class="py-1 text-center">20.47 x 28.35</td></tr><tr><td class="py-1">54</td><td class="py-1">ISO B1+</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">720.00 x 1020.00</td><td class="py-1 text-center">28.35 x 40.16</td></tr><tr><td class="py-1">55</td><td class="py-1">ISO B0+</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">1118.00 x 1580.00</td><td class="py-1 text-center">44.02 x 62.20</td></tr><tr><td class="py-1">56</td><td class="py-1">ISO B8</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">62.00 x 88.00</td><td class="py-1 text-center">2.44 x 3.46</td></tr><tr><td class="py-1">57</td><td class="py-1">ISO B7</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">88.00 x 125.00</td><td class="py-1 text-center">3.46 x 4.92</td></tr><tr><td class="py-1">58</td><td class="py-1">ISO B6</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">125.00 x 176.00</td><td class="py-1 text-center">4.92 x 6.93</td></tr><tr><td class="py-1">59</td><td class="py-1">ISO B5</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">176.00 x 250.00</td><td class="py-1 text-center">6.93 x 9.84</td></tr><tr><td class="py-1">60</td><td class="py-1">ISO B4</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">250.00 x 353.00</td><td class="py-1 text-center">9.84 x 13.90</td></tr><tr><td class="py-1">61</td><td class="py-1">ISO B3</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">353.00 x 500.00</td><td class="py-1 text-center">13.90 x 19.68</td></tr><tr><td class="py-1">62</td><td class="py-1">ISO B2</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">500.00 x 707.00</td><td class="py-1 text-center">19.68 x 27.83</td></tr><tr><td class="py-1">63</td><td class="py-1">ISO B1</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">707.00 x 1000.00</td><td class="py-1 text-center">27.83 x 39.37</td></tr><tr><td class="py-1">64</td><td class="py-1">ISO B0</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">1000.00 x 1414.00</td><td class="py-1 text-center">39.37 x 55.67</td></tr><tr><td class="py-1">65</td><td class="py-1">ISO C6</td><td class="py-1 text-center select-none">ISO C Sizes</td><td class="py-1 text-center">114.00 x 162.00</td><td class="py-1 text-center">4.49 x 6.38</td></tr><tr><td class="py-1">66</td><td class="py-1">ISO C5</td><td class="py-1 text-center select-none">ISO C Sizes</td><td class="py-1 text-center">162.00 x 229.00</td><td class="py-1 text-center">6.38 x 9.02</td></tr><tr><td class="py-1">67</td><td class="py-1">ISO C4</td><td class="py-1 text-center select-none">ISO C Sizes</td><td class="py-1 text-center">229.00 x 324.00</td><td class="py-1 text-center">9.02 x 12.76</td></tr><tr><td class="py-1">68</td><td class="py-1">ISO C3</td><td class="py-1 text-center select-none">ISO C Sizes</td><td class="py-1 text-center">324.00 x 458.00</td><td class="py-1 text-center">12.76 x 18.03</td></tr><tr><td class="py-1">69</td><td class="py-1">ISO C2</td><td class="py-1 text-center select-none">ISO C Sizes</td><td class="py-1 text-center">458.00 x 648.00</td><td class="py-1 text-center">18.03 x 25.51</td></tr><tr><td class="py-1">70</td><td class="py-1">ISO C1</td><td class="py-1 text-center select-none">ISO C Sizes</td><td class="py-1 text-center">648.00 x 917.00</td><td class="py-1 text-center">25.51 x 36.10</td></tr><tr><td class="py-1">71</td><td class="py-1">ISO C0</td><td class="py-1 text-center select-none">ISO C Sizes</td><td class="py-1 text-center">917.00 x 1297.00</td><td class="py-1 text-center">36.10 x 51.06</td></tr><tr><td class="py-1">72</td><td class="py-1">ANSI A</td><td class="py-1 text-center select-none">North American ANSI Sizes</td><td class="py-1 text-center">216.00 x 279.00</td><td class="py-1 text-center">8.50 x 10.98</td></tr><tr><td class="py-1">73</td><td class="py-1">ANSI B</td><td class="py-1 text-center select-none">North American ANSI Sizes</td><td class="py-1 text-center">279.00 x 432.00</td><td class="py-1 text-center">10.98 x 17.01</td></tr><tr><td class="py-1">74</td><td class="py-1">ANSI C</td><td class="py-1 text-center select-none">North American ANSI Sizes</td><td class="py-1 text-center">432.00 x 559.00</td><td class="py-1 text-center">17.01 x 22.01</td></tr><tr><td class="py-1">75</td><td class="py-1">ANSI D</td><td class="py-1 text-center select-none">North American ANSI Sizes</td><td class="py-1 text-center">559.00 x 864.00</td><td class="py-1 text-center">22.01 x 34.02</td></tr><tr><td class="py-1">76</td><td class="py-1">ANSI E</td><td class="py-1 text-center select-none">North American ANSI Sizes</td><td class="py-1 text-center">1118.00 x 864.00</td><td class="py-1 text-center">44.02 x 34.02</td></tr><tr><td class="py-1">77</td><td class="py-1">ARCH A</td><td class="py-1 text-center select-none">North American ARCH Sizes</td><td class="py-1 text-center">229.00 x 305.00</td><td class="py-1 text-center">9.02 x 12.01</td></tr><tr><td class="py-1">78</td><td class="py-1">ARCH B</td><td class="py-1 text-center select-none">North American ARCH Sizes</td><td class="py-1 text-center">305.00 x 457.00</td><td class="py-1 text-center">12.01 x 17.99</td></tr><tr><td class="py-1">79</td><td class="py-1">ARCH C</td><td class="py-1 text-center select-none">North American ARCH Sizes</td><td class="py-1 text-center">457.00 x 610.00</td><td class="py-1 text-center">17.99 x 24.02</td></tr><tr><td class="py-1">80</td><td class="py-1">ARCH D</td><td class="py-1 text-center select-none">North American ARCH Sizes</td><td class="py-1 text-center">610.00 x 914.00</td><td class="py-1 text-center">24.02 x 35.98</td></tr><tr><td class="py-1">81</td><td class="py-1">ARCH E</td><td class="py-1 text-center select-none">North American ARCH Sizes</td><td class="py-1 text-center">1914.00 x 1219.00</td><td class="py-1 text-center">75.35 x 47.99</td></tr><tr><td class="py-1">82</td><td class="py-1">ARCH E1</td><td class="py-1 text-center select-none">North American ARCH Sizes</td><td class="py-1 text-center">762.00 x 1067.00</td><td class="py-1 text-center">30.00 x 42.01</td></tr><tr><td class="py-1">83</td><td class="py-1">ARCH E2</td><td class="py-1 text-center select-none">North American ARCH Sizes</td><td class="py-1 text-center">660.00 x 965.00</td><td class="py-1 text-center">25.98 x 37.99</td></tr><tr><td class="py-1">84</td><td class="py-1">ARCH E3</td><td class="py-1 text-center select-none">North American ARCH Sizes</td><td class="py-1 text-center">686.00 x 991.00</td><td class="py-1 text-center">27.01 x 39.02</td></tr></tbody></table></div></div><div class="card-actions justify-end"><button class="btn btn-sm" onclick="this.closest('dialog').close()">Close</button></div></div>"#);
            Ok(())
        }
    }

    mod tests_search_suggestions {
        use config::Config;
        use models::{Recipe, settings::UserSettingDetails, user::User};
        use test_models::a_complete_recipe_for_create;

        use super::*;

        fn url(term: &str) -> String {
            format!("/search-suggestions?q={term}")
        }

        #[tokio::test]
        async fn test_categories_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let _ = insert_basic_recipe(config).await;

            let res = server.get(&url("cat:")).await;

            res.assert_status_ok();
            pretty_assertions::assert_eq!(
                res.text()
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" "),
                r#"<li><a tabindex="0" _="on click set #search-recipes.value to 'cat:dinner' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">dinner</a></li>"#
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_cuisines_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            let _ = insert_basic_recipe(config).await;
            let (mut recipe, _) = a_complete_recipe_for_create();
            recipe.name = "Test Recipe".into();
            recipe.cuisine = Some("Italian".into());
            let user = User::all(&state.mm).await?[0].clone();
            let settings = UserSettingDetails::get(&state.mm, user.id).await?;
            let _ = Recipe::create(&state.mm, user.id, &recipe, &settings).await?;

            let res = server.get(&url("cui:")).await;

            res.assert_status_ok();
            pretty_assertions::assert_eq!(
                res.text()
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" "),
                r#"<li><a tabindex="0" _="on click set #search-recipes.value to 'cui:italian' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">italian</a></li><li><a tabindex="0" _="on click set #search-recipes.value to 'cui:thai' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">thai</a></li>"#
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_ingredients_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let _ = insert_basic_recipe(config).await;

            let res = server.get(&url("ing:")).await;

            res.assert_status_ok();
            pretty_assertions::assert_eq!(
                res.text()
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" "),
                r#"<li><a tabindex="0" _="on click set #search-recipes.value to 'ing:blue spinach' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">blue spinach</a></li><li><a tabindex="0" _="on click set #search-recipes.value to 'ing:cinnamon' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">cinnamon</a></li><li><a tabindex="0" _="on click set #search-recipes.value to 'ing:lemon juice' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">lemon juice</a></li><li><a tabindex="0" _="on click set #search-recipes.value to 'ing:top quality chicken filet' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">top quality chicken filet</a></li>"#
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_keywords_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let _ = insert_basic_recipe(config).await;

            let res = server.get(&url("kw:")).await;

            res.assert_status_ok();
            pretty_assertions::assert_eq!(
                res.text()
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" "),
                r#"<li><a tabindex="0" _="on click set #search-recipes.value to 'kw:tofu' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">tofu</a></li><li><a tabindex="0" _="on click set #search-recipes.value to 'kw:vegetarian' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">vegetarian</a></li>"#
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_tools_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let _ = insert_basic_recipe(config).await;

            let res = server.get(&url("tool:")).await;

            res.assert_status_ok();
            pretty_assertions::assert_eq!(
                res.text()
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" "),
                r#"<li><a tabindex="0" _="on click set #search-recipes.value to 'tool:frying pan' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">frying pan</a></li><li><a tabindex="0" _="on click set #search-recipes.value to 'tool:wok' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">wok</a></li>"#
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_sources_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let _ = insert_basic_recipe(config).await;

            let res = server.get(&url("src:")).await;

            res.assert_status_ok();
            pretty_assertions::assert_eq!(
                res.text()
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" "),
                r#"<li><a tabindex="0" _="on click set #search-recipes.value to 'src:www.allrecipes.com' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">www.allrecipes.com</a></li>"#
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_invalid_term_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let _ = insert_basic_recipe(config).await;

            let res = server.get(&url("source:")).await;

            res.assert_status_ok();
            assert!(res.text().is_empty());
            Ok(())
        }

        async fn insert_basic_recipe(config: Config) -> Result<()> {
            let state = create_app_state(config.clone()).await;
            let user = User::all(&state.mm).await?[0].clone();
            let settings = UserSettingDetails::get(&state.mm, user.id).await?;
            let (recipe, _) = a_complete_recipe_for_create();
            let _ = Recipe::create(&state.mm, user.id, &recipe, &settings).await?;
            Ok(())
        }
    }

    mod tests_upload_note_image {
        use std::io::Cursor;

        use image::{ImageBuffer, Rgb};
        use reqwest::StatusCode;
        use serde_json::Value;

        use super::*;

        const BASE_URI: &str = "/upload/note-image";

        fn create_form(is_add_image: bool, is_image_valid: bool) -> MultipartForm {
            let mut form = MultipartForm::new();

            if is_add_image {
                let image = Uuid::new_v4();

                if is_image_valid {
                    let img: ImageBuffer<Rgb<u8>, _> =
                        ImageBuffer::from_pixel(1, 1, Rgb([255, 0, 0]));

                    let mut buf = Vec::new();
                    img.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Jpeg)
                        .unwrap();

                    form = form.add_part(
                        "image",
                        Part::bytes(buf)
                            .file_name(format!("{image}.jpg"))
                            .mime_type("image/jpeg"),
                    );
                } else {
                    let image = Uuid::new_v4();
                    form = form.add_part(
                        "image",
                        Part::file_name(Part::text(image.to_string()), format!("{image}.jpg")),
                    );
                }
            }

            form
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::POST, BASE_URI).await
        }

        #[tokio::test]
        async fn test_no_image_in_form_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server
                .post(BASE_URI)
                .multipart(create_form(false, false))
                .await;

            res.assert_status_bad_request();
            let body: Value = res.json();
            assert_eq!(body, json!({"error": "noFileGiven"}));
            Ok(())
        }

        #[tokio::test]
        async fn test_image_type_not_supported_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server
                .post(BASE_URI)
                .multipart(create_form(true, false))
                .await;

            res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
            let body: Value = res.json();
            assert_eq!(body, json!({"error": "typeNotAllowed"}));
            Ok(())
        }

        #[tokio::test]
        async fn test_image_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server
                .post(BASE_URI)
                .multipart(create_form(true, true))
                .await;

            res.assert_status_ok();
            Ok(())
        }
    }

    mod tests_user_initials {
        use axum::http::Method;

        use test_utils::{assert_must_be_logged_in, build_server_logged_in};

        use super::*;

        const BASE_URI: &str = "/user-initials";

        #[tokio::test]
        async fn test_get_user_initials_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, BASE_URI).await
        }

        #[tokio::test]
        async fn test_get_user_initials_logged_in_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            pretty_assertions::assert_eq!(res.text(), "T");
            Ok(())
        }
    }
}
