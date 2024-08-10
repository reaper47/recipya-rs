use serde::{de, Deserialize, Deserializer};

use crate::schema::AtContext::SchemaDotOrg;

pub mod article;
pub mod common;
pub mod nutrition;
pub mod recipe;

#[derive(Debug, PartialEq)]
pub enum AtContext {
    SchemaDotOrg,
}

impl Default for AtContext {
    fn default() -> Self {
        SchemaDotOrg
    }
}

impl<'de> Deserialize<'de> for AtContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.trim_end_matches('/') {
            "http://schema.org" => Ok(AtContext::SchemaDotOrg),
            "https://schema.org" => Ok(AtContext::SchemaDotOrg),
            _ => Err(de::Error::invalid_value(
                de::Unexpected::Str(&s),
                &"another context",
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AtType {
    AggregateRating,
    Article,
    BreadcrumbList,
    HowToStep,
    ImageObject,
    ListItem,
    NewsArticle,
    NutritionInformation,
    Organization,
    Person,
    Recipe,
    VideoObject,
    Unspecified,
    WebPage,
    WebSite,
}

impl Default for AtType {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl<'de> Deserialize<'de> for AtType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = AtType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v.trim().as_ref() {
                    "AggregateRating" => Ok(AtType::AggregateRating),
                    "Article" => Ok(AtType::Article),
                    "BreadcrumbList" => Ok(AtType::BreadcrumbList),
                    "HowToStep" => Ok(AtType::HowToStep),
                    "ImageObject" => Ok(AtType::ImageObject),
                    "ListItem" => Ok(AtType::ListItem),
                    "NewsArticle" => Ok(AtType::NewsArticle),
                    "NutritionInformation" => Ok(AtType::NutritionInformation),
                    "Organization" => Ok(AtType::Organization),
                    "Person" => Ok(AtType::Person),
                    "Recipe" => Ok(AtType::Recipe),
                    "VideoObject" => Ok(AtType::VideoObject),
                    "WebPage" => Ok(AtType::WebPage),
                    "WebSite" => Ok(AtType::WebSite),
                    _ => Ok(AtType::Unspecified),
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}
