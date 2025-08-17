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
            use fastnum::{$dec, $D, decimal::RoundingMode};

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(UNSIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{$dec, $D, decimal::RoundingMode};

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

        #[rstest(::trace)]
        #[case($dec!(999999999999999999999999999999999999999999.9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999), 154)]
        #[case($dec!(100000000000000000000000000000000000000000.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000), 155)]
        #[case($dec!(134078079299425970995740249982058461274793.65820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095), 155)]
        fn test_digits_count_512(#[case] d: $D, #[case] digits_count: usize) {
            assert_eq!(d.digits_count(), digits_count);
        }
    };


    (COMMON:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D);
    };
    (COMMON:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(COMMON:: 128, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(999999999999999999999999999999999999999999.99999999999999999999999999999999999), 77)]
        #[case($dec!(100000000000000000000000000000000000000000.000000000000000000000000000000000000), 78)]
        #[case($dec!(115792089237316195423570985008687907853269.984665640564039457584007913129639935), 78)]
        fn test_digits_count_256(#[case] d: $D, #[case] digits_count: usize) {
            assert_eq!(d.digits_count(), digits_count);
        }
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
        fn test_eth() {
            let prec: u32 = 18;
            let mut amount = $dec!(20935.706972060549068014);
            for _ in 0..prec {
                amount *= $D::TEN;
            }
            assert_eq!($dec!(20935706972060549068014), amount);

            for _ in 0..prec {
                amount *= $dec!(0.1);
            }
            assert_eq!($dec!(20935.706972060549068014), amount);

            for _ in 0..prec {
                amount *= 10;
            }
            assert_eq!($dec!(20935706972060549068014), amount);
        }

        #[rstest(::trace)]
        fn test_base_math() {
            let d1 = $dec!(101).with_rounding_mode(RoundingMode::Down);
            let d2 = $dec!(0.01).with_rounding_mode(RoundingMode::Down);

            let d3 = (d1 / (1.0_f64 - d2)).round(8);
            assert_eq!(d3, $dec!(102.02020202));

            let d4 = d3 - d1;
            assert_eq!(d4, $dec!(1.02020202));

            let d5 = (d3 / d4).round(10);
            assert_eq!(d5, $dec!(100.0000000196));

            let d6 = d5 - $dec!(0.0000000196);
            assert_eq!(d6, $dec!(100));

            let d7 = d6 * $dec!(2.01);
            assert_eq!(d7, $dec!(201));
        }

        #[rstest(::trace)]
        #[case(vec![2.5, 0.3, 0.001], $dec!(2.801000011968426406383514404296875))]
        #[case(vec![0.1, 0.2], $dec!(0.300000004470348358154296875))]
        fn test_float_sum(#[case] vals: Vec<f32>, #[case] expected: $D) {
            let sum = vals.into_iter().map(|f| $D::try_from(f).unwrap()).sum();
            assert_eq!(expected, sum);
        }

        #[rstest(::trace)]
        #[case($dec!(0), 0, 0, 0)]
        #[case($dec!(0.5), 5, 1, 1)]
        #[case($dec!(1.0), 10, 2, 1)]
        #[case($dec!(1), 1, 1, 0)]
        #[case($dec!(7), 7, 1, 0)]
        #[case($dec!(10), 10, 2, 0)]
        #[case($dec!(1.1), 11, 2, 1)]
        #[case($dec!(1.23), 123, 3, 2)]
        #[case($dec!(123e5), 123, 3, -5)]
        #[case($dec!(8934), 8934, 4, 0)]
        #[case($dec!(999), 999, 3, 0)]
        #[case($dec!(1000), 1000, 4, 0)]
        #[case($dec!(9900), 9900, 4, 0)]
        #[case($dec!(9999), 9999, 4, 0)]
        #[case($dec!(10000), 10000, 5, 0)]
        #[case($dec!(99999), 99999, 5, 0)]
        #[case($dec!(100000), 100000, 6, 0)]
        #[case($dec!(999999), 999999, 6, 0)]
        #[case($dec!(1000000), 1000000, 7, 0)]
        #[case($dec!(9999999), 9999999, 7, 0)]
        #[case($dec!(999999999999), 999999999999, 12, 0)]
        #[case($dec!(18446744073709551615), 18446744073709551615, 20, 0)]
        fn test_digits(
            #[case] d: $D,
            #[case] digits: u64,
            #[case] digits_count: usize,
            #[case] fractional_digits_count: i16)
        {
            assert_eq!(d.digits(), digits.into());
            assert_eq!(d.digits_count(), digits_count);
            assert_eq!(d.fractional_digits_count(), fractional_digits_count);
        }

        #[rstest(::trace)]
        fn test_bug_shift() {
            let fee = $dec!(0e-22);
            let amount = $dec!(530188e-4);
            let res = amount / ($dec!(1) - fee);
            assert_eq!(res, $dec!(53.0188));
        }

        #[rstest(::trace)]
        #[case($dec!(0), 0)]
        #[case($dec!(1), 1)]
        #[case($dec!(2), 1)]
        #[case($dec!(10), 2)]
        #[case($dec!(11), 2)]
        #[case($dec!(1.1), 2)]
        #[case($dec!(1.00), 3)]
        #[case($dec!(1.23), 3)]
        #[case($dec!(9.99), 3)]
        #[case($dec!(1.000), 4)]
        #[case($dec!(1.234), 4)]
        #[case($dec!(9.999), 4)]
        #[case($dec!(1.0000), 5)]
        #[case($dec!(1.2345), 5)]
        #[case($dec!(9.9999), 5)]
        #[case($dec!(1.00000), 6)]
        #[case($dec!(1.23456), 6)]
        #[case($dec!(9.99999), 6)]
        #[case($dec!(1.000000), 7)]
        #[case($dec!(1.234567), 7)]
        #[case($dec!(9.999999), 7)]
        #[case($dec!(9999999999999999.9999999999999999999999), 38)]
        #[case($dec!(1000000000000000.00000000000000000000000), 39)]
        #[case($dec!(3402823669209384.63463374607431768211455), 39)]
        fn test_digits_count(#[case] d: $D, #[case] digits_count: usize) {
            assert_eq!(d.digits_count(), digits_count);
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

    };
}

pub(crate) use test_impl;
