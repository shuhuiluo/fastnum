mod _macro;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::{hint::black_box, str::FromStr};

use _macro::benchmark_op;

macro_rules! benchmark {
    ($case: literal, $group: ident, [$($bits: literal),*], $a: literal, $b: literal $($arb: ident)?) => {
        benchmark_op!(/, $case, $group, [$($bits),*], $a, $b $($arb)?);
    };
}

criterion_group!(div, bench);
criterion_main!(div);

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("a/b");

    benchmark!("C1", group, [64, 128], "1", "1");
    benchmark!("C2", group, [64, 128], "1", "2.0");
    benchmark!("C3", group, [64, 128], "1", "3.0");
    benchmark!("C4", group, [64, 128], "1", "2.50");
    benchmark!("C5", group, [64, 128], "2.0", "25.0");
    benchmark!("C6", group, [64, 128], "100.0", "5.5");
    benchmark!("C7", group, [64, 128], "33", "12.5000");
    benchmark!("C8", group, [64, 128], "1.0", "4000.00");
    benchmark!("C9", group, [64, 128], "1.0", "4500.000");
    benchmark!("C10", group, [64, 128], "100000000", "3.0");
    benchmark!("C11", group, [64, 128], "1", "300000000.0");
    benchmark!("C12", group, [64, 128], "789.012", "12.345");
    benchmark!("C13", group, [64, 128], "789.0120", "12.345");
    benchmark!("C14", group, [64, 128], "789.01200", "12.345");
    benchmark!("C15", group, [64, 128], "789.012000", "12.345");
    benchmark!("C16", group, [64, 128], "789.0120000", "12.345");
    benchmark!(
        "C17",
        group,
        [128],
        "500549251119075878721813",
        "209481029831"
    );
    benchmark!("C18", group, [128], "0.3", "340282366920938463463374607431768211455" A);
    benchmark!("C19", group, [128], "340282366920938463463374607431768211455", "500549251119075878721813" A);

    group.finish();
}
