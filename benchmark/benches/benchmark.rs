mod allocation;
mod parse;

use criterion::{criterion_group, criterion_main};

criterion_group!(allocation, allocation::vector);
criterion_group!(parse, parse::from_str);

criterion_main!(allocation, parse);
