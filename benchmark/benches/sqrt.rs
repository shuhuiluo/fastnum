use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fastnum::dec128;

criterion_group!(sqrt, sqrt_bench);
criterion_main!(sqrt);

fn sqrt_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("sqrt");

    let x = dec128!(3);
    let x_f64 = 3.0_f64;
    let x_bd = bigdecimal::BigDecimal::try_from(3.0_f64).unwrap();

    group.bench_with_input(BenchmarkId::new("f64", "3"), &x_f64, |bench, x| {
        bench.iter(|| black_box(x.sqrt()))
    });

    group.bench_with_input(BenchmarkId::new("fastnum", "3"), &x, |bench, x| {
        bench.iter(|| black_box(x.sqrt()))
    });

    group.bench_with_input(BenchmarkId::new("bigdecimal", "3"), &x_bd, |bench, x| {
        bench.iter(|| black_box(x.sqrt().unwrap()))
    });

    group.finish();
}
