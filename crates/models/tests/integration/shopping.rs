use std::assert_matches;

use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use app::state::AppState;
use models::shopping::ShareShoppingList;
use models::user::UserForCreate;
use models::{
    Error, Recipe,
    settings::UserSettingDetails,
    shopping::{
        ShoppingList, ShoppingListDetails, ShoppingListItemDetails, ShoppingListItemForCreate,
        ShoppingListItemForUpdate, ShoppingListRecipeDetails,
    },
    user::User,
};
use repository::{ModelManager, schema};
use test_db::default_config;
use test_harness::{build_server_anonymous, create_app_state};

use crate::recipe::utils::a_complete_recipe_for_create;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

fn a_meat_item() -> ShoppingListItemForCreate {
    ShoppingListItemForCreate::new(
        Some("1 cup".into()),
        "chicken",
        Some("Meat".into()),
        Some("new notes".into()),
        None,
    )
}

fn other_meat_item() -> ShoppingListItemForCreate {
    ShoppingListItemForCreate::new(
        Some("500g".into()),
        "beef",
        Some("Meat".into()),
        Some("other notes".into()),
        None,
    )
}

fn a_list_name() -> String {
    String::from("Costco")
}

fn an_item_with_recipe(recipe_id: i64) -> ShoppingListItemForCreate {
    let mut item = a_meat_item();
    item.recipe_id = Some(recipe_id);
    item
}

fn other_item_with_recipe(recipe_id: i64) -> ShoppingListItemForCreate {
    let mut item = other_meat_item();
    item.recipe_id = Some(recipe_id);
    item
}

#[tokio::test]
async fn test_get_shopping_list_by_id_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let list_id = ShoppingList::create(&state.mm, &a_list_name(), user_id).await?;

    let list = ShoppingList::get(&state.mm, list_id, user_id).await?;

    assert_eq!(list.name, a_list_name());
    Ok(())
}

#[tokio::test]
async fn test_get_all_shopping_lists_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    for i in 0..3 {
        let _ = ShoppingList::create(&state.mm, &format!("List {i}"), user_id).await?;
    }

    let lists = ShoppingList::get_all(&state.mm, user_id).await?;

    assert_eq!(lists.len(), 3);
    pretty_assertions::assert_eq!(
        lists.into_iter().map(|l| l.name).collect::<Vec<_>>(),
        vec![
            "List 0".to_string(),
            "List 1".to_string(),
            "List 2".to_string(),
        ]
    );
    Ok(())
}

#[tokio::test]
async fn test_add_item_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let list_id = ShoppingList::create(&state.mm, &a_list_name(), user_id).await?;

    let item = ShoppingList::add_item(&state.mm, list_id, a_meat_item(), user_id).await?;

    let got = ShoppingListDetails::get(&state.mm, list_id, user_id).await?;
    pretty_assertions::assert_eq!(
        got,
        ShoppingListDetails {
            id: list_id,
            name: "Costco".into(),
            items: vec![ShoppingListItemDetails {
                id: item.id,
                ingredient: "chicken".into(),
                quantity: Some("1 cup".into()),
                notes: Some("new notes".into()),
                label_id: item.label_id,
                label: "Meat".into(),
                position: 1,
                recipe: None,
                is_checked: false,
                created_at: got.items[0].created_at,
                updated_at: got.items[0].updated_at,
            }],
            created_at: got.created_at,
            updated_at: got.updated_at,
        }
    );
    let num_items = ShoppingList::items_count(&state.mm, list_id)
        .await
        .unwrap_or_default();
    assert_eq!(num_items, 1);
    Ok(())
}

#[tokio::test]
#[allow(unused)]
async fn test_add_item_to_list_that_does_not_belong_to_user_err() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let list_id = Uuid::new_v4();

    let res = ShoppingList::add_item(&state.mm, list_id, a_meat_item(), user_id).await;

    assert_matches!(res, Err(Error::EntityNotFound { .. }));
    Ok(())
}

#[tokio::test]
async fn test_duplicate_shopping_list_name_err() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let _ = ShoppingList::create(&state.mm, "wintersun", user_id).await?;

    let got_res = ShoppingList::create(&state.mm, "WINTERSUN", user_id).await;

    assert_matches!(got_res, Err(Error::Diesel(_)));
    Ok(())
}

