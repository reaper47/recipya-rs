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

    /// Assembles the URL to fetch a login token.
    pub fn login_url(&self) -> String {
        format!("{}/api-token-auth/", self.0)
    }

    /// Assembles the URL to fetch recipes.
    pub fn recipes_url(&self) -> String {
        format!("{}/api/recipe/", self.0)
    }

    /// Assembles the URL to fetch a recipe.
    pub fn recipe_url(&self, id: u32) -> String {
        format!("{}/api/recipe/{}/", self.0, id)
    }
}
