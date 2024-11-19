use std::str::FromStr;

use criterion::{black_box, Criterion};

use bigdecimal::BigDecimal;
use fastnum::{udec128};

pub fn add(c: &mut Criterion) {
    let mut group = c.benchmark_group("a+b");

    group.bench_function("f64", |b| {
        b.iter(|| black_box(14028236093846.346337460743176821145_f64 + 140282366920934633.68211455_f64))
    });

    let a = udec128!(14028236093846.346337460743176821145);
    let b = udec128!(140282366920934633.68211455);
    group.bench_with_input("UD128", &(a, b), |bench, (a, b)| {
        bench.iter(|| black_box(*a + *b))
    });

    let a = BigDecimal::from_str("14028236093846.346337460743176821145").unwrap();
    let b = BigDecimal::from_str("140282366920934633.68211455").unwrap();

    group.bench_with_input("BigDecimal", &(a, b), |bench, (a, b)| {
        bench.iter(|| black_box(a + b))
    });

    group.finish();
}

pub fn div(c: &mut Criterion) {
    let mut group = c.benchmark_group("a/b");

    group.bench_function("f64", |b| {
        b.iter(|| black_box(500549251119075878721813_f64 / 209481029831_f64))
    });

    let a = udec128!(500549251119075878721813);
    let b = udec128!(209481029831);
    group.bench_with_input("UD128", &(a, b), |bench, (a, b)| {
        bench.iter(|| black_box(*a / *b))
    });

    let a = BigDecimal::from_str("500549251119075878721813").unwrap();
    let b = BigDecimal::from_str("209481029831").unwrap();

    group.bench_with_input("BigDecimal", &(a, b), |bench, (a, b)| {
        bench.iter(|| black_box(a / b))
    });

    group.finish();
}
