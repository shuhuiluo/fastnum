use fastnum::{udec128, UD128};

use crate::decimal::common::extras::serde::{
    test_from_int_impl, test_json_impl, test_str_impl, test_try_from_float_impl,
    test_try_from_int_impl,
};

test_str_impl!(UD128, udec128);
test_from_int_impl!(UD128, test_deserialize, U8, U16, U32, U64);
test_try_from_int_impl!(UD128, test_deserialize, I8, I16, I32, I64);
test_try_from_float_impl!(UD128, test_deserialize, F32, F64);
test_json_impl!(UD128);
