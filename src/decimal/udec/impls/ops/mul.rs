use core::ops::{Mul, MulAssign};

use crate::decimal::{impls::ops::ops_impl, UnsignedDecimal};

ops_impl!(UnsignedDecimal, U, Mul, mul, MulAssign, mul_assign);
