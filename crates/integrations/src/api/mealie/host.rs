use uuid::Uuid;

#[derive(Clone)]
pub struct Host(String);

impl Host {
    /// Creates a new host instance.
    pub fn new(host: impl Into<String>) -> Self {
        let mut host = host.into();
        if host.ends_with('/') {
            host.pop();
        }
        Self(host)
    }

    /// Assembles the login URL.
    pub fn login_url(&self) -> String {
        format!("{}/api/auth/token", self.0)
    }

    /// Assembles the logout URL.
    pub fn logout_url(&self) -> String {
        format!("{}/api/auth/logout", self.0)
    }

    /// Assembles the URL to fetch a recipe image.
    pub fn recipe_image_url(&self, recipe_id: Uuid) -> String {
        format!(
            "{}/api/media/recipes/{recipe_id}/images/original.webp",
            self.0
        )
    }

    /// Assembles the URL to fetch a recipe.
    pub fn recipe_url(&self, recipe_id: Uuid) -> String {
        format!("{}/api/recipes/{recipe_id}", self.0)
    }

    /// Assembles the URL to fetch recipes.
    pub fn recipes_url(&self, page: String) -> String {
        format!("{}/api/recipes?page={page}", self.0)
    }

    /// Assembles the URL to fetch a user.
    pub fn user_url(&self, user_id: Uuid) -> String {
        format!("{}/api/admin/users/{user_id}", self.0)
    }
}

impl AsRef<str> for Host {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_host_new() {
        let host = Host::new("https://example.com");

        assert_eq!(host.as_ref(), "https://example.com");
    }

    #[test]
    fn test_host_login_url() {
        let got = Host::new("https://example.com").login_url();

        assert_eq!(got, "https://example.com/api/auth/token");
    }

    #[test]
    fn test_host_logout_url() {
        let got = Host::new("https://example.com").logout_url();

        assert_eq!(got, "https://example.com/api/auth/logout");
    }

    #[test]
    fn test_host_recipe_image_url() {
        let host = Host::new("https://example.com");
        let recipe_id = Uuid::new_v4();

        let got = host.recipe_image_url(recipe_id);

        assert_eq!(
            got,
            format!(
                "https://example.com/api/media/recipes/{}/images/original.webp",
                recipe_id
            )
        );
    }

    #[test]
    fn test_host_recipe_url() {
        let host = Host::new("https://example.com");
        let recipe_id = Uuid::new_v4();

        let got = host.recipe_url(recipe_id);

        assert_eq!(
            got,
            format!("https://example.com/api/recipes/{}", recipe_id)
        );
    }

    #[test]
    fn test_host_recipes_url() {
        let got = Host::new("https://example.com").recipes_url("1".to_string());

        assert_eq!(got, "https://example.com/api/recipes?page=1");
    }

    #[test]
    fn test_host_user_url() {
        let host = Host::new("https://example.com");
        let user_id = Uuid::new_v4();

        let got = host.user_url(user_id);

        assert_eq!(
            got,
            format!("https://example.com/api/admin/users/{}", user_id)
        );
    }
}
