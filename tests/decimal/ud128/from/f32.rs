use rstest::*;

use fastnum::{udec128, UD128};

use crate::decimal::common::from::f32::{test_impl, test_impl_unsigned};

test_impl!(udec128, UD128);
test_impl_unsigned!(udec128, UD128);

#[rstest(::trace)]
#[case(f32::MIN_POSITIVE)]
#[should_panic(expected = "(fastnum) number too large to fit in target type")]
fn test_from_f32_overflow_128(#[case] n: f32) {
    let _ = UD128::try_from(n).unwrap();
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
    let _ = UD128::try_from(n).unwrap();
}
