use rstest::*;

use serde_test::{assert_tokens, Token};

use fastnum::{udec256, UD256};

use crate::decimal::common::extras::serde::{
    test_from_int_impl, test_json_impl, test_str_impl, test_try_from_float_impl,
    test_try_from_int_impl,
};

test_str_impl!(UD256, udec256);

#[rstest(::trace)]
#[case(udec256!(115.792089237316195423570985008687907853269984665640564039457584007913129639935), "115.792089237316195423570985008687907853269984665640564039457584007913129639935")]
fn test_serialize_deserialize_str_256(#[case] dec: UD256, #[case] expected: &'static str) {
    let expected = Token::Str(expected);
    assert_tokens(&dec, &[expected]);
}

test_from_int_impl!(UD256, test_deserialize, U8, U16, U32, U64);
test_try_from_int_impl!(UD256, test_deserialize, I8, I16, I32, I64);
test_try_from_float_impl!(UD256, test_deserialize, F32, F64);
test_json_impl!(UD256);
