use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

criterion_group!(intrinsics, div_rem_digit);
criterion_main!(intrinsics);

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
