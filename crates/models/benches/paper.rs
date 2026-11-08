mod common;

use divan::{Bencher, counter::ItemsCount};

use models::paper::PaperSize;

use crate::common::{RUNTIME, STATE};

fn main() {
    let state = &*STATE;

    RUNTIME.block_on(async {
        let all = PaperSize::get_all(&state.mm).await.unwrap();
        assert!(!all.is_empty(), "paper sizes table is empty");
        let first_id = all.values().flatten().next().unwrap().id;
        assert_eq!(
            PaperSize::get(&state.mm, first_id).await.unwrap().id,
            first_id
        );
    });

    divan::main();
}

#[divan::bench(args = [1_i16, i16::MAX])]
fn get(bencher: Bencher, id: i16) {
    let mm = &STATE.mm;

    bencher.bench_local(|| RUNTIME.block_on(PaperSize::get(mm, id)));
}

#[divan::bench]
fn get_all(bencher: Bencher) {
    let mm = &STATE.mm;
    let rows: usize = RUNTIME
        .block_on(PaperSize::get_all(mm))
        .unwrap()
        .values()
        .map(Vec::len)
        .sum();

    bencher
        .counter(ItemsCount::new(rows))
        .bench_local(|| RUNTIME.block_on(PaperSize::get_all(mm)));
}
