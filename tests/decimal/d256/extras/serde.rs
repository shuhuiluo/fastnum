use fastnum::{dec256, D256};

use crate::decimal::common::extras::serde::{
    test_from_int_impl, test_json_impl, test_str_impl, test_try_from_float_impl,
};

test_str_impl!(D256, dec256);
test_from_int_impl!(D256, test_deserialize, U8, U16, U32, U64, I8, I16, I32, I64);
test_try_from_float_impl!(D256, test_deserialize, F32, F64);
test_json_impl!(D256);
