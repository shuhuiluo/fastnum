use core::ops::{Div, DivAssign};

use crate::bint::{impls::ops::div::div_impl, Int};

div_impl!(Int, I);
