mod common;

use divan::{Bencher, counter::ItemsCount};

use models::language::Language;

use crate::common::{RUNTIME, STATE};

fn main() {
    let state = &*STATE;

    RUNTIME.block_on(async {
        let all = Language::get_all(&state.mm).await.unwrap();
        assert!(!all.is_empty(), "languages table is empty");
    });

    divan::main();
}

#[divan::bench(sample_count = 50)]
fn get_all(bencher: Bencher) {
    let mm = &STATE.mm;
    let rows = RUNTIME.block_on(Language::get_all(mm)).unwrap().len();

    bencher
        .counter(ItemsCount::new(rows))
        .bench_local(|| RUNTIME.block_on(Language::get_all(mm)));
}
