use criterion::{criterion_group, criterion_main};
use template_benchmark::{big_table, teams};

criterion_main!(benches);
criterion_group!(benches, big_table, teams);
