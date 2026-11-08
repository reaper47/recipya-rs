use divan::{Bencher, black_box, counter::BytesCount};

use support::b64::{b64u_decode, b64u_decode_to_string, b64u_encode};

fn main() {
    divan::main();
}

const A_PRIME_NUMBER: usize = 251;

const SIZES: [usize; 4] = [11, 256, 4 * 1024, 1024 * 1024];

fn payload(n: usize) -> Vec<u8> {
    (0..n)
        .map(|i| u8::try_from(i % A_PRIME_NUMBER).unwrap())
        .collect()
}

#[divan::bench(args = SIZES)]
fn b64u_encode_bench(bencher: Bencher, size: usize) {
    let data = payload(size);

    bencher
        .counter(BytesCount::new(size))
        .bench_local(|| b64u_encode(black_box(&data)));
}

#[divan::bench(args = SIZES)]
fn b64u_decode_bench(bencher: Bencher, size: usize) {
    let encoded_data = b64u_encode(payload(size));

    bencher
        .counter(BytesCount::new(size))
        .bench_local(|| b64u_decode(black_box(&encoded_data)));
}

#[divan::bench(args = SIZES)]
fn b64u_decode_to_string_bench(bencher: Bencher, size: usize) {
    let encoded_data = b64u_encode(payload(size));

    bencher
        .counter(BytesCount::new(size))
        .bench_local(|| b64u_decode_to_string(black_box(&encoded_data)));
}
