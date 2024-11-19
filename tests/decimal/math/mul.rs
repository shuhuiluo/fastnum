use rstest::rstest;

use fastnum::{
    dec128,
    decimal::{ArithmeticError, RoundingMode},
    D128,
};

use crate::decimal::common::math::mul::test_impl;

test_impl!(D, 128);
test_impl!(D, 256);
test_impl!(D, 512);

test_impl!(UD, 128);
test_impl!(UD, 256);
test_impl!(UD, 512);

#[rstest(::trace)]
#[case(dec128!(340282366920938463463374607431768211455), dec128!(1.0), dec128!(340282366920938463463374607431768211455))]
#[case(dec128!(995052931372975485719.533153137), dec128!(4.523087321), dec128!(4500711297616988541501.8369669931160760))]
#[case(dec128!(8.37664968), dec128!(1.9086963714056968482094712882596748), dec128!(15.9884808487526916537308762397695926703))]
// #[case(dec128!(), dec128!(), dec128!())]
// #[case(dec128!(), dec128!(), dec128!())]
// #[case(dec128!(), dec128!(), dec128!())]
// #[case(dec128!(), dec128!(), dec128!())]
fn test_mul_inexact(#[case] a: D128, #[case] b: D128, #[case] expected: D128) {
    let res = a * b;

    assert_eq!(res, expected);
    assert_eq!(
        res.fractional_digits_count(),
        expected.fractional_digits_count()
    );

    let res = a.mul(b, RoundingMode::HalfUp);
    assert_eq!(res.ok_or_err().unwrap_err(), ArithmeticError::Inexact);
}
