use uuid::Uuid;

#[derive(Clone)]
pub struct Host(String);

impl Host {
    pub fn new(host: impl Into<String>) -> Self {
        let mut host = host.into();
        if host.ends_with('/') {
            host.pop();
        }
        Self(host)
    }

    pub fn login_url(&self) -> String {
        format!("{}/api/auth/token", self.0)
    }

    pub fn logout_url(&self) -> String {
        format!("{}/api/auth/logout", self.0)
    }

    pub fn recipe_image_url(&self, recipe_id: Uuid) -> String {
        format!(
            "{}/api/media/recipes/{recipe_id}/images/original.webp",
            self.0
        )
    }

    pub fn recipe_url(&self, recipe_id: Uuid) -> String {
        format!("{}/api/recipes/{recipe_id}", self.0)
    }

    pub fn recipes_url(&self, page: String) -> String {
        format!("{}/api/recipes?page={page}", self.0)
    }

    pub fn user_url(&self, user_id: Uuid) -> String {
        format!("{}/api/admin/users/{user_id}", self.0)
    }
}
