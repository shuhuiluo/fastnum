use std::str::FromStr;

use fastnum::{decimal::RoundingMode::No, *};

fn main() {
    println!("Competitors:");

    let a = rust_decimal::Decimal::from_str("789.0120").unwrap();
    let b = rust_decimal::Decimal::from_str("12.345").unwrap();

    let d = a / b;
    println!("rust_decimal:\t{a} / {b} = {d}");

    let a = dec64!(789.0120).with_rounding_mode(No);
    let b = dec64!(12.345).with_rounding_mode(No);

    let d = a / b;
    println!("fastnum (D64):\t{a} / {b} = {d} ({d:?})");

    let a = dec128!(789.0120).with_rounding_mode(No);
    let b = dec128!(12.345).with_rounding_mode(No);

    let d = a / b;
    println!("fastnum (D128):\t{a} / {b} = {d} ({d:?})");
}
