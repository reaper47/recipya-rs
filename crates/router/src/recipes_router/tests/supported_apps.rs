#[cfg(test)]
mod tests {
    use axum::http::header::CONTENT_TYPE;
    use reqwest::Method;

    use test_db::TestDb;
    use test_utils::{assert_must_be_logged_in, build_server_logged_in};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    const BASE_URI: &str = "/recipes/supported-applications";

    #[tokio::test]
    async fn test_must_be_logged_in_ok() -> Result<()> {
        assert_must_be_logged_in(Method::GET, BASE_URI).await
    }

    #[tokio::test]
    async fn test_returns_list_of_apps_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let server = build_server_logged_in(config).await?;

        let res = server.get(BASE_URI).await;

        res.assert_status_ok();
        res.assert_header(CONTENT_TYPE, "text/html; charset=utf-8");
        res.assert_text_contains(r#"<tr class="text-center"><td>1</td><td><a class="underline" href="https://www.accuchef.com" target="_blank">AccuChef</a></td><td></td></tr><tr class="text-center"><td>2</td><td><a class="underline" href="https://www.bigoven.com" target="_blank">BigOven</a></td><td>.txt</td></tr><tr class="text-center"><td>3</td><td><a class="underline" href="https://cheftap.com" target="_blank">ChefTap</a></td><td>.txt</td></tr><tr class="text-center"><td>4</td><td><a class="underline" href="https://cooklang.org/" target="_blank">Cooklang</a></td><td>.cook</td></tr><tr class="text-center"><td>5</td><td><a class="underline" href="https://cooklang.org/" target="_blank">COOKmate</a></td><td>.mcb, .mmf, .rk, .xml</td></tr><tr class="text-center"><td>6</td><td><a class="underline" href="https://crouton.app" target="_blank">Crouton</a></td><td>.crumb</td></tr><tr class="text-center"><td>7</td><td><a class="underline" href="https://easy-recipe-deluxe.software.informer.com" target="_blank">Easy Recipe Deluxe</a></td><td></td></tr><tr class="text-center"><td>8</td><td><a class="underline" href="https://www.kalorio.de" target="_blank">Kalorio</a></td><td>.txt, .xml</td></tr><tr class="text-center"><td>9</td><td><a class="underline" href="https://www.mastercook.com" target="_blank">MasterCook</a></td><td>.mx2, .mxp, .mz2, .txt</td></tr><tr class="text-center"><td>10</td><td><a class="underline" href="https://web.archive.org/web/20081221021301/http://episoft.home.comcast.net/~episoft/mmdown.htm" target="_blank">Meal-Master</a></td><td>.mx2, .mxp, .mz2, .txt</td></tr><tr class="text-center"><td>11</td><td><a class="underline" href="https://www.paprikaapp.com" target="_blank">Paprika</a></td><td>.paprikarecipes</td></tr><tr class="text-center"><td>12</td><td><a class="underline" href="https://recipekeeperonline.com" target="_blank">Recipe Keeper</a></td><td></td></tr><tr class="text-center"><td>13</td><td><a class="underline" href="https://recipemd.org/" target="_blank">RecipeMD</a></td><td>.md</td></tr><tr class="text-center"><td>14</td><td><a class="underline" href="https://recipesage.com" target="_blank">RecipeSage</a></td><td>.json, .txt, .xml</td></tr><tr class="text-center"><td>15</td><td><a class="underline" href="https://www.rezkonv.de/" target="_blank">Rezkonv</a></td><td>.rk</td></tr><tr class="text-center"><td>16</td><td><a class="underline" href="https://www.mysaffronapp.com" target="_blank">Saffron</a></td><td>.txt</td></tr>"#);
        Ok(())
    }
}
