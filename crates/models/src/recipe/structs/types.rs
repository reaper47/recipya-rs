use std::fmt;

use diesel::{
    backend::Backend,
    deserialize::{self, FromSqlRow},
    expression::AsExpression,
    serialize,
    sql_types::Text,
};

/// Represents the type for a source.
#[derive(Debug, Clone, Eq, PartialEq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum Source {
    Url(String),
    Other(String),
}

impl Default for Source {
    fn default() -> Self {
        Self::Other(String::new())
    }
}

impl Source {
    /// Creates a new source from a string.
    pub fn new(source: impl Into<String>) -> Self {
        let src = source.into();
        match url::Url::parse(&src) {
            Ok(_) => Self::Url(src),
            Err(_) => Self::Other(src),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Url(url) => url,
            Self::Other(other) => other,
        }
    }

    pub fn into_string(self) -> String {
        match self {
            Self::Url(url) => url,
            Self::Other(other) => other,
        }
    }
}

impl std::ops::Deref for Source {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Url(url) => url,
            Self::Other(other) => other,
        }
    }
}

impl From<String> for Source {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<Source> for String {
    fn from(s: Source) -> Self {
        s.into_string()
    }
}

impl From<Option<String>> for Source {
    fn from(v: Option<String>) -> Self {
        Self::new(v.unwrap_or_default())
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<B: Backend> serialize::ToSql<Text, B> for Source
where
    String: serialize::ToSql<Text, B>,
{
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, B>) -> serialize::Result {
        <String as serialize::ToSql<Text, B>>::to_sql(
            match self {
                Self::Url(url) => url,
                Self::Other(other) => other,
            },
            out,
        )
    }
}

impl<B: Backend> deserialize::FromSql<Text, B> for Source
where
    String: deserialize::FromSql<Text, B>,
{
    fn from_sql(bytes: <B as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        <String as deserialize::FromSql<Text, B>>::from_sql(bytes).map(Self::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_source {
        use super::*;

        #[test]
        fn test_source_new_is_url() {
            let source = Source::new("https://www.example.com");

            assert!(matches!(source, Source::Url(_)));
            assert_eq!(source.as_str(), "https://www.example.com");
        }

        #[test]
        fn test_source_new_invalid_url() {
            let source = Source::new("not a url");

            assert!(matches!(source, Source::Other(_)));
            assert_eq!(source.as_str(), "not a url");
        }

        #[test]
        fn test_source_new_handles_various_schemes() {
            let http = Source::new("http://example.com");
            assert!(matches!(http, Source::Url(_)));

            let https = Source::new("https://example.com");
            assert!(matches!(https, Source::Url(_)));

            let ftp = Source::new("ftp://example.com");
            assert!(matches!(ftp, Source::Url(_)));
        }

        #[test]
        fn test_source_new_partial_urls_is_other() {
            let source = Source::new("www.example.com");
            assert!(matches!(source, Source::Other(_)));

            let source2 = Source::new("example.com");
            assert!(matches!(source2, Source::Other(_)));
        }

        #[test]
        fn test_source_default() {
            let source = Source::default();

            assert!(matches!(source, Source::Other(_)));
            assert_eq!(source.as_str(), "");
        }

        #[test]
        fn test_source_as_str() {
            let url_source = Source::new("https://example.com");
            assert_eq!(url_source.as_str(), "https://example.com");

            let other_source = Source::new("some text");
            assert_eq!(other_source.as_str(), "some text");
        }

        #[test]
        fn test_source_into_string() {
            let url_source = Source::new("https://example.com");
            assert_eq!(url_source.into_string(), "https://example.com");

            let other_source = Source::new("some text");
            assert_eq!(other_source.into_string(), "some text");
        }

        #[test]
        fn test_source_deref() {
            let source = Source::new("https://example.com");

            assert_eq!(source.len(), 22);
            assert!(source.starts_with("https://"));
        }

        #[test]
        fn test_source_from_string() {
            let source: Source = String::from("https://example.com").into();
            assert!(matches!(source, Source::Url(_)));

            let source2: Source = String::from("not a url").into();
            assert!(matches!(source2, Source::Other(_)));
        }

        #[test]
        fn test_source_from_option_string() {
            let source: Source = Some(String::from("https://example.com")).into();

            assert!(matches!(source, Source::Url(_)));
            assert_eq!(source.as_str(), "https://example.com");
        }

        #[test]
        fn test_source_from_option_string_none() {
            let opt: Option<String> = None;
            let source: Source = opt.into();

            assert!(matches!(source, Source::Other(_)));
            assert_eq!(source.as_str(), "");
        }

        #[test]
        fn test_source_display() {
            let url_source = Source::new("https://example.com");
            assert_eq!(format!("{url_source}"), "https://example.com");

            let other_source = Source::new("some text");
            assert_eq!(format!("{other_source}"), "some text");
        }
    }
}
