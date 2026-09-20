use diesel::dsl::count_star;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use time_tz::timezones;
use uuid::Uuid;

use auth::pwd::{ContentToHash, hash_pwd};
use repository::{ModelManager, schema};

use crate::{
    Error, Recipe, Result,
    recipe::structs::recipe::{Category, Keyword},
};

/// Represents a user in the system.
#[derive(Clone, Debug, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub is_remember_me: bool,
    pub is_email_verified: bool,
    pub is_admin: bool,

    pub password_hash: String,
    pub password_salt: Uuid,
    pub token_salt: Uuid,
}

/// A struct representing a user for login purposes.
pub struct UserForLogin<'a> {
    pub id: Uuid,
    pub email: &'a str,
    pub password_hash: &'a str,
    pub password_salt: Uuid,
    pub token_salt: Uuid,
}

/// A struct for creating a new user from client-provided data.
#[derive(Deserialize)]
pub struct UserForCreate {
    pub email: String,
    pub password_clear: String,
}

/// A struct for inserting a new user into the database.
#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub(crate) struct UserForInsert {
    pub email: String,
    pub password_hash: String,
    pub password_salt: Uuid,
    pub is_admin: bool,
}

/// Represents an entry in the user categories table.
#[derive(Identifiable, Insertable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Category))]
#[diesel(table_name = schema::users_categories)]
#[diesel(primary_key(user_id, category_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserCategory {
    pub user_id: Uuid,
    pub category_id: i64,
}

/// Represents an entry in the user keywords table.
#[derive(Identifiable, Insertable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Keyword))]
#[diesel(table_name = schema::users_keywords)]
#[diesel(primary_key(user_id, keyword_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct UserKeyword {
    pub user_id: Uuid,
    pub keyword_id: i64,
}

/// A struct representing a user with authentication-related data.
pub struct UserForAuth {
    pub id: Uuid,
    pub email: String,
    pub token_salt: Uuid,
    pub is_remember_me: bool,
    pub is_admin: bool,
}

impl User {
    /// Retrieves all of the users in the database.
    pub async fn all(mm: &ModelManager) -> Result<Vec<Self>> {
        Ok(schema::users::table
            .select(Self::as_select())
            .load::<Self>(&mut mm.pool.get().await?)
            .await?)
    }

    /// Retrieves all of a user's recipe categories.
    pub async fn categories(mm: &ModelManager, user_id: Uuid) -> Result<Vec<Category>> {
        let mut conn = mm.pool.get().await?;

        let user = schema::users::table
            .filter(schema::users::id.eq(user_id))
            .select(Self::as_select())
            .get_result(&mut conn)
            .await?;

        let categories = UserCategory::belonging_to(&user)
            .inner_join(schema::categories::table)
            .select(Category::as_select())
            .load(&mut conn)
            .await?;

        Ok(categories)
    }

    /// Deletes a user from the database.
    pub async fn delete(mm: &ModelManager, user_id: Uuid) -> Result<()> {
        use schema::users::dsl::{id, users};

        let num_deleted = diesel::delete(users.filter(id.eq(user_id)))
            .execute(&mut mm.pool.get().await?)
            .await?;

        match num_deleted {
            0 => Err(Error::EntityNotFound {
                entity: "user",
                id: user_id.to_string(),
            }),
            _ => Ok(()),
        }
    }

    /// Finds a user by their email address.
    pub async fn get_user_by_email(mm: &ModelManager, user_email: &str) -> Result<Option<Self>> {
        use schema::users::dsl::{email, users};

        let user = users
            .filter(email.eq(user_email))
            .select(Self::as_select())
            .first::<Self>(&mut mm.pool.get().await?)
            .await
            .optional()?;

        Ok(user)
    }

    /// Finds a user by their user ID.
    pub async fn get_user_by_id(mm: &ModelManager, user_id: Uuid) -> Result<Option<Self>> {
        use schema::users::dsl::{id, users};

        let user = users
            .filter(id.eq(user_id))
            .select(Self::as_select())
            .first::<Self>(&mut mm.pool.get().await?)
            .await
            .optional()?;

        Ok(user)
    }

    /// Finds user authentication data by email.
    pub async fn get_user_auth_by_email(
        mm: &ModelManager,
        user_email: &str,
    ) -> Result<Option<UserForAuth>> {
        match Self::get_user_by_email(mm, user_email).await? {
            Some(user) => Ok(Some(UserForAuth {
                id: user.id,
                email: user.email,
                token_salt: user.token_salt,
                is_remember_me: user.is_remember_me,
                is_admin: user.is_admin,
            })),
            None => Err(Error::EntityNotFound {
                id: "-1".into(),
                entity: "user for auth",
            }),
        }
    }

    /// Finds the first user authentication data by email.
    pub async fn get_first_admin(mm: &ModelManager) -> Result<Option<Self>> {
        let user = schema::users::table
            .filter(schema::users::is_admin.eq(true))
            .select(Self::as_select())
            .order(schema::users::id.asc())
            .first::<Self>(&mut mm.pool.get().await?)
            .await
            .optional()?;

        Ok(user)
    }

