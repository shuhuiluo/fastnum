use rstest::*;

use serde_test::{assert_tokens, Token};

use fastnum::{udec512, UD512};

use crate::decimal::common::extras::serde::{
    test_from_int_impl, test_json_impl, test_str_impl, test_try_from_float_impl,
    test_try_from_int_impl,
};

test_str_impl!(UD512, udec512);

#[rstest(::trace)]
#[case(udec512!(115.792089237316195423570985008687907853269984665640564039457584007913129639935), "115.792089237316195423570985008687907853269984665640564039457584007913129639935")]
#[case(udec512!(1.3407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095), "1.3407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095")]
fn test_serialize_deserialize_str_512(#[case] dec: UD512, #[case] expected: &'static str) {
    let expected = Token::Str(expected);
    assert_tokens(&dec, &[expected]);
}

test_from_int_impl!(UD512, test_deserialize, U8, U16, U32, U64);
test_try_from_int_impl!(UD512, test_deserialize, I8, I16, I32, I64);
test_try_from_float_impl!(UD512, test_deserialize, F32, F64);
test_json_impl!(UD512);
