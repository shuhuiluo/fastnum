mod allocation;
mod math;
mod parse;

use criterion::{criterion_group, criterion_main};

criterion_group!(allocation, allocation::vector);
criterion_group!(parse, parse::from_str);
// criterion_group!(math, math::test/*, math::div, math::add*/);

// criterion_main!(/*parse, */math);
criterion_main!(allocation, parse);
