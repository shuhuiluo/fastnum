mod _macro;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::{hint::black_box, str::FromStr};

criterion_group!(mul, bench);
criterion_main!(mul);

use _macro::benchmark_op;

macro_rules! benchmark {
    ($case: literal, $group: ident, [$($bits: literal),*], $a: literal, $b: literal $($arb: ident)?) => {
        benchmark_op!(*, $case, $group, [$($bits),*], $a, $b $($arb)?);
    };
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("a*b");

    benchmark!("C1", group, [64, 128], "1", "1");
    benchmark!("C2", group, [64, 128], "1", "10");
    benchmark!("C3", group, [64, 128], "-5", "2.5");
    benchmark!("C4", group, [64, 128], "-5.25", "2.1234");
    benchmark!("C5", group, [64, 128], "123", "0.00000000025");
    benchmark!("C6", group, [64, 128], "789.012", "12.345");
    benchmark!("C7", group, [128], "1.175470587012343730098", "577575785");
    benchmark!("C8", group, [256], "8.37664968", "1.9086963714056968482094712882596748" A);
    benchmark!("C9", group, [512], "15.988480848752691653730876239769592670324064", "1.15988480848752691653730876239769592670324064" A);

    group.finish();
}
