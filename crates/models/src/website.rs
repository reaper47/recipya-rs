use diesel::{Queryable, Selectable};
use diesel_async::RunQueryDsl;

use repository::ModelManager;
use repository::schema::websites;

use super::Result;

/// A trait for converting a type into an HTML table representation.
pub trait ToHtmlTable {
    /// Renders the websites as a string of HTML table rows.
    fn to_html_table_rows(&self) -> String;
}

/// Holds some components of a website.
#[derive(Clone, Debug, PartialEq, Queryable, Selectable)]
#[diesel(table_name = websites)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Website {
    pub id: i64,
    pub host: String,
    pub url: String,
}

impl Website {
    /// Fetches all websites that can be scraped by the application.
    pub async fn supported_websites(mm: &ModelManager) -> Result<Vec<Website>> {
        let mut conn = mm.pool.get().await?;

        let websites = websites::table.load::<Website>(&mut conn).await?;

        Ok(websites)
    }
}

impl ToHtmlTable for Vec<Website> {
    fn to_html_table_rows(&self) -> String {
        let mut html = String::new();

        if self.is_empty() {
            html.push_str(r#"<tr class="border text-center">"#);
            html.push_str(r#"<td>-1</td>"#);
            html.push_str(r#"<td>No result</td>"#);
            html.push_str(r#"</tr>"#);
        } else {
            for website in self {
                html.push_str(r#"<tr class="text-center">"#);
                html.push_str(&format!(r#"<td>{}</td>"#, website.id));
                html.push_str(&format!(
                    r#"<td><a class="underline" href="{}" target="_blank">{}</a></td>"#,
                    website.url, website.host
                ));
                html.push_str("</tr>");
            }
        }

        html
    }
}

#[cfg(test)]
mod tests {
    use crate::website::{ToHtmlTable, Website};

    use testing::utils::{TestDb, create_app_state};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_website_to_html_table_rows() {
        let websites = some_websites();

        let got = websites.to_html_table_rows();

        pretty_assertions::assert_eq!(
            r#"<tr class="text-center"><td>1</td><td><a class="underline" href="https://15gram.be/recepten" target="_blank">15gram.be</a></td></tr><tr class="text-center"><td>2</td><td><a class="underline" href="https://www.750g.com" target="_blank">750g.com</a></td></tr><tr class="text-center"><td>3</td><td><a class="underline" href="https://101cookbooks.com" target="_blank">101cookbooks.com</a></td></tr>"#,
            got,
        );
    }

    #[tokio::test]
    async fn test_supported_websites_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;

        let got = Website::supported_websites(&state.mm).await?;

        pretty_assertions::assert_eq!(some_websites(), &got[0..3]);
        Ok(())
    }

    fn some_websites() -> Vec<Website> {
        vec![
            Website {
                id: 1,
                url: "https://15gram.be/recepten".into(),
                host: "15gram.be".into(),
            },
            Website {
                id: 2,
                url: "https://www.750g.com".into(),
                host: "750g.com".into(),
            },
            Website {
                id: 3,
                url: "https://101cookbooks.com".into(),
                host: "101cookbooks.com".into(),
            },
        ]
    }
}
