use std::{hint::black_box, str::FromStr};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

criterion_group!(mul, bench);
criterion_main!(mul);

macro_rules! macro_impl {
    ($group: ident, $bits: literal, $a: literal, $b: literal) => {{
        let ctx = fastnum::decimal::Context::default();

        let a = fastnum::decimal::Decimal::<{ $bits / 64 }>::from_str($a, ctx).unwrap();
        let b = fastnum::decimal::Decimal::<{ $bits / 64 }>::from_str($b, ctx).unwrap();

        let size = (a * b).digits_count();

        let a_f64 = f64::from_str($a).unwrap();
        let b_f64 = f64::from_str($b).unwrap();

        let a_bd = bigdecimal::BigDecimal::from_str($a).unwrap();
        let b_bd = bigdecimal::BigDecimal::from_str($b).unwrap();

        let a_rd = rust_decimal::Decimal::from_str($a).unwrap();
        let b_rd = rust_decimal::Decimal::from_str($b).unwrap();

        $group.bench_with_input(
            BenchmarkId::new("f64", size),
            &(a_f64, b_f64),
            |bench, (a, b)| bench.iter(|| black_box(*a) * black_box(*b)),
        );

        $group.bench_with_input(
            BenchmarkId::new("rust_decimal", size),
            &(a_rd, b_rd),
            |bench, (a, b)| bench.iter(|| black_box(*a) * black_box(*b)),
        );

        $group.bench_with_input(
            BenchmarkId::new("fastnum", size),
            &(a, b),
            |bench, (a, b)| bench.iter(|| black_box(*a) * black_box(*b)),
        );

        $group.bench_with_input(
            BenchmarkId::new("bigdecimal", size),
            &(a_bd, b_bd),
            |bench, (a, b)| bench.iter(|| black_box(a) * black_box(b)),
        );
    }};
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("a*b");

    macro_impl!(group, 128, "-1.1", "2.5");
    macro_impl!(group, 128, "-1.175470587012343730098", "577575785");
    macro_impl!(
        group,
        256,
        "-8.37664968",
        "-1.9086963714056968482094712882596748"
    );
    macro_impl!(
        group,
        512,
        "15.988480848752691653730876239769592670324064",
        "-1.15988480848752691653730876239769592670324064"
    );

    group.finish();
}
