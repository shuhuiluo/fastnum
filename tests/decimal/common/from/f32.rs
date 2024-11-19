macro_rules! test_impl {
    ($udec: ident, $UD: ident) => {
        $crate::decimal::common::from::float::test_ok_impl!(f32, test_from_f32_ok, $udec, $UD);
        $crate::decimal::common::from::float::test_infinity_impl!(f32, test_from_f32_infinity, $UD);
        $crate::decimal::common::from::float::test_nan_impl!(
            f32,
            0b0111_1111_1100_0000_0000_0000_0000_0000,
            test_from_f32_nan,
            $UD
        );

        #[rstest(::trace)]
        #[case(0.1, $udec!(0.100000001490116119384765625))]
        #[case(1e-1, $udec!(0.100000001490116119384765625))]
        #[case(2e-1, $udec!(0.20000000298023223876953125))]
        #[case(0.01, $udec!(0.00999999977648258209228515625))]
        #[case(1e-2, $udec!(0.00999999977648258209228515625))]
        #[case(0.001, $udec!(0.001000000047497451305389404296875))]
        #[case(1e-5, $udec!(0.00000999999974737875163555145263671875))]
        #[case(12.34, $udec!(12.340000152587890625))]
        #[case(0.3333333, $udec!(0.333333313465118408203125))]
        #[case(0.333333333333333333333333333333, $udec!(0.3333333432674407958984375))]
        #[case(1.0 / 3.0, $udec!(0.3333333432674407958984375))]
        #[case(core::f32::consts::PI, $udec!(3.1415927410125732421875))]
        #[case(core::f32::consts::PI * 10000.0, $udec!(31415.927734375))]
        #[case(core::f32::consts::PI * 30000.0, $udec!(94247.78125))]
        #[case(core::f32::consts::E, $udec!(2.71828174591064453125))]
        #[case(f32::EPSILON, $udec!(1.1920928955078125E-7))]
        #[case(3.0000000000000004, $udec!(3.0))]
        #[case(0.07155292, $udec!(0.07155291736125946044921875))]
        #[case(21509.2, $udec!(21509.19921875))]
        #[case(2289620000.0, $udec!(2289619968))]
        #[case(80000197e0, $udec!(80000200))]
        #[case(2.3283064e-10, $udec!(0.00000000023283064365386962890625))]
        #[case(0.14693861798803098, $udec!(0.146938621997833251953125))]
        #[case(1e20, $udec!(100000002004087734272))]
        #[case(1e30, $udec!(1000000015047466219876688855040))]
        #[case(1e38, $udec!(99999996802856924650656260769173209088))]
        #[case(317e36, $udec!(317000006395220278118691742155288870912))]
        #[case(6.99999952316, $udec!(6.999999523162841796875))]
        #[case(1.58456325029e+29, $udec!(158456325028528675187087900672))]
        #[case(4294967295., $udec!(4294967296))]
        fn test_from_f32_ok_ex(#[case] n: f32, #[case] expected: $UD) {
            let d = $UD::try_from(n).unwrap();
            assert_eq!(d, expected);
        }
    };
}

macro_rules! test_impl_unsigned {
    ($udec: ident, $UD: ident) => {
        $crate::decimal::common::from::float::test_impl_unsigned_negative!(
            f32,
            test_from_f32_negative,
            $udec,
            $UD
        );
    };
}

macro_rules! test_impl_signed {
    ($dec: ident, $D: ident) => {
        $crate::decimal::common::from::float::test_ok_impl_signed!(
            f32,
            32,
            test_from_f32_ok_signed,
            $dec,
            $D
        );
        $crate::decimal::common::from::float::test_infinity_impl_signed!(
            f32,
            32,
            test_from_f32_infinity_signed,
            $dec,
            $D
        );
    };
}

pub(crate) use test_impl;
pub(crate) use test_impl_signed;
pub(crate) use test_impl_unsigned;
