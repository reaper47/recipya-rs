use serde::Deserialize;

use crate::Result;

#[derive(Deserialize)]

pub struct Mealie {
    token: String,
}

pub struct MealieRecipe;

impl Mealie {
    /// Connects to a Mealie instance using the [/api/auth/token](https://demo.mealie.io/docs#/Users%3A%20Authentication/get_token_api_auth_token_post) endpoint.
    pub fn new(host: String, username: String, password: String) -> Result<Self> {
        Ok(Self {})
    }

    pub fn fetch_recipes(&self) -> Result<Vec<MealieRecipe>> {
        todo!("Fetch recipes from Mealie")
    }
}

impl Drop for Mealie {
    fn drop(&mut self) {
        todo!("Call /api/auth/logout")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mealie = Mealie::new(
            "http://localhost".to_string(),
            "user".to_string(),
            "password".to_string(),
        );

        assert!(mealie.is_ok());
    }
}
