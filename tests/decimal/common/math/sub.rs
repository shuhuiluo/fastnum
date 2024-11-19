macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: $bits, [< dec $bits >], [<D $bits>]); }
        paste::paste! { test_impl!(SIGNED: $bits, [< dec $bits >], [<D $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: $bits, [< udec $bits >], [<UD $bits>]); }
        paste::paste! { test_impl!(UNSIGNED ONLY: $bits, [< udec $bits >], [<UD $bits>]); }
    };
    (UNSIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{$dec, $D};

            super::test_impl!(UNSIGNED:: $bits, $dec, $D);
        }
    };
    (UNSIGNED ONLY: $bits: tt, $dec: ident, $D: ident) => {
        paste::paste! {
            mod [< $dec _unsigned >]{
                use rstest::*;
                use fastnum::{$dec, $D};

                super::test_impl!(UNSIGNED ONLY:: $bits, $dec, $D);
            }
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
    (UNSIGNED ONLY:: 512, $dec: ident, $D: ident) => {
        super::test_impl!(UNSIGNED ONLY:: 256, $dec, $D);
    };
    (SIGNED:: 512, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (UNSIGNED ONLY:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(UNSIGNED ONLY:: 128, $dec, $D);
    };
    (SIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(0), $dec!(0), $dec!(0))]
        #[case($dec!(1), $dec!(0), $dec!(1))]
        #[case($dec!(1.0), $dec!(0), $dec!(1.0))]
        #[case($dec!(1), $dec!(0.75), $dec!(0.25))]
        #[case($dec!(12.34), $dec!(1.234), $dec!(11.106))]
        #[case($dec!(1234e6), $dec!(1234e-6), $dec!(1233999999.998766))]
        #[case($dec!(85616001e4), $dec!(0), $dec!(856160010000))]
        fn test_sub(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a - b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());

            let mut a = a;

            a -= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
        }
    };
    (UNSIGNED ONLY:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(0), $dec!(2))]
        #[case($dec!(0.003), $dec!(0.3))]
        #[should_panic(expected = "(fastnum) operation has negative result for unsigned type")]
        fn test_sub_negative_panic(#[case] a: $D, #[case] b: $D) {
            let _ = a - b;
        }
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(0), $dec!(0), $dec!(0))]
        #[case($dec!(-1), $dec!(0), $dec!(-1))]
        #[case($dec!(-1), $dec!(0.75), $dec!(-1.75))]
        #[case($dec!(0), $dec!(2), $dec!(-2))]
        #[case($dec!(0.003), $dec!(0.3), $dec!(-0.297))]
        #[case($dec!(12.34), $dec!(-1.234), $dec!(13.574))]
        #[case($dec!(1234e-6), $dec!(1234e6), $dec!(-1233999999.998766))]
        #[case($dec!(712911676e-6), $dec!(4856259269250829), $dec!(-4856259269250116.088324))]
        #[case($dec!(0), $dec!(5207.07672), $dec!(-520707672e-5))]
        #[case($dec!(99291289e5), $dec!(0), $dec!(9929128900000))]
        #[case($dec!(0.7051277471570131), $dec!(1), $dec!(-0.2948722528429869))]
        #[case($dec!(40686030.22763836), $dec!(-10), $dec!(40686040.22763836))]
        fn test_sub(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a - b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());

            let mut a = a;

            a -= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
        }
    };
}

pub(crate) use test_impl;
