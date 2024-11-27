use rstest::*;

use fastnum::{
    dec128,
    decimal::{ArithmeticError, RoundingMode},
    D128,
};
use fastnum::decimal::{ArithmeticPolicy, OverflowPolicy, RoundingPolicy};
use crate::decimal::common::math::add::test_impl;

test_impl!(D, 128);
test_impl!(D, 256);
test_impl!(D, 512);

test_impl!(UD, 128);
test_impl!(UD, 256);
test_impl!(UD, 512);

#[rstest(::trace)]
#[case(dec128!(340282366920938463463374607431768211455), dec128!(0.5), dec128!(34028236692093846346337460743176821146e1))]
#[case(dec128!(340282366920938463463374607431768211455), dec128!(0.1), dec128!(340282366920938463463374607431768211455))]
#[case(dec128!(34028236692093846346337460743176821145), dec128!(0.01), dec128!(34028236692093846346337460743176821145.0))]
#[case(dec128!(34028236692093846346337460743176821145), dec128!(0.05), dec128!(34028236692093846346337460743176821145.1))]
#[case(dec128!(184467440737e3380), dec128!(0), dec128!(184467440737000000000000000000000000000e3353))]
#[case(dec128!(340282366920938463463374607431768211455), dec128!(340282366920938463463374607431768211455), dec128!(68056473384187692692674921486353642291e1))]
#[case(dec128!(0.340282366920938463463374607431768211455), dec128!(0.340282366920938463463374607431768211455), dec128!(0.68056473384187692692674921486353642291))]
fn test_add_inexact(#[case] a: D128, #[case] b: D128, #[case] expected: D128) {
    let res = a + b;

    assert_eq!(res, expected);
    assert_eq!(
        res.fractional_digits_count(),
        expected.fractional_digits_count()
    );
    
    let policy = ArithmeticPolicy::new(OverflowPolicy::Strict, RoundingPolicy::Strict);

    let res = a.add(b, RoundingMode::HalfUp);
    assert_eq!(res.ok_or_err_with_policy(&policy).unwrap_err(), ArithmeticError::Inexact);
}
