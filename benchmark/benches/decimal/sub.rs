mod _macro;

use std::{hint::black_box, str::FromStr};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

criterion_group!(sub, bench);
criterion_main!(sub);

use _macro::benchmark_op;

macro_rules! benchmark {
    ($case: literal, $group: ident, [$($bits: literal),*], $a: literal, $b: literal $($arb: ident)?) => {
        benchmark_op!(-, $case, $group, [$($bits),*], $a, $b $($arb)?);
    };
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("a-b");

    benchmark!("C1", group, [64, 128], "1", "1");
    benchmark!("C2", group, [64, 128], "5", "2.5");
    benchmark!("C3", group, [64, 128], "4565645", "2.46665");
    benchmark!("C4", group, [64, 128], "123", "0.00000000025");
    benchmark!("C5", group, [128], "500549251119075878721813", "209481029831");
    benchmark!("C6", group, [128], "14028236093846.346337460743176821145", "140282366920934633.68211455");
    benchmark!("C7", group, [256], "3.1415926535897932384626433832795028841971693993751058209749445923078164062862", "2.5" A);
    benchmark!("C8", group, [512], "3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067982148086513282306647093844609550582231725359408128481", "1.41" A);

    group.finish();
}
