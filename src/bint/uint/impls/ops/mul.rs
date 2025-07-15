use core::ops::{Mul, MulAssign};

use crate::bint::{impls::ops::mul::mul_impl, UInt};

mul_impl!(UInt, U);
