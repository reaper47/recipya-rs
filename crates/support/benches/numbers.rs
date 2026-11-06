use divan::{Bencher, black_box};

use support::numbers::fmt_without_trailing_zeroes;

fn main() {
    divan::main();
}

#[divan::bench(args = [
    216.0000,                // integer
    -216.0,                  // negative integer
    216.0001,                // short fraction
    0.5,                     // typical recipe fraction
    123_456_789.123_456_789, // large fraction
    216e15,                  // large integer
    1.0 / 3.0,               // vinculum
])]
fn fmt_without_trailing_zeroes_bench(bencher: Bencher, input: f64) {
    bencher.bench_local(|| fmt_without_trailing_zeroes(black_box(input)));
}
