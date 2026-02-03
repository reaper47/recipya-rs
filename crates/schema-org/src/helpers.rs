use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};

/// Deserialize a value as either a single value or an array of values.
pub fn one_or_many<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    use serde::de::Error;
    use serde_json::Value;

    let value = Value::deserialize(deserializer)?;

    match value {
        Value::Null => Ok(Vec::new()),
        Value::String(ref s) if s.trim().is_empty() => Ok(Vec::new()),
        Value::Array(arr) => {
            arr.into_iter()
                .filter(|v| !v.is_null()) // Basic cleanup
                .map(|v| serde_json::from_value(v).map_err(D::Error::custom))
                .collect()
        }
        other => {
            let item = serde_json::from_value::<T>(other).map_err(D::Error::custom)?;
            Ok(vec![item])
        }
    }
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
}
