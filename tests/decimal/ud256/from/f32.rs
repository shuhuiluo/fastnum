use rstest::*;

use fastnum::{udec256, UD256};

use crate::decimal::common::from::f32::{test_impl, test_impl_unsigned};

test_impl!(udec256, UD256);
test_impl_unsigned!(udec256, UD256);

#[rstest(::trace)]
#[case(f32::MAX, udec256!(340282346638528859811704183484516925440))]
fn test_from_f32_ok_ex_256(#[case] n: f32, #[case] expected: UD256) {
    let d = UD256::try_from(n).unwrap();
    assert_eq!(d, expected);

    let n = f32::from_bits(n.to_bits() | (1 << 31));
    let r = UD256::try_from(n);
    assert!(r.is_err());
}

#[rstest(::trace)]
#[case(f32::MIN_POSITIVE)]
#[should_panic(expected = "(fastnum) number too large to fit in target type")]
fn test_from_f32_overflow_256(#[case] n: f32) {
    let _ = UD256::try_from(n).unwrap();
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
    let _ = UD256::try_from(n).unwrap();
}