#[tokio::test]
async fn test_duplicate_shopping_list_label_name_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let list_id = ShoppingList::create(&state.mm, &a_list_name(), user_id).await?;

    let item1 = ShoppingList::add_item(&state.mm, list_id, a_meat_item(), user_id).await?;
    let item2 = ShoppingList::add_item(&state.mm, list_id, other_meat_item(), user_id).await?;

    let got = ShoppingListDetails::get(&state.mm, list_id, user_id).await?;
    pretty_assertions::assert_eq!(
        got,
        ShoppingListDetails {
            id: list_id,
            name: "Costco".into(),
            items: vec![
                ShoppingListItemDetails {
                    id: item1.id,
                    ingredient: "chicken".into(),
                    quantity: Some("1 cup".into()),
                    notes: Some("new notes".into()),
                    label_id: item1.label_id,
                    label: "Meat".into(),
                    position: 1,
                    recipe: None,
                    is_checked: false,
                    created_at: got.items[0].created_at,
                    updated_at: got.items[0].updated_at
                },
                ShoppingListItemDetails {
                    id: item2.id,
                    ingredient: "beef".into(),
                    quantity: Some("500g".into()),
                    notes: Some("other notes".into()),
                    label_id: item2.label_id,
                    label: "Meat".into(),
                    position: 2,
                    recipe: None,
                    is_checked: false,
                    created_at: got.items[1].created_at,
                    updated_at: got.items[1].updated_at
                },
            ],
            created_at: got.created_at,
            updated_at: got.updated_at,
        },
    );
    let mut conn = state.mm.pool.get().await?;
    let count: i64 = schema::users_shopping_list_labels::table
        .count()
        .get_result(&mut conn)
        .await?;
    assert_eq!(count, 1);
    Ok(())
}

#[tokio::test]
async fn test_add_item_with_recipe_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let settings = UserSettingDetails::get(&state.mm, user_id).await?;
    let list_id = ShoppingList::create(&state.mm, &a_list_name(), user_id).await?;
    let recipe1 = a_complete_recipe_for_create().0;
    let recipe_id = Recipe::create(&state.mm, user_id, &recipe1, &settings).await?;
    let mut recipe2 = a_complete_recipe_for_create().0;
    recipe2.name = "Blueberry Pie".into();
    let recipe_id2 = Recipe::create(&state.mm, user_id, &recipe2, &settings).await?;
    let item1 = an_item_with_recipe(recipe_id);
    let item2 = other_item_with_recipe(recipe_id2);

    let item1 = ShoppingList::add_item(&state.mm, list_id, item1, user_id).await?;
    let item2 = ShoppingList::add_item(&state.mm, list_id, item2, user_id).await?;

    let got = ShoppingListDetails::get(&state.mm, list_id, user_id).await?;
    pretty_assertions::assert_eq!(
        got,
        ShoppingListDetails {
            id: list_id,
            name: "Costco".into(),
            items: vec![
                ShoppingListItemDetails {
                    id: item1.id,
                    ingredient: "chicken".into(),
                    quantity: Some("1 cup".into()),
                    notes: Some("new notes".into()),
                    label_id: item1.label_id,
                    label: "Meat".into(),
                    position: 1,
                    recipe: Some(ShoppingListRecipeDetails {
                        id: recipe_id,
                        name: recipe1.name,
                    }),
                    is_checked: false,
                    created_at: got.items[0].created_at,
                    updated_at: got.items[0].updated_at
                },
                ShoppingListItemDetails {
                    id: item2.id,
                    ingredient: "beef".into(),
                    quantity: Some("500g".into()),
                    notes: Some("other notes".into()),
                    label_id: item2.label_id,
                    label: "Meat".into(),
                    position: 2,
                    recipe: Some(ShoppingListRecipeDetails {
                        id: recipe_id2,
                        name: recipe2.name,
                    }),
                    is_checked: false,
                    created_at: got.items[1].created_at,
                    updated_at: got.items[1].updated_at
                },
            ],
            created_at: got.created_at,
            updated_at: got.updated_at,
        },
    );
    Ok(())
}

