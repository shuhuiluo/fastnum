macro_rules! test_impl {
    ($udec: ident, $UD: ident) => {
        $crate::decimal::common::from::float::test_ok_impl!(f64, test_from_f64_ok, $udec, $UD);
        $crate::decimal::common::from::float::test_infinity_impl!(f64, test_from_f64_infinity, $UD);
        $crate::decimal::common::from::float::test_nan_impl!(
            f64,
            0b0111_1111_1111_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
            test_from_f64_nan,
            $UD
        );

        #[rstest(::trace)]
        #[case(f64::EPSILON, $udec!(2.220446049250313080847263336181640625e-16))]
        #[case(2289620000.0, $udec!(2289620000))]
        #[case(80000197e0, $udec!(80000197))]
        #[case(1e20, $udec!(100000000000000000000))]
        #[case(1e30, $udec!(1000000000000000019884624838656))]
        #[case(1e38, $udec!(99999999999999997748809823456034029568))]
        #[case(317e36, $udec!(317000000000000010053141138001136451584))]
        #[case(4294967295., $udec!(4294967295))]
        #[case(1.58456325029e+29, $udec!(158456325029000005035589894144))]
        fn test_from_f64_ok_ex(#[case] n: f64, #[case] expected: $UD) {
            let d = $UD::try_from(n).unwrap();
            assert_eq!(d, expected);
        }

        #[rstest(::trace)]
        #[case(8.544283616667655e-306)]
        #[case(3e300)]
        #[case(2.81341650018752E-308)]
        #[case(f64::MAX)]
        #[should_panic(expected = "(fastnum) number too large to fit in target type")]
        fn test_from_f64_overflow_ex(#[case] n: f64) {
            let _ = $UD::try_from(n).unwrap();
        }

        #[rstest(::trace)]
        #[case(1.0e-308)]
        #[case(4.940656e-324)]
        #[should_panic(expected = "(fastnum) number too large to fit in target type")]
        fn test_from_f64_subnormal(#[case] n: f64) {
            assert!(n.is_subnormal());
            let _ = $UD::try_from(n).unwrap();
        }
    };
}

macro_rules! test_impl_unsigned {
    ($udec: ident, $UD: ident) => {
        $crate::decimal::common::from::float::test_impl_unsigned_negative!(
            f64,
            test_from_f64_negative,
            $udec,
            $UD
        );
    };
}

macro_rules! test_impl_signed {
    ($dec: ident, $D: ident) => {
        $crate::decimal::common::from::float::test_ok_impl_signed!(
            f64,
            64,
            test_from_f64_ok_signed,
            $dec,
            $D
        );
        $crate::decimal::common::from::float::test_infinity_impl_signed!(
            f64,
            64,
            test_from_f64_infinity_signed,
            $dec,
            $D
        );

        #[rstest(::trace)]
        #[case(f64::MIN)]
        #[should_panic(expected = "(fastnum) number too large to fit in target type")]
        fn test_from_f64_overflow_ex_signed(#[case] n: f64) {
            let _ = $D::try_from(n).unwrap();
        }
    };
}

pub(crate) use test_impl;
pub(crate) use test_impl_signed;
pub(crate) use test_impl_unsigned;
