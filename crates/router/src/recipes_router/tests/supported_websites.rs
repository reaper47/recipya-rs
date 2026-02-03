#[cfg(test)]
mod tests {
    use axum::http::Method;
    use axum_test::http::header::CONTENT_TYPE;

    use testing::utils::{TestDb, assert_must_be_logged_in, build_server_logged_in};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    const BASE_URI: &str = "/recipes/supported-websites";

    #[tokio::test]
    async fn test_must_be_logged_in_ok() -> Result<()> {
        assert_must_be_logged_in(Method::GET, BASE_URI).await
    }

    #[tokio::test]
    async fn test_fetch_websites_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let server = build_server_logged_in(config).await?;

        let res = server.get(BASE_URI).await;

        res.assert_status_ok();
        res.assert_header(CONTENT_TYPE, "text/html; charset=utf-8");
        res.assert_text_contains(r#"<tr class="text-center"><td>1</td><td><a class="underline" href="https://https://www.allrecipes.com/" target="_blank">Allrecipes</a></td></tr><tr class="text-center"><td>2</td><td><a class="underline" href="https://https://zaatarandzaytoun.com/recipe-index/" target="_blank">Zaatar & Zaytoun</a></td></tr><tr class="text-center"><td>3</td><td><a class="underline" href="https://https://zabihahalal.com/recipes/" target="_blank">Zabiha Halal</a></td></tr><tr class="text-center"><td>4</td><td><a class="underline" href="https://https://zagleft.com/recipe-list/" target="_blank">Zagleft</a></td></tr><tr class="text-center"><td>5</td><td><a class="underline" href="https://https://zardyplants.com" target="_blank">ZardyPlants</a></td></tr><tr class="text-center"><td>6</td><td><a class="underline" href="https://https://zarskitchen.com" target="_blank">Zars Kitchen</a></td></tr><tr class="text-center"><td>7</td><td><a class="underline" href="https://https://zeezest.com/recipes/" target="_blank">ZZest</a></td></tr><tr class="text-center"><td>8</td><td><a class="underline" href="https://https://www.zenbelly.com/recipes/" target="_blank">Zen Belly</a></td></tr><tr class="text-center"><td>9</td><td><a class="underline" href="https://https://zenhealth.net/recipe-index/" target="_blank">Zenhealth</a></td></tr><tr class="text-center"><td>10</td><td><a class="underline" href="https://https://zenkimchi.com" target="_blank">Zen Kimchi</a></td></tr><tr class="text-center"><td>11</td><td><a class="underline" href="https://https://zestfulkitchen.com/category/all-recipes-from-zestful-kitchen/" target="_blank">Zestful Kitchen</a></td></tr><tr class="text-center"><td>12</td><td><a class="underline" href="https://https://zestysouthindiankitchen.com" target="_blank">Zesty South Indian Kitchen</a></td></tr><tr class="text-center"><td>13</td><td><a class="underline" href="https://https://zhangcatherine.com" target="_blank">Catherine Desserts</a></td></tr><tr class="text-center"><td>14</td><td><a class="underline" href="https://https://ziahatchchileco.com" target="_blank">Zia Hatch Chile Company</a></td></tr><tr class="text-center"><td>15</td><td><a class="underline" href="https://https://zibakitchen.com" target="_blank">Ziba Kitchen</a></td></tr><tr class="text-center"><td>16</td><td><a class="underline" href="https://https://www.zoebakes.com" target="_blank">Zoë Bakes</a></td></tr><tr class="text-center"><td>17</td><td><a class="underline" href="https://https://zonacooks.com" target="_blank">Zona Cooks</a></td></tr><tr class="text-center"><td>18</td><td><a class="underline" href="https://https://zsuzsaisinthekitchen.blogspot.com/" target="_blank">Zsuzsa is in the kitchen</a></td></tr><tr class="text-center"><td>19</td><td><a class="underline" href="https://https://zumavalley.com/blogs/smoothies-bowls" target="_blank">Zuma Valley</a></td></tr><tr class="text-center"><td>20</td><td><a class="underline" href="https://https://zuranazrecipe.com/" target="_blank">Zuranaz</a></td></tr><tr class="text-center"><td>21</td><td><a class="underline" href="https://https://zweigles.com/recipes/" target="_blank">Zweigles</a></td></tr>"#);
        Ok(())
    }
}
