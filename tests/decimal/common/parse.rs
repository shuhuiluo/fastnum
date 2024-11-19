macro_rules! test_impl {
    ($uint: ident, $UINT: ident, $UD: ident) => {
        #[rstest(::trace)]
        #[case("0", $uint!(0), 0)]
        #[case("1", $uint!(1), 0)]
        #[case("01", $uint!(1), 0)]
        #[case("001", $uint!(1), 0)]
        #[case("0010", $uint!(10), 0)]
        #[case("1331.107", $uint!(1331107), -3)]
        #[case(".107",  $uint!(107), -3)]
        #[case("0.0000012345", $uint!(12345), -10)]
        #[case("5e9", $uint!(5), 9)]
        #[case("10000000000000000000", $uint!(10000000000000000000), 0)]
        #[case("10000000000000000001", $uint!(10000000000000000001), 0)]
        #[case("100000000000000000000", $uint!(100000000000000000000), 0)]
        #[case("0.000000000000000000000000000000000000001", $uint!(1), -39)]
        #[case("18446744073709551615", $uint!(18446744073709551615), 0)]
        #[case("18446744073709551616", $uint!(18446744073709551616), 0)]
        #[case("18_446_744_073_709_551_615", $uint!(18446744073709551615), 0)]
        #[case("20935706972060549068014", $uint!(20935706972060549068014), 0)]
        #[case("1.0", $uint!(10), -1)]
        #[case("2e1", $uint!(2), 1)]
        #[case("2e0", $uint!(2), 0)]
        #[case("0.00123", $uint!(123), -5)]
        #[case("123", $uint!(123), 0)]
        #[case("1230", $uint!(1230), 0)]
        #[case("12.3", $uint!(123), -1)]
        #[case("123e-1", $uint!(123), -1)]
        #[case("1.23e+1", $uint!(123), -1)]
        #[case("1.23E+3", $uint!(123), 1)]
        #[case("1.23E-8", $uint!(123), -10)]
        #[case("1.23E-10", $uint!(123), -12)]
        #[case("123_", $uint!(123), 0)]
        #[case("31_862_140.830_686_979", $uint!(31862140830686979), -9)]
        #[case("1_1.2_2", $uint!(1122), -2)]
        #[case("999.521_939", $uint!(999521939), -6)]
        #[case("679.35_84_03E-2", $uint!(679358403), -8)]
        #[case("271576662.__E4", $uint!(271576662), 4)]
        #[case("1_._2", $uint!(12), -1)]
        #[case("25.8", $uint!(258), -1)]
        #[case("0.000000034283", $uint!(34283), -12)]
        #[case("20935.706972060549068014", $uint!(20935706972060549068014), -18)]
        #[case("0.20935706972060549068014", $uint!(20935706972060549068014), -23)]
        fn test_parse_ok(#[case] s: &str, #[case] _int: $UINT, #[case] exp: i64) {
            let dec = $UD::from_str(s).unwrap();
            assert_eq!(dec.decimal_digits(), _int);
            assert_eq!(dec.fractional_digits_count(), -exp);
        }

        #[rstest(::trace)]
        #[case::empty("")]
        #[case::only_decimal_and_underscore("_._")]
        #[case::empty_exponent("123.123E")]
        #[case::only_decimal_point(".")]
        #[case::only_decimal_and_exponent(".e4")]
        #[case::only_exponent("e4")]
        #[should_panic(expected = "(fastnum) cannot parse decimal from empty string")]
        fn test_parse_empty(#[case] s: &str) {
            let _ = $UD::from_str(s).unwrap();
        }

        #[rstest(::trace)]
        #[case::hello("hello")]
        #[case::nan("nan")]
        #[case::several_dots("123.45.67")]
        #[case::invalid_char("12z3.12")]
        #[case::invalid_char("12💖3.12")]
        #[case::invalid_char("💖")]
        #[case::nan_exponent("123.123eg")]
        #[case::multiple_decimal_points("123.12.45")]
        #[case::string_hex("0xCafeBeef")]
        #[case::several_exponent("123.34e-1e-2")]
        #[case::invalid_exponent("123.34e-1.5")]
        #[should_panic(expected = "(fastnum) invalid literal found in string")]
        fn test_parse_invalid_digit(#[case] s: &str) {
            let _ = $UD::from_str(s).unwrap();
        }

        #[rstest(::trace)]
        #[case("1e-9223372036854775808")]
        #[case("1e9223372036854775809")]
        #[should_panic(expected = "(fastnum) exponent is too large to fit in target type")]
        fn test_parse_exponent_overflow(#[case] s: &str) {
            let _ = $UD::from_str(s).unwrap();
        }
    };
}

macro_rules! test_impl_unsigned {
    ($uint: ident, $UINT: ident, $UD: ident) => {
        #[rstest(::trace)]
        #[case::minus_sign("-0")]
        #[case::minus_sign("-1")]
        #[case::minus_sign("-1.434343")]
        #[case::plus_sign("+0")]
        #[case::plus_sign("+1")]
        #[case::plus_sign("+35.55")]
        #[should_panic(expected = "(fastnum) invalid literal found in string")]
        fn test_parse_unsigned(#[case] s: &str) {
            let _ = $UD::from_str(s).unwrap();
        }
    };
}

macro_rules! test_impl_signed {
    ($uint: ident, $UINT: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case("-0", Sign::Minus, $uint!(0), 0)]
        #[case("+0", Sign::Plus, $uint!(0), 0)]
        #[case("1", Sign::NoSign, $uint!(1), 0)]
        #[case("-1", Sign::Minus, $uint!(1), 0)]
        #[case("+1", Sign::Plus, $uint!(1), 0)]
        #[case("+01", Sign::Plus, $uint!(1), 0)]
        #[case("-.107", Sign::Minus, $uint!(107), -3)]
        #[case("-0.000000000000000000000000000000000000001", Sign::Minus, $uint!(1), -39)]
        #[case("-123", Sign::Minus, $uint!(123), 0)]
        #[case("-1230", Sign::Minus, $uint!(1230), 0)]
        #[case("-1.23E-10", Sign::Minus, $uint!(123), -12)]
        #[case("-1_1.2_2", Sign::Minus, $uint!(1122), -2)]
        fn test_parse_ok_signed(
            #[case] s: &str,
            #[case] sign: Sign,
            #[case] _int: $UINT,
            #[case] exp: i64,
        ) {
            let dec = $D::from_str(s).unwrap();
            assert_eq!(dec.decimal_digits(), _int);
            assert_eq!(dec.sign(), sign);
            assert_eq!(dec.fractional_digits_count(), -exp);
        }

        #[rstest(::trace)]
        #[case::minus_sign("-0-1")]
        #[case::minus_sign("-1+")]
        #[case::plus_sign("+0+1")]
        #[case::plus_sign("--1")]
        #[case::plus_sign("+-35.55")]
        #[should_panic(expected = "(fastnum) invalid literal found in string")]
        fn test_parse_invalid_signed(#[case] s: &str) {
            let _ = $D::from_str(s).unwrap();
        }

        #[rstest(::trace)]
        #[case::invalid_exponent("-1e9223372036854775809")]
        #[case::invalid_exponent("-1e-9223372036854775808")]
        #[should_panic(expected = "(fastnum) exponent is too large to fit in target type")]
        fn test_parse_exponent_overflow_signed(#[case] s: &str) {
            let _ = $D::from_str(s).unwrap();
        }
    };
}

pub(crate) use test_impl;
pub(crate) use test_impl_signed;
pub(crate) use test_impl_unsigned;
