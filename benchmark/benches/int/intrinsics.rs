use num_integer::Integer;
use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use fastnum::*;

criterion_group!(
    intrinsics,
    power_of_ten,
    // decimal_digits,
    div_rem,
    // div_rem_digit,
    // remaining_decimal_digits,
    // cmp_gt
);
criterion_main!(intrinsics);

#[allow(dead_code)]
fn div_rem(c: &mut Criterion) {
    let mut group = c.benchmark_group("div_rem");

    let a_64 = 17014118346046923173;
    let a_64_fn = fastnum::U64::from_digit(a_64);
    let a_64_bn = bnum::BUint::<1>::from_digit(a_64);

    let b_64 = 100000000000;
    let b_64_fn = fastnum::U64::from_digit(b_64);
    let b_64_bn = bnum::BUint::<1>::from_digit(b_64);

    let a_128 = 170141183460469231731687303715884105728_u128;
    let a_128_fn = fastnum::U128::from_u128(a_128).unwrap();
    let a_128_bn = bnum::BUint::<2>::from_digits(*a_128_fn.digits());

    let b_128 = 165145986286327928205253101623384281993_u128;
    let b_128_fn = fastnum::U128::from_u128(b_128).unwrap();
    let b_128_bn = bnum::BUint::<2>::from_digits(*b_128_fn.digits());

    group.bench_with_input(
        BenchmarkId::new("uint", "64"),
        &(a_64, b_64),
        |bench, &(a, b)| bench.iter(|| black_box(a).div_rem(black_box(&b))),
    );

    group.bench_with_input(
        BenchmarkId::new("bnum", "64"),
        &(a_64_bn, b_64_bn),
        |bench, &(a, b)| bench.iter(|| black_box(a).div_rem(black_box(&b))),
    );

    group.bench_with_input(
        BenchmarkId::new("fastnum", "64"),
        &(a_64_fn, b_64_fn),
        |bench, &(a, b)| bench.iter(|| black_box(a).div_rem(black_box(b))),
    );

    group.bench_with_input(
        BenchmarkId::new("uint", "128"),
        &(a_128, b_128),
        |bench, &(a, b)| bench.iter(|| black_box(a).div_rem(black_box(&b))),
    );

    group.bench_with_input(
        BenchmarkId::new("bnum", "128"),
        &(a_128_bn, b_128_bn),
        |bench, &(a, b)| bench.iter(|| black_box(a).div_rem(black_box(&b))),
    );

    group.bench_with_input(
        BenchmarkId::new("fastnum", "128"),
        &(a_128_fn, b_128_fn),
        |bench, &(a, b)| bench.iter(|| black_box(a).div_rem(black_box(b))),
    );

    group.finish();
}

#[allow(dead_code)]
fn div_rem_digit(c: &mut Criterion) {
    let mut group = c.benchmark_group("div_rem_digit");

    let a_64 = fastnum::u64!(17014118346046923173);
    let a_128 = fastnum::u128!(170141183460469231731687303715884105728);
    let a_256 = fastnum::u256!(
        115792089237316195423570985008687907853269984665640564039457584007913129639935
    );
    let a_512 = fastnum::u512!(13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095);
    let b = 100000000000_u64;

    group.bench_with_input(BenchmarkId::new("div", "64"), &(), |bench, &_| {
        bench.iter(|| {
            let a = 17014118346046923173;
            let b = 100000000000_u64;

            black_box(a) / b
        })
    });
    group.bench_with_input(BenchmarkId::new("rem", "64"), &(), |bench, &_| {
        bench.iter(|| {
            let a = 17014118346046923173;
            let b = 100000000000_u64;

            black_box(a) % black_box(b)
        })
    });

    group.bench_with_input(BenchmarkId::new("div", "128"), &(), |bench, &_| {
        bench.iter(|| {
            let a = 170141183460469231731687303715884105728;
            let b = 100000000000_u128;

            black_box(a) / b
        })
    });

    group.bench_with_input(BenchmarkId::new("rem", "128"), &(), |bench, &_| {
        bench.iter(|| {
            let a = 170141183460469231731687303715884105728;
            let b = 100000000000_u128;

            black_box(a) % black_box(b)
        })
    });

    group.bench_with_input(
        BenchmarkId::new("div_rem_digit", "64"),
        &(a_64, b),
        |bench, &(a, b)| bench.iter(|| black_box(a).div_rem_digit(black_box(b))),
    );

    group.bench_with_input(
        BenchmarkId::new("div_rem_digit", "128"),
        &(a_128, b),
        |bench, &(a, b)| bench.iter(|| black_box(a).div_rem_digit(black_box(b))),
    );

    group.bench_with_input(
        BenchmarkId::new("div_rem_digit", "256"),
        &(a_256, b),
        |bench, &(a, b)| bench.iter(|| black_box(a).div_rem_digit(black_box(b))),
    );

    group.bench_with_input(
        BenchmarkId::new("div_rem_digit", "512"),
        &(a_512, b),
        |bench, &(a, b)| bench.iter(|| black_box(a).div_rem_digit(black_box(b))),
    );

    group.finish();
}

