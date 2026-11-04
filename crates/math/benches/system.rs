use divan::{Bencher, black_box, counter::ItemsCount};

use math::{Error, cooking::units::system::MeasurementSystem};

const SYSTEMS: [MeasurementSystem; 4] = [
    MeasurementSystem::ImperialUK,
    MeasurementSystem::Metric,
    MeasurementSystem::MetricAustralia,
    MeasurementSystem::UsCustomary,
];

fn main() {
    divan::main();
}

#[divan::bench_group]
mod from_id {
    use super::*;

    #[divan::bench(args = [1, 2, 3, 4])]
    fn valid(id: i16) -> Result<MeasurementSystem, Error> {
        MeasurementSystem::from_id(black_box(id))
    }

    #[divan::bench(args = [i16::MIN, -5, -1, 0, i16::MAX])]
    fn invalid(id: i16) -> Result<MeasurementSystem, Error> {
        MeasurementSystem::from_id(black_box(id))
    }

    #[divan::bench]
    fn mixed(bencher: Bencher) {
        let ids: Vec<i16> = (-10..=10).collect();

        bencher.bench_local(|| {
            for &id in &ids {
                let _ = black_box(MeasurementSystem::from_id(black_box(id)));
            }
        });
    }
}

#[divan::bench_group]
mod scale {
    use super::*;

    fn run(bencher: Bencher, system: &MeasurementSystem, samples: &[&str]) {
        const NUM_INPUTS: usize = 64;

        bencher
            .counter(ItemsCount::new(NUM_INPUTS))
            .with_inputs(|| make_inputs(samples, NUM_INPUTS))
            .bench_values(|input| black_box(system).scale(input, black_box(2.0)));
    }

    fn make_inputs(samples: &[&str], n: usize) -> Vec<String> {
        samples
            .iter()
            .cycle()
            .take(n)
            .map(|s| s.to_string())
            .collect()
    }

    #[divan::bench(args = SYSTEMS)]
    fn unparseable(bencher: Bencher, system: MeasurementSystem) {
        run(
            bencher,
            &system,
            &["salt", "a pinch of pepper", "to taste", "fresh basil"],
        );
    }

    #[divan::bench(args = SYSTEMS)]
    fn remapped_volume(bencher: Bencher, system: MeasurementSystem) {
        run(
            bencher,
            &system,
            &["2 tsp", "1 tbsp", "1 dessertspoon", "1 cup"],
        );
    }

    #[divan::bench(args = SYSTEMS)]
    fn imperial_volume(bencher: Bencher, system: MeasurementSystem) {
        run(
            bencher,
            &system,
            &["8 fl oz", "1 pint", "1 quart", "1 gallon"],
        );
    }

    #[divan::bench(args = SYSTEMS)]
    fn non_volume(bencher: Bencher, system: MeasurementSystem) {
        run(bencher, &system, &["500 g", "2 kg", "1 lb", "8 oz"]);
    }

    #[divan::bench(args = [0.0, 0.5, 1.0, 2.0, 1e12])]
    fn factor(bencher: Bencher, factor: f64) {
        const NUM_INPUTS: usize = 8;

        bencher
            .counter(ItemsCount::new(NUM_INPUTS))
            .with_inputs(|| {
                make_inputs(
                    &[
                        "2 tsp vanilla extract",
                        "pinch of salt",
                        "1 cup flour",
                        "500 g sausages",
                        "fresh basil",
                        "1 pint guiness",
                        "250 mL water",
                    ],
                    NUM_INPUTS,
                )
            })
            .bench_values(|input| MeasurementSystem::Metric.scale(input, black_box(factor)));
    }
}

#[divan::bench_group]
mod from_str_vec {
    use super::*;

    fn run(bencher: Bencher, ingredients: &[&str]) {
        bencher
            .counter(ItemsCount::new(ingredients.len()))
            .with_inputs(|| ingredients.to_vec())
            .bench_values(MeasurementSystem::from);
    }

    #[divan::bench]
    fn empty(bencher: Bencher) {
        run(bencher, &[])
    }

    #[divan::bench]
    fn imperial_uk(bencher: Bencher) {
        run(
            bencher,
            &[
                "2 lb beef chuck roast",
                "4 lb beef chuck roast",
                "1 stone potatoes",
                "8 fl oz beef stock",
                "4 oz carrots",
                "2 oz butter",
                "1 pint milk",
                "6 oz plain flour",
                "3 tbsp olive oil",
                "1 tsp salt",
                "½ tsp black pepper",
            ],
        )
    }

    #[divan::bench]
    fn metric(bencher: Bencher) {
        run(
            bencher,
            &[
                "1 kg chicken breast",
                "500 g pasta",
                "250 ml cream",
                "200 g mushrooms",
                "100 g parmesan cheese",
                "2 l vegetable broth",
                "30 ml olive oil",
                "15 g fresh basil",
                "5 g salt",
                "2 g black pepper",
            ],
        )
    }

    #[divan::bench]
    fn metric_australia(bencher: Bencher) {
        run(
            bencher,
            &[
                "800 g lamb leg",
                "400 g sweet potato",
                "375 ml coconut milk",
                "250 g green beans",
                "125 ml fish sauce",
                "60 ml lime juice",
                "20 ml sesame oil",
                "1 tbsp brown sugar",
                "1 tsp ground coriander",
                "½ tsp chili flakes",
            ],
        )
    }

    #[divan::bench]
    fn us_customary(bencher: Bencher) {
        run(
            bencher,
            &[
                "2 lbs ground turkey",
                "1 lb spaghetti",
                "8 oz cream cheese",
                "6 pt blue cheese",
                "6 floz water",
                "6gallons of gas",
                "1 foot bread",
                "½ cup grated cheddar",
                "¼ cup olive oil",
                "2 tbsp garlic powder",
                "1 tsp dried oregano",
                "½ tsp paprika",
            ],
        )
    }
}
