use core::ops::{Div, DivAssign};

use crate::bint::{impls::ops::div::div_impl, UInt};

div_impl!(UInt, U);