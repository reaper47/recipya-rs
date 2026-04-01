use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

/// Deserialize a value as either a single value or an array of values.
pub fn one_or_many<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    use serde::de::Error;

    match Value::deserialize(deserializer)? {
        Value::Null => Ok(Vec::new()),
        Value::String(ref s) if s.trim().is_empty() => Ok(Vec::new()),
        Value::Array(arr) => arr
            .into_iter()
            .filter(|v| !v.is_null())
            .map(|v| serde_json::from_value(v).map_err(D::Error::custom))
            .collect(),
        other => {
            let item = serde_json::from_value::<T>(other).map_err(D::Error::custom)?;
            Ok(vec![item])
        }
    }
}

/// Deserializes one or many items, coercing JSON strings to numbers if possible.
pub fn one_or_many_string_or_num<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    use serde::de::Error;

    fn clean_value(v: Value) -> Value {
        match v {
            Value::String(s) => s
                .parse::<serde_json::Number>()
                .map_or(Value::String(s), Value::Number),
            Value::Array(arr) => Value::Array(arr.into_iter().map(clean_value).collect()),
            _ => v,
        }
    }

    match Value::deserialize(deserializer)? {
        Value::Null => Ok(Vec::new()),
        Value::String(ref s) if s.trim().is_empty() => Ok(Vec::new()),
        Value::Array(arr) => arr
            .into_iter()
            .filter(|v| !v.is_null())
            .map(clean_value)
            .map(|v| serde_json::from_value(v).map_err(D::Error::custom))
            .collect(),
        other => {
            let cleaned = clean_value(other);
            let item = serde_json::from_value::<T>(cleaned).map_err(D::Error::custom)?;
            Ok(vec![item])
        }
    }
}

/// Deserializes the recipe @type.
pub fn deserialize_type<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = <Option<Value> as serde::Deserialize>::deserialize(deserializer)?;
    Ok(value.and_then(|v| match v {
        Value::String(s) => Some(s),
        Value::Array(arr) => {
            let strings: Vec<String> = arr
                .into_iter()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect();

            let first = strings.first().cloned();

            strings
                .into_iter()
                .find(|s| s.to_lowercase() == "recipe")
                .or(first)
        }
        _ => None,
    }))
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct {
        #[serde(deserialize_with = "one_or_many")]
        values: Vec<String>,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStructTypeOption {
        #[serde(deserialize_with = "deserialize_type")]
        r#type: Option<String>,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStructNumbers {
        #[serde(deserialize_with = "one_or_many")]
        numbers: Vec<i32>,
    }

    #[test]
    fn test_deserialize_single_value() {
        let json = r#"{"values": "single"}"#;

        let got: TestStruct = serde_json::from_str(json).unwrap();

        assert_eq!(got.values, vec!["single"]);
    }

    #[test]
    fn test_deserialize_array() {
        let json = r#"{"values": ["first", "second", "third"]}"#;

        let got: TestStruct = serde_json::from_str(json).unwrap();

        assert_eq!(got.values, vec!["first", "second", "third"]);
    }

    #[test]
    fn test_deserialize_empty_array() {
        let json = r#"{"values": []}"#;

        let got: TestStruct = serde_json::from_str(json).unwrap();

        assert_eq!(got.values, Vec::<String>::new());
    }

    #[test]
    fn test_deserialize_single_number() {
        let json = r#"{"numbers": 42}"#;

        let got: TestStructNumbers = serde_json::from_str(json).unwrap();

        assert_eq!(got.numbers, vec![42]);
    }

    #[test]
    fn test_deserialize_array_of_numbers() {
        let json = r#"{"numbers": [1, 2, 3]}"#;

        let got: TestStructNumbers = serde_json::from_str(json).unwrap();

        assert_eq!(got.numbers, vec![1, 2, 3]);
    }

    #[test]
    fn test_deserialize_invalid_type() {
        let json = r#"{"values": 123}"#;

        let result: Result<TestStruct, _> = serde_json::from_str(json);

        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_type_string() {
        let json = r#"{"type": "Recipe"}"#;

        let got: TestStructTypeOption = serde_json::from_str(json).unwrap();

        assert_eq!(got.r#type, Some("Recipe".to_string()));
    }

    #[test]
    fn test_deserialize_type_array_prefers_recipe() {
        let json = r#"{"type": ["Article", "Recipe", "WebPage"]}"#;

        let got: TestStructTypeOption = serde_json::from_str(json).unwrap();

        assert_eq!(got.r#type, Some("Recipe".to_string()));
    }

    #[test]
    fn test_deserialize_type_array_falls_back_to_first() {
        let json = r#"{"type": ["Article", "WebPage"]}"#;

        let got: TestStructTypeOption = serde_json::from_str(json).unwrap();

        assert_eq!(got.r#type, Some("Article".to_string()));
    }

    #[test]
    fn test_deserialize_type_null() {
        let json = r#"{"type": null}"#;

        let got: TestStructTypeOption = serde_json::from_str(json).unwrap();

        assert_eq!(got.r#type, None);
    }
}
