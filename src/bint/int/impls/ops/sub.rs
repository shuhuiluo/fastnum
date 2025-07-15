use core::ops::{Sub, SubAssign};

use crate::bint::{impls::ops::sub::sub_impl, Int};

sub_impl!(Int, I);
