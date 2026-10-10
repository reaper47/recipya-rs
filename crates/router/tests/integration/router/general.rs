use axum::http::Method;
use axum_test::multipart::{MultipartForm, Part};
use reqwest::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use serde_json::json;
use std::io::Cursor;
use std::path::PathBuf;

use image::{ImageBuffer, Rgb};
use reqwest::StatusCode;
use serde_json::Value;
use uuid::Uuid;

use app::state::AppState;
use config::{AutologinState, Config, States};
use models::{
    Recipe,
    download::{Download, DownloadForCreate},
    settings::UserSettingDetails,
    user::User,
};
use test_db::default_config;
use test_harness::{assert_must_be_logged_in, build_server_logged_in};
use test_models::a_complete_recipe_for_create;

use crate::{FSI, PDI};

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
        let (server, _) = build_server_logged_in(default_config()).await?;

        let res = server.get(&url("")).await;

        res.assert_status_bad_request();
        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_url_ok() -> Result<()> {
        let (server, _) = build_server_logged_in(default_config()).await?;

        let res = server.get(&url("not-a-url")).await;

        res.assert_status_bad_request();
        Ok(())
    }

    #[tokio::test]
    async fn test_non_http_scheme_ok() -> Result<()> {
        let (server, _) = build_server_logged_in(default_config()).await?;

        let res = server.get(&url("ftp://example.com/file.txt")).await;

        res.assert_status_bad_request();
        Ok(())
    }

    #[tokio::test]
    async fn test_file_scheme_ok() -> Result<()> {
        let (server, _) = build_server_logged_in(default_config()).await?;

        let res = server.get(&url("file:///etc/passwd")).await;

        res.assert_status_bad_request();
        Ok(())
    }

    #[tokio::test]
    async fn test_loopback_ip_ok() -> Result<()> {
        let (server, _) = build_server_logged_in(default_config()).await?;

        let res = server.get(&url("http://127.0.0.1/admin")).await;

        res.assert_status_bad_request();
        Ok(())
    }

    #[tokio::test]
    async fn test_private_ip_10_ok() -> Result<()> {
        let (server, _) = build_server_logged_in(default_config()).await?;

        let res = server.get(&url("http://10.0.0.1/internal")).await;

        res.assert_status_bad_request();
        Ok(())
    }

    #[tokio::test]
    async fn test_private_ip_172_ok() -> Result<()> {
        let (server, _) = build_server_logged_in(default_config()).await?;

        let res = server.get(&url("http://172.16.0.1/internal")).await;

        res.assert_status_bad_request();
        Ok(())
    }

    #[tokio::test]
    async fn test_private_ip_192_168_ok() -> Result<()> {
        let (server, _) = build_server_logged_in(default_config()).await?;

        let res = server.get(&url("http://192.168.1.1/internal")).await;

        res.assert_status_bad_request();
        Ok(())
    }

    #[tokio::test]
    async fn test_cloud_metadata_ip_ok() -> Result<()> {
        let (server, _) = build_server_logged_in(default_config()).await?;

        let res = server
            .get(&url("http://169.254.169.254/latest/meta-data"))
            .await;

        res.assert_status_bad_request();
        Ok(())
    }

    #[tokio::test]
    async fn test_unreachable_url_ok() -> Result<()> {
        let (server, _) = build_server_logged_in(default_config()).await?;

        let res = server.get(&url("http://192.0.2.1/")).await;

        res.assert_status_bad_request();
        Ok(())
    }
}

mod tests_index {
    use super::*;

    const BASE_URI: &str = "/";

    #[tokio::test]
    async fn test_get_index_redirect_to_recipes_when_autologin_ok() -> Result<()> {
        let (server, _) = build_server_logged_in(Config {
            states: States {
                autologin: AutologinState::On,
                ..Default::default()
            },
            ..Default::default()
        })
        .await?;

        let res = server.get(BASE_URI).await;

        res.assert_status_see_other();
        res.assert_header("Location", "/recipes");
        Ok(())
    }
}

mod tests_download {

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
        let (server, _) = build_server_logged_in(default_config()).await?;

        let res = server.get(&url(Uuid::new_v4())).await;

