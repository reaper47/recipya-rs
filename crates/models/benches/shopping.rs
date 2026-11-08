mod common;

use std::sync::LazyLock;

use divan::{Bencher, black_box, counter::ItemsCount};
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

use app::state::AppState;
use models::{
    Recipe,
    settings::UserSettingDetails,
    shopping::{
        ShareShoppingList, ShoppingList, ShoppingListDetails, ShoppingListItemDetails,
        ShoppingListItemForCreate, ShoppingListItemForUpdate, ShoppingListRecipeDetails, SortBy,
    },
    user::User,
};
use test_db::default_config;
use test_fixtures::insert_user;
use test_harness::create_app_state;

use crate::common::{RUNTIME, a_complete_recipe_for_create, unique_name};

struct Seeded {
    state: AppState,
    user: User,
    recipe_id: i64,
    list_id: Uuid,
    item_id: i64,
    label_id: i64,
    link: Uuid,
}

const SEEDED_ITEMS: usize = 20;
const LABELS: [&str; 3] = ["Meat", "Produce", "Spices"];

static SEEDED: LazyLock<Seeded> = LazyLock::new(|| {
    RUNTIME.block_on(async {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await.unwrap();
        let settings = UserSettingDetails::get(&state.mm, user.id).await.unwrap();
        let (recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user.id, &recipe, &settings)
            .await
            .unwrap();

        let list_id = ShoppingList::create(&state.mm, "Costco", user.id)
            .await
            .unwrap();

        let mut first = None;
        for i in 0..SEEDED_ITEMS {
            let item =
                ShoppingList::add_item(&state.mm, list_id, item(i, Some(recipe_id)), user.id)
                    .await
                    .unwrap();

            first.get_or_insert((item.id, item.label_id));
        }

        let (item_id, label_id) = first.unwrap();
        let link = ShareShoppingList::new(&state.mm, list_id, user.id, None)
            .await
            .unwrap()
            .link;

        Seeded {
            state,
            user,
            recipe_id,
            list_id,
            item_id,
            label_id,
            link,
        }
    })
});

fn item(i: usize, recipe_id: Option<i64>) -> ShoppingListItemForCreate {
    let mut itemm = ShoppingListItemForCreate::new(
        Some(format!("{i} cup")),
        format!("ingredient {i}"),
        Some(LABELS[i % LABELS.len()].into()),
        Some("notes".into()),
        None,
    );
    itemm.recipe_id = recipe_id;
    itemm
}

fn main() {
    let s = &*SEEDED;

    RUNTIME.block_on(async {
        let list = ShoppingListDetails::get(&s.state.mm, s.list_id, s.user.id)
            .await
            .unwrap();
        assert_eq!(list.items.len(), SEEDED_ITEMS);
        assert_eq!(
            ShoppingList::label(&s.state.mm, 1).await.unwrap(),
            "No label"
        );
    });

    divan::main();
}

fn fresh_list() -> Uuid {
    let s = &*SEEDED;

    RUNTIME
        .block_on(ShoppingList::create(&s.state.mm, &unique_name(), s.user.id))
        .unwrap()
}

fn fresh_list_with_item() -> (Uuid, i64) {
    let s = &*SEEDED;
    let list_id = fresh_list();
    let item = RUNTIME
        .block_on(ShoppingList::add_item(
            &s.state.mm,
            list_id,
            item(0, None),
            s.user.id,
        ))
        .unwrap();
    (list_id, item.id)
}

#[divan::bench_group]
mod reads {
    use super::{
        Bencher, ItemsCount, RUNTIME, SEEDED, SEEDED_ITEMS, ShoppingList, ShoppingListDetails,
        SortBy, fresh_list, item,
    };

    #[divan::bench]
    fn get(bencher: Bencher) {
        let s = &*SEEDED;

        bencher
            .bench_local(|| RUNTIME.block_on(ShoppingList::get(&s.state.mm, s.list_id, s.user.id)));
    }

