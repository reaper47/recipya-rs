use std::fmt::Write;

use axum::response::{Html, IntoResponse};

use recipya_scraper::{ToHtmlTable, Website};

use crate::middleware::mw_auth::RequireAuth;

/// Handles the supported applications endpoint.
#[allow(clippy::too_many_lines)]
pub async fn supported_applications_handler(RequireAuth(_): RequireAuth) -> impl IntoResponse {
    let applications = [
        (
            "AccuChef",
            "https://www.accuchef.com",
            vec![".html", ".txt", ".zip"],
        ),
        ("BigOven", "https://www.bigoven.com", vec![".txt"]),
        (
            "ChefTap",
            "https://cheftap.com",
            vec![".html", ".txt", ".zip"],
        ),
        (
            "Computer Cuisine Deluxe",
            "https://www.inakasoftware.com/computer-cuisine-deluxe-mac-windows-recipe-software-organizer/",
            vec![".csv"],
        ),
        (
            "CookBook",
            "https://cookbookmanager.com",
            vec![".txt", ".zip"],
        ),
        ("Cooklang", "https://cooklang.org/", vec![".cook"]),
        (
            "COOKmate",
            "https://cooklang.org/",
            vec![".mcb", ".mmf", ".rk", ".xml", ".zip"],
        ),
        ("Cook'n", "https://www.dvo.com/", vec![".txt", ".zip"]),
        (
            "Copy Me That",
            "https://www.copymethat.com/",
            vec![".html", ".txt", ".yml", ".zip"],
        ),
        ("Crouton", "https://crouton.app", vec![".crumb"]),
        (
            "Easy Recipe Deluxe",
            "https://easy-recipe-deluxe.software.informer.com",
            vec![],
        ),
        (
            "Le Collectionneur de Recettes",
            "http://www.collectionneurderecettes.com/",
            vec![".html", ".txt", ".zip"],
        ),
        (
            "Home Cookin",
            "https://www.mountainsoftware.com/homecook.php",
            vec![".hc", ".mz2", ".txt", ".xml"],
        ),
        ("Kalorio", "https://www.kalorio.de", vec![".txt", ".xml"]),
        (
            "MasterCook",
            "https://www.mastercook.com",
            vec![".mx2", ".mxp", ".mz2", ".txt"],
        ),
        (
            "Meal-Master",
            "https://web.archive.org/web/20081221021301/http://episoft.home.comcast.net/~episoft/mmdown.htm",
            vec![".mx2", ".mxp", ".mz2", ".txt"],
        ),
        (
            "Mr. Cook",
            "https://www.mrcook.app/en",
            vec![".csv", ".zip"],
        ),
        (
            "My Recipe Box",
            "https://www.myrecipebox.app/en/",
            vec![".csv", ".rtk", ".zip"],
        ),
        (
            "Paprika",
            "https://www.paprikaapp.com",
            vec![".paprikarecipes"],
        ),
        (
            "Pepperplate",
            "https://www.pepperplate.com/",
            vec![".txt", ".zip"],
        ),
        (
            "Recipe Keeper",
            "https://recipekeeperonline.com",
            vec![".zip"],
        ),
        ("RecipeMD", "https://recipemd.org/", vec![".md"]),
        (
            "RecipeSage",
            "https://recipesage.com",
            vec![".json", ".txt", ".xml"],
        ),
        ("Rezkonv", "https://www.rezkonv.de/", vec![".rk"]),
        ("Saffron", "https://www.mysaffronapp.com", vec![".txt"]),
        (
            "Umami",
            "https://www.umami.recipes/",
            vec![".json", ".html", ".md", ".txt", ".zip"],
        ),
    ];

    let mut html = String::new();

    for (i, (name, url, formats)) in applications.into_iter().enumerate() {
        html.push_str(r#"<tr class="text-center">"#);
        let _ = write!(html, "<td>{}</td>", i + 1);
        let _ = write!(
            html,
            r#"<td><a class="underline" href="{url}" target="_blank">{name}</a></td>"#
        );
        let _ = write!(html, "<td>{}</td>", formats.join(", "));
        html.push_str("</tr>");
    }

    Html(html)
}

/// Handles the supported websites endpoint.
pub async fn supported_websites_handler(RequireAuth(_): RequireAuth) -> impl IntoResponse {
    let mut websites = Website::all();
    websites.sort_by_key(|w| w.host);
    Html(websites.to_html_table_rows()).into_response()
}
