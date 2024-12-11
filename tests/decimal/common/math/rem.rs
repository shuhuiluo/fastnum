macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!(SIGNED: $bits, [< dec $bits >], [<D $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: $bits, [< udec $bits >], [<UD $bits>]); }
    };
    (UNSIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{$dec, $D};

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(UNSIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{$dec, $D};

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(SIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (COMMON:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D);
    };
    (UNSIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D);
    };
    (SIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);
    };


    (COMMON:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D);
    };
    (COMMON:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(COMMON:: 128, $dec, $D);
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D);
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (SIGNED:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);
    };
    (SIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };

    (COMMON:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 128, $dec, $D);
    };
    (COMMON:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(1), $dec!(1), $dec!(0))]
        #[case($dec!(2), $dec!(1), $dec!(0))]
        #[case($dec!(1), $dec!(2), $dec!(1))]
        #[case($dec!(2), $dec!(2), $dec!(0))]
        #[case($dec!(0), $dec!(1), $dec!(0))]
        #[case($dec!(0), $dec!(2), $dec!(0))]
        #[case($dec!(1), $dec!(3), $dec!(1))]
        #[case($dec!(2), $dec!(3), $dec!(2))]
        #[case($dec!(3), $dec!(3), $dec!(0))]
        #[case($dec!(3), $dec!(2), $dec!(1))]
        #[case($dec!(100), $dec!(5), $dec!(0))]
        #[case($dec!(2e1), $dec!(1), $dec!(0))]
        #[case($dec!(1), $dec!(5e-1), $dec!(0.0))]
        #[case($dec!(15e-1), $dec!(1), $dec!(0.5))]
        #[case($dec!(1), $dec!(3e-2), $dec!(1e-2))]
        #[case($dec!(10), $dec!(3e-3), $dec!(0.001))]
        #[case($dec!(1234e-2), $dec!(1233e-3), $dec!(10e-3))]
        #[case($dec!(2.4), $dec!(1), $dec!(0.4))]
        #[case($dec!(2.40), $dec!(1), $dec!(0.40))]
        #[case($dec!(2.400), $dec!(1), $dec!(0.400))]
        #[case($dec!(2.4), $dec!(2), $dec!(0.4))]
        #[case($dec!(2.400), $dec!(2), $dec!(0.400))]
        #[case($dec!(2.0), $dec!(2), $dec!(0.0))]
        #[case($dec!(20), $dec!(20), $dec!(0))]
        fn test_rem(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a % b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());
            assert!(res.is_op_ok());

            let mut a = a;

            a %= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
            assert!(a.is_op_ok());
        }
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident) => {

    };
    (SIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(2.4), $dec!(-1), $dec!(0.4))]
        #[case($dec!(-2.4), $dec!(1), $dec!(-0.4))]
        #[case($dec!(-2.4), $dec!(-1), $dec!(-0.4))]
        #[case($dec!(-3), $dec!(2), $dec!(-1))]
        #[case($dec!(3), $dec!(-2), $dec!(1))]
        #[case($dec!(-9.5), $dec!(5.15), $dec!(-4.35))]
        fn test_rem_signed(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a % b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());
            assert!(res.is_op_ok());

            let mut a = a;

            a %= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
            assert!(a.is_op_ok());
        }
    };
}

pub(crate) use test_impl;
