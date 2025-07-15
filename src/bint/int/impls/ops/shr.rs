use core::ops::{Shr, ShrAssign};

use crate::bint::{impls::ops::shift::shift_impl, intrinsics::ExpType, Int};

shift_impl!(Int, I, Shr, shr, ShrAssign, shr_assign);
