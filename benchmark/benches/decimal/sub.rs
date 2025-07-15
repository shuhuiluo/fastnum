use std::{hint::black_box, str::FromStr};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

criterion_group!(sub, bench);
criterion_main!(sub);

// use bigdecimal::BigDecimal;
// use criterion::{black_box, BenchmarkId, Criterion};
// use std::str::FromStr;
// 
// use fastnum::decimal::{Context, Decimal};
// 
macro_rules! macro_impl {
    ($group: ident, $bits: literal, $a: literal, $b: literal) => {{
        let ctx = fastnum::decimal::Context::default();

        let a = fastnum::decimal::Decimal::<{ $bits / 64 }>::from_str($a, ctx).unwrap();
        let b = fastnum::decimal::Decimal::<{ $bits / 64 }>::from_str($b, ctx).unwrap();

        let size = (a - b).digits_count();

        let a_f64 = f64::from_str($a).unwrap();
        let b_f64 = f64::from_str($b).unwrap();

        let a_bd = bigdecimal::BigDecimal::from_str($a).unwrap();
        let b_bd = bigdecimal::BigDecimal::from_str($b).unwrap();
        
        let a_rd = rust_decimal::Decimal::from_str($a).unwrap();
        let b_rd = rust_decimal::Decimal::from_str($b).unwrap();

        $group.bench_with_input(
            BenchmarkId::new("f64", size),
            &(a_f64, b_f64),
            |bench, (a, b)| bench.iter(|| black_box(*a) - black_box(*b)),
        );
        
        $group.bench_with_input(
            BenchmarkId::new("rust_decimal", size),
            &(a_rd, b_rd),
            |bench, (a, b)| bench.iter(|| black_box(*a) - black_box(*b)),
        );

        $group.bench_with_input(
            BenchmarkId::new("fastnum", size),
            &(a, b),
            |bench, (a, b)| bench.iter(|| black_box(*a) - black_box(*b)),
        );

        $group.bench_with_input(
            BenchmarkId::new("bigdecimal", size),
            &(a_bd, b_bd),
            |bench, (a, b)| bench.iter(|| black_box(a) - black_box(b)),
        );
    }};
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("a-b");

    macro_impl!(group, 128, "5", "2.5");
    macro_impl!(group, 128, "4565645", "2.46665");
    macro_impl!(group, 128, "500549251119075878721813", "209481029831");
    macro_impl!(
        group,
        128,
        "14028236093846.346337460743176821145",
        "1.4028236692093463368211455"
    );
    macro_impl!(group, 256, "340282366920938463463374607431768211455", "3.5");
    macro_impl!(
        group,
        256,
        "3.1415926535897932384626433832795028841971693993751058209749445923078164062862",
        "2.5"
    );
    macro_impl!(group, 512, "1.41", "3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067982148086513282306647093844609550582231725359408128481");

    group.finish();
}
