use core::ops::{Div, DivAssign};

use crate::decimal::{impls::ops::ops_impl, Decimal};

ops_impl!(Decimal, I, Div, div, DivAssign, div_assign);