#[tokio::test]
async fn test_add_items_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let settings = UserSettingDetails::get(&state.mm, user_id).await?;
    let list_id = ShoppingList::create(&state.mm, &a_list_name(), user_id).await?;
    let list = ShoppingList::get(&state.mm, list_id, user_id).await?;
    let recipe1 = a_complete_recipe_for_create().0;
    let recipe_id = Recipe::create(&state.mm, user_id, &recipe1, &settings).await?;
    let item1 = an_item_with_recipe(recipe_id);
    let mut item2 = an_item_with_recipe(recipe_id);
    item2.ingredient = "Muffins".into();

    list.add_items_for_recipe(&state.mm, &[item1, item2], recipe_id, user_id)
        .await?;

    let got = ShoppingListDetails::get(&state.mm, list_id, user_id).await?;
    pretty_assertions::assert_eq!(
        got,
        ShoppingListDetails {
            id: list_id,
            name: "Costco".into(),
            items: vec![
                ShoppingListItemDetails {
                    id: got.items[0].id,
                    ingredient: "chicken".into(),
                    quantity: Some("1 cup".into()),
                    notes: Some("new notes".into()),
                    label_id: 1,
                    label: "No label".into(),
                    position: 1,
                    recipe: Some(ShoppingListRecipeDetails {
                        id: got.items[0].recipe.clone().unwrap().id,
                        name: recipe1.name.clone(),
                    }),
                    is_checked: false,
                    created_at: got.items[0].created_at,
                    updated_at: got.items[0].updated_at
                },
                ShoppingListItemDetails {
                    id: got.items[1].id,
                    ingredient: "Muffins".into(),
                    quantity: Some("1 cup".into()),
                    notes: Some("new notes".into()),
                    label_id: 1,
                    label: "No label".into(),
                    position: 2,
                    recipe: Some(ShoppingListRecipeDetails {
                        id: got.items[1].recipe.clone().unwrap().id,
                        name: recipe1.name,
                    }),
                    is_checked: false,
                    created_at: got.items[1].created_at,
                    updated_at: got.items[1].updated_at
                },
            ],
            created_at: got.created_at,
            updated_at: got.updated_at,
        },
    );
    Ok(())
}

#[tokio::test]
async fn test_delete_item_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let list_id = ShoppingList::create(&state.mm, &a_list_name(), user_id).await?;
    let item = ShoppingList::add_item(&state.mm, list_id, a_meat_item(), user_id).await?;

    ShoppingList::delete_item(&state.mm, list_id, item.id, user_id).await?;

    let got = ShoppingListDetails::get(&state.mm, list_id, user_id).await?;
    assert!(got.items.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_update_item_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let list_id = ShoppingList::create(&state.mm, &a_list_name(), user_id).await?;
    let item = ShoppingList::add_item(&state.mm, list_id, a_meat_item(), user_id).await?;
    let new_item = other_meat_item();

    let updated = ShoppingList::update_item(
        &state.mm,
        list_id,
        item.id,
        ShoppingListItemForUpdate {
            ingredient: Some(new_item.ingredient.clone()),
            quantity: new_item.quantity.clone(),
            notes: new_item.notes.clone(),
            position: None,
            label: Some("Super C".into()),
            ..Default::default()
        },
        user_id,
    )
    .await?;

    let got = ShoppingListDetails::get(&state.mm, list_id, user_id).await?;
    assert_eq!(got.items.len(), 1);
    pretty_assertions::assert_eq!(
        got.items,
        vec![ShoppingListItemDetails {
            id: item.id,
            ingredient: new_item.ingredient,
            quantity: new_item.quantity,
            notes: Some("other notes".into()),
            label_id: updated.shopping_list_label_id,
            label: "Super C".into(),
            position: 1,
            recipe: None,
            is_checked: false,
            created_at: got.items[0].created_at,
            updated_at: got.items[0].updated_at
        }]
    );
    Ok(())
}

#[tokio::test]
async fn test_item_toggle_check_item_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let list_id = ShoppingList::create(&state.mm, &a_list_name(), user_id).await?;
    let item = ShoppingList::add_item(&state.mm, list_id, a_meat_item(), user_id).await?;

    ShoppingList::update_item(
        &state.mm,
        list_id,
        item.id,
        ShoppingListItemForUpdate::new_checked(true),
        user_id,
    )
    .await?;
    let got = ShoppingListDetails::get(&state.mm, list_id, user_id).await?;
    assert!(got.items[0].is_checked);

    ShoppingList::update_item(
        &state.mm,
        list_id,
        item.id,
        ShoppingListItemForUpdate::new_checked(false),
        user_id,
    )
    .await?;
    let got = ShoppingListDetails::get(&state.mm, list_id, user_id).await?;
    assert!(!got.items[0].is_checked);
    Ok(())
}

