use core::ops::{Add, AddAssign};

use crate::bint::{impls::ops::add::add_impl, Int};

add_impl!(Int, I);