    #[divan::bench]
    fn get_all(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| {
            RUNTIME.block_on(ShoppingList::get_all(
                &s.state.mm,
                s.user.id,
                SortBy::default(),
            ))
        });
    }

    #[divan::bench]
    fn details_get(bencher: Bencher) {
        let s = &*SEEDED;

        bencher
            .counter(ItemsCount::new(SEEDED_ITEMS))
            .bench_local(|| {
                RUNTIME.block_on(ShoppingListDetails::get(&s.state.mm, s.list_id, s.user.id))
            });
    }

    #[divan::bench]
    fn get_item(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| {
            RUNTIME.block_on(ShoppingList::get_item(
                &s.state.mm,
                s.list_id,
                s.item_id,
                s.user.id,
            ))
        });
    }

    #[divan::bench]
    fn items_count(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| RUNTIME.block_on(ShoppingList::items_count(&s.state.mm, s.list_id)));
    }

    #[divan::bench]
    fn label(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| RUNTIME.block_on(ShoppingList::label(&s.state.mm, 1)));
    }

    #[divan::bench]
    fn labels(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| RUNTIME.block_on(ShoppingList::labels(&s.state.mm, s.user.id)));
    }

    #[divan::bench]
    fn get_or_insert_label_existing(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| {
            RUNTIME.block_on(ShoppingList::get_or_insert_label(
                &s.state.mm,
                "Meat",
                s.user.id,
            ))
        });
    }

    #[divan::bench(args = [1, 10, 100])]
    fn details_get_by_size(bencher: Bencher, n: usize) {
        let s = &*SEEDED;
        let list_id = fresh_list();
        RUNTIME.block_on(async {
            for i in 0..n {
                ShoppingList::add_item(&s.state.mm, list_id, item(i, None), s.user.id)
                    .await
                    .unwrap();
            }
        });

        bencher.counter(ItemsCount::new(n)).bench_local(|| {
            RUNTIME.block_on(ShoppingListDetails::get(&s.state.mm, list_id, s.user.id))
        });
    }
}

#[divan::bench_group]
mod updates {
    use super::{Bencher, RUNTIME, SEEDED, ShoppingList, ShoppingListItemForUpdate, fresh_list};

    #[divan::bench]
    fn update_item(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| {
            RUNTIME.block_on(ShoppingList::update_item(
                &s.state.mm,
                s.list_id,
                s.item_id,
                ShoppingListItemForUpdate {
                    ingredient: Some("fresh basil".into()),
                    quantity: Some("2 cups".into()),
                    notes: Some("no need to preheat oven for this recipe".into()),
                    ..Default::default()
                },
                s.user.id,
            ))
        });
    }

    #[divan::bench]
    fn toggle_item_check(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| {
            RUNTIME.block_on(ShoppingList::toggle_item_check(&s.state.mm, s.item_id))
        });
    }

    #[divan::bench]
    fn update_title(bencher: Bencher) {
        let s = &*SEEDED;
        let list_id = fresh_list();

        bencher.bench_local(|| {
            RUNTIME.block_on(ShoppingList::update_title(
                &s.state.mm,
                list_id,
                "Renamed",
                s.user.id,
            ))
        });
    }

    #[divan::bench]
    fn update_item_labels(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| {
            RUNTIME.block_on(ShoppingList::update_item_labels(
                &s.state.mm,
                s.list_id,
                s.label_id,
                1,
                s.user.id,
            ))
        });
    }
}

#[divan::bench_group]
mod creates {
    use itertools::Itertools;

    use super::{
        Bencher, ItemsCount, RUNTIME, SEEDED, ShoppingList, Uuid, fresh_list, item, unique_name,
    };

    #[divan::bench]
    fn create_list(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.with_inputs(unique_name).bench_values(|name| {
            RUNTIME.block_on(ShoppingList::create(&s.state.mm, &name, s.user.id))
        });
    }

    #[divan::bench]
    fn add_item(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.with_inputs(fresh_list).bench_values(|list_id| {
            RUNTIME.block_on(ShoppingList::add_item(
                &s.state.mm,
                list_id,
                item(0, None),
                s.user.id,
            ))
        });
    }

    #[divan::bench]
    fn add_item_with_recipe(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.with_inputs(fresh_list).bench_values(|list_id| {
            RUNTIME.block_on(ShoppingList::add_item(
                &s.state.mm,
                list_id,
                item(0, Some(s.recipe_id)),
                s.user.id,
            ))
        });
    }

    #[divan::bench(args = [1, 10, 50])]
    fn add_items_for_recipe(bencher: Bencher, n: usize) {
        let s = &*SEEDED;

        bencher
            .counter(ItemsCount::new(n))
            .with_inputs(|| {
                let list_id = fresh_list();
                let list = RUNTIME
                    .block_on(ShoppingList::get(&s.state.mm, list_id, s.user.id))
                    .unwrap();
                let items = (0..n).map(|i| item(i, Some(s.recipe_id))).collect_vec();
                (list, items)
            })
            .bench_values(|(list, items)| {
                RUNTIME.block_on(list.add_items_for_recipe(&s.state.mm, &items, s.user.id))
            });
    }

