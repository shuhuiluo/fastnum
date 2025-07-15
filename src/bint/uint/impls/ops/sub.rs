use core::ops::{Sub, SubAssign};

use crate::bint::{impls::ops::sub::sub_impl, UInt};

sub_impl!(UInt, U);