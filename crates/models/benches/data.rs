mod common;

use divan::{Bencher, black_box};

use models::{
    data::{PaginationData, PaginationHtmxData, SearchbarData},
    params::SearchParams,
    recipe::structs::time::Times,
    time::FormattedTimes,
};

fn main() {
    let t = times(10800, 900);
    assert_eq!(FormattedTimes::from_times(&t).unwrap().total, "3h 15m");
    assert_eq!(pagination(11, 258).num_pages, 18);
    assert_eq!(pagination(193, 2888).num_pages, 193);

    divan::main();
}

fn times(cook: i32, prep: i32) -> Times {
    Times {
        cook_seconds: cook.into(),
        prep_seconds: prep.into(),
        total_seconds: (cook + prep).into(),
        id: 0,
        recipe_id: 0,
    }
}

fn pagination(page: u64, num_results: u64) -> PaginationData {
    PaginationData::new(
        "pagination-recipes",
        "/recipes",
        String::new(),
        page,
        num_results,
        15,
        PaginationHtmxData {
            is_swap: false,
            target: "#content".into(),
        },
        None,
    )
}

#[divan::bench(args = ["hours_and_minutes", "minutes_only", "zero"])]
fn formatted_times(bencher: Bencher, case: &str) {
    let t = match case {
        "hours_and_minutes" => times(10800, 900),
        "minutes_only" => times(960, 900),
        _ => times(0, 0),
    };

    bencher.bench_local(|| FormattedTimes::from_times(black_box(&t)));
}

#[divan::bench(args = ["none", "all_set"])]
fn searchbar_from_params(bencher: Bencher, case: &str) {
    bencher
        .with_inputs(|| match case {
            "none" => SearchParams {
                is_favourites: None,
                q: None,
                sort: None,
                page: None,
            },
            _ => SearchParams {
                is_favourites: None,
                q: Some("hamburger".into()),
                sort: Some("z-a".into()),
                page: Some(4),
            },
        })
        .bench_values(SearchbarData::from_params);
}

#[divan::bench(args = [
    (1, 20),       // two pages, no ellipsis
    (2, 12),       // single page (clamped)
    (4, 258),      // near the start: ellipsis on the right
    (11, 258),     // middle: ellipsis on both sides
    (16, 258),     // near the end: ellipsis on the left
    (18, 258),     // last page
    (193, 2888),   // last page of a very large result set
])]
fn pagination_new(bencher: Bencher, (page, num_results): (u64, u64)) {
    bencher
        .with_inputs(|| PaginationHtmxData {
            is_swap: false,
            target: "#content".into(),
        })
        .bench_values(|htmx| {
            PaginationData::new(
                "pagination-recipes",
                "/recipes",
                String::new(),
                black_box(page),
                black_box(num_results),
                15,
                htmx,
                None,
            )
        });
}

#[divan::bench(args = [100, 10_000, 1_000_000])]
fn pagination_new_by_size(bencher: Bencher, num_results: u64) {
    bencher
        .with_inputs(|| PaginationHtmxData {
            is_swap: false,
            target: "#content".into(),
        })
        .bench_values(|htmx| {
            PaginationData::new(
                "pagination-recipes",
                "/recipes",
                String::new(),
                black_box(num_results / 30),
                black_box(num_results),
                15,
                htmx,
                None,
            )
        });
}
