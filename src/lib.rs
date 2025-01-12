use std::hint::black_box;
use std::time::{Duration, Instant};

use ahash::RandomState;
use criterion::{Bencher, Criterion};
use tmpls::{Benchmark, BigTable, Output, Teams};

macro_rules! for_each {
    ($mod:ident, $c:ident, $input:ident : $Input:ident, $func:ident) => {{
        let mut this = $mod::Benchmark::default();
        $c.bench_function(
            env!(concat!("VERSION_", stringify!($mod))),
            |b| run(
                b,
                &mut this,
                &$input,
                |this, output, input| this.$func(output, input),
            ),
        );
    }};

    ($c:ident, $input:ident : $Input:ident, $func:ident) => {{
        #[cfg(feature = "_contains_compiled")]
        let _ = {
            #[cfg(not(feature = "_contains_interpreted"))]
            const NAME: &str = stringify!($func);
            #[cfg(feature = "_contains_interpreted")]
            const NAME: &str = concat!(stringify!($func), " (compiled)");

            let mut group = $c.benchmark_group(NAME);

            #[cfg(feature = "askama")]
            for_each!(askama, group, $input:$Input, $func);
            #[cfg(feature = "horrorshow")]
            for_each!(horrorshow, group, $input:$Input, $func);
            #[cfg(feature = "markup")]
            for_each!(markup, group, $input:$Input, $func);
            #[cfg(feature = "maud")]
            for_each!(maud, group, $input:$Input, $func);
            #[cfg(feature = "rinja")]
            for_each!(rinja, group, $input:$Input, $func);
            #[cfg(feature = "rinja_git")]
            for_each!(rinja_git, group, $input:$Input, $func);
            #[cfg(feature = "ructe")]
            for_each!(ructe, group, $input:$Input, $func);
            #[cfg(feature = "sailfish")]
            for_each!(sailfish, group, $input:$Input, $func);

            group.finish();
        };
        #[cfg(feature = "_contains_interpreted")]
        let _ = {
            #[cfg(not(feature = "_contains_compiled"))]
            const NAME: &str = stringify!($func);
            #[cfg(feature = "_contains_compiled")]
            const NAME: &str = concat!(stringify!($func), " (interpreted)");

            let mut group = $c.benchmark_group(NAME);

            #[cfg(feature = "handlebars")]
            for_each!(handlebars, group, $input:$Input, $func);
            #[cfg(feature = "minijinja")]
            for_each!(minijinja, group, $input:$Input, $func);
            #[cfg(feature = "tera")]
            for_each!(tera, group, $input:$Input, $func);
            #[cfg(feature = "tinytemplate")]
            for_each!(tinytemplate, group, $input:$Input, $func);
            #[cfg(feature = "upon")]
            for_each!(upon, group, $input:$Input, $func);

            group.finish();
        };
    }};

    ($c:ident, $Input:ident, $func:ident) => {{
        let input = <$Input>::default();
        for_each!($c, input:$Input, $func);
    }};
}

pub fn big_table(c: &mut Criterion) {
    for_each!(c, BigTable, big_table);
}

pub fn teams(c: &mut Criterion) {
    for_each!(c, Teams, teams);
}

fn run<B: Benchmark, I>(
    b: &mut Bencher<'_>,
    this: &mut B,
    input: &I,
    func: impl Fn(&mut B, &mut B::Output, &I) -> Result<(), B::Error>,
) {
    let mut output = B::Output::default();
    func(this, &mut output, input).unwrap();
    // dbg!(std::str::from_utf8(output.as_bytes()).unwrap());
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
