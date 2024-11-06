use std::str::FromStr;

use criterion::{black_box, Criterion};

use bigdecimal::BigDecimal;
use fastnum::UD128;

pub fn from_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_str");

    group.bench_function("f64", |b| {
        b.iter(|| black_box(f64::from_str("0.12345678910111213").unwrap()))
    });

    group.bench_function("UD128", |b| {
        b.iter(|| black_box(UD128::from_str("0.12345678910111213").unwrap()))
    });

    group.bench_function("BigDecimal", |b| {
        b.iter(|| black_box(BigDecimal::from_str("0.12345678910111213").unwrap()))
    });

    group.finish();
}
