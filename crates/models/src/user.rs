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
pub(crate) struct UserCategory {
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

#[cfg(test)]
mod tests {
    use crate::{
        recipe::structs::test_utils::a_complete_recipe_for_create, settings::UserSettingDetails,
    };

    use super::*;

    use test_db::default_config;
    use test_utils::{
        TEST_USER_EMAIL, build_server_logged_in, create_app_state, insert_other_user, insert_user,
    };

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod test_all {
        use super::*;

        #[tokio::test]
        async fn test_all_ok() -> Result<()> {
            let state = create_app_state(default_config()).await;
            let user1 = insert_user(&state).await?;
            let user2 = insert_other_user(&state, "slava@ukraini.ua").await?;

            let got = User::all(&state.mm)
                .await?
                .iter()
                .map(|u| u.email.clone())
                .collect::<Vec<_>>();

            pretty_assertions::assert_eq!(got, vec![user1.email, user2.email]);
            Ok(())
        }
    }

    mod test_categories {
        use diesel_async::RunQueryDsl;

        use schema;
        use test_utils::build_server_logged_in;

        use super::*;

        #[derive(Insertable)]
        #[diesel(table_name = schema::users_categories)]
        struct NewUserCategory {
            user_id: Uuid,
            category_id: i64,
        }

        #[tokio::test]
        async fn test_user_categories_ok() -> Result<()> {
            let (_, state) = build_server_logged_in(default_config()).await?;
            let user_id = User::all(&state.mm).await?[0].id;
            let (id, _name) = diesel::insert_into(schema::categories::table)
                .values(schema::categories::name.eq("date"))
                .get_result::<(i64, String)>(&mut state.mm.pool.get().await?)
                .await?;
            diesel::insert_into(schema::users_categories::table)
                .values(&NewUserCategory {
                    category_id: id,
                    user_id,
                })
                .execute(&mut state.mm.pool.get().await?)
                .await?;

            let categories = User::categories(&state.mm, user_id).await?;

            let want = vec![
                Category {
                    id: categories
                        .iter()
                        .find(|c| c.name == "uncategorized")
                        .unwrap()
                        .id,
                    name: "uncategorized".into(),
                },
                Category {
                    id: categories
                        .iter()
                        .find(|c| c.name == "appetizers")
                        .unwrap()
                        .id,
                    name: "appetizers".into(),
                },
                Category {
                    id: categories.iter().find(|c| c.name == "bread").unwrap().id,
                    name: "bread".into(),
                },
                Category {
                    id: categories
                        .iter()
                        .find(|c| c.name == "breakfasts")
                        .unwrap()
                        .id,
                    name: "breakfasts".into(),
                },
                Category {
                    id: categories
                        .iter()
                        .find(|c| c.name == "condiments")
                        .unwrap()
                        .id,
                    name: "condiments".into(),
                },
                Category {
                    id: categories.iter().find(|c| c.name == "dessert").unwrap().id,
                    name: "dessert".into(),
                },
                Category {
                    id: categories.iter().find(|c| c.name == "lunch").unwrap().id,
                    name: "lunch".into(),
                },
                Category {
                    id: categories
                        .iter()
                        .find(|c| c.name == "main dish")
                        .unwrap()
                        .id,
                    name: "main dish".into(),
                },
                Category {
                    id: categories.iter().find(|c| c.name == "salad").unwrap().id,
                    name: "salad".into(),
                },
                Category {
                    id: categories
                        .iter()
                        .find(|c| c.name == "side dish")
                        .unwrap()
                        .id,
                    name: "side dish".into(),
                },
                Category {
                    id: categories.iter().find(|c| c.name == "snacks").unwrap().id,
                    name: "snacks".into(),
                },
                Category {
                    id: categories.iter().find(|c| c.name == "soups").unwrap().id,
                    name: "soups".into(),
                },
                Category {
                    id: categories.iter().find(|c| c.name == "stews").unwrap().id,
                    name: "stews".into(),
                },
                Category {
                    id: categories.iter().find(|c| c.name == "date").unwrap().id,
                    name: "date".into(),
                },
            ];
            pretty_assertions::assert_eq!(want, categories);
            Ok(())
        }
    }

    mod test_count {
        use super::*;

        #[tokio::test]
        async fn test_num_users() -> Result<()> {
            let state = create_app_state(default_config()).await;
            insert_user(&state).await?;
            insert_other_user(&state, "slava@ukraini.ua").await?;

            let num_users = User::num_users(&state.mm).await?;

            pretty_assertions::assert_eq!(2, num_users);
            Ok(())
        }
    }

    mod test_keywords {
        use diesel_async::RunQueryDsl;

        use super::*;
        use test_utils::build_server_logged_in;

        #[derive(Insertable)]
        #[diesel(table_name = schema::users_keywords)]
        struct NewUserKeyword {
            user_id: Uuid,
            keyword_id: i64,
        }

