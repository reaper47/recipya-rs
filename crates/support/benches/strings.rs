use divan::{
    Bencher, black_box,
    counter::{BytesCount, ItemsCount},
};
use support::strings::{
    auto_convert_to_utf8, calc_seconds_from_parts, extract_number, find_indexes,
    insert_space_after_leading_number, normalise_vulgar_fractions,
};

fn main() {
    divan::main();
}

#[divan::bench(args = [
    "empty",
    "ascii",
    "utf8 multibyte",
    "utf8 bom",
    "windows 1252",
    "mixed windows 1252",
    "undefined windows 1252",
    "long ascii",
    "long windows 1252",
])]
fn auto_convert_to_utf8_bench(bencher: Bencher, input: &str) {
    let input = input_to_case(input);

    bencher
        .counter(BytesCount::of_slice(&input))
        .bench_local(|| auto_convert_to_utf8(&input));
}

fn input_to_case(input: &str) -> Vec<u8> {
    match input {
        "empty" => Vec::new(),
        "ascii" => b"Hello World!".to_vec(),
        "utf8 multibyte" => "Hello, 世界! 🌍".as_bytes().to_vec(),
        "utf8 bom" => vec![0xEF, 0xBB, 0xBF],
        "windows 1252" => "Café résumé naïve".as_bytes().to_vec(),
        "mixed windows 1252" => vec![0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0xE9],
        "undefined windows 1252" => vec![0x81, 0x8D, 0x8F, 0x90, 0x9D],
        "long ascii" => b"A".repeat(10_000).to_vec(),
        "long windows 1252" => vec![0x63, 0x61, 0x66, 0xE9].repeat(2_500),
        _ => unreachable!("unreachable case"),
    }
}

#[divan::bench(args = [
    "",
    "420 years of love",
    "years of 420 love",
    "years of love 420",
])]
fn extract_number_bench(bencher: Bencher, input: &str) {
    bencher.bench_local(|| extract_number::<i64>(black_box(input)));
}

#[divan::bench(args = [
    "",
    "½ ⅓ ⅔ ¼ ¾ ⅕ ⅖ ⅗ ⅘ ⅙ ⅚ ⅐ ⅛ ⅜ ⅝ ⅞ ⅑ ⅒",
    "Add ½ cup of sugar and ¼ tsp of salt.",
    "⅓ + ⅓ + ⅓ = 1",
    "Add 1/6 cup of sugar and 1/2 tsp of salt.",
    "1 ½ cups general-purpose flour",
])]
fn normalise_vulgar_fractions_bench(bencher: Bencher, input: &str) {
    bencher
        .counter(BytesCount::new(input.len()))
        .bench_local(|| normalise_vulgar_fractions(black_box(input)));
}

#[divan::bench(args = ["1eggs whole fresh, beaten"])]
fn insert_space_after_leading_number_bench(bencher: Bencher, input: &str) {
    bencher
        .counter(ItemsCount::new(input.len()))
        .bench_local(|| insert_space_after_leading_number(black_box(input)));
}

#[divan::bench]
fn find_indexes_bench(bencher: Bencher) {
    const RECIPE: &[&str; 11] = &[
        "butter",
        "white sugar",
        "brown sugar",
        "eggs",
        "vanilla extract",
        "baking soda",
        "hot water",
        "salt",
        "all-purpose flour",
        "semisweet chocolate chips",
        "chopped walnuts",
    ];

    const INPUT: &[&str; 9] = &[
        "Gather your ingredients, making sure your butter is softened, and your eggs are room temperature.",
        "Preheat the oven to 350 degrees F (175 degrees C). Beat butter, white sugar, and brown sugar together in a large bowl with an electric mixer until smooth and creamy.",
        "Beat in eggs, one at a time, then stir in vanilla.",
        "Dissolve baking soda in hot water; add to batter along with salt and mix until combined.",
        "Stir in flour, chocolate chips, and walnuts until a soft dough forms.",
        "Drop rounded spoonfuls of cookie dough 2 inches apart onto ungreased baking sheets.",
        "Bake in the preheated oven until edges are lightly browned, about 10 minutes.",
        "Cool on the baking sheets briefly before transferring to a wire rack to cool completely.",
        "Store in an airtight container or serve immediately and enjoy!",
    ];

    bencher
        .counter(ItemsCount::new(INPUT.len()))
        .bench_local(|| find_indexes(black_box(INPUT), black_box(RECIPE)));
}

#[divan::bench(args = [
    "",
    "05:50",
    "00:05:80",
    "06:05:50",
])]
fn calc_seconds_from_parts_bench(bencher: Bencher, input: &str) {
    bencher.bench_local(|| calc_seconds_from_parts(black_box(input)));
}