#[allow(dead_code)]
fn decimal_digits(c: &mut Criterion) {
    let mut group = c.benchmark_group("decimal_digits");

    for power in [1, 10000, 1000000000, 10000000000000000000] {
        let u = U64::from_digit(power);
        group.bench_with_input(BenchmarkId::new("D64", u), &u, |b, u| {
            b.iter(|| black_box(u).decimal_digits())
        });
    }

    for power in [
        1,
        100000,
        10000000000,
        10000000000000000000,
        100000000000000000000000000000000000000,
    ] {
        let u = U128::from_u128(power).unwrap();
        group.bench_with_input(BenchmarkId::new("U128", u), &u, |b, u| {
            b.iter(|| black_box(u).decimal_digits())
        });
    }
    group.finish();
}

#[allow(dead_code)]
fn remaining_decimal_digits(c: &mut Criterion) {
    let mut group = c.benchmark_group("remaining_decimal_digits");

    for power in [1, 10000, 1000000000, 10000000000000000000] {
        let u = U64::from_digit(power);
        group.bench_with_input(BenchmarkId::new("D64", u), &u, |b, u| {
            b.iter(|| black_box(u).remaining_decimal_digits())
        });
    }

    for power in [
        1,
        10000,
        1000000000,
        1000000000000000000,
        100000000000000000000000000000000000000,
    ] {
        let u = U128::from_u128(power).unwrap();
        group.bench_with_input(BenchmarkId::new("U128", u), &u, |b, u| {
            b.iter(|| black_box(u).remaining_decimal_digits())
        });
    }
    group.finish();
}

#[allow(dead_code)]
fn cmp_gt(c: &mut Criterion) {
    let mut group = c.benchmark_group("cmp_gt");

    for power in [1, 10000, 100000, 100000000000, 10000000000000000000] {
        let u = U64::from_digit(power);
        let mid = U64::MAX.shr(1);
        group.bench_with_input(BenchmarkId::new("D64", u), &u, |b, u| {
            b.iter(|| black_box(u).gt(black_box(&mid)))
        });
    }

    for power in [
        1,
        10000000000,
        100000000000000000000,
        1000000000000000000000000000000,
        100000000000000000000000000000000000000,
    ] {
        let u = U128::from_u128(power).unwrap();
        let mid = U128::MAX.shr(1);
        group.bench_with_input(BenchmarkId::new("U128", u), &u, |b, u| {
            b.iter(|| black_box(u).gt(black_box(&mid)))
        });
    }
    group.finish();
}

#[allow(dead_code)]
fn power_of_ten(c: &mut Criterion) {
    let mut group = c.benchmark_group("div");

    let u64 = u64::MAX;
    let uu64 = U64::from_u64(u64);

    let u128 = u128::MAX;
    let uu128 = U128::from_u128(u128).unwrap();

    group.bench_with_input(BenchmarkId::new("u64_const", 10), &u64, |b, x| {
        b.iter(|| black_box(x) / 10)
    });

    for n in [10, 100, 10000, 100000, 100000000000, 10000000000000000000] {
        let uuu64 = U64::from_u64(n);

        group.bench_with_input(BenchmarkId::new("u64", n), &(u64, n), |b, (x, y)| {
            b.iter(|| black_box(x) / y)
        });

        group.bench_with_input(BenchmarkId::new("U64", n), &(uu64, n), |b, (x, y)| {
            b.iter(|| black_box(x).div_digit(*y))
        });

        group.bench_with_input(BenchmarkId::new("U64_", n), &(uu64, uuu64), |b, (x, y)| {
            b.iter(|| black_box(x).div(*y))
        });
    }

    for n in [
        1,
        10,
        100,
        10000000000,
        100000000000000000000,
        1000000000000000000000000000000,
        100000000000000000000000000000000000000,
    ] {
        let uuu128 = U128::from_u128(n).unwrap();

        group.bench_with_input(BenchmarkId::new("u128", n), &(u128, n), |b, (x, y)| {
            b.iter(|| black_box(x) / y)
        });

        group.bench_with_input(BenchmarkId::new("U128_", n), &(uu128, uuu128), |b, (x, y)| {
            b.iter(|| black_box(x).div(*y))
        });
    }
    group.finish();
}
