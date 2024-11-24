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
            use fastnum::{$dec, $D};

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
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(100), $dec!(5), $dec!(0))]
        #[case($dec!(2e1), $dec!(1), $dec!(0))]
        #[case($dec!(2), $dec!(1), $dec!(0))]
        #[case($dec!(1), $dec!(3), $dec!(1))]
        #[case($dec!(1), $dec!(5e-1), $dec!(0.0))]
        #[case($dec!(15e-1), $dec!(1), $dec!(0.5))]
        #[case($dec!(1), $dec!(3e-2), $dec!(1e-2))]
        #[case($dec!(10), $dec!(3e-3), $dec!(0.001))]
        #[case($dec!(3), $dec!(2), $dec!(1))]
        #[case($dec!(1234e-2), $dec!(1233e-3), $dec!(10e-3))]
        fn test_rem(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let prod = a % b;

            assert_eq!(prod, expected);
            assert_eq!(prod.fractional_digits_count(), expected.fractional_digits_count());

            let mut a = a;

            a %= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
        }
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(-3), $dec!(2), $dec!(-1))]
        #[case($dec!(3), $dec!(-2), $dec!(1))]
        #[case($dec!(3), $dec!(-2), $dec!(1))]
        #[case($dec!(-9.5), $dec!(5.15), $dec!(-4.35))]
        fn test_rem(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a % b;
        
            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());
        
            let mut a = a;
        
            a %= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
        }
    };
}

pub(crate) use test_impl;