use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use smallvec::SmallVec;

pub fn one_or_many<'de, D, T, C>(deserializer: D) -> Result<C, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
    C: FromIterator<T> + DeserializeOwned,
{
    use serde::de::Error;
    use serde_json::Value;

    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Array(arr) => serde_json::from_value(Value::Array(arr)).map_err(Error::custom),
        other => {
            let item: T = serde_json::from_value(other).map_err(Error::custom)?;
            Ok(std::iter::once(item).collect())
        }
    }
}

pub fn is_smallvec_empty<A: smallvec::Array>(sv: &SmallVec<A>) -> bool {
    sv.is_empty()
}
