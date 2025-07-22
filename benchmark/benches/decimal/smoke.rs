use std::{hint::black_box, str::FromStr};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

criterion_group!(smoke, jsilhan_bench);
criterion_main!(smoke);

/// Smoke benchmark by [Jan Šilhan](https://github.com/jsilhan).
///
/// Fastnum D64/D128 vs Rust Decimal Performance.
///
/// https://github.com/neogenie/fastnum/issues/32
///
/// String parsing: Fastnum 222.69ms vs Rust Decimal 105.50ms (2.11x slower)
///
/// Addition: Fastnum 25.50ms vs Rust Decimal 9.08ms (2.81x slower)
///
/// Subtraction: Fastnum 27.15ms vs Rust Decimal 9.63ms (2.82x slower)
///
/// Multiplication: Fastnum 35.88ms vs Rust Decimal 8.85ms (4.05x slower)
///
/// Division: Fastnum 1435.93ms vs Rust Decimal 71.45ms (20.10x slower)
fn jsilhan_bench(c: &mut Criterion) {
    let ctx = fastnum::decimal::Context::default();

    let a_fn_64 = fastnum::dec64!(789.012);
    let b_fn_64 = fastnum::dec64!(12.345);

    let a_fn_128 = fastnum::dec128!(789.012);
    let b_fn_128 = fastnum::dec128!(12.345);

    let a_rd = rust_decimal::Decimal::from_str("789.012").unwrap();
    let b_rd = rust_decimal::Decimal::from_str("12.345").unwrap();

    let mut group = c.benchmark_group("str_parse");

    for str in vec!["0.05", "-0.05", "123.456", "0.000001", "1000000.123456789"] {
        group.bench_with_input(
            BenchmarkId::new("rust_decimal", str.len()),
            str,
            |bench, str| bench.iter(|| rust_decimal::Decimal::from_str(black_box(str)).unwrap()),
        );

        group.bench_with_input(
            BenchmarkId::new("fastnum64", str.len()),
            str,
            |bench, str| bench.iter(|| fastnum::D64::from_str(black_box(str), ctx).unwrap()),
        );

        group.bench_with_input(
            BenchmarkId::new("fastnum128", str.len()),
            str,
            |bench, str| bench.iter(|| fastnum::D128::from_str(black_box(str), ctx).unwrap()),
        );
    }

    group.finish();

    let mut group = c.benchmark_group("add");

    group.bench_with_input("rust_decimal", &(a_rd, b_rd), |bench, (a, b)| {
        bench.iter(|| black_box(*a) + black_box(*b))
    });

    group.bench_with_input("fastnum64", &(a_fn_64, b_fn_64), |bench, (a, b)| {
        bench.iter(|| black_box(*a) + black_box(*b))
    });

    group.bench_with_input("fastnum128", &(a_fn_128, b_fn_128), |bench, (a, b)| {
        bench.iter(|| black_box(*a) + black_box(*b))
    });

    group.finish();

    let mut group = c.benchmark_group("sub");

    group.bench_with_input("rust_decimal", &(a_rd, b_rd), |bench, (a, b)| {
        bench.iter(|| black_box(*a) - black_box(*b))
    });

    group.bench_with_input("fastnum64", &(a_fn_64, b_fn_64), |bench, (a, b)| {
        bench.iter(|| black_box(*a) - black_box(*b))
    });

    group.bench_with_input("fastnum128", &(a_fn_128, b_fn_128), |bench, (a, b)| {
        bench.iter(|| black_box(*a) - black_box(*b))
    });

    group.finish();

    let mut group = c.benchmark_group("mul");

    group.bench_with_input("rust_decimal", &(a_rd, b_rd), |bench, (a, b)| {
        bench.iter(|| black_box(*a) * black_box(*b))
    });

    group.bench_with_input("fastnum64", &(a_fn_64, b_fn_64), |bench, (a, b)| {
        bench.iter(|| black_box(*a) * black_box(*b))
    });

    group.bench_with_input("fastnum128", &(a_fn_128, b_fn_128), |bench, (a, b)| {
        bench.iter(|| black_box(*a) * black_box(*b))
    });

    group.finish();

    let mut group = c.benchmark_group("div");

    group.bench_with_input("rust_decimal", &(a_rd, b_rd), |bench, (a, b)| {
        bench.iter(|| black_box(*a) / black_box(*b))
    });

    group.bench_with_input("fastnum64", &(a_fn_64, b_fn_64), |bench, (a, b)| {
        bench.iter(|| black_box(*a) / black_box(*b))
    });

    group.bench_with_input("fastnum128", &(a_fn_128, b_fn_128), |bench, (a, b)| {
        bench.iter(|| black_box(*a) / black_box(*b))
    });

    group.finish();
}
