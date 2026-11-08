mod common;

use std::sync::LazyLock;

use divan::{Bencher, counter::ItemsCount};
use uuid::Uuid;

use models::{
    Recipe,
    recipe::{
        RecipeSearch,
        structs::{
            recipe::RecipeForCreate,
            section::{Item, SectionComponents},
            tool::ToolForCreate,
        },
    },
    settings::UserSettingDetails,
    types::Rating,
    user::User,
};
use repository::ModelManager;
use test_db::default_config;
use test_fixtures::insert_user;
use test_harness::create_app_state;

use crate::common::{RUNTIME, Seeded, a_complete_recipe_for_create};

const LIBRARY_SIZE: usize = 60;

const QUERIES: [(&str, &str); 10] = [
    ("name", "chinese"),
    ("category", "cat:Breakfast"),
    ("category_and_term", "tacos cat:Breakfast"),
    ("cuisine", "cui:THAI"),
    ("ingredients", "ing:cayenne pepper,chicken"),
    ("instructions", "ins:melt butter medium heat"),
    ("keywords", "kw:very fat,air fryer"),
    ("rating", "stars:1"),
    ("tools", "tool:wok"),
    ("no_match", "zzzunobtainium"),
];

static SEEDED: LazyLock<Seeded> = LazyLock::new(|| {
    RUNTIME.block_on(async {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await.unwrap();
        insert_recipes(&state.mm, &user, LIBRARY_SIZE).await;
        Seeded { state, user }
    })
});

fn recipe(i: usize) -> RecipeForCreate {
    let (mut r, _) = a_complete_recipe_for_create();
    let matching = i.is_multiple_of(3);

    r.name = format!(
        "Recipe {i} {}",
        ["Chinese Taco Tuesday", "Chicken Jersey", "Pasta Night"][i % 3]
    );
    r.category = Some(["breakfast", "Meat", "dessert"][i % 3].to_string());
    r.cuisine = Some(["thai", "chinese", "italian"][i % 3].to_string());

    r.keywords = if matching {
        vec!["very fat".to_string(), "air fryer".to_string()]
    } else {
        vec!["healthy".to_string(), "vegan".to_string()]
    };

    r.rating = Rating::from(u8::try_from(i % 5).unwrap() + 1);

    r.ingredients = SectionComponents::Flat(if matching {
        vec![
            Item::new("1 tbsp of hot cayenne pepper"),
            Item::new("3 lbs of chicken breasts"),
            Item::new("1/2 cups of lettuce"),
        ]
    } else {
        vec![Item::new("tomato"), Item::new("1/2 cups of lettuce")]
    });

    r.instructions = SectionComponents::Flat(vec![
        Item::new(
            [
                "Sauté veggies: In a large pot, melt butter over medium heat.",
                "Boil pasta: Bring a large pot of salted water to a boil.",
                "Make roux: Sprinkle flour over the mushrooms and stir well.",
            ][i % 3],
        )
        .with_duration(300),
    ]);

    r.tools = vec![ToolForCreate {
        name: ["wok", "frying pan", "oven"][i % 3].to_string(),
        quantity: 1,
    }];
    r
}

async fn insert_recipes(mm: &ModelManager, user: &User, n: usize) {
    let settings = UserSettingDetails::get(mm, user.id).await.unwrap();
    for i in 0..n {
        Recipe::create(mm, user.id, &recipe(i), &settings)
            .await
            .unwrap();
    }
}

fn search(query: &str, user_id: Uuid) -> usize {
    let s = &*SEEDED;
    let rs = RecipeSearch::new(query, 1, false, user_id).unwrap();

    RUNTIME.block_on(rs.search(&s.state.mm)).unwrap().len()
}

fn main() {
    let s = &*SEEDED;
    let failed: Vec<_> = QUERIES
        .iter()
        .filter(|(name, q)| *name != "no_match" && search(q, s.user.id) == 0)
        .map(|(_, q)| *q)
        .collect();

    assert!(failed.is_empty(), "queries matched nothing: {failed:?}");
    assert_eq!(search("zzzunobtainium", s.user.id), 0);

    divan::main();
}

#[divan::bench(args = QUERIES)]
fn by_filter(bencher: Bencher, (name, query): &(&str, &str)) {
    let _ = name;
    let s = &*SEEDED;
    let rs = RecipeSearch::new(query, 1, false, s.user.id).unwrap();

    bencher.bench_local(|| RUNTIME.block_on(rs.search(&s.state.mm)));
}

#[divan::bench(args = [1, 2, 4])]
fn by_page(bencher: Bencher, page: i64) {
    let s = &*SEEDED;
    let rs = RecipeSearch::new("recipe", page, false, s.user.id).unwrap();

    bencher.bench_local(|| RUNTIME.block_on(rs.search(&s.state.mm)));
}

#[divan::bench(args = [false, true])]
fn favourites_only(bencher: Bencher, only_favourites: bool) {
    let s = &*SEEDED;
    let rs = RecipeSearch::new("recipe", 1, only_favourites, s.user.id).unwrap();

    bencher.bench_local(|| RUNTIME.block_on(rs.search(&s.state.mm)));
}

#[divan::bench(args = QUERIES)]
fn parse_query(bencher: Bencher, (name, query): &(&str, &str)) {
    let _ = name;
    let id = Uuid::nil();

    bencher.bench_local(|| RecipeSearch::new(divan::black_box(query), 1, false, id));
}

#[divan::bench(args = [10, 100, 300])]
fn by_library_size(bencher: Bencher, n: usize) {
    let s = &*SEEDED;
    let user = RUNTIME.block_on(async {
        let user = test_fixtures::insert_other_user(
            &s.state.mm,
            &format!("{}@bench.test", Uuid::new_v4()),
        )
        .await
        .unwrap();
        insert_recipes(&s.state.mm, &user, n).await;
        user
    });
    let rs = RecipeSearch::new("tool:wok", 1, false, user.id).unwrap();

    bencher
        .counter(ItemsCount::new(n))
        .bench_local(|| RUNTIME.block_on(rs.search(&s.state.mm)));
}
