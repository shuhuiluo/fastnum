macro_rules! test_str_impl {
    ($DEC: ident, $dec: ident) => {
        #[rstest::rstest(::trace)]
        #[case($dec!(1.0), "1.0")]
        #[case($dec!(0.5), "0.5")]
        #[case($dec!(50.), "50")]
        #[case($dec!(50000), "50000")]
        #[case($dec!(1e-3), "0.001")]
        #[case($dec!(10e11), "1000000000000")]
        #[case($dec!(0.25), "0.25")]
        #[case($dec!(12.34e1), "123.4")]
        #[case($dec!(40.0010), "40.0010")]
        fn test_serialize_deserialize_str(#[case] dec: $DEC, #[case] expected: &'static str) {
            let expected = serde_test::Token::Str(expected);
            serde_test::assert_tokens(&dec, &[expected]);
        }
    };
}

macro_rules! test_from_int_impl {
    ($DEC: ident, $name: ident, $($ttype:ident),+) => {
        $( paste::paste! { test_from_int_impl!($DEC : [< $name _ $ttype:lower >] : $ttype : [<$ttype:lower>]); } )*
    };
    ($DEC: ident : $name:ident : $tt:ident : $t:ident) => {
        #[rstest::rstest(::trace)]
        #[case(0)]
        #[case(1)]
        #[case($t::MIN)]
        #[case($t::MAX)]
        fn $name(#[case] int: $t) {
            use fastnum::decimal::extras::serde::DeserializeMode;
            let tokens = [serde_test::Token::$tt(int)];

            match DeserializeMode::default() {
                DeserializeMode::Strict => {
                    let err = format!("invalid type: integer `{}`, expected formatted decimal string in strict mode", int);
                    serde_test::assert_de_tokens_error::<$DEC>(&tokens, &err);
                }
                DeserializeMode::Stringify | DeserializeMode::Any => {
                    let expected = $DEC::from(int);
                    serde_test::assert_de_tokens(&expected, &tokens);
                }
            }
        }
    };
}

macro_rules! test_try_from_int_impl {
    ($DEC: ident, $name: ident, $($ttype:ident),+) => {
        $( paste::paste! { test_try_from_int_impl!($DEC : [< $name _ $ttype:lower >] : $ttype : [<$ttype:lower>]); } )*
    };
    ($DEC: ident : $name:ident : $tt:ident : $t:ident) => {
        #[rstest::rstest(::trace)]
        #[case(0)]
        #[case(1)]
        #[case(10)]
        #[case(100)]
        fn $name(#[case] int: $t) {
            use fastnum::decimal::extras::serde::DeserializeMode;
            let tokens = [serde_test::Token::$tt(int)];

            match DeserializeMode::default() {
                DeserializeMode::Strict => {
                    let err = format!("invalid type: integer `{}`, expected formatted decimal string in strict mode", int);
                    serde_test::assert_de_tokens_error::<$DEC>(&tokens, &err);
                }
                DeserializeMode::Stringify | DeserializeMode::Any => {
                    let expected = $DEC::try_from(int).unwrap();
                    serde_test::assert_de_tokens(&expected, &tokens);
                }
            }
        }
    };
}

