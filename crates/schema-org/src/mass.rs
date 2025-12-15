use serde::{Deserialize, Serialize};

///<https://schema.org/Mass>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub struct Mass(String);

impl Mass {
    /// Creates a new Mass struct with the given value.
    pub fn new(mass: impl Into<String>) -> Self {
        Self(mass.into())
    }

    /// Converts the mass to a number.
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

impl AsRef<String> for Mass {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_number() {
        let mass = Mass::new("100 grams");

        assert_eq!(mass.to_number(), 100);
    }
}