#[tokio::test]
async fn test_update_shopping_list_name_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let list_id = ShoppingList::create(&state.mm, &a_list_name(), user_id).await?;
    let original_list = ShoppingList::get_all(&state.mm, user_id).await?[0].clone();

    ShoppingList::update_title(&state.mm, list_id, "New Title", user_id).await?;

    let modified_list = ShoppingList::get_all(&state.mm, user_id).await?[0].clone();
    pretty_assertions::assert_ne!(original_list, modified_list);
    Ok(())
}

#[tokio::test]
async fn test_delete_shopping_list_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let list_id = ShoppingList::create(&state.mm, &a_list_name(), user_id).await?;
    let _ = ShoppingList::add_item(&state.mm, list_id, a_meat_item(), user_id).await?;

    ShoppingList::delete(&state.mm, list_id, user_id).await?;

    let got = ShoppingList::get_all(&state.mm, user_id).await?;
    assert!(got.is_empty());
    let got_res = ShoppingListDetails::get(&state.mm, list_id, user_id).await;
    assert!(got_res.is_err());
    Ok(())
}

#[tokio::test]
async fn test_get_item_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let list_id = ShoppingList::create(&state.mm, &a_list_name(), user_id).await?;
    let item = ShoppingList::add_item(&state.mm, list_id, a_meat_item(), user_id).await?;

    let got = ShoppingList::get_item(&state.mm, list_id, item.id, user_id).await?;

    assert_eq!(got.id, item.id);
    assert_eq!(got.ingredient, a_meat_item().ingredient);
    Ok(())
}

#[tokio::test]
async fn test_label_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;

    let got = ShoppingList::label(&state.mm, 1).await?;

    assert_eq!(got, "No label");
    Ok(())
}

#[tokio::test]
async fn test_labels_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let list_id = ShoppingList::create(&state.mm, &a_list_name(), user_id).await?;
    let _ = ShoppingList::add_item(&state.mm, list_id, a_meat_item(), user_id).await?;
    let _ = ShoppingList::add_item(
        &state.mm,
        list_id,
        ShoppingListItemForCreate::new(None, "Beans", Some("Produce".into()), None, None),
        user_id,
    )
    .await?;

    let got = ShoppingList::labels(&state.mm, user_id).await?;

    pretty_assertions::assert_eq!(got, vec!["Meat", "Produce"]);
    Ok(())
}

#[tokio::test]
async fn test_get_or_insert_label_label_exists_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let list_id = ShoppingList::create(&state.mm, &a_list_name(), user_id).await?;
    let _ = ShoppingList::add_item(&state.mm, list_id, a_meat_item(), user_id).await?;

    let got = ShoppingList::get_or_insert_label(&state.mm, "No label", user_id).await?;

    assert_eq!(got, 1);
    Ok(())
}

#[tokio::test]
async fn test_get_or_insert_label_label_not_exists_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let list_id = ShoppingList::create(&state.mm, &a_list_name(), user_id).await?;
    let item = ShoppingList::add_item(&state.mm, list_id, a_meat_item(), user_id).await?;

    let got = ShoppingList::get_or_insert_label(&state.mm, "Veggies", user_id).await?;

    assert_ne!(got, item.label_id);
    Ok(())
}

#[tokio::test]
async fn test_update_item_labels_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let list_id = ShoppingList::create(&state.mm, &a_list_name(), user_id).await?;
    let item = ShoppingList::add_item(&state.mm, list_id, a_meat_item(), user_id).await?;

    ShoppingList::update_item_labels(&state.mm, list_id, item.label_id, 1, user_id).await?;

    let got = ShoppingListDetails::get(&state.mm, list_id, user_id).await?;
    assert_eq!(got.items[0].label, "No label");
    Ok(())
}

#[tokio::test]
async fn test_toggle_check_item_once_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let list_id = ShoppingList::create(&state.mm, &a_list_name(), user_id).await?;
    let item = ShoppingList::add_item(&state.mm, list_id, a_meat_item(), user_id).await?;

    ShoppingList::toggle_item_check(&state.mm, item.id).await?;

    let got = ShoppingListDetails::get(&state.mm, list_id, user_id).await?;
    assert!(got.items[0].is_checked);
    Ok(())
}

