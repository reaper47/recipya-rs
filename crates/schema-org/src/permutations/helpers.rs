use std::str::FromStr;

use serde::de;
use serde::de::Error;
use serde_json::Value;

use crate::data_type::text::URL;

pub(crate) fn has_defined_term_set_properties(value: &Value) -> bool {
    value.get("hasDefinedTerm").is_some()
}

pub(crate) fn has_creative_work_properties(value: &Value) -> bool {
    value.get("headline").is_some()
        || value.get("author").is_some()
        || value.get("datePublished").is_some()
        || value.get("encoding").is_some()
        || value.get("publisher").is_some()
        || value.get("license").is_some()
        || value.get("creator").is_some()
        || value.get("abstract").is_some()
        || value.get("text").is_some()
        || value.get("position").is_some()
}

pub(crate) fn has_property_properties(value: &Value) -> bool {
    value.get("domainIncludes").is_some() || value.get("rangeIncludes").is_some()
}

pub(crate) fn has_datafeed_properties(value: &Value) -> bool {
    value.as_object().map_or(false, |obj| {
        obj.contains_key("dateCreated")
            || obj.contains_key("dateDeleted")
            || obj.contains_key("dateModified")
            || obj.contains_key("item")
    })
}

pub(crate) fn parse_url_variant<E, T>(v: &str, map_fn: impl FnOnce(URL) -> T) -> Result<T, E>
where
    E: Error
{
    URL::from_str(v).map(map_fn).map_err(|_| Error::invalid_value(de::Unexpected::Str(v), &"a valid URL"))
}
