use core::ops::{Add, AddAssign};

use crate::bint::{impls::ops::add::add_impl, UInt};

add_impl!(UInt, U);