#[tokio::test]
async fn test_toggle_check_item_twice_ok() -> Result<()> {
    let (_, state) = build_server_anonymous(default_config()).await?;
    let user_id = User::all(&state.mm).await?[0].id;
    let list_id = ShoppingList::create(&state.mm, &a_list_name(), user_id).await?;
    let item = ShoppingList::add_item(&state.mm, list_id, a_meat_item(), user_id).await?;

    ShoppingList::toggle_item_check(&state.mm, item.id).await?;
    ShoppingList::toggle_item_check(&state.mm, item.id).await?;

    let got = ShoppingListDetails::get(&state.mm, list_id, user_id).await?;
    assert!(!got.items[0].is_checked);
    Ok(())
}

mod tests_share {
    use super::*;

    async fn insert_recipe(state: &AppState, user_id: Uuid) -> Result<()> {
        let (recipe, _) = a_complete_recipe_for_create();
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
        Ok(())
    }

    async fn add_user(mm: &ModelManager) -> Result<User> {
        Ok(User::new(
            mm,
            UserForCreate {
                email: "another@gmail.com".into(),
                password_clear: "12345677".into(),
            },
        )
        .await?)
    }

    fn assert_share_list(got: &ShareShoppingList, want: &ShareShoppingList) {
        pretty_assertions::assert_eq!(got.id, want.id);
        pretty_assertions::assert_ne!(got.link, Uuid::nil());
        pretty_assertions::assert_eq!(got.user_id, want.user_id);
        pretty_assertions::assert_eq!(got.list_id, want.list_id);
        pretty_assertions::assert_eq!(got.click_count, want.click_count);

        let diff = (got.created_at - want.created_at).whole_nanoseconds();
        assert!(diff.abs() <= 1000, "Created at");

        let diff = (got.expires_at - want.expires_at).whole_nanoseconds();
        assert!(diff.abs() <= 1000, "Expires at");

        let diff = (got.last_accessed - want.last_accessed).whole_nanoseconds();
        assert!(diff.abs() <= 1000, "Last accessed at");
    }

    mod tests_new {
        use time::PrimitiveDateTime;

        use super::*;

