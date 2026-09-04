#[cfg(test)]
mod tests {
    use axum::http::header::CONTENT_TYPE;
    use reqwest::Method;

    use test_db::default_config;
    use test_utils::{assert_must_be_logged_in, build_server_logged_in};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    const BASE_URI: &str = "/recipes/supported-applications";

    #[tokio::test]
    async fn test_must_be_logged_in_ok() -> Result<()> {
        assert_must_be_logged_in(Method::GET, BASE_URI).await
    }

    #[tokio::test]
    async fn test_returns_list_of_apps_ok() -> Result<()> {
        let (server, _) = build_server_logged_in(default_config()).await?;

        let res = server.get(BASE_URI).await;

        res.assert_status_ok();
        res.assert_header(CONTENT_TYPE, "text/html; charset=utf-8");
        res.assert_text_contains(r#"<tr class="text-center"><td>1</td><td><a class="underline" href="https://www.accuchef.com" target="_blank">AccuChef</a></td><td>.html, .txt, .zip</td></tr><tr class="text-center"><td>2</td><td><a class="underline" href="https://www.bigoven.com" target="_blank">BigOven</a></td><td>.txt</td></tr><tr class="text-center"><td>3</td><td><a class="underline" href="https://cheftap.com" target="_blank">ChefTap</a></td><td>.html, .txt, .zip</td></tr><tr class="text-center"><td>4</td><td><a class="underline" href="https://www.inakasoftware.com/computer-cuisine-deluxe-mac-windows-recipe-software-organizer/" target="_blank">Computer Cuisine Deluxe</a></td><td>.csv</td></tr><tr class="text-center"><td>5</td><td><a class="underline" href="https://cookbookmanager.com" target="_blank">CookBook</a></td><td>.txt, .zip</td></tr><tr class="text-center"><td>6</td><td><a class="underline" href="https://cooklang.org/" target="_blank">Cooklang</a></td><td>.cook</td></tr><tr class="text-center"><td>7</td><td><a class="underline" href="https://cooklang.org/" target="_blank">COOKmate</a></td><td>.mcb, .mmf, .rk, .xml, .zip</td></tr><tr class="text-center"><td>8</td><td><a class="underline" href="https://www.dvo.com/" target="_blank">Cook'n</a></td><td>.txt, .zip</td></tr><tr class="text-center"><td>9</td><td><a class="underline" href="https://www.copymethat.com/" target="_blank">Copy Me That</a></td><td>.html, .txt, .yml, .zip</td></tr><tr class="text-center"><td>10</td><td><a class="underline" href="https://crouton.app" target="_blank">Crouton</a></td><td>.crumb</td></tr><tr class="text-center"><td>11</td><td><a class="underline" href="https://easy-recipe-deluxe.software.informer.com" target="_blank">Easy Recipe Deluxe</a></td><td></td></tr><tr class="text-center"><td>12</td><td><a class="underline" href="http://www.collectionneurderecettes.com/" target="_blank">Le Collectionneur de Recettes</a></td><td>.html, .txt, .zip</td></tr><tr class="text-center"><td>13</td><td><a class="underline" href="https://www.mountainsoftware.com/homecook.php" target="_blank">Home Cookin</a></td><td>.hc, .mz2, .txt, .xml</td></tr><tr class="text-center"><td>14</td><td><a class="underline" href="https://www.kalorio.de" target="_blank">Kalorio</a></td><td>.txt, .xml</td></tr><tr class="text-center"><td>15</td><td><a class="underline" href="https://www.mastercook.com" target="_blank">MasterCook</a></td><td>.mx2, .mxp, .mz2, .txt</td></tr><tr class="text-center"><td>16</td><td><a class="underline" href="https://web.archive.org/web/20081221021301/http://episoft.home.comcast.net/~episoft/mmdown.htm" target="_blank">Meal-Master</a></td><td>.mx2, .mxp, .mz2, .txt</td></tr><tr class="text-center"><td>17</td><td><a class="underline" href="https://www.mrcook.app/en" target="_blank">Mr. Cook</a></td><td>.csv, .zip</td></tr><tr class="text-center"><td>18</td><td><a class="underline" href="https://www.myrecipebox.app/en/" target="_blank">My Recipe Box</a></td><td>.csv, .rtk, .zip</td></tr><tr class="text-center"><td>19</td><td><a class="underline" href="https://www.paprikaapp.com" target="_blank">Paprika</a></td><td>.paprikarecipes</td></tr><tr class="text-center"><td>20</td><td><a class="underline" href="https://www.pepperplate.com/" target="_blank">Pepperplate</a></td><td>.txt, .zip</td></tr><tr class="text-center"><td>21</td><td><a class="underline" href="https://recipekeeperonline.com" target="_blank">Recipe Keeper</a></td><td>.zip</td></tr><tr class="text-center"><td>22</td><td><a class="underline" href="https://recipemd.org/" target="_blank">RecipeMD</a></td><td>.md</td></tr><tr class="text-center"><td>23</td><td><a class="underline" href="https://recipesage.com" target="_blank">RecipeSage</a></td><td>.json, .txt, .xml</td></tr><tr class="text-center"><td>24</td><td><a class="underline" href="https://www.rezkonv.de/" target="_blank">Rezkonv</a></td><td>.rk</td></tr><tr class="text-center"><td>25</td><td><a class="underline" href="https://www.mysaffronapp.com" target="_blank">Saffron</a></td><td>.txt, .zip</td></tr><tr class="text-center"><td>26</td><td><a class="underline" href="https://www.shopncook.com/" target="_blank">Shop'NCook</a></td><td>.html, .scx, .txt, .zip</td></tr><tr class="text-center"><td>27</td><td><a class="underline" href="https://www.umami.recipes/" target="_blank">Umami</a></td><td>.json, .html, .md, .txt, .zip</td></tr>"#);
        Ok(())
    }
}
