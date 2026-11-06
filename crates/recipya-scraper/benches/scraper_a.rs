use std::sync::{Arc, LazyLock};

use divan::{Bencher, black_box, counter::BytesCount};
use tokio::runtime::Runtime;

use recipya_scraper::{
    Scraper, Website,
    tests::support::benchmark::{InMemoryClient, load_fixture},
};
use support::fs::MockFs;

const WEBSITES: [Website; 15] = [
    Website::AaronParecki,
    Website::AceCanning,
    Website::ADayInTheLifeOnTheFarm,
    Website::AdultBar,
    Website::AllRecipes,
    Website::AllRoadsLeadToTheKitchen,
    // Website::AllThingsMamma, // TODO: Fix the test because the website broke
    Website::Almanac,
    Website::AlmondAndFig,
    Website::AlphaFoodie,
    // Website::AlsoTheCrumbsPlease, // TODO: Fix the test because the website broke
    Website::AlvaradoStreetBakery,
    Website::Alwadi,
    Website::AnnamsRecipes,
    Website::Arbuz,
    Website::ArgiroBarbarigou,
    Website::ASicilianPeasantsTable,
    // Website::ATreatsAffair, // TODO: Fix the test because the website broke
];

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());

fn main() {
    divan::main();
}

#[divan::bench(args = WEBSITES)]
fn scrape_bench(bencher: Bencher, website: &Website) {
    let url = website.test_urls()[0].to_string();
    let html = load_fixture(*website, 0);
    let client = InMemoryClient::new(&url, &html);
    let scraper = Scraper::with_client(Arc::new(client), Arc::new(MockFs));
    RUNTIME.block_on(scraper.scrape(&url)).unwrap();

    bencher
        .counter(BytesCount::new(html.len()))
        .bench_local(|| RUNTIME.block_on(scraper.scrape(black_box(&url))));
}
