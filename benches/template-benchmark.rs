use std::time::Duration;

use criterion::{Criterion, PlottingBackend, criterion_group, criterion_main};
use template_benchmark::{big_table, teams};

criterion_main!(benches);
criterion_group! {
    name = benches;
    config = new_criterion();
    targets = big_table, teams
}

fn new_criterion() -> Criterion {
    Criterion::default()
        .sample_size(500)
        .confidence_level(0.98)
        .significance_level(0.02)
        .warm_up_time(Duration::from_secs(5))
        .plotting_backend(PlottingBackend::Gnuplot)
        .measurement_time(Duration::from_secs(60))
}
