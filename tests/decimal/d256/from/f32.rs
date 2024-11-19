use rstest::*;

use fastnum::{dec256, D256};

use crate::decimal::common::from::f32::{test_impl, test_impl_signed};

test_impl!(dec256, D256);
test_impl_signed!(dec256, D256);

#[rstest(::trace)]
#[case(f32::MAX, dec256!(340282346638528859811704183484516925440))]
fn test_from_f32_ok_256(#[case] n: f32, #[case] expected: D256) {
    let d = D256::try_from(n).unwrap();
    assert_eq!(d, expected);

    let n = f32::from_bits(n.to_bits() | (1 << 31));
    let d = D256::try_from(n).unwrap();
    assert_eq!(d, expected.neg());
}

#[rstest(::trace)]
#[case(f32::MIN_POSITIVE)]
#[should_panic(expected = "(fastnum) number too large to fit in target type")]
fn test_from_f32_overflow_256(#[case] n: f32) {
    let _ = D256::try_from(n).unwrap();
}

#[rstest(::trace)]
#[case(1.0e-40)]
#[case(1.0e-39)]
#[case(3.92E-39)]
#[case(1.40129846432e-45)]
#[case(1e-42)]
#[should_panic(expected = "(fastnum) number too large to fit in target type")]
fn test_from_f32_subnormal(#[case] n: f32) {
    assert!(n.is_subnormal());
    let _ = D256::try_from(n).unwrap();
}
