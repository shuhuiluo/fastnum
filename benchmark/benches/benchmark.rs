mod allocation;
mod math;
mod parse;

use criterion::{criterion_group, criterion_main};

criterion_group!(allocation, allocation::vector);
criterion_group!(parse, parse::from_str);
criterion_group!(math, math::div, math::add);

criterion_main!(allocation, parse, math);
