use bigdecimal::BigDecimal;
use criterion::{black_box, BenchmarkId, Criterion};
use std::str::FromStr;

use fastnum::decimal::{Context, Decimal};

macro_rules! macro_impl {
    ($group: ident, $bits: literal, $a: literal, $b: literal) => {{
        let ctx = Context::default();

        let a = Decimal::<{ $bits / 64 }>::from_str($a, ctx).unwrap();
        let b = Decimal::<{ $bits / 64 }>::from_str($b, ctx).unwrap();

        let size = (a / b).digits_count();

        let a_f64 = f64::from_str($a).unwrap();
        let b_f64 = f64::from_str($b).unwrap();

        let a_bd = BigDecimal::from_str($a).unwrap();
        let b_bd = BigDecimal::from_str($b).unwrap();

        $group.bench_with_input(
            BenchmarkId::new("f64", size),
            &(a_f64, b_f64),
            |bench, (a, b)| bench.iter(|| black_box(*a / *b)),
        );

        $group.bench_with_input(
            BenchmarkId::new("fastnum", size),
            &(a, b),
            |bench, (a, b)| bench.iter(|| black_box(*a / *b)),
        );

        $group.bench_with_input(
            BenchmarkId::new("bigdecimal", size),
            &(a_bd, b_bd),
            |bench, (a, b)| bench.iter(|| black_box(a / b)),
        );
    }};
}

pub fn div(c: &mut Criterion) {
    let mut group = c.benchmark_group("a/b");

    macro_impl!(group, 128, "-5", "2.5");
    macro_impl!(group, 128, "500549251119075878721813", "209481029831");
    macro_impl!(group, 256, "-1", "3");
    macro_impl!(group, 512, "1", "-3");

    group.finish();
}
