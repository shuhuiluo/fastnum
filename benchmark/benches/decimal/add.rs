mod _macro;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::{hint::black_box, str::FromStr};

criterion_group!(add, bench);
criterion_main!(add);

use _macro::benchmark_op;

macro_rules! benchmark {
    ($case: literal, $group: ident, [$($bits: literal),*], $a: literal, $b: literal $($arb: ident)?) => {
        benchmark_op!(+, $case, $group, [$($bits),*], $a, $b $($arb)?);
    };
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("a+b");

    benchmark!("C1", group, [64, 128], "1", "1");
    benchmark!("C2", group, [64, 128], "1", "10");
    benchmark!("C3", group, [64, 128], "-5", "2.5");
    benchmark!("C4", group, [64, 128], "123", "0.00000000025");
    benchmark!("C5", group, [128], "500549251119075878721813", "209481029831");
    benchmark!("C6", group, [128], "14028236093846.346337460743176821145", "140282366920934633.68211455");
    benchmark!("C7", group, [256], "340282366920938463463374607431768211455", "340282366920938463463374607431768211455.5" A);
    benchmark!("C8", group, [512], "1.414213562373095048801688724209698078569671875376948073176679730000000000000000000000000000000000000", "1.41421356237309504880168872420969807856967187537694807317667974000000000" A);

    group.finish();
}
