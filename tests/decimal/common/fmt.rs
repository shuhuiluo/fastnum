macro_rules! test_impl {
    ($udec: ident, $UD: ident) => {
        #[rstest(::trace)]
        #[case($udec!(0), "0")]
        #[case($udec!(0.00), "0.00")]
        #[case($udec!(1), "1")]
        #[case($udec!(10), "10")]
        #[case($udec!(0.123), "0.123")]
        #[case($udec!(0.0123), "0.0123")]
        #[case($udec!(0.00123), "0.00123")]
        #[case($udec!(0.000123), "0.000123")]
        #[case($udec!(1.23E-4), "0.000123")]
        #[case($udec!(123.), "123")]
        #[case($udec!(123.e1), "1230")]
        fn test_fmt(#[case] d: $UD, #[case] expected: &str) {
            let formated = format!("{d}");
            assert_eq!(formated.as_str(), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(0), concat!(stringify!($UD), r#"("0e0")"#))]
        #[case($udec!(1), concat!(stringify!($UD), r#"("1e0")"#))]
        #[case($udec!(123.400), concat!(stringify!($UD), r#"("123400e-3")"#))]
        #[case($udec!(123.4e-2), concat!(stringify!($UD), r#"("1234e-3")"#))]
        #[case($udec!(123.456), concat!(stringify!($UD), r#"("123456e-3")"#))]
        #[case($udec!(01.20), concat!(stringify!($UD), r#"("120e-2")"#))]
        #[case($udec!(1.20), concat!(stringify!($UD), r#"("120e-2")"#))]
        #[case($udec!(01.2E3), concat!(stringify!($UD), r#"("12e2")"#))]
        #[case($udec!(6.02214076e1023), concat!(stringify!($UD), r#"("602214076e1015")"#))]
        #[case($udec!(1e99999999999999), concat!(stringify!($UD), r#"("1e99999999999999")"#))]
        fn test_fmt_debug_alt(#[case] d: $UD, #[case] expected: &str) {
            let formated = format!("{d:#?}");
            assert_eq!(formated.as_str(), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(1),     "1",      "1.0",    "1.0000",   " 1.0",   "+01.0",   "1.0 ")]
        #[case($udec!(0.1),   "0.1",    "0.1",    "0.1000",   " 0.1",   "+00.1",   "0.1 ")]
        #[case($udec!(0.01),  "0.01",   "0.0",    "0.0100",   " 0.0",   "+00.0",   "0.0 ")]
        #[case($udec!(100),   "100",  "100.0",  "100.0000",  "100.0",  "+100.0",  "100.0")]
        fn test_fmt_options(
            #[case] d: $UD,
            #[case] expected: &str,
            #[case] expected_d1: &str,
            #[case] expected_d4: &str,
            #[case] expected_4d1: &str,
            #[case] expected_p05d1: &str,
            #[case] expected_l4d1: &str,
        ) {
            assert_eq!(format!("{}", d), expected);
            assert_eq!(format!("{:.1}", d), expected_d1);
            assert_eq!(format!("{:.4}", d), expected_d4);
            assert_eq!(format!("{:4.1}", d), expected_4d1);
            assert_eq!(format!("{:+05.1}", d), expected_p05d1);
            assert_eq!(format!("{:<4.1}", d), expected_l4d1);
        }

        #[rstest(::trace)]
        #[case($udec!(1), "1")]
        #[case($udec!(10), "10")]
        #[case($udec!(0.1), "0.1")]
        #[case($udec!(0.9), "0.9")]
        #[case($udec!(800e-3), "0.800")]
        #[case($udec!(123456), "123456")]
        #[case($udec!(9999999), "9999999")]
        #[case($udec!(19073.97235939614856), "19073.97235939614856")]
        #[case($udec!(1764031078e-13), "0.0001764031078")]
        #[case($udec!(1e15), "1000000000000000")]
        #[case($udec!(1e16), "1e+16")]
        #[case($udec!(491326e-12), "4.91326E-7")]
        #[case($udec!(0.00003102564500), "0.00003102564500")]
        #[case($udec!(1E-10000), "1E-10000")]
        #[case($udec!(1e100000), "1e+100000")]
        #[case($udec!(1234506789e5), "123450678900000")]
        #[case($udec!(1234506789e15), "1234506789000000000000000")]
        #[case($udec!(1234506789e16), "1234506789e+16")]
        #[case($udec!(13400476439814628800e2502), "13400476439814628800e+2502")]
        fn test_fmt_options_default(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(1), "1")]
        #[case($udec!(10), "10")]
        #[case($udec!(0.1), "0")]
        #[case($udec!(0.9), "1")]
        #[case($udec!(800e-3), "1")]
        #[case($udec!(19073.97235939614856), "19074")]
        #[case($udec!(1e15), "1000000000000000")]
        #[case($udec!(1e16), "10000000000000000")]
        #[case($udec!(491326e-12), "5E-7")]
        #[case($udec!(0.00003102564500), "0")]
        fn test_fmt_options_d0(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.0}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(9999999), "1e+7")]
        #[case($udec!(0.00003102564500), "3e-5")]
        fn test_fmt_options_d0e(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.0e}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(1), "1.0")]
        #[case($udec!(10), "10.0")]
        #[case($udec!(0.1), "0.1")]
        #[case($udec!(0.9), "0.9")]
        #[case($udec!(800e-3), "0.8")]
        #[case($udec!(123456), "123456.0")]
        #[case($udec!(19073.97235939614856), "19074.0")]
        #[case($udec!(1764031078e-13), "0.0")]
        #[case($udec!(1e15), "1000000000000000.0")]
        #[case($udec!(491326e-12), "4.9E-7")]
        #[case($udec!(1E-10000), "1.0E-10000")]
        #[case($udec!(1e100000), "1e+100000")]
        #[case($udec!(1234506789e5), "123450678900000.0")]
        #[case($udec!(1234506789e15), "1234506789000000000000000.0")]
        #[case($udec!(13400476439814628800e2502), "13400476439814628800e+2502")]
        fn test_fmt_options_d1(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.1}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(9999999), "1.0e+7")]
        #[case($udec!(0.00003102564500), "3.1e-5")]
        fn test_fmt_options_d1e(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.1e}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(1), "1.00")]
        #[case($udec!(10), "10.00")]
        #[case($udec!(0.1), "0.10")]
        #[case($udec!(1e16), "10000000000000000.00")]
        fn test_fmt_options_d2(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.2}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(9999999), "1.00e+7")]
        fn test_fmt_options_d2e(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.2e}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(800e-3), "0.800")]
        #[case($udec!(19073.97235939614856), "19073.972")]
        #[case($udec!(1764031078e-13), "0.000")]
        #[case($udec!(491326e-12), "4.913E-7")]
        #[case($udec!(1234506789e5), "123450678900000.000")]
        #[case($udec!(1234506789e15), "1234506789000000000000000.000")]
        fn test_fmt_options_d3(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.3}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(1), "1.0000")]
        #[case($udec!(0.9), "0.9000")]
        #[case($udec!(123456), "123456.0000")]
        #[case($udec!(19073.97235939614856), "19073.9724")]
        #[case($udec!(1764031078e-13), "0.0002")]
        #[case($udec!(0.00003102564500), "0.0000")]
        #[case($udec!(1E-10000), "1.0000E-10000")]
        #[case($udec!(1e100000), "1e+100000")]
        #[case($udec!(1234506789e5), "123450678900000.0000")]
        fn test_fmt_options_d4(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.4}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(9999999), "1.0000e+7")]
        #[case($udec!(0.00003102564500), "3.1026e-5")]
        fn test_fmt_options_d4e(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.4e}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(1764031078e-13), "0.00018")]
        #[case($udec!(491326e-12), "4.91326E-7")]
        #[case($udec!(0.00003102564500), "0.00003")]
        fn test_fmt_options_d5(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.5}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(491326e-12), "4.913260E-7")]
        fn test_fmt_options_d6(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.6}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(9999999), "9.999999e+6")]
        fn test_fmt_options_d6e(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.6e}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(9999999), "9.9999990e+6")]
        fn test_fmt_options_d7e(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.7e}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(9999999), "9999999.00000000")]
        fn test_fmt_options_d8(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.8}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(800e-3), "0.800000000")]
        #[case($udec!(491326e-12), "4.913260000E-7")]
        fn test_fmt_options_d9(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.9}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(0.00003102564500), "0.0000310256")]
        fn test_fmt_options_d10(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.10}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(9999999), "9.9999990000e+6")]
        fn test_fmt_options_d10e(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.10e}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(1764031078e-13), "0.0001764031078")]
        fn test_fmt_options_d13(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.13}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(0.00003102564500), "0.00003102564500")]
        fn test_fmt_options_d14(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.14}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(0.00003102564500), "0.00003102564500000")]
        fn test_fmt_options_d17(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.17}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(1764031078e-13), "0.00017640310780000000")]
        #[case($udec!(491326e-12), "4.91326000000000000000E-7")]
        fn test_fmt_options_d20(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.20}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(1), " 1.0")]
        #[case($udec!(123456), "123456.0")]
        fn test_fmt_options_4d1(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:4.1}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(19073.97235939614856), "19073.972")]
        fn test_fmt_options_8d3(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:8.3}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(19073.97235939614856), " 19073.972")]
        fn test_fmt_options_10d3(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:10.3}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(19073.97235939614856), "019073.972")]
        fn test_fmt_options_010d3(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:010.3}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(123456), "      123456.00")]
        fn test_fmt_options_15d2(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:15.2}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(1), " 1.0")]
        fn test_fmt_options_r4d1(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:>4.1}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(123456), "      123456.00")]
        fn test_fmt_options_r15d2(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:>15.2}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(1234506789e5), "   123450678900000.0000")]
        fn test_fmt_options_r23d4(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:>23.4}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(1), "1.0 ")]
        fn test_fmt_options_l4d1(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:<4.1}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(123456), "123456.00      ")]
        fn test_fmt_options_l15d2(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:<15.2}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(1234506789e5), "123450678900000.0000   ")]
        fn test_fmt_options_l23d4(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:<23.4}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(1), "+01.0")]
        fn test_fmt_options_p05d1(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:+05.1}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(123456), "+123456.0000000")]
        fn test_fmt_options_p05d7(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:+05.7}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(19073.97235939614856), "+19073.9723594")]
        fn test_fmt_options_pd7(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:+.7}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(1234506789e15), "+1234506789000000000000000.00   ")]
        fn test_fmt_options_l32d2(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:<+32.2}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(1234506789e15), "   +1234506789000000000000000.00")]
        fn test_fmt_options_r32d2(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:>+32.2}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(1), "1e+0")]
        #[case($udec!(9999999), "9.999999e+6")]
        #[case($udec!(0.00003102564500), "3.102564500e-5")]
        fn test_fmt_options_exp(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:e}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(0.00003102564500), "3.102564500e-5")]
        fn test_fmt_options_d_exp(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:.e}", d), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(1), "1E+0")]
        #[case($udec!(9999999), "9.999999E+6")]
        #[case($udec!(0.00003102564500), "3.102564500E-5")]
        fn test_fmt_options_exp_upper(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(format!("{:E}", d), expected);
        }

        #[rstest(::trace)]
        #[case(format!("1E{}", i32::MAX), "1e+2147483647")]
        #[case(format!("1E{}", i64::MAX), "1e+9223372036854775807")]
        fn test_fmt_boundaries(#[case] src: String, #[case] expected: &str) {
            let d: $UD = src.parse().unwrap();
            let result = d.to_string();
            assert_eq!(result, expected);

            let round_trip = $UD::from_str(&result).unwrap();
            assert_eq!(round_trip, d);

            let sci = d.to_scientific_notation();
            let sci_round_trip = $UD::from_str(&sci).unwrap();
            assert_eq!(sci_round_trip, d);

            let eng = d.to_engineering_notation();
            let eng_round_trip = $UD::from_str(&eng).unwrap();
            assert_eq!(eng_round_trip, d);
        }

        #[rstest(::trace)]
        #[case("1E-9223372036854775807".to_string(), "1E-9223372036854775807")]
        fn test_fmt_boundaries_invalid(#[case] src: String, #[case] expected: &str) {
            let d: $UD = src.parse().unwrap();
            let result = d.to_string();
            assert_eq!(result, expected);

            let round_trip = $UD::from_str(&result).unwrap();
            assert_eq!(round_trip, d);

            let sci = d.to_scientific_notation();
            let sci_round_trip = $UD::from_str(&sci).unwrap();
            assert_eq!(sci_round_trip, d);

            let eng = d.to_engineering_notation();
            let eng_round_trip = $UD::from_str(&eng);
            assert!(eng_round_trip.is_err());
        }

        #[rstest(::trace)]
        #[case($udec!(4159248078.2835), "4.1592480782835e9")]
        #[case($udec!(0.00001234), "1.234e-5")]
        #[case($udec!(0), "0e0")]
        #[case($udec!(1), "1e0")]
        #[case($udec!(2.00), "2.00e0")]
        fn test_fmt_scientific_notation(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(d.to_scientific_notation(), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(4159248078.2835), "4.1592480782835e9")]
        #[case($udec!(0.00001234), "12.34e-6")]
        #[case($udec!(0), "0e0")]
        #[case($udec!(1), "1e0")]
        #[case($udec!(2.00), "2.00e0")]
        #[case($udec!(5.31e4), "53.1e3")]
        #[case($udec!(5.31e5), "531e3")]
        #[case($udec!(5.31e6), "5.31e6")]
        #[case($udec!(5.31e7), "53.1e6")]
        #[case($udec!(1e2), "100e0")]
        #[case($udec!(1e19), "10e18")]
        #[case($udec!(1e3000), "1e3000")]
        #[case($udec!(4.2e7), "42e6")]
        #[case($udec!(4.2e8), "420e6")]
        #[case($udec!(4e99999999999999), "4e99999999999999")]
        #[case($udec!(4e99999999999998), "400e99999999999996")]
        #[case($udec!(44e99999999999998), "4.4e99999999999999")]
        #[case($udec!(4e99999999999997), "40e99999999999996")]
        #[case($udec!(41e99999999999997), "410e99999999999996")]
        #[case($udec!(413e99999999999997), "4.13e99999999999999")]
        fn test_fmt_engineering_notation(#[case] d: $UD, #[case] expected: &str) {
            assert_eq!(d.to_engineering_notation(), expected);
        }
    };
}

