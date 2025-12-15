use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum EnergyOrText {
    Energy(Energy),
    Text(String),
}

///<https://schema.org/Energy>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub struct Energy(String);

impl Energy {
    /// Creates a new Energy struct with the given value.
    pub fn new(energy: impl Into<String>) -> Self {
        Self(energy.into())
    }

    /// Converts the energy to a number.
    pub fn to_number(&self) -> i16 {
        Some(self.as_ref())
            .map(|s| {
                s.split(' ')
                    .map(|s| s.parse::<i16>().unwrap_or_default())
                    .sum()
            })
            .unwrap_or_default()
    }
}

impl AsRef<String> for Energy {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_number() {
        let energy = Energy::new("100 kcal");

        assert_eq!(energy.to_number(), 100);
    }
}
