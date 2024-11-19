macro_rules! test_ok_impl {
    ($f: ident, $name: ident, $udec: ident, $UD: ident) => {
        #[rstest(::trace)]
        #[case(0., $udec!(0))]
        #[case(0.0, $udec!(0))]
        #[case(1., $udec!(1.0))]
        #[case(1.0, $udec!(1.0))]
        #[case(2.0, $udec!(2.0))]
        #[case(3.0, $udec!(3.0))]
        #[case(0.5, $udec!(0.5))]
        #[case(0.25, $udec!(0.25))]
        #[case(7.5, $udec!(7.5))]
        #[case(50., $udec!(50))]
        #[case(1234., $udec!(1234))]
        #[case(50000., $udec!(50000))]
        #[case(5.0 * 0.03125, $udec!(0.15625))]
        #[case(0.033203125, $udec!(0.033203125))]
        #[case(4.5, $udec!(4.5))]
        #[case(0.15625, $udec!(0.15625))]
        #[case(1401757440., $udec!(1401757440))]
        #[case(10000000., $udec!(10000000))]
        #[case(1048576., $udec!(1048576))]
        fn $name(#[case] n: $f, #[case] expected: $UD) {
            let d = $UD::try_from(n).unwrap();
            assert_eq!(d, expected);
        }
    };
}

macro_rules! test_infinity_impl {
    ($f: ident, $name: ident, $UD: ident) => {
        #[rstest(::trace)]
        #[should_panic(expected = "(fastnum) number is infinite")]
        fn $name() {
            let n = $f::INFINITY;
            let _ = $UD::try_from(n).unwrap();
        }
    };
}

macro_rules! test_nan_impl {
    ($f: ident, $bits: literal, $name: ident, $UD: ident) => {
        #[rstest(::trace)]
        #[case($f::NAN)]
        #[case($f::from_bits($bits))]
        #[should_panic(expected = "NaN")]
        fn $name(#[case] n: $f) {
            let _ = $UD::try_from(n).unwrap();
        }
    };
}

macro_rules! test_impl_unsigned_negative {
    ($f: ty, $name: ident, $udec: ident, $UD: ident) => {
        #[rstest(::trace)]
        #[case(-0.)]
        #[case(-0.0)]
        #[case(-1.0)]
        #[case($f::NEG_INFINITY)]
        #[case($f::MIN)]
        #[should_panic(expected = "(fastnum) number would be signed for unsigned type")]
        fn $name(#[case] n: $f) {
            let _ = $UD::try_from(n).unwrap();
        }
    };
}

macro_rules! test_ok_impl_signed {
    ($f: ident, $bits: literal, $name: ident, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case(0., $dec!(0))]
        #[case(0.0, $dec!(0))]
        #[case(1., $dec!(1.0))]
        #[case(1.0, $dec!(1.0))]
        #[case(2.0, $dec!(2.0))]
        #[case(3.0, $dec!(3.0))]
        #[case(0.5, $dec!(0.5))]
        #[case(0.25, $dec!(0.25))]
        #[case(7.5, $dec!(7.5))]
        #[case(50., $dec!(50))]
        #[case(1234., $dec!(1234))]
        #[case(50000., $dec!(50000))]
        #[case(5.0 * 0.03125, $dec!(0.15625))]
        #[case(0.033203125, $dec!(0.033203125))]
        #[case(4.5, $dec!(4.5))]
        #[case(0.15625, $dec!(0.15625))]
        #[case(1401757440., $dec!(1401757440))]
        #[case(10000000., $dec!(10000000))]
        fn $name(#[case] n: $f, #[case] expected: $D) {
            let n = $f::from_bits(n.to_bits() | (1 << ($bits - 1)));
            let d = $D::try_from(n).unwrap();
            assert_eq!(d, expected.neg());
        }
    };
}

macro_rules! test_infinity_impl_signed {
    ($f: ident, $bits: literal, $name: ident, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[should_panic(expected = "(fastnum) number is infinite")]
        fn $name() {
            let n = $f::NEG_INFINITY;
            let _ = $D::try_from(n).unwrap();
        }
    };
}

pub(crate) use test_infinity_impl;
pub(crate) use test_nan_impl;
pub(crate) use test_ok_impl;

pub(crate) use test_impl_unsigned_negative;

pub(crate) use test_infinity_impl_signed;
pub(crate) use test_ok_impl_signed;
