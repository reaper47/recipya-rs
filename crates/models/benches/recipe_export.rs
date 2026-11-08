mod common;

use divan::{Bencher, black_box, counter::BytesCount};
use uuid::Uuid;

use models::{
    RecipeDetails,
    recipe::structs::section::{Item, SectionComponents, SectionItem},
};

use crate::common::a_complete_recipe;

const BASE_URL: &str = "https://example.com/images";

fn main() {
    let r = complete_recipe();
    assert!(
        r.to_markdown(BASE_URL)
            .unwrap()
            .starts_with("# Best Chinese Kale")
    );
    assert!(
        r.to_text(BASE_URL)
            .unwrap()
            .starts_with("Best Chinese Kale\n-----")
    );

    divan::main();
}

fn complete_recipe() -> RecipeDetails {
    let (mut recipe, _) = a_complete_recipe();
    recipe.recipe.image = Some(Uuid::parse_str("637bcefb-9fa4-4970-b490-b73023ac772f").unwrap());
    recipe
}

fn big_recipe(n: usize) -> RecipeDetails {
    let mut recipe = complete_recipe();

    recipe.ingredients = SectionComponents::Grouped(vec![SectionItem {
        title: "Everything".into(),
        items: (0..n)
            .map(|i| Item::new(format!("{i} cup ingredient {i}")))
            .collect(),
    }]);
    recipe.instructions = SectionComponents::Grouped(vec![SectionItem {
        title: "Steps".into(),
        items: (0..n)
            .map(|i| Item::new(format!("Step {i}: stir the pot gently and wait a while.")))
            .collect(),
    }]);
    recipe
}

#[divan::bench_group]
mod complete_recipe_render {
    use super::{BASE_URL, Bencher, BytesCount, black_box, complete_recipe};

    #[divan::bench]
    fn to_markdown(bencher: Bencher) {
        let recipe = complete_recipe();
        let len = recipe.to_markdown(BASE_URL).unwrap().len();

        bencher
            .counter(BytesCount::new(len))
            .bench_local(|| black_box(&recipe).to_markdown(black_box(BASE_URL)));
    }

    #[divan::bench]
    fn to_text(bencher: Bencher) {
        let recipe = complete_recipe();
        let len = recipe.to_text(BASE_URL).unwrap().len();

        bencher
            .counter(BytesCount::new(len))
            .bench_local(|| black_box(&recipe).to_text(black_box(BASE_URL)));
    }
}

#[divan::bench(args = ["markdown", "text"])]
fn no_image(bencher: Bencher, format: &str) {
    let mut recipe = complete_recipe();
    recipe.recipe.image = None;

    match format {
        "markdown" => bencher.bench_local(|| black_box(&recipe).to_markdown(BASE_URL)),
        _ => bencher.bench_local(|| black_box(&recipe).to_text(BASE_URL)),
    }
}

#[divan::bench_group]
mod by_size {
    use super::{BASE_URL, Bencher, BytesCount, big_recipe, black_box};

    #[divan::bench(args = [4, 50, 500])]
    fn to_markdown(bencher: Bencher, n: usize) {
        let recipe = big_recipe(n);
        let len = recipe.to_markdown(BASE_URL).unwrap().len();

        bencher
            .counter(BytesCount::new(len))
            .bench_local(|| black_box(&recipe).to_markdown(black_box(BASE_URL)));
    }

    #[divan::bench(args = [4, 50, 500])]
    fn to_text(bencher: Bencher, n: usize) {
        let recipe = big_recipe(n);
        let len = recipe.to_text(BASE_URL).unwrap().len();

        bencher
            .counter(BytesCount::new(len))
            .bench_local(|| black_box(&recipe).to_text(black_box(BASE_URL)));
    }
}