    #[divan::bench]
    fn get_or_insert_label_new(bencher: Bencher) {
        let s = &*SEEDED;

        bencher
            .with_inputs(|| format!("label-{}", Uuid::new_v4()))
            .bench_values(|label| {
                RUNTIME.block_on(ShoppingList::get_or_insert_label(
                    &s.state.mm,
                    &label,
                    s.user.id,
                ))
            });
    }
}

#[divan::bench_group]
mod deletes {
    use super::{Bencher, RUNTIME, SEEDED, ShoppingList, fresh_list_with_item};

    #[divan::bench]
    fn delete_item(bencher: Bencher) {
        let s = &*SEEDED;

        bencher
            .with_inputs(fresh_list_with_item)
            .bench_values(|(list_id, item_id)| {
                RUNTIME.block_on(ShoppingList::delete_item(
                    &s.state.mm,
                    list_id,
                    item_id,
                    s.user.id,
                ))
            });
    }

    #[divan::bench]
    fn delete_list_with_item(bencher: Bencher) {
        let s = &*SEEDED;

        bencher
            .with_inputs(|| fresh_list_with_item().0)
            .bench_values(|list_id| {
                RUNTIME.block_on(ShoppingList::delete(&s.state.mm, list_id, s.user.id))
            });
    }
}

#[divan::bench_group]
mod share {
    use super::{Bencher, RUNTIME, SEEDED, ShareShoppingList, Uuid, fresh_list};

    #[divan::bench]
    fn new(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.with_inputs(fresh_list).bench_values(|list_id| {
            RUNTIME.block_on(ShareShoppingList::new(
                &s.state.mm,
                list_id,
                s.user.id,
                None,
            ))
        });
    }

    #[divan::bench]
    fn new_already_shared(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| {
            RUNTIME.block_on(ShareShoppingList::new(
                &s.state.mm,
                s.list_id,
                s.user.id,
                None,
            ))
        });
    }

    #[divan::bench(args = ["hit", "miss"])]
    fn get_by_link(bencher: Bencher, which: &str) {
        let s = &*SEEDED;
        let link = if which == "hit" {
            s.link
        } else {
            Uuid::new_v4()
        };

        bencher.bench_local(|| RUNTIME.block_on(ShareShoppingList::get_by_link(&s.state.mm, link)));
    }
}

#[divan::bench_group]
mod write {
    use super::{
        Bencher, ItemsCount, OffsetDateTime, PrimitiveDateTime, ShoppingListDetails,
        ShoppingListItemDetails, ShoppingListRecipeDetails, Uuid, black_box,
    };

    fn build_list(n: usize) -> ShoppingListDetails {
        let now = OffsetDateTime::now_utc();
        let ts = PrimitiveDateTime::new(now.date(), now.time());
        let items = (0..n)
            .map(|i| ShoppingListItemDetails {
                id: i64::try_from(i).unwrap(),
                ingredient: format!("ingredient {i}"),
                quantity: (i % 4 != 0).then(|| format!("{i}g")),
                notes: (i % 2 == 0).then(|| "some notes".to_string()),
                label_id: i64::try_from((i % 3) + 1).unwrap(),
                label: ["No label", "Spices", "Meat"][i % 3].into(),
                position: i32::try_from(i).unwrap(),
                recipe: (i % 3 == 0).then(|| ShoppingListRecipeDetails {
                    id: 1,
                    name: "Grandma's slow-cooker chicken".into(),
                }),
                is_checked: false,
                created_at: ts,
                updated_at: ts,
            })
            .collect();

        ShoppingListDetails {
            id: Uuid::new_v4(),
            name: "Main Shopping List".into(),
            items,
            created_at: ts,
            updated_at: ts,
        }
    }

    #[divan::bench(args = [3, 30, 300])]
    fn write_text(bencher: Bencher, n: usize) {
        let list = build_list(n);

        bencher
            .counter(ItemsCount::new(n))
            .with_inputs(Vec::new)
            .bench_values(|mut buf| {
                black_box(&list).write_text(&mut buf).unwrap();
                buf
            });
    }

    #[divan::bench(args = [3, 30, 300])]
    fn write_markdown(bencher: Bencher, n: usize) {
        let list = build_list(n);

        bencher
            .counter(ItemsCount::new(n))
            .with_inputs(Vec::new)
            .bench_values(|mut buf| {
                black_box(&list).write_markdown(&mut buf).unwrap();
                buf
            });
    }
}
