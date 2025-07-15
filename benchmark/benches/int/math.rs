use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

criterion_group!(math, bench);
criterion_main!(math);

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("math");

    let a_u128 = 170_141_183_460_469_231_731_687_303_715_884_105_728u128;
    let b_u128 = 170_141_183_460_469_231_731_687_303_715_884_105_727u128;

    assert_eq!(u128::MAX, a_u128 + b_u128);

    let a_f = fastnum::u128!(170141183460469231731687303715884105728);
    let b_f = fastnum::u128!(170141183460469231731687303715884105727);

    group.bench_with_input(
        BenchmarkId::new("u128", "+"),
        &(a_u128, b_u128),
        |bench, &(a, b)| bench.iter(|| a.checked_add(black_box(b))),
    );

    group.bench_with_input(
        BenchmarkId::new("u128", "-"),
        &(a_u128, b_u128),
        |bench, &(a, b)| bench.iter(|| a.checked_sub(black_box(b))),
    );

    group.bench_with_input(
        BenchmarkId::new("u128", "*"),
        &(a_u128, b_u128),
        |bench, &(a, b)| bench.iter(|| a.overflowing_mul(black_box(b))),
    );

    group.bench_with_input(
        BenchmarkId::new("fastnum", "+"),
        &(a_f, b_f),
        |bench, &(a, b)| bench.iter(|| a.checked_add(black_box(b))),
    );

    group.bench_with_input(
        BenchmarkId::new("fastnum", "-"),
        &(a_f, b_f),
        |bench, &(a, b)| bench.iter(|| a.checked_sub(black_box(b))),
    );

    group.bench_with_input(
        BenchmarkId::new("fastnum", "*"),
        &(a_f, b_f),
        |bench, &(a, b)| bench.iter(|| a.overflowing_mul(black_box(b))),
    );

    group.finish();
}