macro_rules! test_try_from_float_impl {
    ($DEC: ident, $name: ident, $($ttype:ident),+) => {
        $( paste::paste! {
            #[rstest::rstest(::trace)]
            #[case(0.)]
            #[case(1.)]
            #[case(10.)]
            #[case(100.)]
            #[case(1.2)]
            fn [< $name _ $ttype:lower >](#[case] n: [<$ttype:lower>]) {
                use fastnum::decimal::extras::serde::DeserializeMode;
                use crate::decimal::common::extras::serde::WithDecimalPoint;

                let tokens = [serde_test::Token::$ttype(n)];

                match DeserializeMode::default() {
                    DeserializeMode::Strict => {
                        let err = format!("invalid type: floating point `{}`, expected formatted decimal string in strict mode", WithDecimalPoint(n as f64));
                        serde_test::assert_de_tokens_error::<$DEC>(&tokens, &err);
                    }
                    DeserializeMode::Stringify | DeserializeMode::Any => {
                        let expected = $DEC::try_from(n).unwrap();
                        serde_test::assert_de_tokens(&expected, &tokens);
                    }
                }
            }

            #[rstest::rstest(::trace)]
            fn [< $name _ $ttype:lower _ nan >]() {
                use fastnum::decimal::extras::serde::DeserializeMode;
                let tokens = [ serde_test::Token::$ttype([<$ttype:lower>]::NAN) ];

                match DeserializeMode::default() {
                    DeserializeMode::Strict => {
                        let err = "invalid type: floating point `NaN`, expected formatted decimal string in strict mode";
                        serde_test::assert_de_tokens_error::<$DEC>(&tokens, err);
                    }
                    DeserializeMode::Stringify | DeserializeMode::Any => {
                        serde_test::assert_de_tokens_error::<$DEC>(&tokens, "(fastnum) number is NaN");
                    }
                }
            }
        } )*
    };
}

macro_rules! test_json_impl {
    ($DEC: ident) => {
        #[rstest::rstest(::trace)]
        #[case("1")]
        #[case("2")]
        #[case("3")]
        #[case("1.0")]
        #[case("12.34")]
        #[case("0.1")]
        #[case("0.01")]
        #[case("0.001")]
        #[case("0.0001")]
        fn test_json(#[case] num: &'static str) {
            use serde::{Deserialize, Serialize};

            use fastnum::decimal::extras::serde::DeserializeMode;

            #[derive(Serialize, Deserialize)]
            struct TestStruct {
                name: String,
                value: $DEC,
            }

            let d: $DEC = num.parse().unwrap();

            let json_src = format!("{{\"name\":\"foo\",\"value\":\"{}\"}}", num);

            let my_struct: TestStruct = serde_json::from_str(&json_src).unwrap();
            assert_eq!(&my_struct.name, "foo");
            assert_eq!(my_struct.value, d);

            let s = serde_json::to_string(&my_struct).unwrap();
            assert_eq!(s, json_src);

            let json = format!("{{\"name\":\"foo\",\"value\":{}}}", num);

            match DeserializeMode::default() {
                DeserializeMode::Strict => {
                    assert!(serde_json::from_str::<$DEC>(&json).is_err());
                }
                DeserializeMode::Stringify | DeserializeMode::Any => {
                    let my_struct: TestStruct = serde_json::from_str(&json).unwrap();
                    assert_eq!(&my_struct.name, "foo");
                    assert_eq!(my_struct.value, d);

                    let s = serde_json::to_string(&my_struct).unwrap();
                    assert_eq!(s, json_src);
                }
            }
        }
    };
}

pub(crate) use test_from_int_impl;
pub(crate) use test_json_impl;
pub(crate) use test_str_impl;
pub(crate) use test_try_from_float_impl;
pub(crate) use test_try_from_int_impl;

use core::fmt;
use core::fmt::{Display, Write};

pub(crate) struct WithDecimalPoint(pub f64);

impl Display for WithDecimalPoint {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        struct LookForDecimalPoint<'f, 'a> {
            formatter: &'f mut fmt::Formatter<'a>,
            has_decimal_point: bool,
        }

        impl<'f, 'a> fmt::Write for LookForDecimalPoint<'f, 'a> {
            fn write_str(&mut self, fragment: &str) -> fmt::Result {
                self.has_decimal_point |= fragment.contains('.');
                self.formatter.write_str(fragment)
            }

            fn write_char(&mut self, ch: char) -> fmt::Result {
                self.has_decimal_point |= ch == '.';
                self.formatter.write_char(ch)
            }
        }

        if self.0.is_finite() {
            let mut writer = LookForDecimalPoint {
                formatter,
                has_decimal_point: false,
            };
            write!(writer, "{}", self.0)?;
            if !writer.has_decimal_point {
                formatter.write_str(".0")?;
            }
        } else {
            write!(formatter, "{}", self.0)?;
        }
        Ok(())
    }
}
