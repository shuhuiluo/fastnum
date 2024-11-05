use rstest::*;

use fastnum::{udec128, UD128};

use crate::decimal::common::from::f64::{test_impl, test_impl_unsigned};

test_impl!(udec128, UD128);
test_impl_unsigned!(udec128, UD128);

#[rstest(::trace)]
#[case(0.1)]
#[case(0.01)]
#[case(12.34)]
#[case(0.333333333333333333333333333333)]
#[case(1.0 / 3.0)]
#[case(core::f64::consts::PI)]
#[case(core::f64::consts::E)]
#[case(3.0000000000000004)]
#[case(0.07155292)]
#[case(21509.2)]
#[case(2.3283064e-10)]
#[case(0.14693861798803098)]
#[case(6.99999952316)]
#[should_panic(expected = "(fastnum) number too large to fit in target type")]
fn test_from_f64_overflow_128(#[case] n: f64) {
    let _ = UD128::try_from(n).unwrap();
}
