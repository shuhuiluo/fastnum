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
        
        #[rstest(::trace)]
        #[case($dec!(340282366920938463463374607431768211455), $dec!(0.5), $dec!(340282366920938463463374607431768211455.5))]
        #[case($dec!(340282366920938463463374607431768211455), $dec!(0.05), $dec!(340282366920938463463374607431768211455.05))]
        fn test_add_256(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a + b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());

            let mut a = a;

            a += b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
        }
    };
    (SIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(0), $dec!(0), $dec!(0))]
        #[case($dec!(0), $dec!(0.00), $dec!(0.00))]
        #[case($dec!(1), $dec!(0), $dec!(1))]
        #[case($dec!(10), $dec!(0.00), $dec!(10.00))]
        #[case($dec!(12.34), $dec!(1.234), $dec!(13.574))]
        #[case($dec!(1234e6), $dec!(1234e-6), $dec!(1234000000.001234))]
        #[case($dec!(18446744073709551616.0), $dec!(1), $dec!(18446744073709551617.0))]
        #[case($dec!(0), $dec!(77.6), $dec!(77.6))]
        #[case($dec!(80802295e5), $dec!(0), $dec!(8080229500000))]
        #[case($dec!(23.9200), $dec!(0.0101), $dec!(23.9301))]
        #[case($dec!(46.636423395767125), $dec!(123), $dec!(169.636423395767125))]
        #[case($dec!(1.2345), $dec!(123.45), $dec!(124.6845))]
        #[case($dec!(123.43e5), $dec!(1.2345), $dec!(12343001.2345))]
        #[case($dec!(22132e2), $dec!(0.0000), $dec!(2213200.0000))]
        #[case($dec!(14028236093846.346337460743176821145), $dec!(140282366920934633.68211455), $dec!(140296395157028480.028452010743176821145))]
        fn test_add(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a + b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());

            let mut a = a;

            a += b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
        }
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(-0), $dec!(-0), $dec!(-0))]
        #[case($dec!(-1), $dec!(0), $dec!(-1))]
        #[case($dec!(12.34), $dec!(-1.234), $dec!(11.106))]
        #[case($dec!(12.34), $dec!(-12.34), $dec!(0))]
        #[case($dec!(23.9200), $dec!(-101), $dec!(-77.0800))]
        #[case($dec!(-316.79), $dec!(0e-6), $dec!(-316.790000))]
        fn test_add(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a + b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());

            let mut a = a;

            a += b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
        }
    };
}

pub(crate) use test_impl;
