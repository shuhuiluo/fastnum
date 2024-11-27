use rstest::*;

use fastnum::{
    dec128,
    decimal::{ArithmeticError, ArithmeticPolicy, OverflowPolicy, RoundingMode, RoundingPolicy},
    D128,
};

use crate::decimal::common::math::sub::test_impl;

test_impl!(D, 128);
test_impl!(D, 256);
test_impl!(D, 512);

test_impl!(UD, 128);
test_impl!(UD, 256);
test_impl!(UD, 512);

#[rstest(::trace)]
#[case(D128::from(i128::MAX), dec128!(0.1), D128::from(i128::MAX))]
#[case(D128::from(i128::MAX), dec128!(0.5), D128::from(i128::MAX - 1))]
#[case(D128::from(u128::MAX), dec128!(0.1), D128::from(u128::MAX))]
#[case(D128::from(u128::MAX), dec128!(0.5), D128::from(u128::MAX - 1))]
#[case(dec128!(340282366920938463463374607431768211455), dec128!(0.5), dec128!(340282366920938463463374607431768211454))]
#[case(dec128!(340282366920938463463374607431768211455), dec128!(0.1), dec128!(340282366920938463463374607431768211455))]
#[case(dec128!(34028236692093846346337460743176821145), dec128!(0.01), dec128!(34028236692093846346337460743176821145.0))]
#[case(dec128!(34028236692093846346337460743176821145), dec128!(0.05), dec128!(34028236692093846346337460743176821144.9))]
#[case(dec128!(184467440737e3380), dec128!(0), dec128!(184467440737000000000000000000000000000e3353))]
fn test_sub_128_inexact(#[case] a: D128, #[case] b: D128, #[case] expected: D128) {
    let res = a - b;
    assert_eq!(res, expected);

    let res = a.sub(b, RoundingMode::HalfUp);
    let policy = ArithmeticPolicy::new(OverflowPolicy::Strict, RoundingPolicy::Strict);
    assert_eq!(
        res.ok_or_err_with_policy(&policy).unwrap_err(),
        ArithmeticError::Inexact
    );
}
