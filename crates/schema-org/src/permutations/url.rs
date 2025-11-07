use std::fmt::Formatter;

use schemars::JsonSchema;
use serde::de::{Error, MapAccess};
use serde::{Deserialize, de};

use crate::data_type::text::URL;
use crate::permutations::helpers::parse_url_variant;
use crate::thing::creative_work::WebPage;
use crate::thing::intangible::{EntryPoint, SpeakableSpecification};
use crate::thing::medical_entity::ImageObject;

#[derive(Debug, PartialEq, JsonSchema)]
pub enum EntryPointOrURL {
    EntryPoint(EntryPoint),
    URL(URL),
}

impl<'de> Deserialize<'de> for EntryPointOrURL {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = EntryPointOrURL;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a URL as a string or an EntryPoint object")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                parse_url_variant(v, Self::Value::URL)
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_str(&v)
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let e = EntryPoint::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(EntryPointOrURL::EntryPoint(e))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq, JsonSchema)]
pub enum ImageObjectOrURL {
    ImageObject(ImageObject),
    URL(URL),
}

impl Default for ImageObjectOrURL {
    fn default() -> Self {
        Self::URL(URL::default())
    }
}

impl<'de> Deserialize<'de> for ImageObjectOrURL {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = ImageObjectOrURL;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("ImageObject or URL")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                parse_url_variant(v, Self::Value::URL)
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_str(&v)
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let v = ImageObject::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(Self::Value::ImageObject(v))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq, JsonSchema)]
pub enum URLOrWebPage {
    URL(URL),
    WebPage(WebPage),
}

impl Default for URLOrWebPage {
    fn default() -> Self {
        URLOrWebPage::URL(URL::default())
    }
}

impl<'de> Deserialize<'de> for URLOrWebPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = URLOrWebPage;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a URL as a string or a WebPage object")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                parse_url_variant(v, Self::Value::URL)
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_str(&v)
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let w = WebPage::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(URLOrWebPage::WebPage(w))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq, JsonSchema)]
pub enum SpeakableSpecificationOrURL {
    SpeakableSpecification(SpeakableSpecification),
    URL(URL),
}

impl Default for SpeakableSpecificationOrURL {
    fn default() -> Self {
        Self::URL(URL::default())
    }
}

impl<'de> Deserialize<'de> for SpeakableSpecificationOrURL {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = SpeakableSpecificationOrURL;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a URL as a string or a SpeakableSpecification object")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                parse_url_variant(v, Self::Value::URL)
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_str(&v)
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let w = SpeakableSpecification::deserialize(
                    de::value::MapAccessDeserializer::new(map),
                )?;
                Ok(SpeakableSpecificationOrURL::SpeakableSpecification(w))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}
