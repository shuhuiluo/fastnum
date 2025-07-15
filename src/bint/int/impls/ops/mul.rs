use core::ops::{Mul, MulAssign};

use crate::bint::{impls::ops::mul::mul_impl, Int};

mul_impl!(Int, I);
