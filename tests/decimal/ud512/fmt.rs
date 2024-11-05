use rstest::*;
use std::str::FromStr;

use fastnum::{udec512, UD512};

use crate::decimal::common::fmt::{test_impl, test_impl_unsigned};

test_impl!(udec512, UD512);
test_impl_unsigned!(udec512, UD512);
