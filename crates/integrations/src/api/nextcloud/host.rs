#[derive(Clone)]
pub struct Host(String);

impl Host {
    /// Creates a new `Host` instance.
    pub fn new(base_url: impl Into<String>) -> Self {
        let mut host = base_url.into();
        if host.ends_with('/') {
            host.pop();
        }
        Self(host)
    }

    /// Assembles the URL to fetch recipes.
    pub fn recipes_url(&self) -> String {
        format!("{}/apps/cookbook/api/v1/recipes", self.0)
    }

    /// Assembles the URL to fetch the details of a recipe.
    pub fn recipe_url(&self, id: &str) -> String {
        format!("{}/apps/cookbook/api/v1/recipes/{id}", self.0)
    }

    /// Assembles the URL to fetch the main image of a recipes.
    pub fn recipe_image_url(&self, id: &str) -> String {
        format!("{}/apps/cookbook/api/v1/recipes/{id}/image", self.0)
    }
}
