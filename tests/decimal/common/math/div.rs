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
            use fastnum::{*, decimal::{*, RoundingMode::*}};

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(UNSIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{*, decimal::{*, RoundingMode::*}};

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(SIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (COMMON:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(1), $dec!(3), $dec!(0.3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333), HalfUp)]
        #[case($dec!(1), $dec!(3), $dec!(0.3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333), Down)]
        #[case($dec!(1), $dec!(3), $dec!(0.3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333334), Up)]
        #[case($dec!(2), $dec!(3), $dec!(0.6666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666667), HalfUp)]
        #[case($dec!(2), $dec!(3), $dec!(0.6666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666), Down)]
        #[case($dec!(2), $dec!(3), $dec!(0.6666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666667), Up)]
        #[case($dec!(8), $dec!(9), $dec!(0.8888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888889), HalfUp)]
        #[case($dec!(8), $dec!(9), $dec!(0.8888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888888), Down)]
        #[case($dec!(12.34), $dec!(1.233), $dec!(10.008110300081103000811030008110300081103000811030008110300081103000811030008110300081103000811030008110300081103000811030008110300081103000811030008110300), HalfUp)]
        #[case($dec!(125348), $dec!(352.2283), $dec!(355.8714617763535752237966114591019517738921035021887792661748076460636467881768727839301952739175131583691600021917602872909416988924512879856615723381682), HalfUp)]
        fn test_div_inexact_512(
            #[case] a: $D,
            #[case] b: $D,
            #[case] expected: $D,
            #[case] mode: RoundingMode,
        ) {
            let ctx = Context::default().with_rounding_mode(mode);
            let res = a.div(b, ctx);

            assert_eq!(res, expected);
            assert_eq!(
                res.fractional_digits_count(),
                expected.fractional_digits_count()
            );

            assert!(res.is_op_inexact());
            assert!(res.is_op_rounded());
        }
    };
    (UNSIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D);
    };
    (SIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);
    };


    (COMMON:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(1), $dec!(3), $dec!(0.33333333333333333333333333333333333333333333333333333333333333333333333333333), HalfUp)]
        #[case($dec!(1), $dec!(3), $dec!(0.33333333333333333333333333333333333333333333333333333333333333333333333333333), Down)]
        #[case($dec!(1), $dec!(3), $dec!(0.33333333333333333333333333333333333333333333333333333333333333333333333333334), Up)]
        #[case($dec!(2), $dec!(3), $dec!(0.66666666666666666666666666666666666666666666666666666666666666666666666666667), HalfUp)]
        #[case($dec!(2), $dec!(3), $dec!(0.66666666666666666666666666666666666666666666666666666666666666666666666666666), Down)]
        #[case($dec!(2), $dec!(3), $dec!(0.66666666666666666666666666666666666666666666666666666666666666666666666666667), Up)]
        #[case($dec!(8), $dec!(9), $dec!(0.88888888888888888888888888888888888888888888888888888888888888888888888888889), HalfUp)]
        #[case($dec!(8), $dec!(9), $dec!(0.88888888888888888888888888888888888888888888888888888888888888888888888888888), Down)]
        #[case($dec!(12.34), $dec!(1.233), $dec!(10.0081103000811030008110300081103000811030008110300081103000811030008110300081), HalfUp)]
        #[case($dec!(125348), $dec!(352.2283), $dec!(355.87146177635357522379661145910195177389210350218877926617480764606364678818), HalfUp)]
        fn test_div_inexact_256(
            #[case] a: $D,
            #[case] b: $D,
            #[case] expected: $D,
            #[case] mode: RoundingMode,
        ) {
            let ctx = Context::default().with_rounding_mode(mode);
            let res = a.div(b, ctx);

            assert_eq!(res, expected);
            assert_eq!(
                res.fractional_digits_count(),
                expected.fractional_digits_count()
            );

            assert!(res.is_op_inexact());
            assert!(res.is_op_rounded());
        }
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

        #[rstest(::trace)]
        #[case($dec!(1), $dec!(3), $dec!(0.333333333333333333333333333333333333333), HalfUp)]
        #[case($dec!(1), $dec!(3), $dec!(0.333333333333333333333333333333333333333), Down)]
        #[case($dec!(1), $dec!(3), $dec!(0.333333333333333333333333333333333333334), Up)]
        #[case($dec!(2), $dec!(3), $dec!(0.66666666666666666666666666666666666667), HalfUp)]
        #[case($dec!(2), $dec!(3), $dec!(0.66666666666666666666666666666666666666), Down)]
        #[case($dec!(2), $dec!(3), $dec!(0.66666666666666666666666666666666666667), Up)]
        #[case($dec!(8), $dec!(9), $dec!(0.88888888888888888888888888888888888889), HalfUp)]
        #[case($dec!(8), $dec!(9), $dec!(0.88888888888888888888888888888888888888), Down)]
        fn test_div_inexact_128(
            #[case] a: $D,
            #[case] b: $D,
            #[case] expected: $D,
            #[case] mode: RoundingMode,
        ) {
            let ctx = Context::default().with_rounding_mode(mode);
            let res = a.div(b, ctx);

            assert_eq!(res, expected);
            assert_eq!(
                res.fractional_digits_count(),
                expected.fractional_digits_count()
            );

            assert!(res.is_op_inexact());
            assert!(res.is_op_rounded());
        }
    };
    (COMMON:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(0), $dec!(1), $dec!(0))]
        #[case($dec!(0), $dec!(10), $dec!(0))]
        #[case($dec!(2), $dec!(1), $dec!(2))]
        #[case($dec!(2e1), $dec!(1), $dec!(2e1))]
        #[case($dec!(10), $dec!(10), $dec!(1))]
        #[case($dec!(100), $dec!(10.0), $dec!(1e1))]
        #[case($dec!(20.0), $dec!(200), $dec!(0.1))]
        #[case($dec!(4.0), $dec!(2), $dec!(2.0))]
        #[case($dec!(15.0), $dec!(3), $dec!(5.0))]
        #[case($dec!(1), $dec!(2), $dec!(0.5))]
        #[case($dec!(1), $dec!(4), $dec!(0.25))]
        #[case($dec!(1), $dec!(8), $dec!(0.125))]
        #[case($dec!(1), $dec!(25), $dec!(0.04))]
        #[case($dec!(2), $dec!(16), $dec!(0.125))]
        #[case($dec!(1), $dec!(1024), $dec!(0.0009765625))]
        #[case($dec!(1), $dec!(2e-2), $dec!(5e1))]
        #[case($dec!(1), $dec!(0.2), $dec!(5))]
        #[case($dec!(1.0), $dec!(0.02), $dec!(5e1))]
        #[case($dec!(1), $dec!(0.020), $dec!(5e1))]
        #[case($dec!(5.0), $dec!(4.00), $dec!(1.25))]
        #[case($dec!(5.0), $dec!(4.000), $dec!(1.25))]
        #[case($dec!(5), $dec!(4.000), $dec!(1.25))]
        #[case($dec!(5), $dec!(4), $dec!(125e-2))]
        #[case($dec!(100), $dec!(5), $dec!(20))]
        #[case($dec!(500549251119075878721813), $dec!(209481029831), $dec!(2389472934723))]
        #[case($dec!(500549251119075878721813), $dec!(2389472934723), $dec!(209481029831))]
        #[case($dec!(15.22756), $dec!(1.234), $dec!(12.34))]
        #[case($dec!(15.22756), $dec!(12.34), $dec!(1.234))]
        #[case($D::MAX, $D::MAX, $dec!(1))]
        fn test_div(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let prod = a / b;

            assert_eq!(prod, expected);
            assert_eq!(prod.fractional_digits_count(), expected.fractional_digits_count());
            assert!(prod.is_op_ok());

            let mut a = a;

            a /= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
            assert!(a.is_op_ok());
        }

        #[rstest(::trace)]
        #[case($dec!(0), $dec!(0))]
        #[case($D::MAX, $dec!(0))]
        #[case($D::MAX, $D::ZERO)]
        #[should_panic(expected = "(fastnum) division by zero")]
        fn test_div_zero_panic(#[case] a: $D, #[case] b: $D) {
            let _ = a / b;
        }

        #[rstest(::trace)]
        #[case($dec!(1), $dec!(3))]
        #[case($dec!(1), $dec!(7))]
        #[case($dec!(1), $dec!(9))]
        #[case($dec!(1), $dec!(73))]
        #[case($dec!(73), $dec!(72))]
        #[case($dec!(79), $dec!(12345))]
        #[case($dec!(2048), $dec!(1025))]
        #[case($dec!(99), $dec!(98))]
        #[case($dec!(999), $dec!(998))]
        #[case($dec!(5), $dec!(11))]
        #[case($dec!(5), $dec!(14))]
        #[case($dec!(5), $dec!(18))]
        #[case($dec!(7), $dec!(11))]
        #[case($dec!(25), $dec!(39))]
        #[case($dec!(36), $dec!(55))]
        #[case($dec!(222), $dec!(1111))]
        #[case($dec!(0.3), $dec!(340282366920938463463374607431768211455))]
        #[case($dec!(0.3), $dec!(34028236692093846346337460743176821159))]
        #[case($dec!(340282366920938463463374607431768211455), $dec!(9.99))]
        #[case($dec!(68056473384187692692674921486353642291), $dec!(11))]
        #[case($dec!(690564733841876926926749214863536423), $dec!(340282366920938463463374607431768211455))]
        #[case($dec!(340282366920938463463374607431768211455), $dec!(340282366920938463463374607431768211454))]
        #[case($dec!(340282366920938463463374607431768211454), $dec!(340282366920938463463374607431768211455))]
        fn test_div_inexact(#[case] a: $D, #[case] b: $D) {
            let res = a / b;

            assert!(res.is_op_inexact());
            assert!(res.is_op_rounded());
        }

        #[rstest(::trace)]
        #[case($dec!(1e-32767), $D::MAX)]
        #[should_panic(expected = "(fastnum) underflow was occurred while performing arithmetic operation")]
        fn test_div_underflow_panic(#[case] a: $D, #[case] b: $D) {
            let ctx = Context::default().with_signal_traps(SignalsTraps::default().set(Signal::OP_UNDERFLOW));
            let _ = with_context!(ctx, {
                a / b
            });
        }

        #[rstest(::trace)]
        #[case($D::MAX, $dec!(1e-32767))]
        #[should_panic(expected = "(fastnum) overflow was occurred while performing arithmetic operation")]
        fn test_div_overflow_panic(#[case] a: $D, #[case] b: $D) {
            let _ = a / b;
        }

        #[rstest(::trace)]
        #[case($D::NAN, $dec!(1))]
        #[case($dec!(1), $D::NAN)]
        #[case($D::NAN, $D::NAN)]
        #[should_panic(expected = "(fastnum) invalid operation")]
        fn test_div_nan_panic(#[case] a: $D, #[case] b: $D) {
            let _ = a / b;
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
        #[case($D::MIN, $dec!(1), $D::MIN)]
        #[case($D::MIN, $D::MIN, $dec!(1))]
        #[case($dec!(-50), $dec!(5), $dec!(-10))]
        #[case($dec!(200), $dec!(-5), $dec!(-40.))]
        fn test_div_signed(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a / b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());
            assert!(res.is_op_ok());

            let mut a = a;

            a /= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
            assert!(a.is_op_ok());
        }

        #[rstest(::trace)]
        #[case($dec!(0), $dec!(-0))]
        #[case($dec!(-0), $dec!(-0))]
        #[case($dec!(-0), $dec!(0))]
        #[should_panic(expected = "(fastnum) division by zero")]
        fn test_div_zero_panic_signed(#[case] a: $D, #[case] b: $D) {
            let _ = a / b;
        }
    };
}

pub(crate) use test_impl;