        res.assert_status_not_found();
        Ok(())
    }

    #[tokio::test]
    async fn test_download_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let file_path = std::env::temp_dir().join("test_export.zip");
        tokio::fs::write(&file_path, b"some zip bytes").await?;
        let token = Uuid::new_v4();
        Download::create(
            &state.mm,
            DownloadForCreate::new(user_id, token, &file_path),
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
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let token = Uuid::new_v4();
        Download::create(
            &state.mm,
            DownloadForCreate {
                user_id,
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
        let (server, _) = build_server_logged_in(default_config()).await?;

        let res: axum_test::TestResponse = server.get(BASE_URI).await;

        res.assert_status_ok();
        res.assert_text_contains(&format!(r#"<div class="card bg-base-100 shadow-sm p-2 min-w-[50vw]"><div class="card-body"><h3 class="mb-1 grid grid-flow-col"><label class="input input-sm"><svg class="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g stroke-linejoin="round" stroke-linecap="round" stroke-width="2.5" fill="none" stroke="currentColor"><circle cx="11" cy="11" r="8"></circle><path d="m21 21-4.3-4.3"></path></g></svg><input type="search" placeholder="Search a paper size" _="on input show <tbody>tr/> in next <table/> when its textContent.toLowerCase() contains my value.toLowerCase()"></label></h3><div class="overflow-auto h-[50vh]"><table class="table table-zebra table-sm"><thead><tr class="text-center"><th class="py-1 text-left"></th><th class="py-1">Name</th><th class="py-1 text-left">Category</th><th class="py-1">Size (mm)</th><th class="py-1">Size (inches)</th></tr></thead><tbody id="search-results"><tr><td class="py-1">1</td><td class="py-1">US Government Legal</td><td class="py-1 text-center select-none">Standard US Sizes</td><td class="py-1 text-center">{FSI}216{PDI} × {FSI}330{PDI}</td><td class="py-1 text-center">{FSI}8.5039{PDI} × {FSI}12.9921{PDI}</td></tr><tr><td class="py-1">2</td><td class="py-1">US Government Letter</td><td class="py-1 text-center select-none">Standard US Sizes</td><td class="py-1 text-center">{FSI}203{PDI} × {FSI}267{PDI}</td><td class="py-1 text-center">{FSI}7.9921{PDI} × {FSI}10.5118{PDI}</td></tr><tr><td class="py-1">3</td><td class="py-1">US Half Letter</td><td class="py-1 text-center select-none">Standard US Sizes</td><td class="py-1 text-center">{FSI}140{PDI} × {FSI}216{PDI}</td><td class="py-1 text-center">{FSI}5.5118{PDI} × {FSI}8.5039{PDI}</td></tr><tr><td class="py-1">4</td><td class="py-1">US Junior Legal</td><td class="py-1 text-center select-none">Standard US Sizes</td><td class="py-1 text-center">{FSI}203.2{PDI} × {FSI}127{PDI}</td><td class="py-1 text-center">{FSI}8{PDI} × {FSI}5{PDI}</td></tr><tr><td class="py-1">5</td><td class="py-1">US Executive</td><td class="py-1 text-center select-none">Standard US Sizes</td><td class="py-1 text-center">{FSI}190.5{PDI} × {FSI}254{PDI}</td><td class="py-1 text-center">{FSI}7.5{PDI} × {FSI}10{PDI}</td></tr><tr><td class="py-1">6</td><td class="py-1">US Tabloid</td><td class="py-1 text-center select-none">Standard US Sizes</td><td class="py-1 text-center">{FSI}279{PDI} × {FSI}432{PDI}</td><td class="py-1 text-center">{FSI}10.9843{PDI} × {FSI}17.0079{PDI}</td></tr><tr><td class="py-1">7</td><td class="py-1">US Legal</td><td class="py-1 text-center select-none">Standard US Sizes</td><td class="py-1 text-center">{FSI}215.9{PDI} × {FSI}355.6{PDI}</td><td class="py-1 text-center">{FSI}8.5{PDI} × {FSI}14{PDI}</td></tr><tr><td class="py-1">8</td><td class="py-1">US Letter</td><td class="py-1 text-center select-none">Standard US Sizes</td><td class="py-1 text-center">{FSI}216{PDI} × {FSI}279{PDI}</td><td class="py-1 text-center">{FSI}8.5039{PDI} × {FSI}10.9843{PDI}</td></tr><tr><td class="py-1">9</td><td class="py-1">US Envelope 13½</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">{FSI}330{PDI} × {FSI}254{PDI}</td><td class="py-1 text-center">{FSI}12.9921{PDI} × {FSI}10{PDI}</td></tr><tr><td class="py-1">10</td><td class="py-1">US Envelope 10½</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">{FSI}305{PDI} × {FSI}229{PDI}</td><td class="py-1 text-center">{FSI}12.0079{PDI} × {FSI}9.0157{PDI}</td></tr><tr><td class="py-1">11</td><td class="py-1">US Envelope A9 Diplomat</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">{FSI}222{PDI} × {FSI}146{PDI}</td><td class="py-1 text-center">{FSI}8.7402{PDI} × {FSI}5.748{PDI}</td></tr><tr><td class="py-1">12</td><td class="py-1">US Envelope A7 Besselheim</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">{FSI}184{PDI} × {FSI}133{PDI}</td><td class="py-1 text-center">{FSI}7.2441{PDI} × {FSI}5.2362{PDI}</td></tr><tr><td class="py-1">13</td><td class="py-1">US Envelope A6 Thompsons</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">{FSI}165{PDI} × {FSI}121{PDI}</td><td class="py-1 text-center">{FSI}6.4961{PDI} × {FSI}4.7638{PDI}</td></tr><tr><td class="py-1">14</td><td class="py-1">US Envelope A2 Lady Grey</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">{FSI}146{PDI} × {FSI}111{PDI}</td><td class="py-1 text-center">{FSI}5.748{PDI} × {FSI}4.3701{PDI}</td></tr><tr><td class="py-1">15</td><td class="py-1">US Envelope 14</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">{FSI}292{PDI} × {FSI}127{PDI}</td><td class="py-1 text-center">{FSI}11.4961{PDI} × {FSI}5{PDI}</td></tr><tr><td class="py-1">16</td><td class="py-1">US Envelope 11</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">{FSI}264{PDI} × {FSI}114{PDI}</td><td class="py-1 text-center">{FSI}10.3937{PDI} × {FSI}4.4882{PDI}</td></tr><tr><td class="py-1">17</td><td class="py-1">US Envelope 10</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">{FSI}241{PDI} × {FSI}104{PDI}</td><td class="py-1 text-center">{FSI}9.4882{PDI} × {FSI}4.0945{PDI}</td></tr><tr><td class="py-1">18</td><td class="py-1">US Envelope 9</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">{FSI}225{PDI} × {FSI}98{PDI}</td><td class="py-1 text-center">{FSI}8.8583{PDI} × {FSI}3.8583{PDI}</td></tr><tr><td class="py-1">19</td><td class="py-1">US Envelope 7¾ Monarch</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">{FSI}191{PDI} × {FSI}98{PDI}</td><td class="py-1 text-center">{FSI}7.5197{PDI} × {FSI}3.8583{PDI}</td></tr><tr><td class="py-1">20</td><td class="py-1">US Envelope 6¾</td><td class="py-1 text-center select-none">US Envelope Sizes</td><td class="py-1 text-center">{FSI}165{PDI} × {FSI}92{PDI}</td><td class="py-1 text-center">{FSI}6.4961{PDI} × {FSI}3.622{PDI}</td></tr><tr><td class="py-1">21</td><td class="py-1">ISO Envelope S5</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">{FSI}185{PDI} × {FSI}255{PDI}</td><td class="py-1 text-center">{FSI}7.2835{PDI} × {FSI}10.0394{PDI}</td></tr><tr><td class="py-1">22</td><td class="py-1">ISO Envelope S4</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">{FSI}250{PDI} × {FSI}330{PDI}</td><td class="py-1 text-center">{FSI}9.8425{PDI} × {FSI}12.9921{PDI}</td></tr><tr><td class="py-1">23</td><td class="py-1">ISO Envelope R7</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">{FSI}120{PDI} × {FSI}135{PDI}</td><td class="py-1 text-center">{FSI}4.7244{PDI} × {FSI}5.315{PDI}</td></tr><tr><td class="py-1">24</td><td class="py-1">ISO Envelope C7</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">{FSI}81{PDI} × {FSI}114{PDI}</td><td class="py-1 text-center">{FSI}3.189{PDI} × {FSI}4.4882{PDI}</td></tr><tr><td class="py-1">25</td><td class="py-1">ISO Envelope C6</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">{FSI}114{PDI} × {FSI}162{PDI}</td><td class="py-1 text-center">{FSI}4.4882{PDI} × {FSI}6.378{PDI}</td></tr><tr><td class="py-1">26</td><td class="py-1">ISO Envelope C5</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">{FSI}162{PDI} × {FSI}229{PDI}</td><td class="py-1 text-center">{FSI}6.378{PDI} × {FSI}9.0157{PDI}</td></tr><tr><td class="py-1">27</td><td class="py-1">ISO Envelope C4M</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">{FSI}318{PDI} × {FSI}229{PDI}</td><td class="py-1 text-center">{FSI}12.5197{PDI} × {FSI}9.0157{PDI}</td></tr><tr><td class="py-1">28</td><td class="py-1">ISO Envelope C4</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">{FSI}229{PDI} × {FSI}324{PDI}</td><td class="py-1 text-center">{FSI}9.0157{PDI} × {FSI}12.7559{PDI}</td></tr><tr><td class="py-1">29</td><td class="py-1">ISO Envelope C3</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">{FSI}324{PDI} × {FSI}458{PDI}</td><td class="py-1 text-center">{FSI}12.7559{PDI} × {FSI}18.0315{PDI}</td></tr><tr><td class="py-1">30</td><td class="py-1">ISO Envelope B6</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">{FSI}125{PDI} × {FSI}176{PDI}</td><td class="py-1 text-center">{FSI}4.9213{PDI} × {FSI}6.9291{PDI}</td></tr><tr><td class="py-1">31</td><td class="py-1">ISO Envelope B5</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">{FSI}176{PDI} × {FSI}250{PDI}</td><td class="py-1 text-center">{FSI}6.9291{PDI} × {FSI}9.8425{PDI}</td></tr><tr><td class="py-1">32</td><td class="py-1">ISO Envelope B4</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">{FSI}250{PDI} × {FSI}353{PDI}</td><td class="py-1 text-center">{FSI}9.8425{PDI} × {FSI}13.8976{PDI}</td></tr><tr><td class="py-1">33</td><td class="py-1">ISO Envelope DL</td><td class="py-1 text-center select-none">ISO Envelopes</td><td class="py-1 text-center">{FSI}110{PDI} × {FSI}220{PDI}</td><td class="py-1 text-center">{FSI}4.3307{PDI} × {FSI}8.6614{PDI}</td></tr><tr><td class="py-1">34</td><td class="py-1">ISO A3+</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">{FSI}329{PDI} × {FSI}483{PDI}</td><td class="py-1 text-center">{FSI}12.9528{PDI} × {FSI}19.0157{PDI}</td></tr><tr><td class="py-1">35</td><td class="py-1">ISO A1+</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">{FSI}609{PDI} × {FSI}914{PDI}</td><td class="py-1 text-center">{FSI}23.9764{PDI} × {FSI}35.9843{PDI}</td></tr><tr><td class="py-1">36</td><td class="py-1">ISO A0+</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">{FSI}914{PDI} × {FSI}1292{PDI}</td><td class="py-1 text-center">{FSI}35.9843{PDI} × {FSI}50.8661{PDI}</td></tr><tr><td class="py-1">37</td><td class="py-1">ISO 4A0</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">{FSI}1682{PDI} × {FSI}2378{PDI}</td><td class="py-1 text-center">{FSI}66.2205{PDI} × {FSI}93.622{PDI}</td></tr><tr><td class="py-1">38</td><td class="py-1">ISO 2A0</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">{FSI}1189{PDI} × {FSI}1682{PDI}</td><td class="py-1 text-center">{FSI}46.811{PDI} × {FSI}66.2205{PDI}</td></tr><tr><td class="py-1">39</td><td class="py-1">ISO A13</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">{FSI}9{PDI} × {FSI}13{PDI}</td><td class="py-1 text-center">{FSI}0.3543{PDI} × {FSI}0.5118{PDI}</td></tr><tr><td class="py-1">40</td><td class="py-1">ISO A12</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">{FSI}13{PDI} × {FSI}18{PDI}</td><td class="py-1 text-center">{FSI}0.5118{PDI} × {FSI}0.7087{PDI}</td></tr><tr><td class="py-1">41</td><td class="py-1">ISO A11</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">{FSI}18{PDI} × {FSI}24{PDI}</td><td class="py-1 text-center">{FSI}0.7087{PDI} × {FSI}0.9449{PDI}</td></tr><tr><td class="py-1">42</td><td class="py-1">ISO A10</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">{FSI}26{PDI} × {FSI}37{PDI}</td><td class="py-1 text-center">{FSI}1.0236{PDI} × {FSI}1.4567{PDI}</td></tr><tr><td class="py-1">43</td><td class="py-1">ISO A9</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">{FSI}37{PDI} × {FSI}52{PDI}</td><td class="py-1 text-center">{FSI}1.4567{PDI} × {FSI}2.0472{PDI}</td></tr><tr><td class="py-1">44</td><td class="py-1">ISO A8</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">{FSI}52{PDI} × {FSI}74{PDI}</td><td class="py-1 text-center">{FSI}2.0472{PDI} × {FSI}2.9134{PDI}</td></tr><tr><td class="py-1">45</td><td class="py-1">ISO A7</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">{FSI}74{PDI} × {FSI}105{PDI}</td><td class="py-1 text-center">{FSI}2.9134{PDI} × {FSI}4.1339{PDI}</td></tr><tr><td class="py-1">46</td><td class="py-1">ISO A6</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">{FSI}105{PDI} × {FSI}148{PDI}</td><td class="py-1 text-center">{FSI}4.1339{PDI} × {FSI}5.8268{PDI}</td></tr><tr><td class="py-1">47</td><td class="py-1">ISO A5</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">{FSI}148{PDI} × {FSI}210{PDI}</td><td class="py-1 text-center">{FSI}5.8268{PDI} × {FSI}8.2677{PDI}</td></tr><tr><td class="py-1">48</td><td class="py-1">ISO A4</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">{FSI}210{PDI} × {FSI}297{PDI}</td><td class="py-1 text-center">{FSI}8.2677{PDI} × {FSI}11.6929{PDI}</td></tr><tr><td class="py-1">49</td><td class="py-1">ISO A3</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">{FSI}297{PDI} × {FSI}420{PDI}</td><td class="py-1 text-center">{FSI}11.6929{PDI} × {FSI}16.5354{PDI}</td></tr><tr><td class="py-1">50</td><td class="py-1">ISO A2</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">{FSI}420{PDI} × {FSI}594{PDI}</td><td class="py-1 text-center">{FSI}16.5354{PDI} × {FSI}23.3858{PDI}</td></tr><tr><td class="py-1">51</td><td class="py-1">ISO A1</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">{FSI}594{PDI} × {FSI}841{PDI}</td><td class="py-1 text-center">{FSI}23.3858{PDI} × {FSI}33.1102{PDI}</td></tr><tr><td class="py-1">52</td><td class="py-1">ISO A0</td><td class="py-1 text-center select-none">ISO A Sizes</td><td class="py-1 text-center">{FSI}841{PDI} × {FSI}1189{PDI}</td><td class="py-1 text-center">{FSI}33.1102{PDI} × {FSI}46.811{PDI}</td></tr><tr><td class="py-1">53</td><td class="py-1">ISO B2+</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">{FSI}520{PDI} × {FSI}720{PDI}</td><td class="py-1 text-center">{FSI}20.4724{PDI} × {FSI}28.3465{PDI}</td></tr><tr><td class="py-1">54</td><td class="py-1">ISO B1+</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">{FSI}720{PDI} × {FSI}1020{PDI}</td><td class="py-1 text-center">{FSI}28.3465{PDI} × {FSI}40.1575{PDI}</td></tr><tr><td class="py-1">55</td><td class="py-1">ISO B0+</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">{FSI}1118{PDI} × {FSI}1580{PDI}</td><td class="py-1 text-center">{FSI}44.0157{PDI} × {FSI}62.2047{PDI}</td></tr><tr><td class="py-1">56</td><td class="py-1">ISO B8</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">{FSI}62{PDI} × {FSI}88{PDI}</td><td class="py-1 text-center">{FSI}2.4409{PDI} × {FSI}3.4646{PDI}</td></tr><tr><td class="py-1">57</td><td class="py-1">ISO B7</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">{FSI}88{PDI} × {FSI}125{PDI}</td><td class="py-1 text-center">{FSI}3.4646{PDI} × {FSI}4.9213{PDI}</td></tr><tr><td class="py-1">58</td><td class="py-1">ISO B6</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">{FSI}125{PDI} × {FSI}176{PDI}</td><td class="py-1 text-center">{FSI}4.9213{PDI} × {FSI}6.9291{PDI}</td></tr><tr><td class="py-1">59</td><td class="py-1">ISO B5</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">{FSI}176{PDI} × {FSI}250{PDI}</td><td class="py-1 text-center">{FSI}6.9291{PDI} × {FSI}9.8425{PDI}</td></tr><tr><td class="py-1">60</td><td class="py-1">ISO B4</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">{FSI}250{PDI} × {FSI}353{PDI}</td><td class="py-1 text-center">{FSI}9.8425{PDI} × {FSI}13.8976{PDI}</td></tr><tr><td class="py-1">61</td><td class="py-1">ISO B3</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">{FSI}353{PDI} × {FSI}500{PDI}</td><td class="py-1 text-center">{FSI}13.8976{PDI} × {FSI}19.685{PDI}</td></tr><tr><td class="py-1">62</td><td class="py-1">ISO B2</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">{FSI}500{PDI} × {FSI}707{PDI}</td><td class="py-1 text-center">{FSI}19.685{PDI} × {FSI}27.8346{PDI}</td></tr><tr><td class="py-1">63</td><td class="py-1">ISO B1</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">{FSI}707{PDI} × {FSI}1000{PDI}</td><td class="py-1 text-center">{FSI}27.8346{PDI} × {FSI}39.3701{PDI}</td></tr><tr><td class="py-1">64</td><td class="py-1">ISO B0</td><td class="py-1 text-center select-none">ISO B Sizes</td><td class="py-1 text-center">{FSI}1000{PDI} × {FSI}1414{PDI}</td><td class="py-1 text-center">{FSI}39.3701{PDI} × {FSI}55.6693{PDI}</td></tr><tr><td class="py-1">65</td><td class="py-1">ISO C6</td><td class="py-1 text-center select-none">ISO C Sizes</td><td class="py-1 text-center">{FSI}114{PDI} × {FSI}162{PDI}</td><td class="py-1 text-center">{FSI}4.4882{PDI} × {FSI}6.378{PDI}</td></tr><tr><td class="py-1">66</td><td class="py-1">ISO C5</td><td class="py-1 text-center select-none">ISO C Sizes</td><td class="py-1 text-center">{FSI}162{PDI} × {FSI}229{PDI}</td><td class="py-1 text-center">{FSI}6.378{PDI} × {FSI}9.0157{PDI}</td></tr><tr><td class="py-1">67</td><td class="py-1">ISO C4</td><td class="py-1 text-center select-none">ISO C Sizes</td><td class="py-1 text-center">{FSI}229{PDI} × {FSI}324{PDI}</td><td class="py-1 text-center">{FSI}9.0157{PDI} × {FSI}12.7559{PDI}</td></tr><tr><td class="py-1">68</td><td class="py-1">ISO C3</td><td class="py-1 text-center select-none">ISO C Sizes</td><td class="py-1 text-center">{FSI}324{PDI} × {FSI}458{PDI}</td><td class="py-1 text-center">{FSI}12.7559{PDI} × {FSI}18.0315{PDI}</td></tr><tr><td class="py-1">69</td><td class="py-1">ISO C2</td><td class="py-1 text-center select-none">ISO C Sizes</td><td class="py-1 text-center">{FSI}458{PDI} × {FSI}648{PDI}</td><td class="py-1 text-center">{FSI}18.0315{PDI} × {FSI}25.5118{PDI}</td></tr><tr><td class="py-1">70</td><td class="py-1">ISO C1</td><td class="py-1 text-center select-none">ISO C Sizes</td><td class="py-1 text-center">{FSI}648{PDI} × {FSI}917{PDI}</td><td class="py-1 text-center">{FSI}25.5118{PDI} × {FSI}36.1024{PDI}</td></tr><tr><td class="py-1">71</td><td class="py-1">ISO C0</td><td class="py-1 text-center select-none">ISO C Sizes</td><td class="py-1 text-center">{FSI}917{PDI} × {FSI}1297{PDI}</td><td class="py-1 text-center">{FSI}36.1024{PDI} × {FSI}51.063{PDI}</td></tr><tr><td class="py-1">72</td><td class="py-1">ANSI A</td><td class="py-1 text-center select-none">North American ANSI Sizes</td><td class="py-1 text-center">{FSI}216{PDI} × {FSI}279{PDI}</td><td class="py-1 text-center">{FSI}8.5039{PDI} × {FSI}10.9843{PDI}</td></tr><tr><td class="py-1">73</td><td class="py-1">ANSI B</td><td class="py-1 text-center select-none">North American ANSI Sizes</td><td class="py-1 text-center">{FSI}279{PDI} × {FSI}432{PDI}</td><td class="py-1 text-center">{FSI}10.9843{PDI} × {FSI}17.0079{PDI}</td></tr><tr><td class="py-1">74</td><td class="py-1">ANSI C</td><td class="py-1 text-center select-none">North American ANSI Sizes</td><td class="py-1 text-center">{FSI}432{PDI} × {FSI}559{PDI}</td><td class="py-1 text-center">{FSI}17.0079{PDI} × {FSI}22.0079{PDI}</td></tr><tr><td class="py-1">75</td><td class="py-1">ANSI D</td><td class="py-1 text-center select-none">North American ANSI Sizes</td><td class="py-1 text-center">{FSI}559{PDI} × {FSI}864{PDI}</td><td class="py-1 text-center">{FSI}22.0079{PDI} × {FSI}34.0157{PDI}</td></tr><tr><td class="py-1">76</td><td class="py-1">ANSI E</td><td class="py-1 text-center select-none">North American ANSI Sizes</td><td class="py-1 text-center">{FSI}1118{PDI} × {FSI}864{PDI}</td><td class="py-1 text-center">{FSI}44.0157{PDI} × {FSI}34.0157{PDI}</td></tr><tr><td class="py-1">77</td><td class="py-1">ARCH A</td><td class="py-1 text-center select-none">North American ARCH Sizes</td><td class="py-1 text-center">{FSI}229{PDI} × {FSI}305{PDI}</td><td class="py-1 text-center">{FSI}9.0157{PDI} × {FSI}12.0079{PDI}</td></tr><tr><td class="py-1">78</td><td class="py-1">ARCH B</td><td class="py-1 text-center select-none">North American ARCH Sizes</td><td class="py-1 text-center">{FSI}305{PDI} × {FSI}457{PDI}</td><td class="py-1 text-center">{FSI}12.0079{PDI} × {FSI}17.9921{PDI}</td></tr><tr><td class="py-1">79</td><td class="py-1">ARCH C</td><td class="py-1 text-center select-none">North American ARCH Sizes</td><td class="py-1 text-center">{FSI}457{PDI} × {FSI}610{PDI}</td><td class="py-1 text-center">{FSI}17.9921{PDI} × {FSI}24.0157{PDI}</td></tr><tr><td class="py-1">80</td><td class="py-1">ARCH D</td><td class="py-1 text-center select-none">North American ARCH Sizes</td><td class="py-1 text-center">{FSI}610{PDI} × {FSI}914{PDI}</td><td class="py-1 text-center">{FSI}24.0157{PDI} × {FSI}35.9843{PDI}</td></tr><tr><td class="py-1">81</td><td class="py-1">ARCH E</td><td class="py-1 text-center select-none">North American ARCH Sizes</td><td class="py-1 text-center">{FSI}1914{PDI} × {FSI}1219{PDI}</td><td class="py-1 text-center">{FSI}75.3543{PDI} × {FSI}47.9921{PDI}</td></tr><tr><td class="py-1">82</td><td class="py-1">ARCH E1</td><td class="py-1 text-center select-none">North American ARCH Sizes</td><td class="py-1 text-center">{FSI}762{PDI} × {FSI}1067{PDI}</td><td class="py-1 text-center">{FSI}30{PDI} × {FSI}42.0079{PDI}</td></tr><tr><td class="py-1">83</td><td class="py-1">ARCH E2</td><td class="py-1 text-center select-none">North American ARCH Sizes</td><td class="py-1 text-center">{FSI}660{PDI} × {FSI}965{PDI}</td><td class="py-1 text-center">{FSI}25.9843{PDI} × {FSI}37.9921{PDI}</td></tr><tr><td class="py-1">84</td><td class="py-1">ARCH E3</td><td class="py-1 text-center select-none">North American ARCH Sizes</td><td class="py-1 text-center">{FSI}686{PDI} × {FSI}991{PDI}</td><td class="py-1 text-center">{FSI}27.0079{PDI} × {FSI}39.0157{PDI}</td></tr></tbody></table></div></div><div class="card-actions justify-end"><button class="btn btn-sm" onclick="this.closest('dialog').close()">Close</button></div></div>"#));
        Ok(())
    }
}

mod tests_search_suggestions {
    use super::*;

    fn url(term: &str) -> String {
        format!("/search-suggestions?q={term}")
    }

    #[tokio::test]
    async fn test_categories_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let _ = insert_basic_recipe(&state).await;

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
        let (server, state) = build_server_logged_in(default_config()).await?;
        let _ = insert_basic_recipe(&state).await;
        let (mut recipe, _) = a_complete_recipe_for_create();
        recipe.name = "Test Recipe".into();
        recipe.cuisine = Some("Italian".into());
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

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
    async fn test_general_ingredients_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let _ = insert_basic_recipe(&state).await;

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
        let (server, state) = build_server_logged_in(default_config()).await?;
        let _ = insert_basic_recipe(&state).await;

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
        let (server, state) = build_server_logged_in(default_config()).await?;
        let _ = insert_basic_recipe(&state).await;

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
        let (server, state) = build_server_logged_in(default_config()).await?;
        let _ = insert_basic_recipe(&state).await;

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
        let (server, state) = build_server_logged_in(default_config()).await?;
        let _ = insert_basic_recipe(&state).await;

        let res = server.get(&url("source:")).await;

        res.assert_status_ok();
        assert!(res.text().is_empty());
        Ok(())
    }

    async fn insert_basic_recipe(state: &AppState) -> Result<i64> {
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
        Ok(recipe_id)
    }
}

mod tests_upload_note_image {
    use super::*;

    const BASE_URI: &str = "/upload/note-image";

    fn create_form(is_add_image: bool, is_image_valid: bool) -> MultipartForm {
        let mut form = MultipartForm::new();

        if is_add_image {
            let image = Uuid::new_v4();

            if is_image_valid {
                let img: ImageBuffer<Rgb<u8>, _> = ImageBuffer::from_pixel(1, 1, Rgb([255, 0, 0]));

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
        let (server, _) = build_server_logged_in(default_config()).await?;

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
        let (server, _) = build_server_logged_in(default_config()).await?;

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
        let (server, _) = build_server_logged_in(default_config()).await?;

        let res = server
            .post(BASE_URI)
            .multipart(create_form(true, true))
            .await;

        res.assert_status_ok();
        Ok(())
    }
}

mod tests_user_initials {
    use super::*;

    const BASE_URI: &str = "/user-initials";

    #[tokio::test]
    async fn test_get_user_initials_must_be_logged_in_ok() -> Result<()> {
        assert_must_be_logged_in(Method::GET, BASE_URI).await
    }

    #[tokio::test]
    async fn test_get_user_initials_logged_in_ok() -> Result<()> {
        let (server, _) = build_server_logged_in(default_config()).await?;

        let res = server.get(BASE_URI).await;

        res.assert_status_ok();
        pretty_assertions::assert_eq!(res.text(), "T");
        Ok(())
    }
}