        #[tokio::test]
        #[allow(clippy::cast_possible_wrap)]
        async fn test_keywords_ok() -> Result<()> {
            let (_, state) = build_server_logged_in(default_config()).await?;
            let user_id = User::all(&state.mm).await?[0].id;
            let want_keywords = ["main:cheeses", "main:meat"];
            for kw in want_keywords {
                let (id, _name) = diesel::insert_into(schema::keywords::table)
                    .values(schema::keywords::name.eq(kw))
                    .get_result::<(i64, String)>(&mut state.mm.pool.get().await?)
                    .await?;
                diesel::insert_into(schema::users_keywords::table)
                    .values(&NewUserKeyword {
                        keyword_id: id,
                        user_id,
                    })
                    .execute(&mut state.mm.pool.get().await?)
                    .await?;
            }

            let keywords = User::keywords(&state.mm, user_id).await?;

            let want = want_keywords
                .into_iter()
                .map(|name| Keyword {
                    id: keywords
                        .iter()
                        .find(|kw| kw.name == name)
                        .map_or(1, |kw| kw.id),
                    name: name.to_string(),
                })
                .collect::<Vec<_>>();
            pretty_assertions::assert_eq!(want, keywords);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_user_new_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;

        insert_user(&state).await?;

        let user = User::get_user_by_email(&state.mm, TEST_USER_EMAIL)
            .await?
            .expect("Test user should have been present");
        pretty_assertions::assert_eq!(user.email, TEST_USER_EMAIL);
        Ok(())
    }

    #[tokio::test]
    async fn test_user_new_already_exists_err() -> Result<()> {
        let state = create_app_state(default_config()).await;
        insert_user(&state).await?;

        let res = insert_user(&state).await;

        assert!(res.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_user_not_exist_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user_id = Uuid::new_v4();

        match User::delete(&state.mm, user_id).await {
            Ok(()) => Err("An error was supposed to be thrown".into()),
            Err(_) => Ok(()),
        }
    }

    #[tokio::test]
    async fn test_delete_user_exists_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state).await?;

        User::delete(&state.mm, user.id).await?;

        match User::get_user_by_email(&state.mm, TEST_USER_EMAIL).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    #[tokio::test]
    async fn test_favourite_recipes() -> Result<()> {
        let (_, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (recipe1, _) = a_complete_recipe_for_create();
        let (mut recipe2, _) = a_complete_recipe_for_create();
        recipe2.name = "recipe 2".into();
        recipe2.is_favourite = true;
        let (mut recipe3, _) = a_complete_recipe_for_create();
        recipe3.name = "recipe 3".into();
        recipe3.is_favourite = true;
        let _ = Recipe::create(&state.mm, user_id, &recipe1, &settings).await?;
        let _ = Recipe::create(&state.mm, user_id, &recipe2, &settings).await?;
        let _ = Recipe::create(&state.mm, user_id, &recipe3, &settings).await?;

        let got = User::favourite_recipes(&state.mm, user_id).await?;

        let mut got = got.into_iter().map(|r| r.name).collect::<Vec<_>>();
        let mut expected = vec![recipe2, recipe3]
            .into_iter()
            .map(|r| r.name)
            .collect::<Vec<_>>();
        got.sort();
        expected.sort();
        pretty_assertions::assert_eq!(got, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_user_by_id_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state).await?;

        let got_user = User::get_user_by_id(&state.mm, user.id)
            .await?
            .expect("User should have been present");

        pretty_assertions::assert_eq!(user.email, got_user.email);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_user_auth_by_email_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state).await?;

        let got_user = User::get_user_auth_by_email(&state.mm, TEST_USER_EMAIL)
            .await?
            .expect("User should have been present");

        pretty_assertions::assert_eq!(user.email, got_user.email);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_paper_size_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state).await?;

        user.update_paper_size(&state.mm, 2).await?;

        let got = UserSettingDetails::get(&state.mm, user.id).await?;
        assert_eq!(got.paper_size_id, 2);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_password_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state).await?;
        let password_before = String::from(&user.password_hash);

        user.update_password(&state.mm, "new password").await?;

        let password_after = User::get_user_by_id(&state.mm, user.id)
            .await?
            .expect("User should have been present")
            .password_hash;
        pretty_assertions::assert_ne!(password_before, password_after);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_password_by_user_id_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state).await?;
        let password_before = user.password_hash;

        User::update_password_by_user_id(&state.mm, user.id, "new password").await?;

        let user = User::get_user_by_id(&state.mm, user.id)
            .await?
            .expect("User should have been present");
        let password_after = user.password_hash;
        pretty_assertions::assert_ne!(password_before, password_after);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_remember_me_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state).await?;

        User::update_remember_me(&state.mm, user.id, true).await?;

        let user = User::get_user_by_id(&state.mm, user.id)
            .await?
            .expect("User should have been present");
        pretty_assertions::assert_eq!(user.is_remember_me, true);
        Ok(())
    }
}