    /// Fetches all of the user's favourite recipes.
    pub async fn favourite_recipes(mm: &ModelManager, user_id: Uuid) -> Result<Vec<Recipe>> {
        let recipes = schema::recipes::table
            .filter(schema::recipes::user_id.eq(user_id))
            .filter(schema::recipes::is_favourite.eq(true))
            .select(Recipe::as_select())
            .load::<Recipe>(&mut mm.pool.get().await?)
            .await?;

        Ok(recipes)
    }

    /// Retrieves all of a user's recipe keywords.
    pub async fn keywords(mm: &ModelManager, user_id: Uuid) -> Result<Vec<Keyword>> {
        let mut conn = mm.pool.get().await?;

        let user = schema::users::table
            .filter(schema::users::id.eq(user_id))
            .select(Self::as_select())
            .get_result(&mut conn)
            .await?;

        let keywords = UserKeyword::belonging_to(&user)
            .inner_join(schema::keywords::table)
            .select(Keyword::as_select())
            .load(&mut conn)
            .await?;

        Ok(keywords)
    }

    /// Creates a new user from the provided user creation data.
    pub async fn new(mm: &ModelManager, user_c: UserForCreate) -> Result<Self> {
        let salt = Uuid::new_v4();
        let hash = hash_pwd(ContentToHash {
            content: user_c.password_clear.clone(),
            salt,
        })
        .await?;

        Self::new_with_hash(mm, user_c, salt, hash).await
    }

    /// Creates a new user given the password salt and hash.
    ///
    /// The function is hidden as it is only meant to be used in tests for fixtures.
    #[doc(hidden)]
    pub async fn new_with_hash(
        mm: &ModelManager,
        user_c: UserForCreate,
        password_salt: Uuid,
        password_hash: String,
    ) -> Result<Self> {
        use schema::users::dsl::users;

        let user = diesel::insert_into(users)
            .values(&UserForInsert {
                email: user_c.email.clone(),
                password_hash,
                password_salt,
                is_admin: Self::all(mm).await?.is_empty(),
            })
            .returning(Self::as_returning())
            .get_result(&mut mm.pool.get().await?)
            .await?;

        Ok(user)
    }

    /// Fetches the number of users in the database.
    pub async fn num_users(mm: &ModelManager) -> Result<i64> {
        let count = schema::users::table
            .select(count_star())
            .first::<i64>(&mut mm.pool.get().await?)
            .await?;

        Ok(count)
    }

    /// Updates the user's preference on whether to bold ingredients in instructions.
    pub async fn update_bold_ingredients(
        &self,
        mm: &ModelManager,
        new_bold_ingredients: bool,
    ) -> Result<()> {
        use schema::user_settings::dsl::{bold_ingredients, user_id, user_settings};

        diesel::update(user_settings.filter(user_id.eq(self.id)))
            .set(bold_ingredients.eq(new_bold_ingredients))
            .execute(&mut mm.pool.get().await?)
            .await?;

        Ok(())
    }

    /// Updates the user's paper size.
    pub async fn update_paper_size(&self, mm: &ModelManager, new_paper_size_id: i16) -> Result<()> {
        use schema::user_settings::dsl::{paper_size_id, user_id, user_settings};

        diesel::update(user_settings.filter(user_id.eq(self.id)))
            .set(paper_size_id.eq(new_paper_size_id))
            .execute(&mut mm.pool.get().await?)
            .await?;

        Ok(())
    }

    /// Updates the user's password.
    pub async fn update_password(&self, mm: &ModelManager, password_clear: &str) -> Result<()> {
        use schema::users::dsl::{id, password_hash, users};

        let hashed_password = hash_pwd(ContentToHash {
            content: password_clear.into(),
            salt: self.password_salt,
        })
        .await?;

        diesel::update(users.find(id))
            .set(password_hash.eq(hashed_password))
            .execute(&mut mm.pool.get().await?)
            .await?;

        Ok(())
    }

    /// Updates the user password for the specified user ID.
    pub async fn update_password_by_user_id(
        mm: &ModelManager,
        id: Uuid,
        new_password: &str,
    ) -> Result<()> {
        match Self::get_user_by_id(mm, id).await? {
            Some(user) => user.update_password(mm, new_password).await,
            None => Err(Error::EntityNotFound {
                id: "-1".into(),
                entity: "user for auth",
            }),
        }
    }

    /// Updates the "remember me" flag for the user.
    pub async fn update_remember_me(
        mm: &ModelManager,
        user_id: Uuid,
        new_value: bool,
    ) -> Result<()> {
        use schema::users::dsl::{is_remember_me, users};

        diesel::update(users.find(user_id))
            .set(is_remember_me.eq(new_value))
            .execute(&mut mm.pool.get().await?)
            .await?;

        Ok(())
    }

    pub async fn update_timezone(&self, mm: &ModelManager, new_tz: &str) -> Result<()> {
        use schema::user_settings::dsl::{timezone, user_id, user_settings};

        if timezones::get_by_name(new_tz).is_none() {
            return Err(Error::Time);
        }

        diesel::update(user_settings.filter(user_id.eq(self.id)))
            .set(timezone.eq(new_tz))
            .execute(&mut mm.pool.get().await?)
            .await?;

        Ok(())
    }
}