        #[tokio::test]
        async fn test_default_expiration_ok() -> Result<()> {
            let state = create_app_state(default_config()).await;
            let user_id = add_user(&state.mm).await?.id;
            insert_recipe(&state, user_id).await?;
            let list_id = ShoppingList::create(&state.mm, &a_list_name(), user_id).await?;

            let got = ShareShoppingList::new(&state.mm, list_id, user_id, None).await?;

            assert_share_list(
                &got,
                &ShareShoppingList {
                    id: got.id,
                    link: got.link,
                    user_id,
                    list_id,
                    created_at: got.created_at,
                    expires_at: got.expires_at,
                    last_accessed: got.last_accessed,
                    click_count: 0,
                },
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_custom_expiration_ok() -> Result<()> {
            let state = create_app_state(default_config()).await;
            let user_id = add_user(&state.mm).await?.id;
            insert_recipe(&state, user_id).await?;
            let list_id = ShoppingList::create(&state.mm, &a_list_name(), user_id).await?;
            let expires_at = {
                let dt = OffsetDateTime::now_utc() + Duration::days(14);
                PrimitiveDateTime::new(dt.date(), dt.time())
            };

            let got = ShareShoppingList::new(&state.mm, list_id, user_id, Some(expires_at)).await?;

            assert_share_list(
                &got,
                &ShareShoppingList {
                    id: got.id,
                    link: got.link,
                    user_id,
                    list_id,
                    created_at: got.created_at,
                    expires_at,
                    last_accessed: got.last_accessed,
                    click_count: 0,
                },
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_already_shared_err() -> Result<()> {
            let state = create_app_state(default_config()).await;
            let user_id = add_user(&state.mm).await?.id;
            insert_recipe(&state, user_id).await?;
            let list_id = ShoppingList::create(&state.mm, &a_list_name(), user_id).await?;
            let share = ShareShoppingList::new(&state.mm, list_id, user_id, None).await?;

            let res = ShareShoppingList::new(&state.mm, list_id, user_id, None).await;

            assert_matches!(res, Ok(got) if got.id == share.id);
            Ok(())
        }
    }

    mod tests_fetch_by_link {
        use super::*;

        #[tokio::test]
        async fn test_exists_ok() -> Result<()> {
            let state = create_app_state(default_config()).await;
            let user = add_user(&state.mm).await?;
            insert_recipe(&state, user.id).await?;
            let list_id = ShoppingList::create(&state.mm, &a_list_name(), user.id).await?;
            let shared = ShareShoppingList::new(&state.mm, list_id, user.id, None).await?;

            let (got, _) = ShareShoppingList::get_by_link(&state.mm, shared.link).await?;

            pretty_assertions::assert_eq!(got.id, shared.id);
            Ok(())
        }

        #[tokio::test]
        async fn test_exists_err() -> Result<()> {
            let state = create_app_state(default_config()).await;
            let user = add_user(&state.mm).await?;
            insert_recipe(&state, user.id).await?;

            let res = ShareShoppingList::get_by_link(&state.mm, user.id).await;

            match res {
                Ok(_) => panic!("Entry should not have been found"),
                Err(_) => Ok(()),
            }
        }
    }
}

mod tests_write {
    use time::PrimitiveDateTime;

    use super::*;

    fn a_recipe() -> ShoppingListRecipeDetails {
        ShoppingListRecipeDetails {
            id: 1,
            name: "Grandma's slow-cooker chicken".into(),
        }
    }

    fn other_recipe() -> ShoppingListRecipeDetails {
        ShoppingListRecipeDetails {
            id: 2,
            name: "Blueberry pie".into(),
        }
    }

    fn a_list_with_no_items() -> ShoppingListDetails {
        let now = OffsetDateTime::now_utc();
        let primitive = PrimitiveDateTime::new(now.date(), now.time());

        ShoppingListDetails {
            id: Uuid::new_v4(),
            name: "Main Shopping List".into(),
            items: vec![],
            created_at: primitive,
            updated_at: primitive,
        }
    }

    fn a_list_with_items_no_labels() -> ShoppingListDetails {
        let now = OffsetDateTime::now_utc();
        let primitive = PrimitiveDateTime::new(now.date(), now.time());

        ShoppingListDetails {
            id: Uuid::new_v4(),
            name: "Main Shopping List".into(),
            items: vec![
                ShoppingListItemDetails {
                    id: 1,
                    ingredient: "chicken".into(),
                    quantity: Some("500g".into()),
                    notes: Some("new notes".into()),
                    label_id: 1,
                    label: "No label".into(),
                    position: 1,
                    recipe: None,
                    is_checked: false,
                    created_at: primitive,
                    updated_at: primitive,
                },
                ShoppingListItemDetails {
                    id: 2,
                    ingredient: "vegetable broth".into(),
                    quantity: None,
                    notes: Some("Buy from the most popular brand".into()),
                    label_id: 1,
                    label: "No label".into(),
                    position: 2,
                    recipe: Some(a_recipe()),
                    is_checked: false,
                    created_at: primitive,
                    updated_at: primitive,
                },
                ShoppingListItemDetails {
                    id: 3,
                    ingredient: "paprika".into(),
                    quantity: Some("5g".into()),
                    notes: None,
                    label_id: 1,
                    label: "No label".into(),
                    position: 3,
                    recipe: Some(a_recipe()),
                    is_checked: false,
                    created_at: primitive,
                    updated_at: primitive,
                },
            ],
            created_at: primitive,
            updated_at: primitive,
        }
    }

    fn a_list_with_mix_labels() -> ShoppingListDetails {
        let now = OffsetDateTime::now_utc();
        let primitive = PrimitiveDateTime::new(now.date(), now.time());

        ShoppingListDetails {
            id: Uuid::new_v4(),
            name: "Main Shopping List".into(),
            items: vec![
                ShoppingListItemDetails {
                    id: 1,
                    ingredient: "Sugar".into(),
                    quantity: Some("1 bag".into()),
                    notes: Some("big notes".into()),
                    label_id: 1,
                    label: "No label".into(),
                    position: 1,
                    recipe: None,
                    is_checked: false,
                    created_at: primitive,
                    updated_at: primitive,
                },
                ShoppingListItemDetails {
                    id: 2,
                    ingredient: "Blueberries".into(),
                    quantity: None,
                    notes: None,
                    label_id: 1,
                    label: "No label".into(),
                    position: 2,
                    recipe: Some(other_recipe()),
                    is_checked: false,
                    created_at: primitive,
                    updated_at: primitive,
                },
                ShoppingListItemDetails {
                    id: 3,
                    ingredient: "paprika".into(),
                    quantity: Some("5g".into()),
                    notes: Some("yay notes".into()),
                    label_id: 2,
                    label: "Spices".into(),
                    position: 3,
                    recipe: None,
                    is_checked: false,
                    created_at: primitive,
                    updated_at: primitive,
                },
                ShoppingListItemDetails {
                    id: 3,
                    ingredient: "ground chili pepper".into(),
                    quantity: Some("15g".into()),
                    notes: Some("some notes".into()),
                    label_id: 2,
                    label: "Spices".into(),
                    position: 3,
                    recipe: None,
                    is_checked: false,
                    created_at: primitive,
                    updated_at: primitive,
                },
                ShoppingListItemDetails {
                    id: 3,
                    ingredient: "Pork necks".into(),
                    quantity: Some("1kg".into()),
                    notes: None,
                    label_id: 3,
                    label: "Meat".into(),
                    position: 5,
                    recipe: Some(a_recipe()),
                    is_checked: false,
                    created_at: primitive,
                    updated_at: primitive,
                },
            ],
            created_at: primitive,
            updated_at: primitive,
        }
    }

    mod tests_text {
        use super::*;

        #[test]
        fn test_no_items_err() {
            let list = a_list_with_no_items();

            let mut text = Vec::new();
            let res = list.write_text(&mut text);

            assert_matches!(res, Err(Error::EmptyInput));
        }

        #[test]
        fn test_items_no_labels_ok() -> Result<()> {
            let list = a_list_with_items_no_labels();

            let mut text = Vec::new();
            list.write_text(&mut text)?;

            pretty_assertions::assert_eq!(
                String::from_utf8(text)?,
                "Main Shopping List\n------------------\n\n- chicken (500g)\n\t*new notes\n- vegetable broth | Grandma's slow-cooker chicken\n\t*Buy from the most popular brand\n- paprika (5g) | Grandma's slow-cooker chicken\n"
            );
            Ok(())
        }

        #[test]
        fn test_mix_labels_ok() -> Result<()> {
            let list = a_list_with_mix_labels();

            let mut text = Vec::new();
            list.write_text(&mut text)?;

            pretty_assertions::assert_eq!(
                String::from_utf8(text)?,
                "Main Shopping List\n------------------\n\n- Sugar (1 bag)\n\t*big notes\n- Blueberries | Blueberry pie\n\n[Spices]\n- paprika (5g)\n\t*yay notes\n- ground chili pepper (15g)\n\t*some notes\n\n[Meat]\n- Pork necks (1kg) | Grandma's slow-cooker chicken\n"
            );
            Ok(())
        }
    }

    mod tests_markdown {
        use super::*;

        #[test]
        fn test_no_items_err() {
            let list = a_list_with_no_items();

            let mut text = Vec::new();
            let res = list.write_markdown(&mut text);

            assert_matches!(res, Err(Error::EmptyInput));
        }

        #[test]
        fn test_items_no_labels_ok() -> Result<()> {
            let list = a_list_with_items_no_labels();

            let mut text = Vec::new();
            list.write_markdown(&mut text)?;

            pretty_assertions::assert_eq!(
                String::from_utf8(text)?,
                "## Main Shopping List\n\n- [ ] chicken (500g)\n\t* new notes\n- [ ] vegetable broth | **Grandma's slow-cooker chicken**\n\t* Buy from the most popular brand\n- [ ] paprika (5g) | **Grandma's slow-cooker chicken**\n"
            );
            Ok(())
        }

        #[test]
        fn test_mix_labels_ok() -> Result<()> {
            let list = a_list_with_mix_labels();

            let mut text = Vec::new();
            list.write_markdown(&mut text)?;

            pretty_assertions::assert_eq!(
                String::from_utf8(text)?,
                "## Main Shopping List\n\n- [ ] Sugar (1 bag)\n\t* big notes\n- [ ] Blueberries | **Blueberry pie**\n\n### Spices\n\n- [ ] paprika (5g)\n\t* yay notes\n- [ ] ground chili pepper (15g)\n\t* some notes\n\n### Meat\n\n- [ ] Pork necks (1kg) | **Grandma's slow-cooker chicken**\n"
            );
            Ok(())
        }
    }
}
