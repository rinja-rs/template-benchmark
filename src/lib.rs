use std::hint::black_box;
use std::time::{Duration, Instant};

use ahash::RandomState;
use criterion::{Bencher, Criterion};
use tmpls::{Benchmark, BigTable, Output, Team, Teams};

macro_rules! for_each {
    ($mod:ident, $c:ident, $input:ident : $Input:ident, $func:ident) => {{
        let mut this = $mod::Benchmark::default();
        $c.bench_function(
            stringify!($mod),
            |b| run(
                b,
                &mut this,
                &$input,
                |this, output, input| this.$func(output, input),
            ),
        );
    }};

    ($c:ident, $input:ident : $Input:ident, $func:ident) => {{
        let mut group = $c.benchmark_group(stringify!($func));

        #[cfg(feature = "askama")]
        for_each!(askama, group, $input:$Input, $func);
        #[cfg(feature = "horrorshow")]
        for_each!(horrorshow, group, $input:$Input, $func);
        #[cfg(feature = "markup")]
        for_each!(markup, group, $input:$Input, $func);
        #[cfg(feature = "minijinja")]
        for_each!(minijinja, group, $input:$Input, $func);
        #[cfg(feature = "rinja")]
        for_each!(rinja, group, $input:$Input, $func);
        #[cfg(feature = "tera")]
        for_each!(tera, group, $input:$Input, $func);
        #[cfg(feature = "tinytemplate")]
        for_each!(tinytemplate, group, $input:$Input, $func);

        group.finish();
    }};
}

pub fn big_table(c: &mut Criterion) {
    const SIZE: usize = 100;

    let mut table = Vec::with_capacity(SIZE);
    for _ in 0..SIZE {
        let mut inner = Vec::with_capacity(SIZE);
        for i in 0..SIZE {
            inner.push(i);
        }
        table.push(inner);
    }
    let input = BigTable { table };

    for_each!(c, input:BigTable, big_table);
}

pub fn teams(c: &mut Criterion) {
    let input = Teams {
        year: 2015,
        teams: vec![
            Team {
                name: "Jiangsu".into(),
                score: 43,
            },
            Team {
                name: "Beijing".into(),
                score: 27,
            },
            Team {
                name: "Guangzhou".into(),
                score: 22,
            },
            Team {
                name: "Shandong".into(),
                score: 12,
            },
        ],
    };

    for_each!(c, input:Teams, teams);
}

fn run<B: Benchmark, I>(
    b: &mut Bencher<'_>,
    this: &mut B,
    input: &I,
    func: impl Fn(&mut B, &mut B::Output, &I) -> Result<(), B::Error>,
) {
    let mut output = B::Output::default();
    func(this, &mut output, input).unwrap();
    let expected_hash = collect_output(&mut output);

    b.iter_custom(|iters| {
        let mut total = 0;
        for _ in 0..iters {
            let start = Instant::now();
            black_box(func(this, black_box(&mut output), black_box(input))).unwrap();
            total += start.elapsed().as_nanos() as u64;

            let hash = collect_output(&mut output);
            assert_eq!(
                hash, expected_hash,
                "hash mismatch: 0x{expected_hash:08x} != 0x{hash:08x}",
            );
        }
        Duration::from_nanos(total)
    });
}

fn collect_output(output: &mut impl Output) -> u64 {
    const PI: [u64; 4] = [
        0x243f_6a88_85a3_08d3,
        0x1319_8a2e_0370_7344,
        0xa409_3822_299f_31d0,
        0x082e_fa98_ec4e_6c89,
    ];

    let hash = RandomState::with_seeds(PI[0], PI[1], PI[2], PI[3]).hash_one(output.as_bytes());
    output.clear();
    hash
}
