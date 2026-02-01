use std::fmt::Formatter;

use reqwest::Url;
use schema_org::Recipe;
use scraper::Html;

use crate::custom;

use super::{Error, Result};

include!("generated_websites.rs");

impl Website {
    /// Creates a `Website` instance from a given URL.
    pub fn from(url: &str) -> Result<Self> {
        let url = Url::parse(url).map_err(|err| Error::Parse(err.to_string()))?;
        let domain = url.domain().ok_or(Error::UnknownWebsite)?;
        Website::from_domain(domain).ok_or(Error::DomainNotImplemented)
    }

    /// Parses the given HTML document manually.
    pub fn parse_manually(&self, doc: &Html, url: &str) -> Result<Recipe> {
        match self {
            Website::Zeezest => custom::parse_zeezest(doc, url),
            Website::ZiaHatchChileCompany => custom::parse_ziahatchchilecompany(doc, url),
            _ => Err(Error::DomainNotImplemented),
        }
    }

    /// Augments LD+JSON data with site-specific information.
    /// Some sites have incomplete or incorrect LD+JSON that needs fixing.
    pub(crate) fn augment_ld_json(&self, doc: &Html, recipe: Recipe) -> Recipe {
        match self {
            Website::ZaatarAndZaytoun => custom::zaatarandzaytoun::add_info(doc, recipe),
            Website::ZabihaHalal => custom::zabihahalal::add_info(doc, recipe),
            Website::ZagLeft => custom::zagleft::add_info(doc, recipe),
            _ => recipe,
        }
    }
}

impl std::fmt::Display for Website {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.domain())
    }
}

/// A trait for converting a type into an HTML table representation.
pub trait ToHtmlTable {
    /// Renders the websites as a string of HTML table rows.
    fn to_html_table_rows(&self) -> String;
}

impl<'a> ToHtmlTable for Vec<WebsiteMetadata<'a>> {
    fn to_html_table_rows(&self) -> String {
        let mut html = String::new();

        if self.is_empty() {
            html.push_str(r#"<tr class="border text-center">"#);
            html.push_str(r#"<td>-1</td>"#);
            html.push_str(r#"<td>No result</td>"#);
            html.push_str(r#"</tr>"#);
        } else {
            self.iter().enumerate().for_each(|(idx, website)| {
                html.push_str(r#"<tr class="text-center">"#);
                html.push_str(&format!(r#"<td>{}</td>"#, idx + 1));
                html.push_str(&format!(
                    r#"<td><a class="underline" href="{}" target="_blank">{}</a></td>"#,
                    website.url, website.host
                ));
                html.push_str("</tr>");
            });
        }

        html
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_website_to_html_table_rows() {
        let websites = some_websites();

        let got = websites.to_html_table_rows();

        pretty_assertions::assert_eq!(
            r#"<tr class="text-center"><td>1</td><td><a class="underline" href="https://15gram.be/recepten" target="_blank">15gram.be</a></td></tr><tr class="text-center"><td>2</td><td><a class="underline" href="https://www.750g.com" target="_blank">750g.com</a></td></tr><tr class="text-center"><td>3</td><td><a class="underline" href="https://101cookbooks.com" target="_blank">101cookbooks.com</a></td></tr>"#,
            got,
        );
    }

    fn some_websites<'a>() -> Vec<WebsiteMetadata<'a>> {
        vec![
            WebsiteMetadata {
                url: "https://15gram.be/recepten".into(),
                host: "15gram.be".into(),
                cuisine: None,
            },
            WebsiteMetadata {
                url: "https://www.750g.com".into(),
                host: "750g.com".into(),
                cuisine: None,
            },
            WebsiteMetadata {
                url: "https://101cookbooks.com".into(),
                host: "101cookbooks.com".into(),
                cuisine: None,
            },
        ]
    }
}
