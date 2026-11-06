#[path = "../src/testdata/nutrition_data.rs"]
mod testdata;

use std::sync::LazyLock;

use divan::Bencher;
use tokio::runtime::Runtime;

use nutrition::{
    NutritionDataSource,
    fdc::parser::{DataFetched, DataNotFetched, FdcParser},
    states::DataFetchedState,
};
use test_db::default_config;
use test_harness::{AppState, create_app_state};
use testdata::{FDC_FF_DATASET_3, FdcClientForTests};

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());

static STATE: LazyLock<AppState> = LazyLock::new(|| {
    RUNTIME.block_on(async {
        let state = create_app_state(default_config()).await;
        let client = FdcClientForTests::new(FDC_FF_DATASET_3);
        let parser: FdcParser<DataFetchedState> = FdcParser::new().fetch(&client).await.unwrap();
        parser.push_into_database(&state.mm).await.unwrap();
        state
    })
});

const RECIPE: &[&str; 10] = &[
    "1 cup white sugar",
    "1/2 cup butter, melted",
    "2 eggs",
    "1 teaspoon vanilla extract",
    "1 cups all-purpose flour",
    "1 teaspoon baking soda",
    "1/2 teaspoon salt",
    "1/2 cup sour cream",
    "1/2 cup chopped walnuts",
    "2 medium bananas, sliced",
];

const FOUR_SERVINGS: i16 = 4;

fn main() {
    divan::main();
}

fn run(bencher: Bencher, source: NutritionDataSource, ingredients: &[&str], num_servings: i16) {
    let mut conn = RUNTIME.block_on(STATE.mm.pool.get()).unwrap();

    bencher.counter(ingredients.len()).bench_local(|| {
        RUNTIME.block_on(source.calculate_nutrition(&mut conn, ingredients, num_servings))
    });
}

#[divan::bench_group]
mod calculate_nutrition {
    use super::*;

    mod fdc {
        use super::*;

        const SOURCE: NutritionDataSource = NutritionDataSource::USDAFoodDataCentral;

        #[divan::bench]
        fn full_recipe(bencher: Bencher) {
            run(bencher, SOURCE, RECIPE, FOUR_SERVINGS)
        }

        #[divan::bench(args = [
            "2 eggs",
            "1 cup white sugar",
            "1/2 cup chopped walnuts",
            "1/2 cup sour cream",
            "1 cup all-purpose flour",
            "2 medium bananas, sliced",
            "1 cup ingredientdoesntexist"
        ])]
        fn single_ingredient(bencher: Bencher, ing: &str) {
            run(bencher, SOURCE, &[ing], 1)
        }
    }
}

#[divan::bench]
fn unknown_source(bencher: Bencher) {
    let mut conn = RUNTIME.block_on(STATE.mm.pool.get()).unwrap();

    bencher.bench_local(|| {
        RUNTIME.block_on(NutritionDataSource::Unknown.calculate_nutrition(
            &mut conn,
            RECIPE,
            FOUR_SERVINGS,
        ))
    });
}
