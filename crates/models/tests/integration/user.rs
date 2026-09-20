use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use models::Recipe;
use models::settings::UserSettingDetails;
use uuid::Uuid;

use models::recipe::structs::recipe::Keyword;
use models::{recipe::structs::recipe::Category, user::User};
use repository::schema;
use test_db::default_config;
use test_fixtures::{TEST_USER_EMAIL, insert_other_user, insert_user};
use test_harness::{build_server_logged_in, create_app_state};

use crate::recipe::utils::a_complete_recipe_for_create;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

mod test_all {
    use super::*;

    #[tokio::test]
    async fn test_all_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user1 = insert_user(&state.mm).await?;
        let user2 = insert_other_user(&state.mm, "slava@ukraini.ua").await?;

        let got = User::all(&state.mm)
            .await?
            .into_iter()
            .map(|u| u.email)
            .collect::<Vec<_>>();

        pretty_assertions::assert_eq!(got, vec![user1.email, user2.email]);
        Ok(())
    }
}

mod test_categories {
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
        insert_user(&state.mm).await?;
        insert_other_user(&state.mm, "slava@ukraini.ua").await?;

        let num_users = User::num_users(&state.mm).await?;

        pretty_assertions::assert_eq!(2, num_users);
        Ok(())
    }
}

mod test_keywords {
    use super::*;

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

    insert_user(&state.mm).await?;

    let user = User::get_user_by_email(&state.mm, TEST_USER_EMAIL)
        .await?
        .expect("test user should have been present");
    pretty_assertions::assert_eq!(user.email, TEST_USER_EMAIL);
    Ok(())
}

#[tokio::test]
async fn test_user_new_already_exists_err() -> Result<()> {
    let state = create_app_state(default_config()).await;
    insert_user(&state.mm).await?;

    let res = insert_user(&state.mm).await;

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
    let user = insert_user(&state.mm).await?;

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
    let user = insert_user(&state.mm).await?;

    let got_user = User::get_user_by_id(&state.mm, user.id)
        .await?
        .expect("user should have been present");

    pretty_assertions::assert_eq!(user.email, got_user.email);
    Ok(())
}

#[tokio::test]
async fn test_get_user_auth_by_email_ok() -> Result<()> {
    let state = create_app_state(default_config()).await;
    let user = insert_user(&state.mm).await?;

    let got_user = User::get_user_auth_by_email(&state.mm, TEST_USER_EMAIL)
        .await?
        .expect("user should have been present");

    pretty_assertions::assert_eq!(user.email, got_user.email);
    Ok(())
}

#[tokio::test]
async fn test_update_paper_size_ok() -> Result<()> {
    let state = create_app_state(default_config()).await;
    let user = insert_user(&state.mm).await?;

    user.update_paper_size(&state.mm, 2).await?;

    let got = UserSettingDetails::get(&state.mm, user.id).await?;
    assert_eq!(got.paper_size_id, 2);
    Ok(())
}

#[tokio::test]
async fn test_update_password_ok() -> Result<()> {
    let state = create_app_state(default_config()).await;
    let user = insert_user(&state.mm).await?;
    let password_before = String::from(&user.password_hash);

    user.update_password(&state.mm, "new password").await?;

    let password_after = User::get_user_by_id(&state.mm, user.id)
        .await?
        .expect("user should have been present")
        .password_hash;
    pretty_assertions::assert_ne!(password_before, password_after);
    Ok(())
}

#[tokio::test]
async fn test_update_password_by_user_id_ok() -> Result<()> {
    let state = create_app_state(default_config()).await;
    let user = insert_user(&state.mm).await?;
    let password_before = user.password_hash;

    User::update_password_by_user_id(&state.mm, user.id, "new password").await?;

    let user = User::get_user_by_id(&state.mm, user.id)
        .await?
        .expect("user should have been present");
    let password_after = user.password_hash;
    pretty_assertions::assert_ne!(password_before, password_after);
    Ok(())
}

#[tokio::test]
async fn test_update_remember_me_ok() -> Result<()> {
    let state = create_app_state(default_config()).await;
    let user = insert_user(&state.mm).await?;

    User::update_remember_me(&state.mm, user.id, true).await?;

    let user = User::get_user_by_id(&state.mm, user.id)
        .await?
        .expect("user should have been present");
    pretty_assertions::assert_eq!(user.is_remember_me, true);
    Ok(())
}
