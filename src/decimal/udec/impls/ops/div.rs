use core::ops::{Div, DivAssign};

use crate::decimal::{impls::ops::ops_impl, UnsignedDecimal};

ops_impl!(UnsignedDecimal, U, Div, div, DivAssign, div_assign);
