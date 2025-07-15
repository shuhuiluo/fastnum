use core::ops::{Sub, SubAssign};

use crate::decimal::{impls::ops::ops_impl, UnsignedDecimal};

ops_impl!(UnsignedDecimal, U, Sub, sub, SubAssign, sub_assign);