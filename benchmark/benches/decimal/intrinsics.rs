use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use fastnum::{
    decimal::{Context, Sign},
    *,
};

criterion_group!(intrinsics, construct);
criterion_main!(intrinsics);

#[allow(dead_code)]
fn construct(c: &mut Criterion) {
    let mut group = c.benchmark_group("construct");

    group.bench_function("D64", |b| {
        b.iter(|| {
            D64::from_parts(
                black_box(U64::MAX),
                black_box(0),
                black_box(Sign::Plus),
                black_box(Context::default()),
            )
        })
    });

    group.bench_function("D128", |b| {
        b.iter(|| {
            D128::from_parts(
                black_box(U128::MAX),
                black_box(0),
                black_box(Sign::Plus),
                black_box(Context::default()),
            )
        })
    });

    group.bench_function("D256", |b| {
        b.iter(|| {
            D256::from_parts(
                black_box(U256::MAX),
                black_box(0),
                black_box(Sign::Plus),
                black_box(Context::default()),
            )
        })
    });

    group.bench_function("D512", |b| {
        b.iter(|| {
            D512::from_parts(
                black_box(U512::MAX),
                black_box(0),
                black_box(Sign::Plus),
                black_box(Context::default()),
            )
        })
    });

    group.bench_function("D1024", |b| {
        b.iter(|| {
            D1024::from_parts(
                black_box(U1024::MAX),
                black_box(0),
                black_box(Sign::Plus),
                black_box(Context::default()),
            )
        })
    });

    group.finish();
}
