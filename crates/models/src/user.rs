use diesel::dsl::count_star;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
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
        let all_users = schema::users::table
            .select(Self::as_select())
            .load::<Self>(&mut mm.pool.get().await?)
            .await?;

        Ok(all_users)
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
    pub async fn get_user_by_email(
        mm: &ModelManager,
        user_email: impl Into<String>,
    ) -> Result<Option<Self>> {
        use schema::users::dsl::{email, users};

        let user = users
            .filter(email.eq(user_email.into()))
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
        user_email: impl Into<String>,
    ) -> Result<Option<UserForAuth>> {
        match Self::get_user_by_email(mm, user_email.into()).await? {
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
        use schema::users::dsl::users;

        let new_password_salt = Uuid::new_v4();
        let new_password = hash_pwd(ContentToHash {
            content: user_c.password_clear,
            salt: new_password_salt,
        })
        .await?;

        let user = diesel::insert_into(users)
            .values(&UserForInsert {
                email: user_c.email.clone(),
                password_hash: new_password,
                password_salt: new_password_salt,
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

    /// Updates the user's paper size.
    pub async fn update_paper_size(&self, mm: &ModelManager, new_paper_size_id: i16) -> Result<()> {
        use schema::user_settings::dsl::*;

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
}

#[cfg(test)]
mod tests {
    use crate::{
        recipe::structs::test_utils::a_complete_recipe_for_create, settings::UserSettingDetails,
    };

    use super::*;

    use test_db::TestDb;
    use test_utils::{
        TEST_USER_EMAIL, build_server_logged_in, create_app_state, insert_other_user, insert_user,
    };

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod test_all {
        use super::*;

        #[tokio::test]
        async fn test_all_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let user1 = insert_user(config.clone()).await?;
            let user2 = insert_other_user(config.clone(), "slava@ukraini.ua").await?;

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
        use super::*;

        use diesel_async::RunQueryDsl;

        use schema;
        use test_utils::{build_server_logged_in, create_app_state};

        #[derive(Insertable)]
        #[diesel(table_name = schema::users_categories)]
        struct NewUserCategory {
            user_id: Uuid,
            category_id: i64,
        }

        #[tokio::test]
        async fn test_categories_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let _ = build_server_logged_in(config.clone()).await?;
            let user_id = User::all(&state.mm).await?[0].id;
            let mut conn = state.mm.pool.get().await?;
            let (id, _name) = diesel::insert_into(schema::categories::table)
                .values(schema::categories::name.eq("date"))
                .get_result::<(i64, String)>(&mut conn)
                .await?;
            diesel::insert_into(schema::users_categories::table)
                .values(&NewUserCategory {
                    category_id: id,
                    user_id,
                })
                .execute(&mut conn)
                .await?;

            let categories = User::categories(&state.mm, user_id).await?;

            let want = vec![
                Category {
                    id: 1,
                    name: "uncategorized".into(),
                },
                Category {
                    id: 2,
                    name: "appetizers".into(),
                },
                Category {
                    id: 3,
                    name: "bread".into(),
                },
                Category {
                    id: 4,
                    name: "breakfasts".into(),
                },
                Category {
                    id: 5,
                    name: "condiments".into(),
                },
                Category {
                    id: 6,
                    name: "dessert".into(),
                },
                Category {
                    id: 7,
                    name: "lunch".into(),
                },
                Category {
                    id: 8,
                    name: "main dish".into(),
                },
                Category {
                    id: 9,
                    name: "salad".into(),
                },
                Category {
                    id: 10,
                    name: "side dish".into(),
                },
                Category {
                    id: 11,
                    name: "snacks".into(),
                },
                Category {
                    id: 12,
                    name: "soups".into(),
                },
                Category {
                    id: 13,
                    name: "stews".into(),
                },
                Category {
                    id: 14,
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
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            insert_user(config.clone()).await?;
            insert_other_user(config.clone(), "slava@ukraini.ua").await?;

            let num_users = User::num_users(&state.mm).await?;

            pretty_assertions::assert_eq!(2, num_users);
            Ok(())
        }
    }

    mod test_keywords {
        use diesel_async::RunQueryDsl;

        use super::*;
        use test_utils::{build_server_logged_in, create_app_state};

        #[derive(Insertable)]
        #[diesel(table_name = schema::users_keywords)]
        struct NewUserKeyword {
            user_id: Uuid,
            keyword_id: i64,
        }

        #[tokio::test]
        #[allow(clippy::cast_possible_wrap)]
        async fn test_keywords_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let _ = build_server_logged_in(config.clone()).await?;
            let user_id = User::all(&state.mm).await?[0].id;
            let mut conn = state.mm.pool.get().await?;
            let want_keywords = ["main:cheeses", "main:meat"];
            for kw in want_keywords {
                let (id, _name) = diesel::insert_into(schema::keywords::table)
                    .values(schema::keywords::name.eq(kw))
                    .get_result::<(i64, String)>(&mut conn)
                    .await?;
                diesel::insert_into(schema::users_keywords::table)
                    .values(&NewUserKeyword {
                        keyword_id: id,
                        user_id,
                    })
                    .execute(&mut conn)
                    .await?;
            }

            let keywords = User::keywords(&state.mm, user_id).await?;

            let want = want_keywords
                .into_iter()
                .enumerate()
                .map(|(id, name)| Keyword {
                    id: id as i64 + 1,
                    name: name.to_string(),
                })
                .collect::<Vec<_>>();
            pretty_assertions::assert_eq!(want, keywords);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_user_new_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;

        insert_user(config.clone()).await?;

        let user = User::get_user_by_email(&state.mm, TEST_USER_EMAIL)
            .await?
            .expect("Test user should have been present");
        pretty_assertions::assert_eq!(user.email, TEST_USER_EMAIL);
        Ok(())
    }

    #[tokio::test]
    async fn test_user_new_already_exists_err() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        insert_user(config.clone()).await?;

        let res = insert_user(config.clone()).await;

        assert!(res.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_user_not_exist_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let user_id = Uuid::new_v4();

        match User::delete(&state.mm, user_id).await {
            Ok(()) => Err("An error was supposed to be thrown".into()),
            Err(_) => Ok(()),
        }
    }

    #[tokio::test]
    async fn test_delete_user_exists_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let user = insert_user(config.clone()).await?;

        User::delete(&state.mm, user.id).await?;

        match User::get_user_by_email(&state.mm, TEST_USER_EMAIL).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    #[tokio::test]
    async fn test_favourite_recipes() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let _ = build_server_logged_in(config.clone()).await?;
        let state = create_app_state(config.clone()).await;
        let user_id = User::all(&state.mm).await?[0].id;
        let (recipe1, _) = a_complete_recipe_for_create();
        let (mut recipe2, _) = a_complete_recipe_for_create();
        recipe2.name = "recipe 2".into();
        recipe2.is_favourite = true;
        let (mut recipe3, _) = a_complete_recipe_for_create();
        recipe3.name = "recipe 3".into();
        recipe3.is_favourite = true;
        let _ = Recipe::create(&state.mm, user_id, &recipe1).await?;
        let _ = Recipe::create(&state.mm, user_id, &recipe2).await?;
        let _ = Recipe::create(&state.mm, user_id, &recipe3).await?;

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
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let user = insert_user(config.clone()).await?;

        let got_user = User::get_user_by_id(&state.mm, user.id)
            .await?
            .expect("User should have been present");

        pretty_assertions::assert_eq!(user.email, got_user.email);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_user_auth_by_email_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let user = insert_user(config.clone()).await?;

        let got_user = User::get_user_auth_by_email(&state.mm, TEST_USER_EMAIL)
            .await?
            .expect("User should have been present");

        pretty_assertions::assert_eq!(user.email, got_user.email);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_paper_size_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let user = insert_user(config.clone()).await?;

        user.update_paper_size(&state.mm, 2).await?;

        let got = UserSettingDetails::get(&state.mm, user.id).await?;
        assert_eq!(got.paper_size_id, 2);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_password_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let user = insert_user(config.clone()).await?;
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
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let user = insert_user(config.clone()).await?;
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
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let user = insert_user(config.clone()).await?;

        User::update_remember_me(&state.mm, user.id, true).await?;

        let user = User::get_user_by_id(&state.mm, user.id)
            .await?
            .expect("User should have been present");
        pretty_assertions::assert_eq!(user.is_remember_me, true);
        Ok(())
    }
}