macro_rules! test_impl_unsigned {
    ($udec: ident, $UD: ident) => {
        #[rstest(::trace)]
        #[case($udec!(0), concat!(stringify!($UD), r#"(scale=0, digits=[0])"#))]
        #[case($udec!(0.00), concat!(stringify!($UD), r#"(scale=2, digits=[0])"#))]
        #[case($udec!(1), concat!(stringify!($UD), r#"(scale=0, digits=[1])"#))]
        #[case($udec!(123.400), concat!(stringify!($UD), r#"(scale=3, digits=[123400])"#))]
        #[case($udec!(123.4e-2), concat!(stringify!($UD), r#"(scale=3, digits=[1234])"#))]
        #[case($udec!(123.456), concat!(stringify!($UD), r#"(scale=3, digits=[123456])"#))]
        #[case($udec!(01.20), concat!(stringify!($UD), r#"(scale=2, digits=[120])"#))]
        #[case($udec!(1.20), concat!(stringify!($UD), r#"(scale=2, digits=[120])"#))]
        #[case($udec!(01.2E3), concat!(stringify!($UD), r#"(scale=-2, digits=[12])"#))]
        #[case($udec!(6.02214076e1023), concat!(stringify!($UD), r#"(scale=-1015, digits=[602214076])"#))]
        #[case($udec!(1e99999999999999), concat!(stringify!($UD), r#"(scale=-99999999999999, digits=[1])"#))]
        fn test_fmt_debug(#[case] d: $UD, #[case] expected: &str) {
            let formated = format!("{d:?}");
            assert_eq!(formated.as_str(), expected);
        }

        #[rstest(::trace)]
        #[case($udec!(0), concat!(stringify!($UD), r#"("0e0")"#))]
        #[case($udec!(1), concat!(stringify!($UD), r#"("1e0")"#))]
        #[case($udec!(123.400), concat!(stringify!($UD), r#"("123400e-3")"#))]
        #[case($udec!(123.4e-2), concat!(stringify!($UD), r#"("1234e-3")"#))]
        #[case($udec!(123.456), concat!(stringify!($UD), r#"("123456e-3")"#))]
        #[case($udec!(01.20), concat!(stringify!($UD), r#"("120e-2")"#))]
        #[case($udec!(1.20), concat!(stringify!($UD), r#"("120e-2")"#))]
        #[case($udec!(01.2E3), concat!(stringify!($UD), r#"("12e2")"#))]
        #[case($udec!(6.02214076e1023), concat!(stringify!($UD), r#"("602214076e1015")"#))]
        #[case($udec!(1e99999999999999), concat!(stringify!($UD), r#"("1e99999999999999")"#))]
        fn test_fmt_debug_alt_unsigned(#[case] d: $UD, #[case] expected: &str) {
            let formated = format!("{d:#?}");
            assert_eq!(formated.as_str(), expected);
        }
    }
}

