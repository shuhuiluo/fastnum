macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: $bits, [< dec $bits >], [<D $bits>]); }
        paste::paste! { test_impl!(SIGNED: $bits, [< dec $bits >], [<D $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: $bits, [< udec $bits >], [<UD $bits>]); }
    };
    (UNSIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{$dec, $D, decimal::{ArithmeticError, RoundingMode}};

            super::test_impl!(UNSIGNED:: $bits, $dec, $D);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        paste::paste! {
            mod [< $dec _signed >]{
                use rstest::*;
                use fastnum::{$dec, $D};

                super::test_impl!(SIGNED:: $bits, $dec, $D);
            }
        }
    };
    (UNSIGNED:: 512, $dec: ident, $D: ident) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D);
    };
    (SIGNED:: 512, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (SIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);

        // #[rstest(::trace)]
        // #[case($dec!(-995052931372975485719.533153137), $dec!(4.523087321), $dec!(-4500711297616988541501.836966993116075977))]
        // #[case($dec!(995052931372975485719.533153137), $dec!(-4.523087321), $dec!(-4500711297616988541501.836966993116075977))]
        // #[case($dec!(-8.37664968), $dec!(-1.9086963714056968482094712882596748), $dec!(15.988480848752691653730876239769592670324064))]
        // fn test_mul_256(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
        //
        // }
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident) => {
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

            let mut a = a;

            a /= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
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
        #[case($dec!(73), $dec!(72))]
        #[case($dec!(79), $dec!(12345))]
        #[case($dec!(2048), $dec!(1025))]
        fn test_div_inexact(#[case] a: $D, #[case] b: $D) {
            let _ = a / b;
            let res = a.div(b, RoundingMode::HalfUp);
            assert_eq!(res.ok_or_err().unwrap_err(), ArithmeticError::Inexact);
        }
        
        #[rstest(::trace)]
        #[case($dec!(1e-9223372036854775807), $D::MAX)]
        #[should_panic(expected = "(fastnum) attempt to perform the operation with overflow")]
        fn test_div_overflow_panic(#[case] a: $D, #[case] b: $D) {
            let _ = a / b;
        }
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($D::MIN, $dec!(1), $D::MIN)]
        #[case($D::MIN, $D::MIN, $dec!(1))]
        #[case($dec!(-50), $dec!(5), $dec!(-10))]
        #[case($dec!(200), $dec!(-5), $dec!(-40.))]
        fn test_div(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a / b;
        
            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());
        
            let mut a = a;
        
            a /= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
        }
    };
}

pub(crate) use test_impl;