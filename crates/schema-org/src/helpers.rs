use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};

pub fn one_or_many<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    use serde::de::Error;
    use serde_json::Value;

    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Array(arr) => serde_json::from_value(Value::Array(arr)).map_err(Error::custom),
        other => Ok(vec![serde_json::from_value(other).map_err(Error::custom)?]),
    }
}
