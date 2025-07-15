use core::ops::{Add, AddAssign};

use crate::decimal::{impls::ops::ops_impl, UnsignedDecimal};

ops_impl!(UnsignedDecimal, U, Add, add, AddAssign, add_assign);
