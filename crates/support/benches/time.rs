use divan::{Bencher, black_box};

use support::time::parse_hours_minutes;

fn main() {
    divan::main();
}

#[divan::bench(args = [
    "3h",
    "2h30m",
    "45 minutes",
    "1 hour",
    "",
    "oops",
])]
fn parse_hours_minutes_benchmark(bencher: Bencher, mut input: &str) {
    bencher.bench_local(|| parse_hours_minutes(black_box(&mut input)));
}
