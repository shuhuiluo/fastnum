use rstest::*;
use std::str::FromStr;

use fastnum::{udec128, UD128};

use crate::decimal::common::fmt::{test_impl, test_impl_unsigned};

test_impl!(udec128, UD128);
test_impl_unsigned!(udec128, UD128);