macro_rules! test_impl_signed {
    ($dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(-0), "-0")]
        #[case($dec!(-0.00), "-0.00")]
        #[case($dec!(-123.e1), "-1230")]
        #[case($dec!(-90037659.6905), "-90037659.6905")]
        fn test_fmt_signed(#[case] d: $D, #[case] expected: &str) {
            let formated = format!("{d}");
            assert_eq!(formated.as_str(), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(-0), concat!(stringify!($D), r#"(sign = "-", scale=0, digits=[0])"#))]
        #[case($dec!(-1), concat!(stringify!($D), r#"(sign = "-", scale=0, digits=[1])"#))]
        #[case($dec!(-123.400), concat!(stringify!($D), r#"(sign = "-", scale=3, digits=[123400])"#))]
        #[case($dec!(+123.4e-2), concat!(stringify!($D), r#"(sign = "+", scale=3, digits=[1234])"#))]
        #[case($dec!(-123.456), concat!(stringify!($D), r#"(sign = "-", scale=3, digits=[123456])"#))]
        #[case($dec!(+01.20), concat!(stringify!($D), r#"(sign = "+", scale=2, digits=[120])"#))]
        #[case($dec!(-1.20), concat!(stringify!($D), r#"(sign = "-", scale=2, digits=[120])"#))]
        #[case($dec!(-01.2E3), concat!(stringify!($D), r#"(sign = "-", scale=-2, digits=[12])"#))]
        #[case($dec!(-6.02214076e1023), concat!(stringify!($D), r#"(sign = "-", scale=-1015, digits=[602214076])"#))]
        #[case($dec!(+1e99999999999999), concat!(stringify!($D), r#"(sign = "+", scale=-99999999999999, digits=[1])"#))]
        #[case($dec!(-144.3308279), concat!(stringify!($D), r#"(sign = "-", scale=7, digits=[1443308279])"#))]
        #[case($dec!(-349983058835858339619e2), concat!(stringify!($D), r#"(sign = "-", scale=-2, digits=[349983058835858339619])"#))]
        #[case($dec!(-90037659.6905), concat!(stringify!($D), r#"(sign = "-", scale=4, digits=[900376596905])"#))]
        fn test_fmt_debug(#[case] d: $D, #[case] expected: &str) {
            let formated = format!("{d:?}");
            assert_eq!(formated.as_str(), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(+0), concat!(stringify!($D), r#"("+0e0")"#))]
        #[case($dec!(-0), concat!(stringify!($D), r#"("-0e0")"#))]
        #[case($dec!(-1), concat!(stringify!($D), r#"("-1e0")"#))]
        #[case($dec!(-123.400), concat!(stringify!($D), r#"("-123400e-3")"#))]
        #[case($dec!(+123.4e-2), concat!(stringify!($D), r#"("+1234e-3")"#))]
        #[case($dec!(-123.456), concat!(stringify!($D), r#"("-123456e-3")"#))]
        #[case($dec!(-01.20), concat!(stringify!($D), r#"("-120e-2")"#))]
        #[case($dec!(-1.20), concat!(stringify!($D), r#"("-120e-2")"#))]
        #[case($dec!(-01.2E3), concat!(stringify!($D), r#"("-12e2")"#))]
        #[case($dec!(-6.02214076e1023), concat!(stringify!($D), r#"("-602214076e1015")"#))]
        #[case($dec!(+1e99999999999999), concat!(stringify!($D), r#"("+1e99999999999999")"#))]
        #[case($dec!(-1e99999999999999), concat!(stringify!($D), r#"("-1e99999999999999")"#))]
        #[case($dec!(-144.3308279), concat!(stringify!($D), r#"("-1443308279e-7")"#))]
        #[case($dec!(-349983058835858339619e2), concat!(stringify!($D), r#"("-349983058835858339619e2")"#))]
        #[case($dec!(-90037659.6905), concat!(stringify!($D), r#"("-900376596905e-4")"#))]
        fn test_fmt_debug_alt_signed(#[case] d: $D, #[case] expected: &str) {
            let formated = format!("{d:#?}");
            assert_eq!(formated.as_str(), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(-1),              "-1",    "-1.0",   "-1.0000",   "-1.0",   "-01.0",  "-1.0" )]
        #[case($dec!(-0.1),            "-0.1",  "-0.1",   "-0.1000",   "-0.1",   "-00.1",  "-0.1")]
        #[case($dec!(-0.01),           "-0.01", "-0.0",   "-0.0100",   "-0.0",   "-00.0",  "-0.0")]
        fn test_fmt_options_signed(#[case] d: $D,
                            #[case] expected: &str,
                            #[case] expected_d1: &str,
                            #[case] expected_d4: &str,
                            #[case] expected_4d1: &str,
                            #[case] expected_p05d1: &str,
                            #[case] expected_l4d1: &str,
        ) {
            assert_eq!(format!("{}", d), expected);
            assert_eq!(format!("{:.1}", d), expected_d1);
            assert_eq!(format!("{:.4}", d), expected_d4);
            assert_eq!(format!("{:4.1}", d), expected_4d1);
            assert_eq!(format!("{:+05.1}", d), expected_p05d1);
            assert_eq!(format!("{:<4.1}", d), expected_l4d1);
        }

        #[rstest(::trace)]
        fn test_fmt_options_signed_n90037659d6905() {
            let d = $dec!(-90037659.6905);

            assert_eq!(format!("{:+.7}", d), "-90037659.6905000");
            assert_eq!(format!("{:.0}", d), "-90037660");
            assert_eq!(format!("{:.3}", d), "-90037659.690");
            assert_eq!(format!("{:.4}", d), "-90037659.6905");
            assert_eq!(format!("{:14.4}", d), "-90037659.6905");
            assert_eq!(format!("{:15.4}", d), " -90037659.6905");
            assert_eq!(format!("{:<17.5}", d), "-90037659.69050  ");
        }

        #[rstest(::trace)]
        #[case($dec!(-57.0), "-5.70e1")]
        #[case($dec!(-0.00001234), "-1.234e-5")]
        #[case($dec!(-0), "-0e0")]
        #[case($dec!(-1), "-1e0")]
        #[case($dec!(-2.00), "-2.00e0")]
        fn test_fmt_scientific_notation_signed(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(d.to_scientific_notation(), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(-57.0), "-57.0e0")]
        #[case($dec!(-0), "-0e0")]
        #[case($dec!(-1), "-1e0")]
        #[case($dec!(-2.00), "-2.00e0")]
        #[case($dec!(-5.31e4), "-53.1e3")]
        #[case($dec!(-413e99999999999997), "-4.13e99999999999999")]
        fn test_fmt_engineering_notation_signed(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(d.to_engineering_notation(), expected);
        }
    };
}

pub(crate) use test_impl;
pub(crate) use test_impl_signed;
pub(crate) use test_impl_unsigned;
