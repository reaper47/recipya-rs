use schema_org::Recipe;

use crate::Result;

pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub trait RecipeClient: Clone + Send + Sync {
    /// Establishes a connection to the host using the provided credentials.
    /// Replaces the client's state with the new authenticated connection.
    async fn login(&mut self, credentials: Credentials) -> Result<()>;

    /// Fetches recipes from connected host.
    async fn fetch_recipes(&self) -> Result<Vec<Recipe>>;

    /// Logs out of the connected host.
    async fn logout(&mut self) -> Result<()>;
}
