use std::str::FromStr;

use rstest::*;

use fastnum::{decimal::signed::Sign, u128, D128, U128};

use crate::decimal::common::parse::{test_impl, test_impl_signed};

test_impl!(u128, U128, D128);
test_impl_signed!(u128, U128, D128);

#[rstest(::trace)]
#[case("340282366920938463463374607431768211455", Sign::NoSign, u128!(340282366920938463463374607431768211455), 0)]
#[case("-34028236692093846346337460743176821145.5", Sign::Minus, u128!(340282366920938463463374607431768211455), -1)]
#[case("+34028236692093846346337460743176821145.5e1000", Sign::Plus, u128!(340282366920938463463374607431768211455), 999)]
#[case("-340282366920938463.463374607431768211455e-1000", Sign::Minus, u128!(340282366920938463463374607431768211455), -1021)]
fn test_parse_ok_128(#[case] s: &str, #[case] sign: Sign, #[case] _int: U128, #[case] exp: i64) {
    let dec = D128::from_str(s).unwrap();
    assert_eq!(dec.significant_digits(), _int);
    assert_eq!(dec.fractional_digit_count(), -exp);
    assert_eq!(sign, dec.sign());
}

#[rstest(::trace)]
#[case("-340282366920938463463374607431768211456")]
#[case("-340282366920938463463374607431768211455.5")]
#[case("-340282366920938463.4633746074317682114551")]
#[case("-1157920892373161954235709850086879078532699846656405640394575840079131296399351")]
#[case("-115792089237316195423570985008687907853269984665640564039457584007913129639935.1")]
#[should_panic(expected = "(fastnum) number too small to fit in target type")]
fn test_parse_overflow_128(#[case] s: &str) {
    let _ = D128::from_str(s).unwrap();
}
