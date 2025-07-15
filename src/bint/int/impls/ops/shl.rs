use core::ops::{Shl, ShlAssign};

use crate::bint::{impls::ops::shift::shift_impl, intrinsics::ExpType, Int};

shift_impl!(Int, I, Shl, shl, ShlAssign, shl_assign);