use rstest::*;

use fastnum::{udec256, UD256};

use crate::decimal::common::fmt::{test_impl, test_impl_unsigned};

test_impl!(udec256, UD256);
test_impl_unsigned!(udec256, UD256);
