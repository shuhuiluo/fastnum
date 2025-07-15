use core::ops::{Mul, MulAssign};

use crate::decimal::{impls::ops::ops_impl, Decimal};

ops_impl!(Decimal, I, Mul, mul, MulAssign, mul_assign);
